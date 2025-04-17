# Redis-like In-Memory Store (Rust Implementation)

[![Rust Version](https://img.shields.io/badge/rust-1.65%2B-blue)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Redis-inspired in-memory data store implemented in Rust, supporting core data structures with atomic operations and network capabilities.


## Features

### Implemented Data Structures
- **Strings**
  - `SET <key> <value>`, `GET <key>`, `DEL <key>`
  - `TTL <key>`
  
- **Lists**
  - `LPUSH <key> <value>`, `RPUSH <key> <value>`
  - `LPOP <key>`, `RPOP <key>`
  - `LRANGE <key> <start> <stop>`

- **Sets**
  - `SADD <key> <member>`, `SREM <key> <member>`
  - `SISMEMBER <key> <member>`, `SMEMBERS <key>`

- **Sorted Sets**
  - `ZADD <key> <score> <member>`, `ZREM <key> <member>`
  - `ZRANGE <key> <start> <stop> [WITHSCORES]`

## Getting Started

### Prerequisites
- Rust 1.65 or newer
- Cargo package manager

### Installation
```bash
git clone https://github.com/Matrx123/redis-like-clone
cd redis-like-clone
cargo build --release
