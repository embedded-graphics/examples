
#----------
# Building
#----------

build: build-next build-0-6

# Build the simulator
build-next:
    #!/usr/bin/env bash
    pushd ./eg-next
    cargo build --examples
    popd

# Run cargo test in release mode
build-0-6:
    #!/usr/bin/env bash
    pushd ./eg-0.6
    cargo build --examples
    popd
