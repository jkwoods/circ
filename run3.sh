#!/bin/bash

names="control1 hinf2 hinf3 hinf4 hinf5 hinf6 hinf7 hinf8 hinf9"


for n in $names; do
  
  echo mv sdpzkif/$n.zkif .
  mv sdpzkif/$n.zkif .
  echo ./flatc_to_json.sh $n.zkif
  ./flatc_to_json.sh $n.zkif
  ls outs/
  mv $n.zkif sdpzkif/

  cd outs
  python3 measure.py $n.json > $n.r1cs
  cd ..

done
