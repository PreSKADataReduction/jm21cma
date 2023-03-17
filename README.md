# Environment building
## Install `rust` compilation environment
Install `rustup` with a software package manager， or directly install following [rustup.rs](https://rustup.rs), by running the command
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install rust compiler：
```
rustup default nightly
```

# clone this repository
```
git clone https://github.com/PreSKADataReduction/jm21cma
cd jm21cma
```

# Usage
The parameters are self-explained as
```
cargo run --bin calc_21cma_jones.rs --release -- --theta-min 0 --theta-max 90 --ntheta 10 --phi-min 0 --phi-max 360 --nphi 37 --freq-min 50 --freq-max 100 --nfreq 10 --zenith0 48 --az0 -90 --array 21cma.yaml --dipole-len 1 --out a.fits
```
