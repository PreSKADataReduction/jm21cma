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
## calculate the jones matrix (not yet validated, use with care)
The parameters are self-explained as
```bash
cargo run --bin calc_21cma_jones.rs --release -- --theta-min 0 --theta-max 90 --ntheta 10 --phi-min 0 --phi-max 360 --nphi 37 --freq-min 50 --freq-max 100 --nfreq 10 --zenith0 48 --az0 -90 --array 21cma.yaml --dipole-len 1 --out a.fits
```

## calculate array beam pattern (Stokes I) centered on some certain direction
1. calculate single antenna beam:

```bash
./scripts/calc_single_ant_beam.sh 32 beam 101 102 103
```

2. calcualte the array beam pattern
```bash
./scripts/calc_array_beam_patch.sh output_beam.fits beam_101.fits beam_102.fits beam_103.fits
```
