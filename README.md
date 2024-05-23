# Armory Atlas

Armory Atlas is a CLI tool designed to manage and interact with the inventory system. It provides an intuitive command-line interface and a Python library for more advanced usage.

## Usage

After building and installing the binary or the Python module, you can run the app and pass either:

```shell
-h  # or --help
```

or 

```shell
help
```

to see the help screen with available options.

### Python Library Usage

The Python library exposes several classes and functions. Refer to the [stub file](./armory_atlas/armory_atlas_lib.pyi) for a complete list of functions and classes.

The main function to use is `run_cli()`. This function starts the CLI tool. It takes one argument, which can be a list of arguments or, preferably, the entire `sys.argv` list to capture all inputs correctly.

## Building

### Requirements

- [Rust](#installing-rust)
- [Maturin (for Python module)](#installing-maturin)
- Python

### Building Python Module (Recommended)

Building the Python module is recommended to avoid setting the `PYTHONPATH` environment variable. This is necessary as long as the database handler is written in Python.

#### Step 1: Install Dependencies

First, install Rust and Maturin. Follow the guides [here](https://www.rust-lang.org/tools/install) and [here](https://www.maturin.rs/installation).

#### Step 2: Build the Project

Run the following commands in the root of the repository:

```shell
cd armory_atlas
maturin build -r  # the -r flag builds the release version
```

Alternatively, you can build and install the wheel:

```shell
maturin develop -r
```

#### Step 3: Install the Wheel

If you didn't use `maturin develop -r`, install the wheel manually:

```shell
pip install ./target/wheels/<name_of_wheel>.whl
```

### Building Rust Binary

Building the Rust binary requires setting the `PYTHONPATH` environment variable, allowing the Rust binary to link Python dynamically and access external modules.

#### Step 1: Install Dependencies & Set Up `PYTHONPATH`

Install Rust and set up the `PYTHONPATH` environment variable:

```shell
python3 generate_python_path.py
```

Copy and paste the script output into your `.bashrc` or `.zshrc` file, then source the file or restart your terminal.

On Windows, modify the environment variable through the Control Panel.

#### Step 2: Build the Project

Run the following command in the root of the repository:

```shell
cargo build -r
```

Alternatively, you can build and install the project using `cargo`:

```shell
cargo install --path .
```

## Installing Module from Wheel

To install the project from a wheel (either built on your system or pre-built from [GitHub Releases](https://github.com/DV1663/ArmoryAtlas/releases)):

Download the `wheels.zip` file, find the correct wheel, and run:

```shell
pip install /path/to/wheel/<name_of_wheel>.whl
```

Or use the provided script [install_wheel.py](./install_wheel.py). Note that manual installation may be required if the script fails.

## Installing Rust

Follow the [Rust Lang install guide](https://www.rust-lang.org/tools/install).

### UNIX Systems

Use this command to install the Rust toolchain:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*This command is from the [Rust Lang install guide](https://www.rust-lang.org/tools/install) as of writing.*

## Installing Maturin

Refer to the [Maturin install guide](https://www.maturin.rs/installation) or run:

```shell
pip install maturin
```

Or install from cargo:

```shell
cargo install --locked maturin
```
