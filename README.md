# nifpga-dll-rs

Unofficial Rust bindings for `NiFpga.dll`.

`nifpga-dll-rs` is an unofficial Rust library for interacting with the APIs exposed by NI’s `NiFpga.dll`. It provides Rust bindings that allow applications to open FPGA sessions, read from and write to registers, and transfer data to and from FIFOs.

The project is composed of three crates:

- `nifpga-dll`
- `nifpga-dll-sys`
- `nifpga-dll-type-macro`

## Disclaimer

`nifpga-dll-rs` is **not** an official NI interface.

This project is an independent and autonomous development and is **not** affiliated with, endorsed by, or maintained by NI. To use `nifpga-dll-rs`, you must have the official `NiFpga.dll` library provided by NI installed on your system.

## License and acknowledgments

`nifpga-dll-rs` is distributed under the MIT License.

This project is a derivative work that incorporates code and ideas from the following MIT-licensed projects:

- [`nifpga-rs`](https://github.com/DBTaylor/nifpga-rs), authored by [David Taylor](https://github.com/DBTaylor), also available on [crates.io](https://crates.io/crates/nifpga)
- [`ni-fpga-rs`](https://github.com/first-rust-competition/ni-fpga-rs), authored by [Connor Worley](https://github.com/connorworley), also available on [crates.io](https://crates.io/crates/ni-fpga)

Special thanks to [@teweitsai](https://github.com/teweitsai) for updating the codebase, IRQ support, and extending compatibility to Linux.

## Contributions and maintenance

If NI, or anyone else, would like to replace this crate with an official or alternative Rust implementation, contribute improvements, or become a maintainer, you are very welcome.

Please feel free to contact us for any inquiry, proposal, or request.


## FAQ ##

### Where I can find the `signature` of my FPGA bit file? ###

The NI FPGA `.lvbitx` files are XML text files.
The `signature` that you need to open the FPGA Session you can find in the  `.lvbitx` file by searching for `<SignatureRegister>`.

### Where I can find address for registers and FIFOs? ###
The NI FPGA `.lvbitx` files are XML text files.
There you can find the address of all objects of your bit file.

## Example: ##


### Notes

On Windows, the `raw-dylib` feature must be enabled in order to link against `NiFpga.dll`.

Add the following dependency to your project's `Cargo.toml`:

```toml
nifpga-dll = { version = "0.4.0", features = ["raw-dylib"] }
```

### Usage

```rust
extern crate nifpga_dll;

use nifpga::{NifpgaError, Session};

fn main() -> Result<(), NifpgaError>{
//open the session
//it will be closed when it goes out of scope
let session = Session::open(
"/home/admin/fpga.lvbitx",
"0DA1668CDC2C6C492F1437AE6DC2151E",//signature search <SignatureRegister> in the .lvbitx
"RIO0",
true, //run on open
true //reset on close
)?;

    //reserve an IRQ context and wait on IRQ 0
    //the context will be unreserved when it goes out of scope
    session.reserve_irq_context()?.wait_on_irqs(1, 1000)?;
    
    //acknowledge IRQ 0
    session.acknowledge_irqs(1)?;

    //open a target-to-host FIFO
    //this configures the FIFO and starts it
    //it will be stopped when it goes out of scope
    let (reader, _) = session.open_read_fifo::<f32>(0, 100)?;
    
    //open a host-to-target FIFO
    let (writer, _) = session.open_write_fifo::<f32>(1, 100)?;
    
    //write 2 values to the FIFO
    writer.write(&[5.0, 5.0], 1000)?;

    // unsafe{
    //     //acquire 4 elements in the host write buffer
    //     //these elements will not be sent to the target until they go out of scope
    //     //this method is unsafe because elements must be dropped in the order they are acquired
    //     let( elements, _, _) = writer.acquire_elements(4, 1000)?;
    //     elements.slice.iter_mut().for_each(|el| {*el = 1.0});
    // }
    
    //read 3 elements from the FIFO
    let mut read_buff = [0.0; 3];
    reader.read(&mut read_buff, 1000)?;
    println!("{:?}", read_buff);
    // unsafe{
    //     //acquire 3 elements in the host read buffer
    //     //this section of the buffer will not be available for the FIFO until these elements are dropped
    //     //this method is unsafe because elements must be dropped in the order they are acquired
    //     let(elements, _, _) = reader.acquire_elements(3, 1000)?;
    //     println!("{:?}", elements.slice)
    // }
    
    //write to control x18004
    session.write::<f32>(0x18004, 5.0)?;
    //read from indicator x1800C
    println!("{:?}", session.read::<f32>(0x1800C)?);

    //write contents of array to control x18000
    let array = [5.0, 5.0];
    session.write_array::<f32>(0x18000, &array)?;

    //read indicator 0x18008 into array
    let mut array = [0.0; 2];
    session.read_array::<f32>(0x18008, &mut array)?;
    println!("{:?}", array);

    Ok(())
}
```
