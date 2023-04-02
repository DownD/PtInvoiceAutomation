from setuptools import find_packages
from distutils.core import setup

setup(
    name='invoice_parser',
    version='0.1',
    packages=find_packages(),
    install_requires=[
        "pydantic>=2.0",
        "qreader>=3.10",
        "opencv-python>=4.7",
        "pandas>=2.0",
        "pdf2image>=1.16",
        "openpyxl>=3.0"
    ],
    entry_points={
        'console_scripts': [
            # Add your command-line scripts here
            # Example: 'yourscript=yourpackage.module:main_function',
            'invoiceparser=invoice_parser.main:main',
        ],
    },
)


