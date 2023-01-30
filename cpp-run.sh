#!/bin/bash

g++ main.cpp -O2 -std=c++20
time for i in $(seq 0 100); do ./a.out 12; done
