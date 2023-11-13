
# Rust Project

This is a Rust project named "my_project" that contains the following files and code functions:

## Rust Source Code
- **Proper usage of Rust syntax**: The project effectively utilizes Rust's rich syntax, demonstrating clear and concise use of functions, structs, and modules. The code is organized and readable, adhering to Rust's idiomatic practices.
- **Effective error handling in Rust**: Error handling is robust, leveraging Rust's Result and Option types. The use of match expressions and the ? operator ensures that errors are handled gracefully, enhancing the reliability of the database operations.
- **Implementation of Rust's unique features**: The project showcases several of Rust's unique features, including ownership, borrowing, and lifetimes. The use of these features contributes to the safety and efficiency of the code, particularly in managing database connections and query execution.

## SQLite Database and CRUD Operations
- **Rust Code**: The project is written in Rust, a language known for its safety and performance. Rust's powerful type system and ownership model ensure memory safety and thread safety.
- **CRUD Operations**: The `lib.rs` file contains the core logic for interacting with the SQLite database. It defines a `Database` struct with methods for each of the CRUD operations:
  - **Create**: Insert new records into the database.
  - **Read**: Fetch existing records from the database.
  - **Update**: Modify existing records in the database.
  - **Delete**: Remove records from the database.

## Use of GitHub Copilot
Throughout the development of this project, GitHub Copilot was extensively used to streamline the coding process. 
- **Explanation**: Copilot was particularly helpful in suggesting Rust-specific syntax and idiomatic code patterns, making the code more efficient and maintainable.
- **Run**: Copilot assisted in writing the build and execution scripts, streamlining the process of compiling and running the program. It offered insights into best practices for Rust project setup and execution, which was particularly helpful for newcomers to the language.
- **Dependencies**: Copilot was instrumental in identifying and suggesting the necessary Rust crates. It provided guidance on version compatibility and syntax for adding dependencies to Cargo.toml. 

## Optimized Rust Binary
The project uses GitHub Actions, configured in .github/workflows/rust.yml, to automate the build and testing process. The workflow compiles the Rust code in release mode, creating an optimized binary. This binary is then made available as a downloadable artifact in GitHub Actions, ensuring easy access and deployment.

## Github Actions
- **Correct testing of Rust code**
- **Correct building of Rust code**
- **Correct linting of Rust code**

## Demo Videos


## `src` Folder and Files

The `src` folder contains the source code for the Rust project. Typically, it includes two main files:

- `lib.rs`: This file often contains library code and data structures.
- `main.rs`: This file typically includes the entry point for the application, sets up the project, and runs the program.

You can add your project-specific code to these files to build your Rust application.

## `Makefile`

The `Makefile` is used to simplify and automate common development tasks. In your project, the Makefile includes the following targets:

- `format`: Formats the code using `cargo fmt --quiet`.
- `lint`: Lints the code using `cargo clippy --quiet`.
- `test`: Runs tests using `cargo test --quiet`.
- `run`: Runs the project using `cargo run`.
- `run-release`: Runs the project in release mode using `cargo run --release --bin my_binary`.
- `build-release`: Builds the project in release mode using `cargo build --release`.
- `all`: A convenience target that runs `format`, `lint`, `test`, and `run` in that order.

These Makefile targets make it easy to format, lint, test, and run your Rust project by simply running `make` commands in your terminal.
