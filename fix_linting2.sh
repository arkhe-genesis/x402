#!/bin/bash
sed -i 's/sum(latencies) sum(latencies)/sum(latencies)/g' python/x402/system_design_orchestrator.py
sed -i 's/except:/except Exception:/g' python/x402/reliability_layer.py
sed -i 's/except:/except Exception:/g' python/x402/database_layer.py
