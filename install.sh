git clone https://github.com/d3r1n/brainsuck

cd ./brainsuck/bs_bin

cargo build --release

sudo mv ./target/release/bs_bin /usr/bin/brainsuck

printf "Building finished...\nCleaning...\n"

cd ..

rm -rf brainsuck

printf "Cleaning finished...\n Done!"