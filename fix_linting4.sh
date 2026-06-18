#!/bin/bash
sed -i 's/  # recent_spikes =/  pass # recent_spikes =/g' python/x402/neuromorphic_bridge_adapter.py
sed -i 's/for sid, _data in/for sid, data in/g' python/x402/optimization_solver_bridge.py
sed -i 's/from qiskit.circuit.library import EfficientSU2/# from qiskit.circuit.library import EfficientSU2/g' python/x402/quantum_neuromorphic_optimizer.py
