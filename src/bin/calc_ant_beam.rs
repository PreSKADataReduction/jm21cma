#![cfg(not(target_family = "wasm"))]
#![allow(non_snake_case)]
use std::fs::read_to_string;

use pest::Parser;

use num::traits::FloatConst;

use healpix_fits::write_map;

use scorus::{
    coordinates::SphCoord,
    healpix::{interp::get_interpol_ring, utils::nside2npix},
};

use necrs::nec_parser::{parse_nec_file, NecParser, Rule};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'n', long = "nec", value_name="nec file")]
    nec: String,

    #[clap(short = 's', long = "nside", value_name="nside")]
    nside: usize,

    #[clap(short = 'f', long = "freq_MHz", value_name="freq in MHz")]
    freq_MHz: f64,

    #[clap(short = 'o', long = "out", value_name="out file")]
    outfile: String,


}

pub fn main() {
    let args=<Args as clap::Parser>::parse();    

    let nec_file_name = &args.nec;
    let freq = args.freq_MHz;
    let nside = args.nside;
    let out_file_name = &args.outfile;

    let mut context = parse_nec_file(
        NecParser::parse(Rule::NecFile, &read_to_string(nec_file_name).unwrap())
            .unwrap()
            .next()
            .unwrap(),
    );

    context.nec_fr_card(0, 1, freq, 0.0);

    let npix = nside2npix(nside);
    let angular_resolution = (4.0 * f64::PI() / npix as f64).sqrt().to_degrees();
    println!("{}", angular_resolution);

    //context.nec_rp_card(0, ntheta as i32, nphi as i32, 1, 0, 0, 0, 0.0, 0.0, dtheta, dphi, 0.0, 0.0);
    let (thetas, phis) = context.rp_from_npix(npix * 4, 0, 1, 0, 0, 0, 0.0, 0.0);
    let mut data = vec![0.0; npix];
    let mut wgt = vec![0.0; npix];

    for (i, &theta) in thetas.iter().enumerate() {
        if theta > 90.0 {
            continue;
        }
        for (j, &phi) in phis.iter().enumerate() {
            let g = (context.nec_gain(0, i as i32, j as i32) / 10.0).exp();
            let dir = SphCoord::new(theta.to_radians(), phi.to_radians());
            let (pix, w) = get_interpol_ring(nside, dir);
            for (&p, &w) in pix.iter().zip(w.iter()) {
                wgt[p] += w;
                data[p] += w * g;
            }
        }
    }
    for (d, &w) in data.iter_mut().zip(wgt.iter()) {
        if w > 0.0 {
            *d /= w;
        }
    }

    let s=data.iter().cloned().sum::<f64>();
    data.iter_mut().for_each(|x| *x/=s);

    let (mut fitsfile, hdu)=write_map(out_file_name, &[&data], false, true);
    hdu.write_key(&mut fitsfile, "FREQ_MHZ", freq).unwrap();
}
