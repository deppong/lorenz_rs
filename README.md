# Lorenz System Visualiser in Rust

basically a port of [my lorenz renderer in C](https://github.com/deppong/lorenz_c).

Implemented some basic 3d matrix rotations. Could add a more encapsulated way of doing this but for now it works I guess.

## Controls 
Key | Operation
--- | ---
W   | Rotate up
S   | Rotate down
A   | Rotate left
D   | Rotate right
Q   | Yaw clockwise
E   | Yaw counter-clockwise
R   | Reset all rotations 
Space | Pause simulation
Esc | Quit

# Building

## Linux

Make sure you have the `libsdl2-dev` package installed on your computer.
On different package managers this may be named something else, so just search for sdl2

## MSVC
Download MSVC development libraries from http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip).
Unpack SDL2-devel-2.0.x-VC.zip to a folder of your choosing (You can delete it afterwards).

Copy all lib files from
> SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

to (for Rust 1.6 and above)

> C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib

or to (for Rust versions 1.5 and below)

> C:\Program Files\Rust\bin\rustlib\x86_64-pc-windows-msvc\lib

or to your library folder of choice, and ensure you have a system environment variable of

> LIB = C:\your\rust\library\folder

For Rustup users, this folder will be in

> C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

Where current toolchain is likely stable-x86_64-pc-windows-msvc.

Copy SDL2.dll from

> SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

to this directory
