# fifio_queue
Generic Implementation of fifo in rust for learning purpose

Even though it's small code in rust ,it's reveal lots about rust Capability

# Unit Testing 
via test macro so that we can call cargo test to run all the unit testing simultaneously

# Cross Compilation
Developing appl for different platform from different platform i.e From windows we can cross compile to MAC,Linux without them.
cargo add target aarch64-apple-darwin
cargo build --target aarch64-apple-darwin

# Calling external function from library.
for example is_empty_older can't used in main.rs because they are private fifo module.

# Cargo 
Package manager and build tool.

# Gurantee You Get
You can't pass any type to the FIFO queue but only the type that both implements Clone and Debug trait.No allocation is allocated as long as it doesn't exceed
provided bytes i.e 20.

## Usage

``` cargo r or cargo run ``` to run the program inside a editor

```cargo test``` for testing.

If you want to see the result without installing rust then go the target/release directory then double click the executable to see the results.Don't worry the thread running slowly to show the results.
