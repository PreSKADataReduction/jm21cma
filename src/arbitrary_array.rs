use std::f64::consts::PI;

use num::complex::Complex;

use scorus::coordinates::{Vec3d};

use crate::utils::angle2vec;

#[allow(clippy::too_many_arguments)]
pub fn calc_array_beam1(
    pointing: &Vec3d<f64>,
    x_list: &[f64],
    y_list: &[f64],
    z_list: &[f64],
    w_list: &[f64],
    phi_list: &[f64],
    lambda: f64,
) -> Complex<f64> {
    x_list
        .iter()
        .zip(
            y_list
                .iter()
                .zip(z_list.iter().zip(w_list.iter().zip(phi_list.iter()))),
        )
        .map(|(&x, (&y, (&z, (&w, &phi))))| {
            let dl = pointing[0] * x + pointing[1] * y + pointing[2] * z;
            let phase = dl / lambda * 2.0 * PI;

            Complex::from_polar(w, phase - phi)
        })
        .sum::<Complex<f64>>()
}

pub fn calc_phase_from_pointing(
    x_list: &[f64],
    y_list: &[f64],
    z_list: &[f64],
    az_from_east: f64,
    zenith: f64,
    lambda: f64,
) -> Vec<f64> {
    let dir = angle2vec(az_from_east, zenith);
    x_list
        .iter()
        .zip(y_list.iter().zip(z_list.iter()))
        .map(|(&x, (&y, &z))| {
            let dl = dir[0] * x + dir[1] * y + dir[2] * z;
            dl / lambda * 2.0 * PI
        })
        .collect()
}
