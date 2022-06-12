use num::complex::Complex;
use scorus::coordinates::{SphCoord,Vec3d};
use std::f64::consts::PI;

pub fn z_dipole_E(theta:f64, lambda:f64, L:f64)->f64{
    if theta.sin()==0.0{
        0.0
    }else{
        let pi=PI;
        let k=2.0*pi/lambda;
        if theta==0.0{
            0.0
        }else{
            ((k*L/2.0*theta.cos()).cos()-(k*L/2.0).cos())/theta.sin()
        }
        
    }
}

pub fn x_dipole_E(az_from_x:f64, pol:f64, lambda:f64, L:f64)->(f64,f64){
    /*
    rotation mat:
    |0 1 0|
    |0 0 1|
    |1 0 0|
    inv:
    |0 0 1|
    |1 0 0|
    |0 1 0|
     */
    let Vec3d{x: dir_z,y:dir_x,z:dir_y}=Vec3d::from(SphCoord::new(pol, az_from_x));
    let sph_new=SphCoord::<f64>::from_xyz(dir_x, dir_y, dir_z);
    //println!("{} {} {}", dir_x, dir_y, dir_z);
    //println!("{}", sph.pol.to_degrees());
    let Vec3d{x,y,z}=sph_new.vdpol();
    //println!("{} {} {}",x,y,z);
    let e_theta=z_dipole_E(sph_new.pol, lambda, L);
    //println!("{}",e_theta);
    let e_xyz=Vec3d{x: z*e_theta, y: x*e_theta, z:y*e_theta};
    let sph=SphCoord::<f64>::new(pol, az_from_x);
    let vpol=sph.vdpol();
    let vaz=sph.vdaz();
    let e_theta=vpol.dot(e_xyz);
    let e_phi=vaz.dot(e_xyz);
    (e_theta, e_phi)
}


pub fn x_dipole_jones(az_from_x: f64, pol: f64, lambda: f64, L: f64)->[Complex<f64>; 4]{
    let (j_xt, j_xp)=x_dipole_E(az_from_x, pol, lambda, L);
    let (j_yt, j_yp)=(j_xp, -j_xt);
    [Complex::from(j_xt), Complex::from(j_xp), Complex::from(j_yt), Complex::from(j_yp)]
}
