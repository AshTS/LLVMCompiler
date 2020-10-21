#!/usr/bin/env fish

cargo run -- $argv[1] -o ./out/avr.ll -O 3 -g llvm --llvm-target avr-none
llc ./out/avr.ll -o ./out/out.S

cat ./out/out.S