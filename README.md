![Crates.io](https://img.shields.io/crates/v/sponge_string.svg?label=CrAtEs.Io&style=flat-square)
# sponge_string_rs
HeY tHeRe! YoU cAn UsE tHiS cRaTe To CrEaTe SpOnGe TeXt.

## InStAlLaTiOn
AdD tHe FoLlOwInG iN yOuR `Cargo.toml`:
```toml
[dependencies.sponge_string]
version = "*"
```
YoU cAn RePlAcE `*` wItH tHe VeRsIoN aT tHe ToP oF tHiS rEaDmE.

## ExAmPlE
```rust
extern crate sponge_string;

use sponge_string::jumble;

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut line = "".to_owned();

        match stdin.read_line(&mut line) {
            Ok(_) => (),
            Err(e) => {
                println!("The following error has occured: {:?}", e);
                continue
            }
        }

        println!("{}", jumble(&line));
    }
}
```