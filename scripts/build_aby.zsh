#!/usr/bin/env zsh

mkdir -p -- third_party/ABY/build
cd third_party/ABY/build
/opt/cmake/bin/cmake -DABY_BUILD_EXE=On -DCMAKE_C_COMPILER=gcc-8 -DCMAKE_CXX_COMPILER=g++-8 ..
make
