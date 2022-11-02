

# Building

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