use scorus::coordinates::{SphCoord, Vec3d};

pub fn angle2vec(az_from_east: f64, zenith: f64) -> Vec3d<f64> {
    let az_from_x = -az_from_east;
    Vec3d::from_sph_coord(SphCoord::new(zenith, az_from_x))
}
