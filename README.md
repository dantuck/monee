# monee [![Latest Version]][crates.io] [![Docs]][docs.rs] [![Conduct svg]][Code of Conduct]
Rust Money parser

[Latest Version]: https://img.shields.io/crates/v/monee.svg
[crates.io]: https://crates.io/crates/monee
[Docs]: https://docs.rs/monee/badge.svg
[docs.rs]: https://docs.rs/monee
[Conduct svg]: code-of-conduct.svg
[Code of Conduct]: CODE_OF_CONDUCT.md

A library that handles parsing and display money.

## Usage


```rust

money!("20", "USD");                            // 20.00

```

## Formatting

```rust

let money = money!("20.00", "USD");
format!("{: >1}", money);                       // $ 20.00

```