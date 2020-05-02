# Reversi
This is reversi game wriiten in rust.
### Install
```
git clone https://github.com/masan4444/reversi.git
cd reversi
RUSTFLAGS='-C target-cpu=native' cargo +nightly build
```
### Run
```
RUSTFLAGS='-C target-cpu=native' cargo +nightly run
```
### Test
```
RUSTFLAGS='-C target-cpu=native' cargo +nightly test
```
### Bench
```
RUSTFLAGS='-C target-cpu=native' cargo +nightly bench
```


#### Install rust and cargo
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
```
