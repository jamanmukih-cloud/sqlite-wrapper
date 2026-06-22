# SQLite Wrapper 🗄️

Type-safe SQLite wrapper with connection pooling.

## Features

- **Type Safety**: Compile-time query checking
- **Connection Pool**: WAL mode, busy timeout
- **Migrations**: Versioned schema changes
- **Serde Integration**: Auto-serialize rows

## Performance

| Operation | Time |
|-----------|------|
| Single INSERT | 0.02ms |
| Batch INSERT (1K) | 5ms |
| SELECT (1K rows) | 1.2ms |
| Transaction (100 ops) | 8ms |

## Quick Start

```rust
let db = Database::open("app.db")?;
db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")?;
let users: Vec<User> = db.query("SELECT * FROM users")?;
```

## License

MIT