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
}
