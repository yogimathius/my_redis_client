# my_redis_client

## Description

`my_redis_client` is a Command Line Interface (CLI) tool written in Rust that interacts with a Redis server. This client provides two modes of operation: a REPL (Read Eval Print Loop) mode and a command-line argument mode, allowing you to issue Redis commands and receive responses directly from the server.

## Features

This CLI tool supports a wide range of Redis commands across various data types:

### General Commands

- `quit` – Exit the CLI
- `echo` – Echoes a given message
- `ping` – Checks if the server is alive
- `flushall` – Clears all data on the Redis server
- `info` – Fetches server information

### Key/Value Commands

- `set` – Set a key to hold a specific value
- `get` – Get the value of a key
- `keys` – List all keys matching a pattern
- `type` – Get the type of a key
- `del` – Delete a key
- `unlink` – Asynchronously delete a key
- `expire` – Set a timeout on a key
- `rename` – Rename a key

### List Commands

- `llen` – Get the length of a list
- `lrem` – Remove elements from a list
- `lindex` – Get an element from a list by its index
- `lpop/rpop` – Remove and return the first/last element from a list
- `lpush/rpush` – Prepend/Append an element to a list
- `lset` – Set the value of an element in a list by its index

### Hash Commands

- `hget` – Get the value of a hash field
- `hexists` – Check if a hash field exists
- `hdel` – Delete one or more hash fields
- `hgetall` – Get all fields and values of a hash
- `hkeys` – Get all field names in a hash
- `hlen` – Get the number of fields in a hash
- `hmset` – Set multiple fields in a hash
- `hset` – Set a field in a hash
- `hvals` – Get all values in a hash

## Requirements

- Rust (latest stable version)
- Tokio
- Structopt
- Bytes

> **Note:** Usage of the `redis-rs` crate is not allowed.

## Installation

To install and run `my_redis_client` locally, ensure you have Rust installed. Then clone the repository and build the project:

```bash
git clone https://git.us.qwasar.io/my_redis_client_169753_14fcqx/my_redis_client.git
cd my_redis_client
cargo build --release
```

## Usage

### REPL Mode

To start the client in REPL mode and connect to the Redis server, use the following command:

```bash
cargo run --bin my_redis_repl
```

You can now interact with the Redis server using supported commands:

```
127.0.0.1:6379> set "hello" "world"
OK
127.0.0.1:6379> get "hello"
"world"
127.0.0.1:6379> quit
```

### Shell Mode

You can also run commands directly by passing them as arguments:

```bash
cargo run --bin my_redis_client GET ciao mondo
OK
cargo run --bin my_redis_client GET ciao
"mondo"
```

## Command Reference

For more information on Redis commands, refer to the [Redis Command Reference](https://redis.io/commands).

## Architecture

The client is built using the asynchronous `Tokio` runtime for handling TCP connections. The parsing of Redis frames and commands is abstracted into distinct modules for ease of maintainability and future extension.

## Bonus Features (not implemented yet)

### Logging

The project includes advanced logging using the `tracing` crate for both debugging and tracing capabilities.

### Streams

Publish/Subscribe streams are implemented, allowing real-time messaging with Redis Streams using crates like `tokio-stream` and `async-stream`.

### SSL Support

Optionally, SSL can be enabled using the `--tls` flag with support for `--cacert` or `--cacertdir`.

## License

This project is licensed under the MIT License.

## Author

Mathius Johnson
