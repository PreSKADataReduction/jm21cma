use fitsio::{
    images::{ImageDescription, ImageType},
    FitsFile,
};

use jm21cma::{
    arbitrary_array::{calc_array_beam1, calc_phase_from_pointing},
    cfg::ArrayCfg,
    constants::LIGHT_SPEED as C,
    single_ant_model::SingleAnt,
};

use ndarray::{s, Array3, ArrayView3};

use scorus::coordinates::{SphCoord, Vec3d};
use serde_yaml::from_reader;

use std::fs::{remove_file, File};

use clap::Parser;

const PROJ: &str = "SIN";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'z', long = "zenith0", value_name = "phase center zenith")]
    zenith0: f64,

    #[clap(
        short = 'a',
        long = "az0",
        allow_hyphen_values = true,
        value_name = "phase center az, east=0, north=90"
    )]
    az0: f64,

    #[clap(short = 'c', long = "cfg", value_name = "array_cfg.yaml")]
    cfg: String,

    #[clap(short = 'A', long = "ant_beam", num_args(1..),  value_name = "ant beam in healpix")]
    ant_beam_name: Vec<String>,

    #[clap(short = 'w', long = "fov_width", value_name = "fov_width in deg")]
    fov_w_deg: f64,

    #[clap(short = 'p', long = "fov_pix", value_name = "width in npix")]
    fovw_pix: usize,

    #[clap(short = 'o', long = "out", value_name = "out prefix")]
    out_prefix: String,
}

//use scorus::{
//    coordinates::Vec3d
//};

fn write_fits(
    fname: &str,
    efield_pattern: ArrayView3<f64>,
    fov_w_deg: f64,
    freq0: f64,
    dfreq: f64,
) {
    let nfreq = efield_pattern.shape()[0];
    let fovw_pix = efield_pattern.shape()[1];

    let image_description = ImageDescription {
        data_type: ImageType::Double,
        dimensions: &[nfreq, fovw_pix, fovw_pix],
    };
    let _ = remove_file(fname);
    let mut output_fits = FitsFile::create(fname)
        .with_custom_primary(&image_description)
        .open()
        .unwrap();

    let hdu = output_fits.primary_hdu().unwrap();
    hdu.write_image(&mut output_fits, efield_pattern.as_slice().unwrap())
        .unwrap();
    hdu.write_key(&mut output_fits, "CTYPE1", "X").unwrap();
    hdu.write_key(&mut output_fits, "CRPIX1", fovw_pix as u32 / 2)
        .unwrap();
    hdu.write_key(&mut output_fits, "CDELT1", fov_w_deg / fovw_pix as f64)
        .unwrap();
    hdu.write_key(&mut output_fits, "CRVAL1", 0.0).unwrap();
    hdu.write_key(&mut output_fits, "CUNIT1", "deg").unwrap();

    hdu.write_key(&mut output_fits, "CTYPE2", "Y").unwrap();
    hdu.write_key(&mut output_fits, "CRPIX2", fovw_pix as u32 / 2)
        .unwrap();
    hdu.write_key(&mut output_fits, "CDELT2", fov_w_deg / fovw_pix as f64)
        .unwrap();
    hdu.write_key(&mut output_fits, "CRVAL2", 0.0).unwrap();
    hdu.write_key(&mut output_fits, "CUNIT2", "deg").unwrap();

    hdu.write_key(&mut output_fits, "CTYPE3", "FREQ").unwrap();
    hdu.write_key(&mut output_fits, "CRPIX3", 1).unwrap();
    hdu.write_key(&mut output_fits, "CRVAL3", freq0).unwrap();
    hdu.write_key(&mut output_fits, "CDELT3", dfreq).unwrap();
    hdu.write_key(&mut output_fits, "CUNIT3", "Hz").unwrap();
}

fn main() {
    let args = Args::parse();

    let cfg: ArrayCfg = from_reader(File::open(&args.cfg).unwrap()).unwrap();

    let (ant_x, (ant_y, ant_z)): (Vec<f64>, (Vec<f64>, Vec<f64>)) = cfg
        .ants
        .iter()
        .map(|x| {
            let (x, y, z) = x.pos;
            (x, (y, z))
        })
        .unzip();

    let w_list: Vec<_> = ant_x.iter().map(|_| 1.0).collect();
    //let mut output_fits=FitsFile::create(out_fits_name).open().unwrap();

    //let phases:Vec<_>=ant_x.iter().map(|_| 0.0).collect();
    //let x=calc_array_beam1(&angle2vec(0.0_f64.to_radians(), 0.0_f64.to_radians()), &ant_x, &ant_y, &ant_z, &w_list, &phases, 1.0);
    //println!("{}", x);

    let mut efield_pattern =
        Array3::<f64>::zeros((args.ant_beam_name.len(), args.fovw_pix, args.fovw_pix));

    let efield_pattern0 =
        Array3::<f64>::zeros((args.ant_beam_name.len(), args.fovw_pix, args.fovw_pix));

    let az_from_east = -args.az0;

    //println!("{:?}", phases);

    let half_fov_pix = args.fovw_pix as isize / 2;
    let dx = args.fov_w_deg.to_radians() / args.fovw_pix as f64;
    let vc = Vec3d::from_angle(args.zenith0.to_radians(), args.az0.to_radians());
    let dirc = SphCoord::new(args.zenith0.to_radians(), args.az0.to_radians());
    let vx = dirc.vdaz() * -1.0;
    let vy = dirc.vdpol() * -1.0;
    let mut freq0 = 0.0;
    let mut dfreq = 0.0;
    for (ifreq, bn) in args.ant_beam_name.iter().enumerate() {
        let ant_beam = SingleAnt::from_fits(bn);
        let freq = ant_beam.freq_MHz * 1e6;
        println!("freq={} MHz", ant_beam.freq_MHz);
        if ifreq == 0 {
            freq0 = ant_beam.freq_MHz * 1e6;
        } else if ifreq == 1 {
            dfreq = ant_beam.freq_MHz * 1e6 - freq0;
        }

        let lambda = C / freq;
        let phases = calc_phase_from_pointing(
            &ant_x,
            &ant_y,
            &ant_z,
            az_from_east.to_radians(),
            args.zenith0.to_radians(),
            lambda,
        );

        let mut beam_max = 0.0;

        for iy in 0..args.fovw_pix {
            let y = -if PROJ == "SIN" {
                // matplotlib imshow convention
                ((iy as isize - half_fov_pix) as f64 * dx).sin()
            } else if PROJ == "TAN" {
                (iy as isize - half_fov_pix) as f64 * dx
            } else {
                panic!("invalid proj")
            };
            for ix in 0..args.fovw_pix {
                let x = if PROJ == "SIN" {
                    ((ix as isize - half_fov_pix) as f64 * dx).sin()
                } else if PROJ == "TAN" {
                    (ix as isize - half_fov_pix) as f64 * dx
                } else {
                    panic!("invalid proj")
                };

                let v = if PROJ == "SIN" {
                    let z = (1.0 - x * x - y * y).sqrt(); //sine proj
                    vc * z + vx * x + vy * y
                } else if PROJ == "TAN" {
                    vc + vx * x + vy * y
                } else {
                    panic!("invalid proj")
                }
                .normalized();
                if v.x.is_normal() && v.y.is_normal() && v.y.is_normal() {
                    let dir = SphCoord::from_vec3d(v);
                    let array_beam =
                        calc_array_beam1(&v, &ant_x, &ant_y, &ant_z, &w_list, &phases, lambda)
                            .norm_sqr();
                    let total_beam = ant_beam.power_pattern(dir.az, dir.pol) * array_beam;
                    efield_pattern[(ifreq, iy, ix)] = total_beam.sqrt();
                    if beam_max < total_beam {
                        beam_max = total_beam;
                    }
                } else {
                    efield_pattern[(ifreq, iy, ix)] = 0.0;
                }
                //efield_pattern[(ifreq,iy, ix)] = 1.0;
            }
        }

        efield_pattern
            .slice_mut(s![ifreq, .., ..])
            .iter_mut()
            .for_each(|x| {
                *x /= beam_max;
            });
    }

    for name in ["xx_re", "yy_re"] {
        write_fits(
            &format!("{}_{}.fits", args.out_prefix, name),
            efield_pattern.view(),
            args.fov_w_deg,
            freq0,
            dfreq,
        );
    }

    for name in ["xx_im", "yy_im", "xy_re", "yx_re", "xy_im", "yx_im"] {
        write_fits(
            &format!("{}_{}.fits", args.out_prefix, name),
            efield_pattern0.view(),
            args.fov_w_deg,
            freq0,
            dfreq,
        );
    }

    //println!("{:?}", result);
}
