#!/usr/bin/env fish

cargo run -- $argv[1] -o ./out/out.ll
llc ./out/out.ll -o ./out/out.S
gcc ./out/out.S -o $argv[2] 
