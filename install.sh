!#/bin/bash

echo "Compiling"

cargo build

echo "Copying binary to bin"

cp ./target/debug/premstash /usr/local/bin/premstash
