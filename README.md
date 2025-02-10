# nd-dirsum
A simple CLI utility to generate a hash of a directory and its contents, recursively.

## Installing
Install Rust and Cargo, the run:
```bash
cargo install --git https://github.com/nathaniel-daniel/nd-dirsum
```

## Usage
```bash
nd-dirsum <path>
```

## Limitations
 * Only SHA256 hashes are supported
 * Paths must be unicode
 * Symlinks are not supported
 * Cannot skip files or directories
 * Hashes are not cross-platform
 * Does not take into account file metadata
 
## License
Licensed under either of
 * Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.
 
## Contributing
Unless you explicitly state otherwise, 
any contribution intentionally submitted for inclusion in the work by you, 
as defined in the Apache-2.0 license, 
shall be dual licensed as above, 
without any additional terms or conditions.