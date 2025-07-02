# BlackBoldOS

Trying to get a Rust kernel to boot under Limine and print something over serial.

Early experiment, doesn't boot yet.

Build:

```
cargo build
```

Tried:

```
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/blackboldos -serial stdio
```

That silently does nothing, need real limine image. Reading limine bare-bones docs.
