# Rust template

# Database setup

- PostgreSQL [in docker](https://hub.docker.com/_/postgres/)
- pgAdmin [install](https://www.pgadmin.org/download/pgadmin-4-apt/)
- [diesel_cli](https://lib.rs/crates/diesel_cli)

```bash
diesel setup --database-url postgresql://postgres:postgres@localhost/animal_fact_db
diesel migration run --database-url postgresql://postgres:postgres@localhost/animal_fact_db
```

# Running

define the environment on which we're running by adding `ENV=<env>`, which will use the `.env.<env>` file

```bash
ENV=dev cargo run
```

# Code quality & security
```bash
cargo fmt --all -- --check
cargo clippy --all-targets
cargo audit
cargo outdated
```

# Testing

Here's what done in order to mock the SPI

- db: every test is creating a new database from `TestContextPostgreSQL` with json fixtures in `test/fixtures` & spawns the app with this database
- http: every test also spins up another rust api (if not already up) with the expected routes but test data in `test/fixtures`

```bash
ENV=test cargo test
```
