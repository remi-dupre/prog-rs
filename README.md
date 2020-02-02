prog-rs
=======

![licence](https://img.shields.io/github/license/remi-dupre/prog-rs)
[![crate](https://img.shields.io/crates/v/prog_rs.svg)](https://crates.io/crates/prog_rs)
[![documentation](https://docs.rs/prog_rs/badge.svg)](https://docs.rs/prog_rs)


A Rust library to help easily build a progress bar.

![animated screenshot](.illustration.gif)


Usage
-----

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
prog_rs = "0.1"
```

Next, add this to your crate root:

```rust
extern crate prog_rs;
```

To add a progress bar while iterating, just add `.progress()` behind your
iterator:

```rust
use prog_rs::prelude::*;

fn main() {
    for _ in (0..1_000).progress() {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
```

Some parameters can be tweaked using `with_` prefixed methods:

```rust
use prog_rs::prelude::*;

fn main() {
    for _ in (0..1_000)
        .progress()
        .with_prefix("Processing...")
        .with_output_stream(prog_rs::OutputStream::StdErr)
        .with_bar_position(prog_rs::BarPosition::Right)
    {
        do_something();
    }
}
```

This same behaviour is also implemented for files:

```rust
use prog_rs::prelude::*;

let f = File::open("../../data/addresses/bano.csv")
    .unwrap()
    .progress()
    .with_prefix(" Read file ...")
    .with_bar_position(BarPosition::Right);
let f = BufReader::new(f);
println!("This file has {} lines", f.lines().count());
```
