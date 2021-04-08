echo This assumes that you have built the rust project with "cargo build"

clang++ -L../target/release/ -I../shared -o testb.out ./src/Main.cpp -lyoutubedl_rust

echo Finished compile, now running
export LD_LIBRARY_PATH=../target/release
./testb.out