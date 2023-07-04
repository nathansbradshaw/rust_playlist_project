live reload with cargo-watch:
install cargo-watch

```shell
cargo install cargo-watch
```

```shell
cargo install sqlx-cli
```

```shell
sqlx migrate run
```

```shell
cargo watch -q -c -w src/ -x run
```

in another terminal if you want to use "quick dev"

```shell
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

Generate offline sqlx schema

```shell
cargo sqlx prepare -- --lib
```
