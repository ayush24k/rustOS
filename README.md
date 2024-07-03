This Prjoect is incomplete and Currently being worked on!!
<hr>

### rustOS Progress:

### 1. Setting up the bare bones of this project:
• Freestandig Rust Executable: <br>
> created a rust executable that doesn't link the standard library which makes it possible to run Rust code instructions directly on logic hardware without an intervening operating system (running rust code on bare metal/machine).

• Rust Kernal: <br>
> Wrote a minimal 64-bit rust kernal for x86 architecture and also built a bootable disk image that can print some things on the screen and used Qemu(Virtual Machine) to run the bootable image that is built on the freestanding rust binary/executable. <br> <br>
> This Rust Kernal only supports Basic Input/Output System (BIOS) firmware standard as it is simple and easy to implement I could have implementd Unified Extensible Firmware Interface (UEFI) but that's a lot of work :kissing: will leave that for later...

attached img: <img src="/DocsImg/KernalPrint.png">


> todo
> - vga_buffers
