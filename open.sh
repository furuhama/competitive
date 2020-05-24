cargo doc --open
open $(rustup toolchain list -v | rg 1.42.0 | cut -f 2)/share/doc/rust/html/std/index.html
