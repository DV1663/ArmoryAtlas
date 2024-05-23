# Armory Atlas

This is a CLI tool to manage and interact with the inventory system Armory Atlas

## Usage

After building and installing the binary or the python module you can run the app and either pass

```shell
-h # or --help
```

Or 

```shell
help
```

to the app to see the help screen to see what options you have to run

### Python Library Usage

The Python library exposes a few classes and functions to use, see the [stub file](./armory_atlas/armory_atlas_lib.pyi) to see all the functions and classes.

The main function that you will need to use is `run_cli()`. This function will start the CLI tool. It takes one argument which is the argument to pass to the CLI, either you can pass a list of arguments or what's recomended to pass is the entire `sys.argv` list so that it will pick everything up corrcetly.

## Building

### Requirments

- Rust
- Maturin (pymodule)
- Python

### Building Python Module (recommended)

Building the Python module is recomended since this will bypass the need of using the `PYTHONPATH` env var to declare what external modules are needed to run the project with Rust. This will be true for as long as the database handler is written in Python and not in Rust.

Since we build a python module, we need to write some code after we have installed the library we have built. See [Python Library Usage](#python-library-usage) for more details.

#### Step 1—Install Dependencies

First, we need to install Rust and Maturin. See guides here: 

#### Step 2—Build the Project

Run the following in the root of the repository

```shell
$ cd armory_atlas
$ maturin build -r # the -r is to build the release version
```

You can also run the command below to build and install the wheel. This will install the wheel for your current Python interperter.

```shell
$ maturin develop -r
```

#### Step 3—Install the wheel

After the build is done, we need to install the wheel (unless you used `maturin develop -r`).

```shell
$ pip install ./target/wheels/<name_of_wheel>.whl
```

### Building Rust Binary

Building and using the Rust binary is only recommended if you are okay with adding the `PYTHONPATH` env var to your system. Adding the `PYTHONPATH` env var will allow the Rust binary to dynamicly link Python and still know where the external modules are on the system. A side effect of this is that every Python enviroment you have will have access to the external modules added in the `PYTHONPATH` env var.

If you are curious about what Python modules are added to the `PYTHONPATH` env var, you can see the [`pymodules`](./pymodules) folder, some modules may not be needed anymore, but they are still kept in the project in case they are needed in the future.

Building the Rust binary will provide a complete app to use and wil not require any other code to run.

#### Step 1—Install Dependencies & `PYTHONPATH`

First, we need to install Rust and Set up the `PYTHONPATH` env var. See guides here:

##### Step 1.2—Set up the `PYTHONPATH` env var

We have provided a script to generate the correct `PYTHONPATH` env var.

```shell
$ python3 generate_python_path.py
```

Copy and paste the output from the script into your `.bashrc` or `.zshrc` file. This should make it so the `PYTHONPATH` env var is set up in your system. Remeber to source `.bashrc` or `.zshrc` to apply the changes, alternatively you can restart your terminal.

On Windows you need to modify the env var using `control panel`

#### Step 2—Build the Project

Run the following in the root of the repository

```shell
$ cargo build -r
```

This will build the project for release and put the output into the `target/release` folder.

We can alternetivly build and install the project using `cargo`.

```shell
$ cargo install --path .
```

## Installing module from wheel

To install the project from a wheel, this can either be from a wheel built on your system or a pre-built wheel provided on [GitHub Releases](https://github.com/DV1663/ArmoryAtlas/releases). 
Download the wheels.zip file and then either find the correct wheel and run the command:

```shell
$ pip install /path/to/wheel/<name_of_wheel>.whl
```

Or use the script provided in the repository called [install_wheel.py](./install_wheel.py) note that this could fail and if it does you will need to install the correct wheel manually.

## Installing Rust

See [Rust Lang install guide](https://www.rust-lang.org/tools/install)

### UNIX Systems

On UNIX systems we can use this command to install the Rust toolchain:

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
*This is taken from the [Rust Lang install guide](https://www.rust-lang.org/tools/install) at the moment of writing*

## Installing Maturin

See [Maturin install guide](https://www.maturin.rs/installation) or run the following

```shell
$ pip install maturin
```

or from cargo

```shell
$ cargo install maturin
```