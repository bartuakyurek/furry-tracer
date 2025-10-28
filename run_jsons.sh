#!/usr/bin/env bash

# List of JSON files to run
json_files=(
    "./input/test.json"
    "./input/simple.json"
    "./input/spheres.json"
    "./input/two_spheres.json"
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
