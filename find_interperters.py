import platform
import subprocess
import re


def get_python_paths():
    # Run the 'maturin list-python' command and capture its output
    result = subprocess.run(['maturin', 'list-python'], text=True, capture_output=True)

    # Check if the command was executed successfully (returncode == 0)
    if result.returncode != 0:
        print("Error running 'maturin list-python'")
        return []

    # Extract the output
    output = result.stderr

    # Use a regular expression to match paths
    # This assumes that the paths are absolute and start with '/' or a drive letter for Windows
    found_paths = re.findall(r'(/[^ ]+|\w:\\[^ ]+)', output)

    return found_paths


def is_python_interpreter(path):
    try:
        output = subprocess.check_output([path, '-c', 'import platform; print(platform.python_implementation())'], stderr=subprocess.STDOUT)
        implementation = output.decode().strip()
        return implementation not in ["PyPy", "GraalVM"]
    except subprocess.CalledProcessError:
        return False
    except Exception as e:
        return False


def main():
    found_paths = get_python_paths()
    for path in found_paths:
        if is_python_interpreter(path.strip()):
            print(path.strip())


if __name__ == "__main__":
    main()