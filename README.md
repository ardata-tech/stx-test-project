# Space and Time Test Project

This project demonstrates how to use the `w3io-partner-space-and-time` crate to interact with the Space and Time decentralized data warehouse.

## Setup

1. Copy `.env.example` to `.env` and fill in your credentials:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your Space and Time credentials:
   - `SPACE_AND_TIME_BISCUIT_USERID`: Your Space and Time user ID
   - `SPACE_AND_TIME_BISCUIT_PRIVATE_KEY`: Your private key in hex format
   - `SPACE_AND_TIME_API_KEY`: Your API key (optional, for low-level API access)

## Running Examples

Run the main examples:
```bash
cargo run
```

Run integration tests (requires valid credentials):
```bash
cargo test -- --ignored
```

## Examples Included

### 1. Query Public Data
Demonstrates querying public blockchain data from the Polygon dataset.

### 2. User Management
Shows how to create new users and manage authentication.

### 3. Table Operations
Examples of creating tables, inserting data, querying, and deleting records.

### 4. Low-Level API Usage
Uses the low-level Space and Time API directly for custom operations.

### 5. Biscuits and Permissions
Demonstrates creating biscuits for fine-grained access control.

## Key Features

- **Authentication**: Secure user authentication using Ed25519 keypairs
- **SQL Queries**: Execute SQL queries against decentralized data
- **Table Management**: Create and manage your own tables
- **Biscuits**: Fine-grained permission management using Biscuit tokens
- **Subscriptions**: Create and manage user subscriptions

## API Overview

### High-Level SDK
```rust
use w3io_partner_space_and_time::{SxT, SxTUser};

// Load and authenticate user
let sxt = SxT::new()?;
let sxt = sxt.authenticate().await?;

// Execute queries
let results = sxt.execute_query::<MyType>("SELECT * FROM table".to_string()).await?;
```

### Table Operations
```rust
use w3io_partner_space_and_time::{SxTTable, TableAccessType};

// Create table instance
let table = SxTTable::new("schema", "table", None, user, TableAccessType::PublicRead);

// Create, insert, select, delete
table.create("id INT PRIMARY KEY, name VARCHAR(100)".to_string()).await?;
table.insert("id, name", "(1, 'test')").await?;
table.select::<MyType>("*", "WHERE id = 1").await?;
table.delete("WHERE id = 1").await?;
```

## Resources

- [Space and Time Documentation](https://docs.spaceandtime.io/)
- [W3.io Protocol Repository](https://github.com/w3io/protocol)