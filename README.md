# RP2040 Blink Twice
This crate runs both RTIC and Embassy on the Pimoroni Tiny 2040 using separate cores. Both flashing and breakpoint debugging can be performed from within the development Docker container.

## Why
Sometimes there is a need for pre-emptive interrupt driven execution, other times there is a need for a cooperative asynchronous runtime with an executor. Why not both?

Why not Zoidberg? 🦀

## How
RTIC monotonic support for the RP2040 is patched to support embassy.

Core 0 runs an RTIC task and flashes the green LED ten times a second.

Core 1 runs an embassy executor and flashes the red LED once a second.

## Setup
udev rules are required for a non-root user to access the debug probe.

```sh
cp 69-probe-rs.rules /etc/udev/rules.d/
sudo !!
```

See [Linux udev rules](https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules).

## Patch
Patch [rtic-monotonics](https://docs.rs/rtic-monotonic/latest/rtic_monotonic/) to use [rp-pac](https://docs.rs/rp-pac/latest/rp_pac/) rather than [rp2040-pac](https://docs.rs/rp2040-pac/latest/rp2040_pac/) for embassy compatibility.

```sh
git submodule update --init --recursive
cd rtic/ && git apply ../rtic-monotonics.patch
```

## Prerequisites
Optional: Install the following prerequisites, or alternatively, deploy with Docker via the vscode dev container.

```sh
cargo install probe-rs-tools
```

See [probe-rs Documentation](https://probe.rs/docs/getting-started/installation/).

## Size

The following binary sizes for debug and release are provided by `cargo-binutils`.

```sh
$ cargo size
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
   text    data     bss     dec     hex filename
  72356       0    9648   82004   14054 rp-blink-twice
$ cargo size --release
    Finished `release` profile [optimized] target(s) in 0.07s
   text    data     bss     dec     hex filename
  11648       0    9620   21268    5314 rp-blink-twice
```

## Deploy
Flash using either a [Raspberry Pi Debug Probe](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html) or a compatible CMSIS-DAP adapter.

```sh
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `probe-rs run --chip RP2040 --protocol swd target/thumbv6m-none-eabi/debug/rp-blink-twice`
      Erasing ✔ [00:00:01] [#########] 72.00 KiB/72.00 KiB @ 54.81 KiB/s (eta 0s )
  Programming ✔ [00:00:01] [#########] 72.00 KiB/72.00 KiB @ 38.51 KiB/s (eta 0s )    Finished in 3.212s
INFO  Hello, from RTIC!
└─ rp_blink_twice::app::rtic_task::{async_fn#0} @ src/main.rs:57  
INFO  Hello, from Embassy!
└─ rp_blink_twice::app::__embassy_task_task::{async_fn#0} @ src/main.rs:72  
```

Alternatively, press F5 to start debugging.

*Green, Green, Green, Red*