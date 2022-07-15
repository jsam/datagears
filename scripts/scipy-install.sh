#!/bin/bash

pip3 uninstall -y numpy pythran
pip3 install cython pybind11
pip3 install --no-binary :all: --no-use-pep517 numpy
pip3 install pythran

OPENBLAS=/opt/homebrew/opt/openblas/lib/ pip3 install --no-binary :all: --no-use-pep517 scipy
