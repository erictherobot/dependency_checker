# Dependency Checker

## Overview

Dependency Checker is a Rust program that traverses a specified directory, looking for JavaScript projects (those with a `package.json` file), checks their npm dependencies and outputs any outdated packages to a CSV report. Additionally, it generates a DOT file representing the dependency graph of the projects.

## Features

- Traverses specified directory to find JavaScript projects
- Checks for outdated npm dependencies
- Generates CSV report for outdated dependencies, including current and latest version numbers
- Generates a DOT file representing the dependency graph of the projects
- Skips `node_modules` directories for efficiency

## Installation

**Prerequisites:**

- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed

To install the tool, clone this repository and build the project:

```bash
git clone https://github.com/erictherobot/dependency_checker.git
cd dependency_checker
cargo build --release
```

## Usage

After installation, you can run the tool with the directory path as an argument:

```bash
cargo run --release /path/to/directory
```

After running, the tool will generate two files in the project root:

- `report.csv`: A CSV file containing the report of outdated packages, including the project paths, package names, current versions, and latest versions
- `dependency_graph.dot`: A DOT file representing the dependency graph of the projects

You can view the DOT file using a tool such as Graphviz.

## Contributing

Contributions to this project are welcome. Please open an issue to discuss proposed changes or report bugs.

## License

This project is licensed under the [MIT License](LICENSE).
