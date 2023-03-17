# fifio_queue
Generic Implementation of fifo in rust for learning purpose

Even though it's small code in rust ,it's reveal lots about rust Capability

Unit Testing via test macro so that we can call cargo test to run all the unit testing simultaneously

# Cross Compilation
Developing appl for different platform from different platform i.e From windows we can cross compile to MAC,Linux without them.

Calling external function from library.
for example is_empty_older can't used in main.rs because they are private fifo module.
cargo build --target aarch64-apple-darwin

Cargo for productivity
