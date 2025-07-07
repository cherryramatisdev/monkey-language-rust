# An implementation of the monkey programming language in rust

Reference: https://interpreterbook.com

## Testing

I'm using snapshots with [insta](https://insta.rs/), to setup it's CLI run:

```sh
curl -LsSf https://insta.rs/install.sh | sh
```

Then just run:

```sh
cargo test
```

And to review the snapshots:

```
cargo insta review
```
