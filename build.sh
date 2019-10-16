#!/bin/sh

set -e

output_dir=target/docker
mkdir -p $output_dir
build_log=`docker build -f build.Dockerfile . | tee /dev/tty`
hash=$(echo "$build_log" | grep "Successfully built" | cut -d ' ' -f 3)
# /build is currently the single binary
docker run --rm --entrypoint cat $hash /build > $output_dir/nem-server
# because it's a binary, currently, lets chmod it.
chmod +x $output_dir/nem-server
