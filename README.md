Parsing of chronorace.be results into a CSV file. Format of the file is the
format expected by stratenlopen.be.

# Prereqs

You will need Rust to compile this.  First install
[rustup](https://www.rustup.rs/), which helps you manage Rust installations.  I
do not recall if that also already install a rust version. If not, `rustup
install stable` probably works. If it does not, follow rustup's instructions.

# Keep Things In Order

* Run the tests with `cargo test`.
* Run `rustfmt` to adhere to Rust's "proper" styles.
* Run [clippy](https://github.com/Manishearth/rust-clippy). Note that it only
  works with Rust nightly. This is why it is a good idea to use rustup, you can
  easily install stable and nightly side-by-side and manage them easily. Then
  it is a matter of `rustup run nightly cargo install clippy` to install clippy
  and `rustup run nightly cargo clippy` to run the linting tool.

# Use

Compile for release with `cargo build --release`. You will find the binary in the
`target/` folder. Run it with `./rust-chronorace RESULTSURL`.
