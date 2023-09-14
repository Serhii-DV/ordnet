#!/bin/bash

# Define constants
COMMAND="target/debug/ordnet"
INPUT_FILE="input.txt"
OUTPUT_FILE="output.txt"

# Function to check if a line contains unwanted URLs
ignore_line() {
    if [[ "$1" =~ "https://translate.google.com/" || "$1" =~ "https://ordnet.dk/" ]]; then
        return 0  # Ignore this line
    else
        return 1  # Don't ignore this line
    fi
}

# Create or clear the output file
> "$OUTPUT_FILE"

# Loop through each word in the input file
while IFS= read -r word; do
    # Run the command and capture the output
    output="$("$COMMAND" "$word")"

    # Check the exit status of the command
    if [ $? -ne 0 ]; then
        # Display the failed command in the console
        echo "Failed command: $COMMAND $word"
    else
        # Loop through each line in the output
        while IFS= read -r line; do
            # Check if the line should be ignored
            if ! ignore_line "$line" && [ -n "$line" ]; then
                # Append the non-ignored and non-empty line to the output file
                echo "$line" >> "$OUTPUT_FILE"
            fi
        done <<< "$output"  # Feed the output into the loop
    fi
done < "$INPUT_FILE"
