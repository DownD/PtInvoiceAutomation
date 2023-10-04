from setuptools import setup, find_packages

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
        # Add your project's dependencies here
        # Example: 'requests>=2.0',
    ],
    entry_points={
        'console_scripts': [
            # Add your command-line scripts here
            # Example: 'yourscript=yourpackage.module:main_function',
            'invoice_parser=invoice_parser.main:main',
        ],
    },
)


