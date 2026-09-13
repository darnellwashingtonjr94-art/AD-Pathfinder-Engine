#!/usr/bin/env bash
set -euo pipefail
echo "Building AD-Pathfinder-Engine..."
cargo build --release
echo "Build complete."
