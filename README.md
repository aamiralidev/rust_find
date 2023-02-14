
# rust_find

`rust_find` is a command-line application developed using the Rust programming language. The tool is designed to find files that match a specified pattern in a list of directories and outputs the matching files to a file. The tool also supports an optional minimum file size parameter.

## Usage

To use `rust_find`, run the following command:

```bash
rust_find --dirs [directory list] --pattern [file pattern] [--size [minimum size]] [--output [output file name]]
``` 

Here, `--dirs` and `--pattern` are required parameters, while `--size` and `--output` are optional parameters.

-   `--dirs`: A list of directories to search in, separated by commas. At least one directory is required.
-   `--pattern`: The pattern to match files against.
-   `--size` (optional): The minimum size of files to include in the output. The default value is 0 bytes.
-   `--output` (optional): The name of the output file. If not specified, the output will be printed to the console.

For example, to search for all text files with a minimum size of 10 KB in the directories `~/Documents` and `~/Downloads`, and output the results to a file called `output.txt`, run the following command:

bashCopy code

`rust_find --dirs ~/Documents,~/Downloads --pattern "*.txt" --size 10k --output output.txt` 

## Installation

To install `rust_find`, follow these steps:

1.  Clone this repository:

```bash
git clone https://github.com/aamirali-dev/rust_find.git
``` 

2.  Build the application:

```bash 
cargo build --release
``` 

3.  Run the application:

```./target/release/rust_find [arguments]``` 

## Contributing

If you find any issues or have any suggestions for the tool, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/aamirali-dev/rust_find/blob/master/LICENCE) file for details.
