#!/bin/bash



names="confidence"

echo start

for n in $names; do
    ./target/release/examples/circ --inputs $n.in $n.c r1cs --action spartan &> $n.log &  
done


