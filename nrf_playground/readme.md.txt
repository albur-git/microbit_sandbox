Note: this setup uses bare metal programming

For infos on microbit see: 
 - https://microbit.org/get-started/features/overview/
 - https://tech.microbit.org/hardware/schematic/

Basic toolchain setup following: https://training.tweede.golf/

A nice little intro to programming the microbit: https://www.youtube.com/watch?v=A9wvA_S6m7Y

now added microbit = "0.15.1"
 - for examples, see: https://github.com/nrf-rs/microbit/tree/ecfc7bcbec225208070b88e79d32b6fd53c38f56/examples

rustup: (see: https://rustup.rs/)
 - Install from website

rustfmt: formatting tool (see: https://github.com/rust-lang/rustfmt)
 - Install:  rustup component add rustfmt
 - Run: cargo fmt

clippy: collection of lints to analyze code (see: https://github.com/rust-lang/rust-clippy) 
 - Install: rustup component add clippy
 - Run: cargo fmt

thumbv7em-none-eabihf toolchain 
 - Install: rustup target add thumbv7em-none-eabihf

Install llmv tools:
 - LLVM (Low-Level Virtual Machine) is a powerful and flexible compiler framework used to develop various programming language compilers, including Rust, C, C++, Swift, and many others. It provides a collection of modular and reusable compiler and toolchain technologies.
 - llvm-tools includes a collection of tools from the LLVM ecosystem that can be used for various low-level analysis, instrumentation, and optimization tasks related to Rust programs. These tools are particularly useful for tasks like:
    - Generating assembly code from LLVM IR.
    - Profiling and analyzing Rust programs at the LLVM level.
    - Custom compiler optimizations and debugging.
 - Install: - rustup component add llvm-tools
 Install cargo-binutils.
 - cargo-binutils is a Rust tool that provides convenient wrappers for LLVM-based binary utilities (llvm-tools) through Cargo. It simplifies working with low-level binary analysis tools by allowing you to run them with cargo commands instead of calling them directly.
 - cargo-binutils integrates with Rust’s llvm-tools-preview to provide a seamless way to inspect and manipulate Rust binaries. It helps in tasks like:
    - Disassembling Rust binaries
    - Inspecting object files
    - Checking binary sizes
    - Profiling and debugging at the LLVM level
 - Install: cargo install cargo-binutils

Install cargo binstall (see: https://github.com/cargo-bins/cargo-binstall)
 - Binstall provides a low-complexity mechanism for installing Rust binaries as an alternative to building from source (via cargo install) or manually downloading packages.
 - Install: cargo install cargo-binstall 

(Install cmake) --> only needed when probe-rs shall be installed from source instead of using cargo binstall
 - Prerequeisite for probe-rs
 - Install from https://cmake.org/download/

Install probe-rs
 - probe-rs is an embedded debugging and flashing tool for Rust that allows developers to interact with microcontrollers (MCUs) using various debugging probes. It provides an alternative to traditional debugging tools like OpenOCD and integrates well with Rust’s embedded ecosystem.
    - Flashing firmware onto embedded devices.
    - Debugging programs running on microcontrollers.
    - Reading and writing memory of a target device.
    - Resetting and halting the target MCU.
    - GDB server mode, allowing debugging with tools like gdb and lldb.
    - Integrated with Cargo, making embedded development smoother.
 - Install by using cargo binstall: https://probe.rs/docs/getting-started/installation/
   --> cargo binstall probe-rs-tools
 - Install by using cmake and cargo install:
   --> cargo install probe-rs-tools --locked

note on cargo.toml
 - The rust analyzer can only properly resolve dependencies whenn they are definitive. So having a dependency with "optional = true" enabled via a feature in the cargo build command makes it inaccessible to the rust-analyzer

in project, add folder .cargo (do not miss the "."!) and then config.toml
 - Configures Cargo's behavior and build settings for the project, basically automatically adds stuff like "--target thumbv7em-none-eabihf" to the cargo build command
 - When you run cargo commands, Cargo reads this file to apply the specified configuration to the build process

in project, add rust-toolchain.toml
 - Specifies the Rust toolchain and its configuration for the project (Used By: rustup and Cargo.)
 - When you run cargo commands (e.g., cargo build), rustup reads this file to ensure the correct toolchain and components are installed and used.

 in project, add memory.x
 - specifies the memory regions of the target microcontrollers
 - The memory.x file in your project is a linker script or memory configuration file that defines the memory layout for your embedded system. 
 - The memory.x file is passed to the linker (usually via the -C link-arg=-Tmemory.x flag) during the build process. The linker uses this file to allocate memory for your program

In main.rs add:
 - #![no_main]
 - #![no_std]
 - #[cortex_m_rt::entry] to main function
 - #[panic_handler] and corresponding function
 - add exit function

Main function
 - Return value of type ! means the function never returns --> needs an infinte loop

Panic handler
- In embedded systems, there is no operating system to handle panics. You must define how the program behaves when a panic occurs.
- Without a panic handler, the program would enter an undefined state, which could lead to unpredictable behavior or a crash.
- The panic handler ensures that the program fails in a controlled manner, providing useful debugging information.

Exit function:
- This function is marked with the return type !, which means it never returns (it diverges).
- In embedded systems, there is no concept of "exiting" a program like in a desktop application. The program typically runs forever unless explicitly stopped.
- When a panic occurs, the program cannot simply exit; instead, it must enter a safe, controlled state.
- The exit() function ensures that the program halts in a predictable way, allowing you to debug the issue (e.g., by connecting a debugger to inspect the program state).

embedded-hal vs nrf52833-hal
 - The nrf52833-hal is a specialization of the embedded-hal
 - Most device-specific HAL crates implement the contracts defined in embedded-hal (Some HALs might not implement all embedded-hal traits if the hardware doesn’t support certain features)
 - this allows to write hardware agnostic code by making it dependant on embedded-hal instead of its device specific implementation that is used

nrf52833-hal vs embassy-nrf
 - This is the Hardware Abstraction Layer (HAL) for the nRF52833 microcontroller. It provides a high-level API to interact with the microcontroller's peripherals (e.g., GPIO, UART, SPI, I2C, etc
 - embassy-nrf (with nrf52833 feature) is part of the Embassy project, which provides an async/await runtime for embedded systems. It includes a HAL for the nRF52833 microcontroller, similar to nrf52833-hal, but with support for asynchronous programming

 add .vscode/settings.json to keep vscode from compiling for the wrong target 
  - vscode will occasionally compile the code in the background to check for errors
  - by standard it will do this for the native target system of the computer, which will result in false positive errors