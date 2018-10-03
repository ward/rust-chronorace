Parsing of chronorace.be results into a CSV file. Format of the file is the
format expected by stratenlopen.be.

# Prereqs

You will need Rust to compile this.  First install
[rustup](https://www.rustup.rs/), which helps you manage Rust installations.  I
do not recall if that also already install a rust version. If not, `rustup
install stable` probably works. If it does not, follow rustup's instructions.

# Keep Things In Order

First, add some extra tools to `cargo`:

* [rustfmt](https://github.com/rust-lang-nursery/rustfmt): `rustup component
  add rustfmt-preview`
* [clippy](https://github.com/rust-lang-nursery/rust-clippy): `rustup component
  add clippy-preview`

Next, try to ensure all these work before pushing code.

* Run the tests with `cargo test`.
* Run `cargo fmt` to adhere to Rust's "proper" styles.
* Run `cargo clippy` for some linting.

# Use

Compile for release with `cargo build --release`. You will find the binary in the
`target/` folder. Run it with `./rust-chronorace RESULTSURL`.
