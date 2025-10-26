#!/usr/bin/env bash

# List of JSON files to run
json_files=(
    "./assets/test.json"
    "./assets/simple.json"
    "./assets/spheres.json"
    "./assets/two_spheres.json"
)

for json_file in "${json_files[@]}"; do
    if [ -f "$json_file" ]; then
        echo "Running cargo for $json_file ..."
        cargo run --release -- "$json_file"
        echo "---------------------------------------"
    else
        echo "File not found: $json_file"
    fi
done
