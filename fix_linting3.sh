#!/bin/bash
sed -i 's/for t in range/for _t in range/g' python/x402/consciousness_simulation.py
sed -i 's/partition_b =/  # partition_b =/g' python/x402/core_foundations.py
sed -i 's/for entry in reversed/for _entry in reversed/g' python/x402/database_layer.py
sed -i 's/recent_spikes =/  # recent_spikes =/g' python/x402/neuromorphic_bridge_adapter.py
sed -i 's/I =/I_in =/g' python/x402/neuromorphic_bridge_adapter.py
sed -i 's/neuron.step(I)/neuron.step(I_in)/g' python/x402/neuromorphic_bridge_adapter.py
sed -i 's/for sid, data in/for sid, _data in/g' python/x402/optimization_solver_bridge.py
sed -i 's/with self.q as prog/with self.q as _prog/g' python/x402/photonic_hardware_driver.py
sed -i 's/zip(squeezings, displacements)/zip(squeezings, displacements, strict=False)/g' python/x402/photonic_hardware_driver.py
sed -i 's/for t in range/for _t in range/g' python/x402/polariton_simulator.py
sed -i 's/I =/I_in =/g' python/x402/polaritonic_snn_trainer.py
sed -i 's/neuron.step(I)/neuron.step(I_in)/g' python/x402/polaritonic_snn_trainer.py
sed -i 's/from qiskit.visualization import plot_histogram/# from qiskit.visualization import plot_histogram/g' python/x402/quantum_bridge_adapter.py
sed -i 's/for depth in range/for _depth in range/g' python/x402/quantum_bridge_adapter.py
sed -i 's/from qiskit import QuantumCircuit/# from qiskit import QuantumCircuit/g' python/x402/quantum_neuromorphic_optimizer.py
sed -i 's/from qiskit.algorithms import VQE/# from qiskit.algorithms import VQE/g' python/x402/quantum_neuromorphic_optimizer.py
sed -i 's/from qiskit.primitives import Estimator/# from qiskit.primitives import Estimator/g' python/x402/quantum_neuromorphic_optimizer.py
sed -i 's/circuit =/# circuit =/g' python/x402/quantum_neuromorphic_optimizer.py
