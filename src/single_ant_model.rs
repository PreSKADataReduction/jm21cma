use fitsio::FitsFile;
use healpix_fits::read_map;
use scorus::{
    coordinates::SphCoord,
    healpix::{interp::natural_interp_ring, npix2nside},
};
pub struct SingleAnt {
    pub data: Vec<f64>,
    pub nside: usize,
    pub freq_MHz: f64,
}

impl SingleAnt {
    pub fn new(data: Vec<f64>, freq_MHz: f64) -> Self {
        let nside = npix2nside(data.len());
        Self {
            data,
            nside,
            freq_MHz,
        }
    }

    pub fn from_fits(fname: &str) -> Self {
        let data = read_map::<f64>(fname, &["TEMPERATURE"], 1).pop().unwrap();
        let mut fitsfile = FitsFile::open(fname).unwrap();
        let hdu = fitsfile.hdu(1).unwrap();
        let freq_MHz=hdu.read_key::<f64>(&mut fitsfile, "FREQ_MHZ").unwrap();
        Self::new(data, freq_MHz)
    }

    pub fn power_pattern(&self, az: f64, pol: f64) -> f64 {
        natural_interp_ring(self.nside, &self.data, SphCoord::new(pol, az))
    }
}
