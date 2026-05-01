# What 

A rust follow along of thebenybox's 3D Software Rendering Tutorial [link](https://www.youtube.com/watch?v=Y_vvC2G7vRo&list=PLEETnX-uPtBUbVOok816vTl1K9vV1GgH5)

# Nodes

Built for debugging with LLDB with vscodium (vadimcn.vscode-lldb)

# Building

## Native

```bash
cargo run
```

## Web

```bash
cargo build --release --target wasm32-unknown-unknown
python -m http.server 8080
# Viewable on http://localhost:8080
```


# Benchmark

Ran with

```bash
cargo bench --features bench
```

## Baseline
**Timer precision:** 40 ns

| Scene | Fastest | Slowest | Median | Mean | Samples | Iters |
|-------|--------:|--------:|-------:|-----:|--------:|------:|
| 0     | 14.19 ms | 21.57 ms | 14.31 ms | 15.28 ms | 10 | 10 |
| 100   | 512.5 ms | 550.6 ms | 535.7 ms | 534.7 ms | 10 | 10 |
| 500   | 2.476 s  | 2.654 s  | 2.509 s  | 2.522 s  | 10 | 10 |


## Removed vectors in gradient
**Timer precision:** 60 ns

| Scene | Fastest | Slowest | Median | Mean | Samples | Iters |
|-------|--------:|--------:|-------:|-----:|--------:|------:|
| 0     | 13.92 ms | 17.04 ms | 14.04 ms | 14.55 ms | 10 | 10 |
| 100   | 500.5 ms | 508.8 ms | 502.6 ms | 503.6 ms | 10 | 10 |
| 500   | 2.451 s  | 2.578 s  | 2.478 s  | 2.492 s  | 10 | 10 |


## Moved from fltk to macroquad
**Timer precision:** 20 ns

| Scene | Fastest | Slowest | Median | Mean | Samples | Iters |
|-------|--------:|--------:|-------:|-----:|--------:|------:|
| 0     | 15.74 ms | 17.56 ms | 16.08 ms | 16.26 ms | 10 | 10 |
| 100   | 431.6 ms | 449.3 ms | 438 ms   | 438.6 ms | 10 | 10 |
| 500   | 2.101 s  | 2.14 s   | 2.104 s  | 2.108 s  | 10 | 10 |


## cleaner gradient struct
**Timer precision:** 40 ns

| Scene | Fastest | Slowest | Median | Mean | Samples | Iters |
|-------|--------:|--------:|-------:|-----:|--------:|------:|
| 0     | 16 ms    | 29.62 ms | 16.17 ms | 17.5 ms | 10 | 10 |
| 100   | 439.5 ms | 445 ms   | 443.4 ms | 442.9 ms | 10 | 10 |
| 500   | 2.091 s  | 2.169 s  | 2.147 s  | 2.134 s  | 10 | 10 |


## Removed excess float multiplication / conversions
**Timer precision:** 20 ns

| Scene | Fastest | Slowest | Median | Mean | Samples | Iters |
|-------|--------:|--------:|-------:|-----:|--------:|------:|
| 0     | 15.6 ms  | 16.77 ms | 15.94 ms | 15.99 ms | 10 | 10 |
| 100   | 361.7 ms | 372.3 ms | 367.5 ms | 367.6 ms | 10 | 10 |
| 500   | 1.729 s  | 1.806 s  | 1.776 s  | 1.772 s  | 10 | 10 |

## Swapped to macroquad images 
Timer precision: 20 ns  
scene            fastest       │ slowest       │ median        │ mean          │ samples │ iters  
╰─ render_scene                │               │               │               │         │  
   ├─ 0          16.77 ms      │ 17.37 ms      │ 16.97 ms      │ 17.03 ms      │ 10      │ 10  
   ├─ 100        359.3 ms      │ 369.9 ms      │ 365.4 ms      │ 365.4 ms      │ 10      │ 10  
   ╰─ 500        1.744 s       │ 1.814 s       │ 1.779 s       │ 1.776 s       │ 10      │ 10

## Light no longer hardcoded and now passed as ref
Timer precision: 20 ns  
scene            fastest       │ slowest       │ median        │ mean          │ samples │ iters  
╰─ render_scene                │               │               │               │         │  
   ├─ 0          18.12 ms      │ 19.1 ms       │ 18.19 ms      │ 18.28 ms      │ 10      │ 10  
   ├─ 100        359.3 ms      │ 376.4 ms      │ 364.4 ms      │ 366.3 ms      │ 10      │ 10  
   ╰─ 500        1.75 s        │ 1.804 s       │ 1.788 s       │ 1.783 s       │ 10      │ 10  

## Colored light source support

Timer precision: 60 ns  
scene            fastest       │ slowest       │ median        │ mean          │ samples │ iters  
╰─ render_scene                │               │               │               │         │  
   ├─ 0          18.63 ms      │ 19.38 ms      │ 18.72 ms      │ 18.83 ms      │ 10      │ 10  
   ├─ 100        383.1 ms      │ 417.3 ms      │ 397.8 ms      │ 398.3 ms      │ 10      │ 10  
   ╰─ 500        1.843 s       │ 1.951 s       │ 1.881 s       │ 1.887 s       │ 10      │ 10
