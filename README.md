# dremio-rust
Dremio Rust Playground...

Primary motivation is to see if I can get this working: https://github.com/developer-advocacy-dremio/quick-guides-from-dremio/blob/main/guides/languages.md#using-rust-with-arrow-flight-into-a-polars-dataframe

## Steps

1. Initialise rust project

```console
# init rust project
cargo init .

# add deps
cargo add polars
cargo add tonic
cargo add arrow-flight
cargo add tokio --features "full"
```
2. Copy code from provided example.
3. Update to use environment variables for configuration.



