from setuptools import setup, find_packages

setup(
    name='armoryatlas',
    version='0.1',
    packages=find_packages(),
    install_requires=[
        'mysql-connector-python',
        'toml',
        'keyring'
    ],
    entry_points={
        'console_scripts': [
        ]
    },
)
