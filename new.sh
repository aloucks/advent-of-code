#!/bin/bash

year=$1
day=$2
file=$year/src/bin/aoc${year}day${day}.rs

echo "file = $file"
echo "press enter to create file or ctrl-c to end"
read
mkdir -p $year/src/bin
echo 'fn main() {}'>> $file
file=$year/src/bin/aoc${year}day${day}_input.txt
echo "file = $file"
echo "press enter to create input file or ctrl-c to end"
read
touch $file
