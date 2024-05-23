"""
This script is to generate the PYTHONPATH for the armory_atlas package and its dependencies.

This was needed since the library PyO3 does not support non builtin modules without specifying where they are via the
PYTHONPATH env var.
"""

import os
import sys
from typing import List


def get_abs_paths(paths: List[str]) -> List[str]:
    """
    Gets the absolute paths of the provided paths.
    :param paths: List of paths
    :return: List of absolute paths
    """
    return [os.path.abspath(path) for path in paths]


def find_modules(paths: List[str]) -> List[str]:
    """
    Finds all Python modules in the provided path.
    :param paths: List of paths
    :return: List of Python modules
    """
    modules = []
    for path in paths:
        for root, dirs, files in os.walk(path):
            if any(file.endswith(".py") for file in files):
                modules.append(root)
    return modules


def generate_python_path(paths: List[str]) -> None:
    """
    Generates the PYTHONPATH for the armory_atlas package and its dependencies.
    :param paths: List of paths
    :return: None
    """
    current_pythonpath = os.environ.get('PYTHONPATH', '')
    abs_paths = get_abs_paths(paths)
    modules = find_modules(abs_paths)
    new_pythonpath = ":".join(modules)

    if current_pythonpath:
        new_pythonpath = f"{current_pythonpath}:{new_pythonpath}"

    os.environ['PYTHONPATH'] = new_pythonpath
    print(f"export PYTHONPATH=\"{os.environ['PYTHONPATH']}\"")


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python generate_pythonpath.py <dir1> <dir2> ... <dirN>")
        sys.exit(1)

    directories = sys.argv[1:]
    generate_python_path(directories)


if __name__ == "__main__":
    main()
