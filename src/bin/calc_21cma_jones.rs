use fitsio::{
    images::{ImageDescription, ImageType},
    FitsFile,
};

use jm21cma::{
    arbitrary_array::{calc_array_beam1, calc_phase_from_pointing},
    cfg::ArrayCfg,
    constants::LIGHT_SPEED as C,
    dipole::lp_ant_jones,
    single_ant_model::SingleAnt,
    utils::angle2vec,
};

use ndarray::Array2;

use serde_yaml::from_reader;

use std::fs::{remove_file, File};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 't', long = "theta_min", value_name = "theta min")]
    theta_min: f64,

    #[clap(short = 'T', long = "theta_max", value_name = "theta max")]
    theta_max: f64,

    #[clap(short = 'n', long = "ntheta", value_name = "n theta")]
    ntheta: usize,

    #[clap(short = 'p', long = "phi_min",allow_hyphen_values = true, value_name = "phi_min")]
    phi_min: f64,

    #[clap(short = 'P', long = "phi_max", allow_hyphen_values = true, value_name = "phi_max")]
    phi_max: f64,

    #[clap(short = 'N', long = "nphi", value_name = "num of phi")]
    nphi: usize,

    #[clap(short = 'z', long = "zenith0", value_name = "phase center zenith")]
    zenith0: f64,

    #[clap(short = 'a', long = "az0", allow_hyphen_values = true, value_name = "phase center az")]
    az0: f64,

    #[clap(short = 'c', long = "cfg", value_name = "array_cfg.yaml")]
    cfg: String,

    #[clap(short = 'A', long = "antenna_beam", num_args(1..), value_name = "ant beam in healpix")]
    ant_beam_name: Vec<String>,

    #[clap(short = 'o', long = "out", value_name = "outfits")]
    outfile: String,
}

//use scorus::{
//    coordinates::Vec3d
//};

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

    //let dfreq=(args.freq_max-args.freq_min)/(args.nfreq-1) as f64;
    let dtheta = (args.theta_max - args.theta_min) / (args.ntheta - 1) as f64;
    let dphi = (args.phi_max - args.phi_min) / (args.nphi - 1) as f64;

    let w_list: Vec<_> = ant_x.iter().map(|_| 1.0).collect();

    let mut buf = Array2::<f64>::zeros((args.ntheta * args.nphi, 10));

    let image_description = ImageDescription {
        data_type: ImageType::Double,
        dimensions: &[args.ntheta * args.nphi, 10],
    };
    let _ = remove_file(&args.outfile);

    let mut output_fits = FitsFile::create(&args.outfile)
        .with_custom_primary(&image_description)
        .open()
        .unwrap();
    //let mut output_fits=FitsFile::create(out_fits_name).open().unwrap();

    //let phases:Vec<_>=ant_x.iter().map(|_| 0.0).collect();
    //let x=calc_array_beam1(&angle2vec(0.0_f64.to_radians(), 0.0_f64.to_radians()), &ant_x, &ant_y, &ant_z, &w_list, &phases, 1.0);
    //println!("{}", x);
    //let ant_beam=SingleAnt::from_fits(&args.ant_beam);

    //for f_idx in 0..args.nfreq{
    for (f_idx, bn) in args.ant_beam_name.iter().enumerate() {
        println!("{}", f_idx);
        let ant_beam = SingleAnt::from_fits(bn);
        let freq = ant_beam.freq_MHz * 1e6;
        println!("freq={} MHz", ant_beam.freq_MHz);

        //let freq = (args.freq_min + f_idx as f64 * dfreq) * 1e6;
        let lambda = C / freq;
        let phases = calc_phase_from_pointing(
            &ant_x,
            &ant_y,
            &ant_z,
            args.az0.to_radians(),
            args.zenith0.to_radians(),
            lambda,
        );
        //println!("{:?}", phases);
        for phi_idx in 0..args.nphi {
            let phi = args.phi_min + phi_idx as f64 * dphi;
            for theta_idx in 0..args.ntheta {
                let row_idx = theta_idx + phi_idx * args.ntheta;
                let theta = args.theta_min + theta_idx as f64 * dtheta;
                let pointing = angle2vec(phi.to_radians(), theta.to_radians());
                let array_beam =
                    calc_array_beam1(&pointing, &ant_x, &ant_y, &ant_z, &w_list, &phases, lambda);
                let ant_pattern = ant_beam.power_pattern(phi.to_radians(), theta.to_radians());
                let ant_jones = lp_ant_jones(phi.to_radians(), theta.to_radians(), ant_pattern);
                //let dipole_jones=x_dipole_jones(-phi.to_radians(), theta.to_radians(), lambda, dipole_len);
                buf[(row_idx, 0)] = theta;
                buf[(row_idx, 1)] = phi;
                buf.row_mut(row_idx)
                    .as_slice_mut()
                    .unwrap()
                    .chunks_exact_mut(2)
                    .skip(1)
                    .zip(ant_jones.iter())
                    .for_each(|(a, &b)| {
                        let g = array_beam * b;
                        //let g=array_beam;
                        a[0] = g.re;
                        a[1] = g.im;
                    });
            }
        }
        let image_description = ImageDescription {
            data_type: ImageType::Double,
            dimensions: &[args.ntheta * args.nphi, 10],
        };
        let hdu = if f_idx == 0 {
            output_fits.primary_hdu().unwrap()
        } else {
            output_fits
                .create_image(format!("freq{}", f_idx), &image_description)
                .unwrap()
        };
        hdu.write_key(&mut output_fits, "FREQ", freq).unwrap();
        hdu.write_image(&mut output_fits, buf.as_slice().unwrap())
            .unwrap();
    }
    //let result=dipole::x_dipole_jones(90.0_f64.to_radians(), 45.0_f64.to_radians(), 1.0, 0.5);
    //println!("{:?}", result);
}
