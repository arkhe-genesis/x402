#!/bin/bash
sed -i 's/for sid, data in substrates.items():/for sid, _data in substrates.items():/g' python/x402/optimization_solver_bridge.py
