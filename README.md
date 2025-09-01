# Virtual Machine in Rust

A simple stack-based virtual machine implemented in Rust that executes custom assembly-like programs. It supports basic stack operations, arithmetic instructions, and error handling for a lightweight, educational VM.

## Features

- **Stack-Based Operations**: Push, pop, and manipulate a stack of integers.
- **Arithmetic Instructions**: Add, subtract, multiply, and divide with overflow and division-by-zero checks.
- **Custom Instruction Set**: Includes PSH (push), POP, ADD, SUB, MUL, DIV, SET (placeholder), HLT (halt), and error handling for unknowns.
- **File-Based Execution**: Loads and runs programs from `.vm` text files.
- **Comprehensive Tests**: Includes unit tests for various scenarios like errors, operations, and edge cases.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone the repository:
   ```
   git clone https://github.com/omniflare/virtual-machine-rust.git
   cd virtual-machine-rust
   ```

## Usage

1. Build the project:
   ```
   cargo build
   ```

2. Create a simple `.vm` program file, e.g., `example.vm`:
   ```
   PSH 10
   PSH 5
   ADD
   HLT
   ```

3. Run the VM with your program:
   ```
   cargo run -- example.vm
   ```
   Expected output: `Final Result: 15`

## Testing

Run the included tests to verify functionality:
```
cargo test
```

This will execute tests for various operations, error cases, and program files located in the `tests/` directory.

## Creator

Created by [omniflare](https://github.com/omniflare/).

## Repository

[GitHub Repository](https://github.com/omniflare/virtual-machine-rust)
