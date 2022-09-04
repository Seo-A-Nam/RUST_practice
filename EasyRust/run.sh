#!/bin/sh

# if you want to run ex13, then type 'sh run.sh 13' on terminal
NAME="ex$1*.rs"
rustc -o result $NAME
./result
