# config-rs enum deserialization error 

An example that re-produces the problem when trying to override config values other than `String` e.g. `u16` or `uszie`.

```
cargo build
APP_METRICS__EXPORTER__QUEUE_SIZE=1 ./target/debug/config-error-repro
```

**Produces:**

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: invalid type: string "1", expected usize', src/main.rs:36:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

