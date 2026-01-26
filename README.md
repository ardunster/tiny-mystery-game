# tiny-mystery-game

Tiny mystery game

## LLM Use

An LLM was used for:

- to help generate some content such as lists of possible names for procedural generation to select from
- to flesh out content such as what items might be found at a blacksmith's shop
- to help learn Rust and Bevy syntax
- to explore game design issues through discussion to better understand options and pros/cons of various approaches

All actual text content and code is handwritten.

## Commands

Turn off dead code compilation warnings while prototyping:

```bash
export RUSTFLAGS="$RUSTFLAGS -A dead_code"
```

Set `RUSTFLAGS` to an empty string or `echo $RUSTFLAGS` to check the prior value to reset when done.

Run the app:

```bash
cargo run
```

Run the app with a custom seed:

```bash
cargo run seed custom-seed
```
