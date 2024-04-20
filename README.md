## Part A: Full Translation of a Short C Program to Safe Rust

This folder (`workspace/short`) contains the extracted source code of a small C program chosen according to your student ID.

## How to Compile and Test the C program

```bash
cd workspace/short/orig_c

# compile
make

# test
./tests.sh
```

The `make` command generates 3 versions of binaries from the same C source: 
- (1) `<prog_name>` 
- (2) `<prog_name>_dbg`
- (3) `<prog_name>_opt`  

Notes on each of the binary:  
- We use (1) in the test script (`test.sh`) since it records line coverage. 
- You can use (2) for debugging (compiled with `-g` debug info).
- The last one (3) can be ignored for this project (compiled with optimizations for your reference).

## How to Create, Compile, and Run the Rust Program

Inside this folder (`workspace/short`), run the following:
```sh
# initialize current folder as a rust project
cargo init
# This creates a Cargo.toml file and a src directory with a main.rs file.

# debug build of the Rust program
cargo build

# run the Rust program
cargo run
cargo run -- <args>
# or
./target/debug/<prog_name>
./target/debug/<prog_name> <args>
```


## Notes about Translation

It can be easier to use chatGPT for quickly understanding the C program and creating a draft for the translation. You can also use chatGPT for learning Rust: ask for explanations of Rust language features in correspondence to the C code.


## Notes about Testing

**If you are assigned `shoco_lib_test.c` or `urlparser_lib_test.c`**: they are C libraries with a main function serving as the unit tests. These programs have sufficient unit tests hard-coded in the program. You don't need to add more tests but you need to make sure the Rust translation of the unit tests (the `main` function of `main.rs`) is correct.

**If you are assigned `csplit.c`, `expr.c`, `fmt.c`, `join.c`, `printf.c`, or `test.c`**: they are standalone C programs (not libraries) and the unit tests will externally execute these programs (with args, stdin, or file input). We have provided some tests in the `tests.sh` script as a starting point. You need to add more tests to reach 85% coverage at least.

## Notes about Debugging 

You are free to use any debugging tools you like. Here are some suggestions:

### Option 1. Debug Printing

Use `println!` (Rust) and `printf` (C) for debugging. If it interferes with the stdout, you can use `eprintln!` (Rust) and `fprintf(stderr, ...)` (C) to print to stderr. 

**NOTE:** When inserting debug print statements to C code, please be reminded to not change the behavior of the program. We assume the C program is functionally correct and the translation should preserve the behavior. Insert debug print statements to C is just for comparison with Rust code.

### Option 2. Debugging with `CodeLLDB` debugger in vscode

Assume you have installed `CodeLLDB` extension in vscode.  

`CodeLLDB` can debug both C program (built with `-g`) and Rust program (`Debug build`). 

To debug a Rust program while specifying command line arguments, one way is to add a configuration to `.vscode/launch.json`. For example:  

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug Rust program",
    "cargo": {
        "args": [
            "build",
            "--bin=short",
            "--package=short"
        ],
        "filter": {
            "name": "short",
            "kind": "bin"
        }
    },
    "args": [<arguments here>],
    "cwd": "${workspaceFolder}"
}
```

To debug the C program (`./orig_c/XXX_dbg`), you can also add a configuration to `.vscode/launch.json`. For example:  

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug C program",
    "program": "${workspaceFolder}/orig_c/XXX_dbg",
    "args": [<arguments here>],
    "cwd": "${workspaceFolder}"
}
```

You can check the provided recording on Canvas for more details, or ask chatGPT for debugging configurations in vscode.

### Option 3. Collect and visualize code coverage for Rust programs

Install `grcov`:
```sh
rustup component add llvm-tools-preview
cargo install grcov --locked
```

Build with coverage instrumentation:
```sh
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
```

Collect Coverage when executing the Rust program:
```sh
export LLVM_PROFILE_FILE="cov-%p-%m.profraw"
# run the program
./target/debug/<prog_name> <args1>
./target/debug/<prog_name> <args2>
...
# collect coverage data
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o lcov.info
```

The `lcov.info` file should be at the root folder of the Rust package (which should also be the root of your vscode workspace).

Open `main.rs` and press `F1` -> `Coverage Gutters: Display Coverage` to visualize the coverage.

### Option 4. Ask chatGPT  

While this is also an option, the current chatGPT can frequently point to the wrong places. However, it can still be helpful for understanding the code. We recommend to use chatGPT in combination with other debugging tools.


## Notes for Dependencies

If you need 3rd party libraries in addition to the standard library, please check the Clarifications slides to see if they are in the allow-list. If they are allowed, you can simply add them to `Cargo.toml` and use them in your Rust code. Otherwise, please contact with TAs to have a check first.