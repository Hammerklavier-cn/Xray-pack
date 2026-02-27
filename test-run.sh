#!/usr/bin/bash

show_help() {
    echo "Usage: $0 [OPTIONS] [ARGS...]"
    echo ""
    echo "Run the Xray application with development profile."
    echo ""
    echo "Options:"
    echo "  help     Show this help message"
    echo ""
    echo "Arguments passed after options will be forwarded to the application."
}

# Check if help was requested
for arg in "$@"; do
    if [ "$arg" == "help" ]; then
        show_help
        exit 0
    fi
done

# Run the cargo command with all arguments
cargo run --profile dev-rel -- "$@"
