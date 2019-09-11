#!/usr/bin/env bash
#curl https://freefr.dl.sourceforge.net/project/gnuplot/gnuplot/5.2.7/gnuplot-5.2.7.tar.gz > gnuplot.tar.gz
#tar -xvf gnuplot.tar.gz
#cd gnuplot-5.2.7
#./configure
#make
#make install
#cd ..
#rm -r gnuplot.tar.gz gnuplot-5.2.7
#brew install gnuplot
cargo build --release
./target/release/trainer data.csv