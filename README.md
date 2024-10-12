# my_redis_client

## Description

`my_redis_client` is a Command Line Interface (CLI) tool written in Rust that interacts with a Redis server. This client provides two modes of operation: a REPL (Read Eval Print Loop) mode and a command-line argument mode, allowing you to issue Redis commands and receive responses directly from the server.

## Features

This CLI tool supports a wide range of Redis commands across various data types:

### General Commands

- [x] `quit` – Exit the CLI
- [x] `ECHO` – Echoes a given message
- [x] `PING` – Checks if the server is alive
- [x] `FLUSHALL` – Clears all data on the Redis server
- [x] `INFO` – Fetches server information

### Key/Value Commands

- [x] `SET` – Set a key to hold a specific value
- [x] `GET` – Get the value of a key
- [x] `KEYS` – List all keys matching a pattern
- [x] `TYPE` – Get the type of a key
- [x] `DEL` – Delete a key
- [x] `UNLINK` – Asynchronously delete a key
- [x] `EXPIRE` – Set a timeout on a key
- [x] `RENAME` – Rename a key

### List Commands

- [x] `LLEN` – Get the length of a list
- [x] `LREM` – Remove elements from a list
- [x] `LINDEX` – Get an element from a list by its index
- [x] `LPOP/RPOP` – Remove and return the first/last element from a list
- [x] `LPUSH/RPUSH` – Prepend/Append an element to a list
- [x] `LSET` – Set the value of an element in a list by its index

### Hash Commands

- [x] `HGET` – Get the value of a hash field
- [x] `HEXISTS` – Check if a hash field exists
- [x] `HDEL` – Delete one or more hash fields
- [x] `HGETALL` – Get all fields and values of a hash
- [x] `HKEYS` – Get all field names in a hash
- [x] `HLEN` – Get the number of fields in a hash
- [x] `HMSET` – Set multiple fields in a hash
- [x] `HSET` – Set a field in a hash
- [x] `HVALS` – Get all values in a hash

## Requirements

- [x] Rust (latest stable version)
- [x] Tokio
- [x] Structopt
- [x] Bytes

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
cargo run --bin my_redis_shell GET ciao mondo
OK
cargo run --bin my_redis_shell GET ciao
"mondo"
```

## Command Reference

For more information on Redis commands, refer to the [Redis Command Reference](https://redis.io/commands).

## Architecture

The client is built using the asynchronous `Tokio` runtime for handling TCP connections. The parsing of Redis frames and commands is abstracted into distinct modules for ease of maintainability and future extension.

## Author

Mathius Johnson
