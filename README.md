# aig (AI Gauge)

A minimalist, concurrent CLI tool for tracking LLM API usage and costs.

`aig` is built with Rust and designed to be fast, silent by default, and composable. It adheres strictly to the UNIX philosophy: do one thing well, support plain text and JSON outputs, and route all errors to `stderr`.

Currently supported providers:
- Anthropic
- Google Gemini
- OpenAI

## Features

- **Concurrent fetching**: Uses `tokio` to fetch data from all configured providers simultaneously.
- **Multiple providers**: Easily extensible modular provider system.
- **JSON & Plain text output**: Choose between human-readable summaries or structured JSON for piping into other tools.
- **Configuration flexibility**: Supports both environment variables and a TOML configuration file.

## Installation

Ensure you have Rust and Cargo installed, then clone the repository and build:

```bash
git clone https://github.com/yourusername/aig.git
cd aig
cargo build --release
```

The executable will be located at `target/release/aig`. You can move it to a directory in your `PATH` (e.g., `~/.local/bin` or `/usr/local/bin`).

## Configuration

You can configure `aig` using either environment variables or a configuration file. Environment variables take precedence over the config file.

### Environment Variables

Set the following environment variables for the providers you wish to use:

- `ANTHROPIC_API_KEY`
- `GEMINI_API_KEY`
- `OPENAI_API_KEY`

Example:
```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

### Configuration File

Create a configuration file at `~/.config/aig/config.toml` (or your OS's equivalent standard config directory).

```toml
[anthropic]
api_key = "sk-ant-..."

[gemini]
api_key = "AIza..."

[openai]
api_key = "sk-..."
```

## Usage

If no options are provided, `aig` prints a simple summary of today's total cost across all configured providers.

```bash
$ aig
Today's Total Cost: $1.2345
```

### Detailed Breakdown

To see a breakdown per provider and model, use the `list` subcommand:

```bash
$ aig list
Anthropic:
  Total Cost: $0.5000
    claude-3-opus-20240229: $0.4000
    claude-3-sonnet-20240229: $0.1000
OpenAI:
  Total Cost: $0.7345
    gpt-4-turbo: $0.5000
    gpt-3.5-turbo: $0.2345

Total Cost (All Providers): $1.2345
```

### JSON Output

For use in scripts or combining with tools like `jq`, use the `--json` flag:

```bash
$ aig --json
[
  {
    "provider_name": "Anthropic",
    "total_cost": 0.5,
    "model_costs": {
      "claude-3-opus-20240229": 0.4,
      "claude-3-sonnet-20240229": 0.1
    },
    "error": null
  },
  {
    "provider_name": "OpenAI",
    "total_cost": 0.7345,
    "model_costs": {
      "gpt-4-turbo": 0.5,
      "gpt-3.5-turbo": 0.2345
    },
    "error": null
  }
]
```

## Philosophy

- **Minimalist**: Does only what is necessary to track costs.
- **Composable**: Designed to be piped to and from other UNIX tools.
- **Silent by default**: Successful executions print only the requested output to `stdout`.
- **Proper error handling**: All errors and warnings are written to `stderr`. Non-zero exit codes are returned on failure.
- **Fast**: Written in Rust, utilizing async I/O for parallel requests.

## Development

Standard `cargo` commands apply:

- `cargo build`
- `cargo test`
- `cargo check`
- `cargo fmt`

## License

This project is licensed under either of the MIT or Apache 2.0 licenses.
