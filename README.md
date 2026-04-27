# What 

A rust follow along of thebenybox's 3D Software Rendering Tutorial [link](https://www.youtube.com/watch?v=Y_vvC2G7vRo&list=PLEETnX-uPtBUbVOok816vTl1K9vV1GgH5)

# Nodes

Built for debugging with LLDB with vscodium (vadimcn.vscode-lldb)

# Benchmark

Timer precision: 40 ns
scene            fastest       │ slowest       │ median        │ mean          │ samples │ iters  
╰─ render_scene                │               │               │               │         │  
   ├─ 0          14.19 ms      │ 21.57 ms      │ 14.31 ms      │ 15.28 ms      │ 10      │ 10  
   ├─ 100        512.5 ms      │ 550.6 ms      │ 535.7 ms      │ 534.7 ms      │ 10      │ 10  
   ╰─ 500        2.476 s       │ 2.654 s       │ 2.509 s       │ 2.522 s       │ 10      │ 10  

