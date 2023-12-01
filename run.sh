#!/bin/bash

# Usage: ./run.sh year day
# Example: ./run.sh 2023 day00

YEAR=$1
DAY=$2

BASE_DIR="./"

PROJECT_PATH="$BASE_DIR/$YEAR/$DAY"

if [ -d "$PROJECT_PATH" ]; then
    (
        cd "$PROJECT_PATH" && 
        cargo run
    )
else
    echo "Project for $YEAR - $DAY does not exist."
fi
