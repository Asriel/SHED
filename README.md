# SHED: Self Hosted Executable Database
A portable key-value store which is appended to the end of an executable file. Currently only working
with ELF Executables.

## Example Code
```rust
// Create a new SHED
let mut shed_example = shed::Shed::new();
// Test wheather this executable already has a SHED
if !shed_example.shed_exists() {
    shed_example
        .initialize_shed()
        .expect("Failed to initialize SHED");
}

// Create a key value store with the value type String
let mut x: shed::Store<String> = shed::Store::new();
x.insert(String::from("Test Key"), String::from("Test Value"));

// Write our key store to the SHED
shed_example
    .write_shed(x)
    .expect("Failed to write data to SHED");

// Read key value store from SHED
let read_shed: shed::Store<String> =
    shed_example.read_shed().expect("Failed to read from SHED");
println!("{:?}", read_shed);
```

## To Do
- [x] Get fully working on Linux
- [ ] Add support for Windows PE executables
- [ ] Make API look nicer
- [ ] Write some documentation

## Warning
This crate is experimental, since it involves an executable file modifying
itself it may or may not trigger antivirus products. So far it hasn't triggered
any of the major antivirus products so it should be good to use.