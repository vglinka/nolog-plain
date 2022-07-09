## nolog-plain logger

20 lines of code with 0 deps.

A non-colored version of the simple
[nolog](https://github.com/vglinka/nolog) logger for writing to a file.
If you need colored output, please refer
to the [nolog crate](https://crates.io/crates/nolog) (there are the same
20 lines of code, but with coloring).

- Support for named format arguments `info!("{line_count} lines.");`.
- Displays the location of the code `[src/main.rs 15:5]`.
- Easy to add timestamp `[2022-07-10 06:49:33.646361181 UTC]`.
- Automatically disabled in the release build `cargo run --release`.
  You can modify the code to disable this. See section "If you don't need
  to disable logging on a release build".
- Same syntax as `log` crate. As the project grows, it is possible
  to migrate to an advanced logger (using the `log` crate facade)
  without changing the code.
- Can be built into the project directly instead of as a dependency.
- Uses Rust's built-in macros.
- Easy to modify to redirect log output (to `stderr` or `file`).
- MIT License.

![nolog](https://raw.githubusercontent.com/vglinka/nolog-plain/main/assets/term.png)

## Using nolog-plain as a dependency for logging to a file

In `Cargo.toml`:

```toml
nolog-plain = "0.2.1"
```

**Download the examples below:**

```sh
git clone https://github.com/vglinka/nolog-plain
cd ./nolog-plain/
```
```sh
cargo run --example base
cargo run --example to-file
cargo run --example to-stderr

cargo run --example to-file-debug-plus-release
cargo run --example to-file-debug-plus-release --release
```
```sh
cd ./examples/as-dep-to-file-timestamp-chrono/
cargo run 
```
```sh
cd ./examples/as-dep-to-file/
cargo run 
```

`logger.rs`:

```rust
/* ----------------------------------------------------------------------------
MIT License

Copyright (c) 2022 Vadim Glinka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
---------------------------------------------------------------------------- */

pub use nolog_plain::{crit, debug, error, info, trace, warn /* writelog */};
// We don't import writelog because we want to override it --- ^^^^^^^^

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
writelog { ( $msg:expr ) => { log($msg) } }
//                            ^^^
// function `log` instead of `println` macro.

use std::fmt::Arguments;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct LogFile;

impl LogFile {
    pub fn path() -> PathBuf {
        PathBuf::from("log.txt")
    }
}

#[rustfmt::skip]
pub fn log(msg: Arguments) {
//              ^^^^^^^^^ std::fmt::Arguments
    let path = LogFile::path();
    let open_file = |path| { OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
    };
    if let Ok(file) = open_file(path) {
        let mut buf = BufWriter::new(file);
        writeln!(buf, "{msg}").ok();
        buf.flush().ok();
    };
}
```

`main.rs`:

```rust
mod logger;

use crate::logger::*;
//         ^^^^^^^^^
// It must be imported in each module in which we will use logging.

use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let path = LogFile::path();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        //^^^^^^^ truncate the file to 0 length if it already exists.
        .open(&path)?;

    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("-- In {path:?} --");
    println!("{contents}");

    /* Output:
    -- In "log.txt" --
    TRCE: text 42,42,42 [src/main.rs 29:5]
    DEBG: text 42,42,24 [src/main.rs 30:5]
    INFO: text 42,24,42 [src/main.rs 31:5]
    WARN: text 42,a,422 [src/main.rs 32:5]
    ERRO: text 42,42,42 [src/main.rs 33:5]
    CRIT: text 42,42,42 [src/main.rs 34:5]
    */

    Ok(())
}
```

## Using nolog-plain directly

The same thing, only in `Cargo.toml` we do not add `nolog-plain`
as a dependency and in `logger.rs` we copy the entire code of this
library (all 20 lines):

```rust
/* ----------------------------------------------------------------------------
MIT License

Copyright (c) 2022 Vadim Glinka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
---------------------------------------------------------------------------- */

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
trace { ( $($msg:expr),* ) => { writelog!(format_args!("TRCE: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
debug { ( $($msg:expr),* ) => { writelog!(format_args!("DEBG: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
info { ( $($msg:expr),* )  => { writelog!(format_args!("INFO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
warn { ( $($msg:expr),* )  => { writelog!(format_args!("WARN: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
error { ( $($msg:expr),* ) => { writelog!(format_args!("ERRO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
crit { ( $($msg:expr),* )  => { writelog!(format_args!("CRIT: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }

#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! trace { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! debug { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! info  { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! warn  { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! error { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! crit  { ( $($msg:expr),* ) => () }

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
writelog { ( $msg:expr ) => { log($msg) } }
//                            ^^^
// function `log` instead of `println` macro.

use std::fmt::Arguments;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct LogFile;

impl LogFile {
    pub fn path() -> PathBuf {
        PathBuf::from("log.txt")
    }
}

#[rustfmt::skip]
pub fn log(msg: Arguments) {
//              ^^^^^^^^^ std::fmt::Arguments
    let path = LogFile::path();
    let open_file = |path| { OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
    };
    if let Ok(file) = open_file(path) {
        let mut buf = BufWriter::new(file);
        writeln!(buf, "{msg}").ok();
        buf.flush().ok();
    };
}
```

Now you have a logger and no new dependencies.

## How to add Timestamp
For example, we will use [chrono crate](https://docs.rs/chrono/0.4.19/chrono/).

```
-- In "log.txt" --
[2022-07-10 06:49:33.646361181 UTC] TRCE: text 42,42,42 [src/main.rs 22:5]
[2022-07-10 06:49:33.646393648 UTC] DEBG: text 42,42,24 [src/main.rs 23:5]
[2022-07-10 06:49:33.646405179 UTC] INFO: text 42,24,42 [src/main.rs 24:5]
[2022-07-10 06:49:33.646415125 UTC] WARN: text 42,a,422 [src/main.rs 25:5]
[2022-07-10 06:49:33.646424722 UTC] ERRO: text 42,42,42 [src/main.rs 26:5]
[2022-07-10 06:49:33.646434216 UTC] CRIT: text 42,42,42 [src/main.rs 27:5]
```
[See example code](https://github.com/vglinka/nolog-plain/blob/main/examples/as-dep-to-file-timestamp-chrono/src/logger.rs).

## If you don't need to disable logging on a release build

This will make the code even shorter:

```rust
/* ----------------------------------------------------------------------------
MIT License

Copyright (c) 2022 Vadim Glinka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
---------------------------------------------------------------------------- */

#[rustfmt::skip] #[macro_export] macro_rules!
trace { ( $($msg:expr),* ) => { writelog!(format_args!("TRCE: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
debug { ( $($msg:expr),* ) => { writelog!(format_args!("DEBG: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
info { ( $($msg:expr),* )  => { writelog!(format_args!("INFO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
warn { ( $($msg:expr),* )  => { writelog!(format_args!("WARN: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
error { ( $($msg:expr),* ) => { writelog!(format_args!("ERRO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
crit { ( $($msg:expr),* )  => { writelog!(format_args!("CRIT: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }

#[rustfmt::skip] #[macro_export] macro_rules!
writelog { ( $msg:expr ) => { println!("{}", $msg) } }
```


## Changelog

- **0.2.1** – Changes in `README.md` and examples.
- **0.2.0** – Macro invocation `format!()` have been replaced with
  invocation `format_args!()` because `format_args!()` avoids heap
  allocations. All examples have been updated. The use of `format_args!()`
  has affected the code for writing to a file. Now `std::fmt::Arguments`
  is used instead of `String`.
  [See example](https://github.com/vglinka/nolog-plain/blob/main/examples/as-dep-to-file/src/logger.rs).
- **0.1.2** – Changes in `README.md` and examples.
- **0.1.1** – Changes in `README.md` and examples.
