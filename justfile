#----------
# Building
#----------

# Build the examples for all versions
build-all: (build "next") (build "0.6")

# Build the examples for the given version
build version:
    cd "eg-{{version}}" && cargo build --examples --release

#----------
# Examples
#----------

# NOTE: Relative to the version sub-directories
# TODO: Use {{justfile_directory()}} when we upgrade to Just ~0.9
screenshots_dir := "../doc/assets"

# Generates a screenshot of an example
generate-example-screenshot version example:
    #!/usr/bin/env bash
    cd "eg-{{version}}"
    mkdir -p "{{screenshots_dir}}/{{version}}"
    echo "Generating {{example}} screenshot"
    EG_SIMULATOR_DUMP="{{screenshots_dir}}/{{version}}/{{example}}.png" \
    cargo run --example {{example}}

# Generates screenshots of all examples for a given version
@generate-example-screenshots-for-version version:
    #!/usr/bin/env bash
    set -e
    for example in eg-{{version}}/examples/*.rs; do
        just generate-example-screenshot {{version}} $(basename "$example" .rs);
    done

generate-example-screenshots: (generate-example-screenshots-for-version "0.6") (generate-example-screenshots-for-version "next")
