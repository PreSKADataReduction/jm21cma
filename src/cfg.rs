use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayCfg {
    pub ants: Vec<AntCfg>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AntCfg {
    pub pos: (f64, f64, f64),
}
