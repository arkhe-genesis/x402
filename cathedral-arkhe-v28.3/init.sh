#!/bin/bash
set -e

echo "Cathedral ARKHE v28.3 - System Initialization"
echo "==============================================="

# Verifying directories
for dir in core/model agent/memory/knowledge_base trust/integrity trust/signatures trust/verification orchestrator/src orchestrator/consensus governance runtime telemetry docs; do
    mkdir -p "$dir"
done

# Basic checks
if [ ! -f "agent/config.yaml" ]; then
    echo "Warning: agent/config.yaml not found. Please create it using the template."
fi

if [ ! -f "core/model/manifest.json" ]; then
    echo "Warning: core/model/manifest.json not found."
fi

echo "Starting Docker Compose services..."
if command -v docker-compose &> /dev/null; then
    cd runtime && docker-compose up -d
else
    echo "docker-compose not found. Please run 'docker compose up -d' in the runtime directory manually."
fi

echo "Initialization complete."
