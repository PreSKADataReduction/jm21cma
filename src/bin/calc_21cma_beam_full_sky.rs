use healpix_fits::write_map;

use jm21cma::{
    arbitrary_array::{calc_array_beam1, calc_phase_from_pointing},
    cfg::ArrayCfg,
    constants::LIGHT_SPEED as C,
    single_ant_model::SingleAnt,
};

use scorus::{coordinates::Vec3d, healpix::pix2ang_ring};
use serde_yaml::from_reader;

use std::fs::File;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'f', long = "freq", value_name = "freq")]
    freq: f64,

    #[clap(short = 'z', long = "zenith0", value_name = "phase center zenith")]
    zenith0: f64,

    #[clap(
        short = 'a',
        long = "az0",
        value_name = "phase center az, east=0, north=90"
    )]
    az0: f64,

    #[clap(short = 'c', long = "cfg", value_name = "array_cfg.yaml")]
    cfg: String,

    #[clap(short = 'A', long = "ant_beam", value_name = "ant beam in healpix")]
    ant_beam_name: String,

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

    let w_list: Vec<_> = ant_x.iter().map(|_| 1.0).collect();
    //let mut output_fits=FitsFile::create(out_fits_name).open().unwrap();

    //let phases:Vec<_>=ant_x.iter().map(|_| 0.0).collect();
    //let x=calc_array_beam1(&angle2vec(0.0_f64.to_radians(), 0.0_f64.to_radians()), &ant_x, &ant_y, &ant_z, &w_list, &phases, 1.0);
    //println!("{}", x);
    let ant_beam = SingleAnt::from_fits(&args.ant_beam_name);

    let freq = args.freq * 1e6;
    let lambda = C / freq;
    let az_from_east = -args.az0;
    let phases = calc_phase_from_pointing(
        &ant_x,
        &ant_y,
        &ant_z,
        az_from_east.to_radians(),
        args.zenith0.to_radians(),
        lambda,
    );
    //println!("{:?}", phases);

    let nside = ant_beam.nside;
    let total_power_beam: Vec<_> = ant_beam
        .data
        .iter()
        .enumerate()
        .map(|(ipix, &ant_pattern)| {
            let ptg = pix2ang_ring::<f64>(nside, ipix);
            let ptg = Vec3d::from_sph_coord(ptg);
            let array_beam =
                calc_array_beam1(&ptg, &ant_x, &ant_y, &ant_z, &w_list, &phases, lambda).norm_sqr();
            ant_pattern * array_beam
        })
        .collect();
    write_map(&args.outfile, &[&total_power_beam], false, true);
    //println!("{:?}", result);
}
