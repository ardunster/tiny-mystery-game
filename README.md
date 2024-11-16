# tiny-mystery-game

Tiny mystery game

## AI Use

Some AI was used to help generate content such as lists of possible names for procedural generation to select from, and to flesh out content such as what items might be found at a blacksmith's shop. All actual text content and code is handwritten.

## Commands

Turn off dead code compilation warnings while prototyping:

```bash
export RUSTFLAGS="$RUSTFLAGS -A dead_code"
```

Run the app:

```bash
cargo run
```

Run the app with a custom seed:

```bash
cargo run seed custom-seed
```
