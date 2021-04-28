This is a demonstration of how to provide atsamd samples where the source is common and the
target is controlled by a feature flag.

     cargo build --release --example blinky_basic --target thumbv7em-none-eabihf --features grand_central_m4
     cargo build --release --example blinky_basic --target thumbv7em-none-eabihf --features feather_m4
     cargo build --release --example blinky_basic --target thumbv6m-none-eabi --features feather_m0

This BSC does not work, as the BSC is different enough that it would require a lot more `cfg`.

     cargo build --release --example blinky_basic --target thumbv7em-none-eabihf --features wio_terminal
