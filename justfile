#----------
# Building
#----------

# Build the examples for all versions
build-all: (build "next") (build "0.6") (build "0.7")

# Build the examples for the given version
build version:
    cd "eg-{{version}}" && cargo build --examples --release

#----------
# READMEs
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

# # Generates screenshots of all examples for a given version
# @generate-example-screenshots-for-version version:
#     #!/usr/bin/env bash
#     set -e
#     for example in eg-{{version}}/examples/*.rs; do
#         just generate-example-screenshot {{version}} $(basename "$example" .rs);
#     done

# generate-example-screenshots: (generate-example-screenshots-for-version "0.6") (generate-example-screenshots-for-version "next")

# Generate a readme with example screenshots and save it into target/
_build-readme version:
    #!/usr/bin/env bash
    set -e
    cd eg-{{version}}

    out="target/README.md"

    # Empty the README file
    echo -n '' > $out

    echo '# Embedded graphics examples and demos for version `{{version}}`' >> $out
    echo '' >> $out

    for example in examples/*.rs; do
        name=$(basename "$example" .rs)
        just generate-example-screenshot {{version}} $name;

        (
        cat <<-EXAMPLE
    ## [$name]($example)

    [![A screenshot of the embedded-graphics version {{version}} example called $name](../doc/assets/{{version}}/$name.png)]($example)

    EXAMPLE
    ) >> $out
    done

generate-readme version: (_build-readme version)
    #!/usr/bin/env bash
    set -euo pipefail
    cp "eg-{{version}}/target/README.md" "eg-{{version}}/README.md"

# Generate readmes for all versions
generate-readmes: (generate-readme "next") (generate-readme "0.6") (generate-readme "0.7")

# Check a readme to see if it needs to be regenerated
check-readme version: (_build-readme version)
    #!/usr/bin/env bash
    set -euo pipefail
    diff -q "eg-{{version}}/target/README.md" "eg-{{version}}/README.md" || ( \
        echo -e "\033[1;31mError:\033[0m README.md for eg-{{version}} needs to be regenerated."; \
        echo -e "       Run 'just generate-readmes' to regenerate.\n"; \
        exit 1 \
    )

# Check all readmes
check-readmes: (check-readme "next") (check-readme "0.6") (check-readme "0.7")
