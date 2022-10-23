# snes_address

For converting between PC and SNES memory map addresses.

## Supported Memory Map Conversions

- [x] LoRom
- [ ] HiRom
- [ ] ExLoRom
- [ ] ExHiRom
Others to be added...

## Usage
### Rust Library:
Add dependency
`https://crates.io/crates/snes_address`
```
use snes_address::errors::AddressError;

fn lorom_to_pc(lorom_address: usize) -> Result<usize, AddressError> {
    snes_address::lorom_to_pc(lorom_address)
}

fn compress_data(pc_address: usize) -> Result<usize, AddressError> {
    snes_address::pc_to_lorom(pc_address)
}
```

### Shared Library:
TODO: Add wrapper project to compile this to so (linux), dll (windows), and dylib (mac)

### Binary:
`cargo install snes_address`
```
Usage:");
    snes_address [option] <address_in_hex>

Options:
    -P2L: decompress
    -L2P: LoRom to PC
```

