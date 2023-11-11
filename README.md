# Rust Lambda HTML Backend Template

This project is a Rust-based Lambda functions for serving HTML content. It uses the crate cargo-lambda for a great developer experience. It is structured to work with AWS Lambda and utilizes HTML templates for content generation. There is also a nix flake with the dependencies. Recommend to use with nix-direnv.

## Getting Started

### Prerequisites

- Rust: Follow the [official guide](https://www.rust-lang.org/tools/install) to install Rust.

### Building and Running

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project and start developing using Cargo:
   ```
  cargo-lambda lambda watch
   ```

## Project Structure

- `src/`: Contains the Rust source code for the Lambda function.
- `templates/`: HTML templates used by the Lambda function.
- `Cargo.toml`: The Rust project manifest file.
- `.envrc`, `.gitignore`, `Cargo.lock`, `flake.lock`, `flake.nix`: Configuration and lock files.

## Contributing

Feel free to fork the project and submit pull requests.

## License

This project is licensed under [MIT License](LICENSE).
