use Crates::add_one;

mod lib;
fn main() {
    println!("{}",add_one(4));
}

// cargo install to download some external binary crate from crates.io.
// A github account is needed to publish own crates on crates.io, which cannot be modified any further.
// The version though can be yanked with the cargo yank --{version}