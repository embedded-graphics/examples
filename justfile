#----------
# Building
#----------

# Build the examples for all versions
build-all: (build "next") (build "0.6")

# Build the examples for the given version
build version:
    cd "eg-{{version}}" && cargo build --examples
