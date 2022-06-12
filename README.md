# Environment building
## Install `rust` compilation environment
Install `rustup` with software package manager， or directly install following [rustup.rs](https://rustup.rs), by running command
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install rust rust compiler：
```
rustup default nightly
```

# clone this repository
```
git clone https://github.com/PreSKADataReduction/jones21cma
cd jones21cma
```

# Usage
The parameters are self-explained as
```
cargo run -- --theta-min 0 --theta-max 90 --ntheta 10 --phi-min 0 --phi-max 360 --nphi 37 --freq-min 50 --freq-max 100 --nfreq 10 --zenith0 0 --az0 -90 --array 21cma.yaml --dipole-len 1 --out a.fits
```
