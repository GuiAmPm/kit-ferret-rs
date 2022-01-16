cd demo-kitferret
mkdir bin
cargo objcopy --release -- -O ihex bin/kit-ferret.hex
cd ..