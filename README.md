# Rust Web Server

This project is a simple web server implemented in Rust. It is designed to handle HTTP requests and serve static files.

## Features

- Handles GET and POST requests
- Serves static files from a specified directory
- Basic routing capabilities

## Requirements

- Rust (latest stable version)
- Cargo (Rust package manager)
- Postgres (latest version)
  
## Testing

You can use the `rest.http` file to test the HTTP endpoints of the server. Make sure you have the `REST Client` extension installed in your code editor.

## Database Configuration

Make sure to set up your Postgres credentials in a `.env` file in the root directory of the project. The `.env` file should contain the following variables:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

Replace `username`, `password`, and `database_name` with your actual Postgres credentials.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rust-web-server.git
    cd rust-web-server
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run database migrations:
    ```sh
    cargo install sqlx-cli
    sqlx migrate run
    ```

## Usage

1. Run the server:
    ```sh
    RUST_LOG=info cargo run
    ```

2. Open your web browser and navigate to `http://localhost:8080`.

## Configuration

You can configure the server by editing the `config.toml` file. Here you can set the port number, the directory for static files, and other settings.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)
- [Hyper](https://hyper.rs/) - Fast and safe HTTP implementation for Rust
