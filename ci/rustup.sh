#!/bin/sh
# Use rustup to locally run the same suite of tests as .travis.yml.
# (You should first install/update 1.19.0, stable, beta, and nightly.)

set -ex

export TRAVIS_RUST_VERSION
for TRAVIS_RUST_VERSION in 1.19.0 stable beta nightly; do
    run="rustup run $TRAVIS_RUST_VERSION"
    $run cargo generate-lockfile
    if [ "$TRAVIS_RUST_VERSION" = 1.19.0 ]; then
      $run cargo update -p num-iter --precise 0.1.43
      $run cargo update -p num-integer --precise 0.1.45
      $run cargo update -p num-traits --precise 0.2.15
      $run cargo update -p libc --precise 0.2.163
    fi
    $run cargo build --verbose
    $run $PWD/ci/test_full.sh
done
