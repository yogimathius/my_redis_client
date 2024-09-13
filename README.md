# my_redis_client

## Description

`my_redis_client` is a Command Line Interface (CLI) tool written in Rust that interacts with a Redis server. This client provides two modes of operation: a REPL (Read Eval Print Loop) mode and a command-line argument mode, allowing you to issue Redis commands and receive responses directly from the server.

## Features

This CLI tool supports a wide range of Redis commands across various data types:

### General Commands

- `quit` – Exit the CLI
- `ECHO` – Echoes a given message
- `PING` – Checks if the server is alive
- `FLUSHALL` – Clears all data on the Redis server
- `INFO` – Fetches server information

### Key/Value Commands

- `SET` – Set a key to hold a specific value
- `GET` – Get the value of a key
- `KEYS` – List all keys matching a pattern
- `TYPE` – Get the type of a key
- `DEL` – Delete a key
- `UNLINK` – Asynchronously delete a key
- `EXPIRE` – Set a timeout on a key
- `RENAME` – Rename a key

### List Commands

- `LLEN` – Get the length of a list
- `LREM` – Remove elements from a list
- `LINDEX` – Get an element from a list by its index
- `LPOP/RPOP` – Remove and return the first/last element from a list
- `LPUSH/RPUSH` – Prepend/Append an element to a list
- `LSET` – Set the value of an element in a list by its index

### Hash Commands

- `HGET` – Get the value of a hash field
- `HEXISTS` – Check if a hash field exists
- `HDEL` – Delete one or more hash fields
- `HGETALL` – Get all fields and values of a hash
- `HKEYS` – Get all field names in a hash
- `HLEN` – Get the number of fields in a hash
- `HMSET` – Set multiple fields in a hash
- `HSET` – Set a field in a hash
- `HVALS` – Get all values in a hash

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
