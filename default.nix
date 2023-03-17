# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "mpi_rust"; # Probably put a more meaningful name here
    buildInputs = [clang
    llvmPackages.libclang.lib
    lapack
    gcc
    blas
    cfitsio
    pkg-config
    libtool
    gfortran
    (lib.getLib gfortran.cc)
    automake
    autoconf
    ];
    hardeningDisable = [ "all" ];
    #buildInputs = [gcc-unwrapped gcc-unwrapped.out gcc-unwrapped.lib];
    LIBCLANG_PATH = llvmPackages.libclang.lib+"/lib";
    LD_LIBRARY_PATH= libGL+"/lib";
    QT_QPA_PLATFORM="wayland";
}
