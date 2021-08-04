## serde-nix

serde-nix provides a way to serialize a Rust type implementing
[serde::ser::Serialize](https://docs.serde.rs/serde/ser/trait.Serialize.html)
into a nix expression.

This crate does not provide a deserializer due to evaluating arbitrary nix
expressions being a little more involved.

#### License

Code in this crate is partially derived from
[serde-rs/json](https://github.com/serde-rs/json) code, and so for simplicity
the same license is retained.

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
