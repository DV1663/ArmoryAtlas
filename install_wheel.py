import os
import platform
import sys
import glob
import subprocess


def get_system_info():
    """Get the current system information."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    python_version = f"cp{sys.version_info.major}{sys.version_info.minor}"

    if "pypy" in sys.version.lower():
        python_version = f"pp{sys.version_info.major}{sys.version_info.minor}"

    return system, machine, python_version


def find_wheel_file(system, machine, python_version):
    """Find the correct wheel file based on system info."""
    base_dir = os.path.join(os.getcwd(), 'dist')
    possible_dirs = glob.glob(os.path.join(base_dir, f"wheels-{system}-{machine}*"))

    if not possible_dirs:
        raise FileNotFoundError("No matching wheels directory found.")

    wheels_dir = possible_dirs[0]
    pattern = os.path.join(wheels_dir, f"*{python_version}*.whl")
    wheel_files = glob.glob(pattern)

    if not wheel_files:
        raise FileNotFoundError(f"No matching wheel file found for Python version {python_version}.")

    return wheel_files[0]


def install_wheel(wheel_file):
    """Install the wheel file using pip."""
    subprocess.check_call([sys.executable, "-m", "pip", "install", wheel_file])


def main():
    system, machine, python_version = get_system_info()
    print(f"System: {system}, Machine: {machine}, Python Version: {python_version}")

    try:
        wheel_file = find_wheel_file(system, machine, python_version)
        print(f"Found wheel file: {wheel_file}")
        # install_wheel(wheel_file)
        print("Installation successful.")
    except (FileNotFoundError, subprocess.CalledProcessError) as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
