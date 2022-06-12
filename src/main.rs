use fitsio::{
    FitsFile
    , images::{
        ImageDescription
        , ImageType
    }
};

use jones21cma::{
    dipole::{
        x_dipole_jones
    }
    , cfg::{
        ArrayCfg
    }
    , arbitrary_array::{
        calc_phase_from_pointing
        , calc_array_beam1
    }
    , utils::{
        angle2vec
    }
    , constants::{
        LIGHT_SPEED as C
    }
};

use ndarray::{
    Array2
};

use serde_yaml::from_reader;

use std::{
    fs::{
        File
        , remove_file
    }
};

use clap::{
    Command,
    Arg
};

//use scorus::{
//    coordinates::Vec3d
//};

fn main() {
    let matches=Command::new("calc_21cma_jones_mat")
    .arg(
        Arg::new("theta_min")
        .long("theta-min")
        .takes_value(true)
        .required(true)
        .value_name("min theta")
    )
    .arg(
        Arg::new("theta_max")
        .long("theta-max")
        .takes_value(true)
        .required(true)
        .value_name("max theta")
    )
    .arg(
        Arg::new("ntheta")
        .long("ntheta")
        .takes_value(true)
        .required(true)
        .value_name("number of theta's")
    )
    .arg(
        Arg::new("phi_min")
        .long("phi-min")
        .takes_value(true)
        .required(true)
        .value_name("min phi")
        .help("East=0, South=90 deg")
    )
    .arg(
        Arg::new("phi_max")
        .long("phi-max")
        .takes_value(true)
        .required(true)
        .value_name("max phi")
        .help("East=0, South=90 deg")
    )
    .arg(
        Arg::new("nphi")
        .long("nphi")
        .takes_value(true)
        .required(true)
        .value_name("number of phi's")
    )
    .arg(
        Arg::new("freq_min")
        .long("freq-min")
        .takes_value(true)
        .required(true)
        .value_name("frequency in MHz")
        .help("min frequency in MHz")
    )
    .arg(
        Arg::new("freq_max")
        .long("freq-max")
        .takes_value(true)
        .required(true)
        .value_name("frequency in MHz")
        .help("max frequency in MHz")
    )
    .arg(
        Arg::new("nfreq")
        .long("nfreq")
        .takes_value(true)
        .required(true)
        .value_name("number of frequencies")
    )
    .arg(
        Arg::new("zenith0")
        .long("zenith0")
        .short('z')
        .takes_value(true)
        .required(true)
        .value_name("zenith of phase center in deg")
    )
    .arg(
        Arg::new("az0")
        .long("az0")
        .short('a')
        .takes_value(true)
        .required(true)
        .allow_hyphen_values(true)
        .value_name("az from east clock-wise in deg")
    )
    .arg(
        Arg::new("array_cfg")
        .long("array")
        .takes_value(true)
        .required(true)
        .value_name("cfg file name")
    )
    .arg(
        Arg::new("dipole_len")
        .long("dipole-len")
        .short('l')
        .takes_value(true)
        .required(true)
        .value_name("dipole len in m")
    )
    .arg(
        Arg::new("outfits")
        .long("out")
        .short('o')
        .takes_value(true)
        .required(true)
        .value_name("output fits file name")
    )
    .get_matches();

    let theta_max=matches.value_of("theta_max").unwrap().parse::<f64>().unwrap();
    let theta_min=matches.value_of("theta_min").unwrap().parse::<f64>().unwrap();
    let ntheta=matches.value_of("ntheta").unwrap().parse::<usize>().unwrap();
    let phi_max=matches.value_of("phi_max").unwrap().parse::<f64>().unwrap();
    let phi_min=matches.value_of("phi_min").unwrap().parse::<f64>().unwrap();
    let nphi=matches.value_of("nphi").unwrap().parse::<usize>().unwrap();
    let freq_max=matches.value_of("freq_max").unwrap().parse::<f64>().unwrap();
    let freq_min=matches.value_of("freq_min").unwrap().parse::<f64>().unwrap();
    let nfreq=matches.value_of("nfreq").unwrap().parse::<usize>().unwrap();
    let zenith0=matches.value_of("zenith0").unwrap().parse::<f64>().unwrap();
    let az0=matches.value_of("az0").unwrap().parse::<f64>().unwrap();
    let cfg_name=matches.value_of("array_cfg").unwrap();
    let dipole_len=matches.value_of("dipole_len").unwrap().parse::<f64>().unwrap();
    let out_fits_name=matches.value_of("outfits").unwrap();
    
    let cfg:ArrayCfg=from_reader(File::open(cfg_name).unwrap()).unwrap();

    let (ant_x, (ant_y, ant_z)):(Vec<f64>, (Vec<f64>, Vec<f64>))=cfg.ants.iter().map(|x| {
        let (x,y,z)=x.pos;
        (x, (y,z))
    }).unzip();

    

    let dfreq=(freq_max-freq_min)/(nfreq-1) as f64;
    let dtheta=(theta_max-theta_min)/(ntheta-1) as f64;
    let dphi=(phi_max-phi_min)/(nphi-1) as f64;

    let w_list:Vec<_>=ant_x.iter().map(|_| 1.0).collect();

    let mut buf=Array2::<f64>::zeros((ntheta*nphi, 10));

    let image_description = ImageDescription {
        data_type: ImageType::Double,
        dimensions: &[ntheta*nphi, 10],
    };
    let _=remove_file(out_fits_name);

    let mut output_fits=FitsFile::create(out_fits_name).with_custom_primary(&image_description).open().unwrap();
    //let mut output_fits=FitsFile::create(out_fits_name).open().unwrap();

    
    //let phases:Vec<_>=ant_x.iter().map(|_| 0.0).collect();
    //let x=calc_array_beam1(&angle2vec(0.0_f64.to_radians(), 0.0_f64.to_radians()), &ant_x, &ant_y, &ant_z, &w_list, &phases, 1.0);
    //println!("{}", x);

    for f_idx in 0..nfreq{
        println!("{}", f_idx);
        let freq=(freq_min+f_idx as f64*dfreq)*1e6;
        let lambda=C/freq;
        let phases=calc_phase_from_pointing(&ant_x, &ant_y, &ant_z, az0.to_radians(), zenith0.to_radians(), freq);
        //println!("{:?}", phases);
        for phi_idx in 0..nphi{
            let phi=phi_min+phi_idx as f64*dphi;
            for theta_idx in 0..ntheta{
                let row_idx=theta_idx+phi_idx*ntheta;
                let theta=theta_min+theta_idx as f64*dtheta;
                let pointing=angle2vec(phi, theta);
                let array_beam=calc_array_beam1(&pointing, &ant_x, &ant_y, &ant_z, &w_list, &phases, lambda);
                let dipole_jones=x_dipole_jones(-phi.to_radians(), theta.to_radians(), lambda, dipole_len);
                buf[(row_idx, 0)]=theta;
                buf[(row_idx, 1)]=phi;
                buf.row_mut(row_idx).as_slice_mut().unwrap().chunks_exact_mut(2).skip(1).zip(dipole_jones.iter()).for_each(|(a,&b)|{
                    let g=array_beam*b;
                    //let g=array_beam;
                    a[0]=g.re;
                    a[1]=g.im;
                });
            }
        }
        let image_description = ImageDescription {
            data_type: ImageType::Double,
            dimensions: &[ntheta*nphi, 10],
        };
        let hdu=if f_idx==0{
            output_fits.primary_hdu().unwrap()
        }else{
            output_fits.create_image(format!("freq{}", f_idx), &image_description).unwrap()
        };
        hdu.write_key(&mut output_fits, "FREQ", freq).unwrap();
        hdu.write_image(&mut output_fits, buf.as_slice().unwrap()).unwrap();
    }
    //let result=dipole::x_dipole_jones(90.0_f64.to_radians(), 45.0_f64.to_radians(), 1.0, 0.5);
    //println!("{:?}", result);
}
