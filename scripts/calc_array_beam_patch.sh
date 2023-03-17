#!/usr/bin/env bash

SCRIPT_HOME=`dirname $0`
DATA_DIR=$SCRIPT_HOME/../data
echo $SCRIPT_HOME
echo $DATA_DIR
LAT=42.552673743
FOV_W=20
FOV_PIX=1024

if [ $# -lt 2 ]
then
    echo "Usage: $0 <outname> <beam f1> [beam f2]..."
    exit
fi


outname=$1
shift

cargo run --bin calc_21cma_beam_patch --release -- -z $LAT -a 90 -c data/21cma_station.yaml -A $@ -w $FOV_W -p $FOV_PIX -o $outname
