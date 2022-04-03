#!/bin/bash

python opt.py 2d 100 500 L2 100000 100
python opt.py 2d 100 500 L1 100000 100
python opt.py 2d 100 500 L2SQ 100000 100
python opt.py 2d 100 500 LINF 100000 100

python opt.py 2d 100 100 L2 100000 100
python opt.py 2d 100 100 L1 100000 100
python opt.py 2d 100 100 L2SQ 100000 100
python opt.py 2d 100 100 LINF 100000 100
