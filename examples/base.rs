/*
In Cargo.toml:
  ```
  nolog-plain = "0.2.1"
  ```

In main.rs:
  ```
  use nolog_plain::*;
  ```

OR

  ```
  use nolog_plain::{crit, debug, error, info, trace, warn, writelog};
  ```

OR

  ```
  #[macro_use]
  pub extern crate nolog_plain;
  ```
*/

#![allow(unused_variables)]

use nolog_plain::*;

fn main() {
    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    /* Outout:
    TRCE: text 42,42,42 [examples/to-stderr.rs 55:5]
    DEBG: text 42,42,24 [examples/to-stderr.rs 56:5]
    INFO: text 42,24,42 [examples/to-stderr.rs 57:5]
    WARN: text 42,a,422 [examples/to-stderr.rs 58:5]
    ERRO: text 42,42,42 [examples/to-stderr.rs 59:5]
    CRIT: text 42,42,42 [examples/to-stderr.rs 60:5]
    */
}
