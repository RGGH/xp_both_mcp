# Rust MCP Server using 'rmcp' 🦀

https://github.com/modelcontextprotocol/rust-sdk?tab=readme-ov-file#rmcp

A flexible server implementation supporting Model Context Protocol

## Features

- Multiple transport methods:
  - Server-Sent Events (SSE) over HTTP
  - Standard input/output (stdio)
- Configurable bind address for SSE server
- Adjustable logging levels
- Simple Counter service demonstration

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [How It Works](#how-it-works)
- [Creating Your Own Service](#creating-your-own-service)

## Installation

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Required dependencies will be automatically installed via Cargo

### Building the Project

```bash
# Clone the repository
git clone https://github.com/rggh/xp_both_mcp.git
cd rmcp-server

# Build the project
cargo build --release
```

## Usage

Run the server with default settings (SSE transport on 127.0.0.1:8000):

```bash
cargo run
```

### Command-line Options

```
Usage: rmcp-server [OPTIONS]

Options:
  -t, --transport <TRANSPORT>        Transport method to use [default: sse] [possible values: stdio, sse]
  -b, --bind-address <BIND_ADDRESS>  Bind address for SSE server (only used with sse transport) [default: 127.0.0.1:8000]
  -l, --log-level <LOG_LEVEL>        Log level (trace, debug, info, warn, error) [default: info]
  -h, --help                         Print help
  -V, --version                      Print version
```

### Examples

Start the server with SSE transport on a custom address:
```bash
cargo run -- --bind-address 0.0.0.0:9000
```

Use stdio transport:
```bash
cargo run -- --transport stdio
```

Set a specific log level:
```bash
cargo run -- --log-level debug
```

## Configuration

The server can be configured using command-line arguments:

| Option | Description | Default |
|--------|-------------|---------|
| `--transport` | Transport method (stdio, sse) | sse |
| `--bind-address` | Address for SSE server to bind to | 127.0.0.1:8000 |
| `--log-level` | Logging verbosity | info |

## How It Works

The server uses the RMCP framework to expose services over different transport methods:

1. **Command-line Parsing**: Uses `clap` to parse and validate arguments.
2. **Logging**: Configurable tracing via `tracing` and `tracing_subscriber`.
3. **Transport Methods**:
   - **SSE**: Server-sent events over HTTP for browser or HTTP client integration
   - **Stdio**: Standard input/output for command-line or pipe-based usage
4. **Error Handling**: Uses `anyhow` for comprehensive error management

## Creating Your Own Service

To create your own service instead of using the built-in Counter:

1. Define your service structure in a new module
2. Implement the necessary RMCP traits
3. Replace `Counter::new()` with your service implementation



