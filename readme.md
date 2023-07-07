# Vector Database in Rust

## About

This project is a simple 3D vector database implemented in Rust. The purpose of this project is to provide a learning platform for exploring concepts around data structures, database operations, and indexation. This is not designed to be a high-performance or production-ready system but rather an illustrative tool for teaching and learning.

The database supports basic CRUD operations:

- `set` to create and add a new vector to the database.
- `get` to retrieve a vector by its index.
- `update` to update a vector given an index.
- `delete` to remove a vector given an index.

Moreover, the database has an indexing feature based on the magnitude of vectors. This allows fetching vectors by their magnitude. 

## Running the Project

To run this project, you'll need to have Rust installed on your machine. If you haven't already installed Rust, you can follow the official guide [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, navigate to the project directory via the terminal, and run the project using Cargo (Rust's package manager), which is included with the Rust installation:

```bash
cargo run
```

This will compile and run the project. The `main.rs` file includes a simple demonstration of creating, updating, deleting, and retrieving vectors in the database.

## Learning Journey

This project is part my plan to learn about implementing a basic database from scratch in Rust. Starting from basic vector operations to database CRUD operations,indexation based on vector magnitude, eventually I hope to make this a full fledged concurrent vector database.

## Contributions

While this project is mainly for learning purposes, any and all suggestions, corrections, and improvements are welcome! Feel free to open a pull request or an issue. 

## License

This project is licensed under the Apache License. Please see the `LICENSE` file for more details.