# Init Project
cargo init rust_note

# Depedency
cargo add axum
cargo add tokio -F full
cargo add tower-http -F "cors"
cargo add serde_json
cargo add serde -F derive
cargo add chrono -F serde
cargo add dotenvy
cargo add uuid -F "serde v4"
cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"


# Build & Run Project
cargo build
cargo run


# CLI For Watch source when running & Automatically rebuild the project
cargo install cargo-watch

# Run with watch the src/
cargo watch -q -c -w src/ -x run


# Run Docker Compose & Detach
docker compose up -d

# (Bonus! for stopping MySQL Docker)
docker compose stop

# CLI For migration
cargo install sqlx-cli

# create a migration
sqlx migrate add -r create_notes_table

# perform migration up
sqlx migrate run

# (Bonus!, perform migration down/revert)
sqlx migrate revert