# ABNF Toolkit

## Description
ABNF Toolkit is a versatile tool designed for testing ABNF (Augmented Backus-Naur Form) formal grammars.

## Features

- **Syntax Checking:** Lex ABNF files and detect any syntax errors.

## Roadmap

- **Grammar Testing:** Test grammars against their rules to ensure adherence.
- **Random Grammar Generation:** Generate random valid grammars based on defined rules.

## Usage

To check the syntax of an ABNF file, provide the file path as an argument:

```bash
$ abnf-toolkit path/to/your/grammar.abnf
```

Replace path/to/your/grammar.abnf with the actual path to your ABNF file.

## Installation

To use ABNF Toolkit, follow these steps:

1. Clone the repository:

    ```bash
    $ git clone https://github.com/fadaei-dev/abnf-toolkit
    $ cd abnf-toolkit
    ```

2. Build the project using Cargo:

    ```bash
    $ cargo build --release
    ```

3. Run the binary:

    ```bash
    $ ./target/release/abnf-toolkit
    ```

Alternatively, you can install directly from GitHub by specifying the repository URL with the `--git` option:

```bash
$ cargo install --git https://github.com/fadaei-dev/abnf-toolkit
```

Now you have ABNF Toolkit installed and ready to use on your system.
 
## License

This project is licensed under the [MIT License. ](https://choosealicense.com/licenses/mit/) 

