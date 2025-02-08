# Rust Dataset Loader

A Rust application for downloading and processing datasets from Hugging Face Hub, with built-in support for Parquet files using Polars.

## Features

- Download datasets from Hugging Face Hub
- Automatic handling of dataset directory structure
- Skip existing files during download
- Progress tracking for downloads
- Parquet file processing capabilities using Polars

## Prerequisites

- Rust (2021 edition)
- Cargo

## Installation

1. Clone the repository
2. Install dependencies:

```bash
cargo build
```

## Dependencies

The project uses several key dependencies:
- `anyhow` - For error handling
- `hf-hub` - Hugging Face Hub API client
- `polars` - Data manipulation library with Parquet support
- `candle` - Machine learning toolkit

## Usage

To download a dataset:

```rust
use loader::load_dataset;

fn main() -> anyhow::Result<()> {
    load_dataset("")?;
    Ok(())
}
```

The dataset will be downloaded to the `datasets` directory in your project root.

## Project Structure

- `src/loader.rs` - Contains the dataset loading functionality
- `src/main.rs` - Main application entry point
- `datasets/` - Directory where downloaded datasets are stored

## Error Handling

The application uses `anyhow` for error handling and provides detailed error messages for:
- Directory creation failures
- Dataset fetch failures
- File copy operations
- API connection issues

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a pull request.
