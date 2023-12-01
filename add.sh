#!/bin/bash

# Usage: ./add.sh year day
# Example: ./add.sh 2023 day01

YEAR=$1
DAY=$2

BASE_DIR="./"

if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi

mkdir -p "$BASE_DIR/$YEAR"

PROJECT_PATH="$BASE_DIR/$YEAR/$DAY"

cargo new "$PROJECT_PATH"

# Remove the .git directory created by cargo
rm -rf "$PROJECT_PATH/.git"

echo "Created Rust project for $YEAR - $DAY"
