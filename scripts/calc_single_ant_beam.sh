#!/usr/bin/env bash

SCRIPT_HOME=`dirname $0`
DATA_DIR=$SCRIPT_HOME/../data
echo $SCRIPT_HOME
echo $DATA_DIR

if [ $# -lt 3 ]
then
    echo "Usage: $0 <nside> <out prefix> [f1 in MHz] [f2 in MHz]..."
    exit
fi


nside=$1
shift
prefix=$1
shift
for freq in $@
do
    echo $freq
    cargo run --bin calc_ant_beam --release -- -n $DATA_DIR/21cma_lp.nec -s $nside -f $freq -o ${prefix}_${freq}.fits
done
