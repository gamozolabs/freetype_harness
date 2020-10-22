# Summary

Whoa, why would this possibly exist. This is just a static-ly compiled
riscv64ima binary which allows for some basic font stuff.

If rv64ima means nothing to you, it's a much more stripped down soft float
version of RISC-V which is capable of running pretty much any application and
thus decreases the cost of making an emulator since you only need to worry
about base instructions + atomics + muls and divs. Much simpler than supporting
floats and stuff.

I'm mainly throwing this on GitHub because getting a custom Rust core+std
being compiled with custom CPU features is gonna be a neat trick for me in
the future. Especially static + glibc, which Rust really doesn't support.

Thus... the fact this works is kinda cool.

# Using

You probably shouldn't use this. If you want to, build
https://github.com/riscv/riscv-gnu-toolchain by doing:

```bash
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
./configure --prefix=/opt/riscv --with-arch=rv64ima --with-abi=lp64
make -j192 linux
```

Then, simply go acquire freetype2, brotli, bzip2, harfbuzz, libpng, icu, zlib,
and hack up the makefiles to get them to work with cross compiling. Install em
all into the `/opt/riscv/sysroot/usr` sysroot and you should be able to
actually run `make` in this project.

It's a lot of work, you don't want this. Why are you fuzzing your targets on
RISC-V anyways? Wait, is this project about fuzzing? No, just coincidence.
I just really needed RISC-V support for my... GUI application that uses fonts!

