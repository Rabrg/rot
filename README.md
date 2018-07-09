rot
===

An extremely simple operating system written in Rust.

Bootloader is generated using the [bootimage](https://crates.io/crates/bootimage) utility.

![screenshot](https://i.imgur.com/lxTprHv.png)

### Installation
Image can be written to USB using the following command

```console
dd if=target/x86_64-rot/debug/bootimage-rot.bin of=/dev/sdX && sync
```

where ```sdX``` is the device name of your USB stick.
