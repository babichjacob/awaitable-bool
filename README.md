<h1 align="center">🎚️ Awaitable Bool</h1>

This Rust library is a bool that can be waited to be set to true or set to false.

## 💻 Installation

This crate is [published to crates.io as `awaitable-bool`](https://crates.io/crates/awaitable-bool), so you can do

```sh
cargo add awaitable-bool
```

to add it to your project's dependencies.

## 🛠 Usage

You probably don't want to use this if you aren't me; I'm not familiar enough with [atomics](https://doc.rust-lang.org/stable/std/sync/atomic/) (which is how `AwaitableBool` is implemented) to know the correctness of the code!

## 😵 Help! I have a question

Create an issue and I'll try to help.

## 😡 Fix! There is something that needs improvement

Create an issue or pull request and I'll try to fix.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

## 🙏 Attribution

@devalain's [`future-bool`](https://crates.io/crates/future-bool) is an existing Rust crate that already works very closely to this.

The idea is highly inspired by [Python's `asyncio.Event`](https://docs.python.org/3/library/asyncio-sync.html#asyncio.Event), but an `AwaitableBool` can be waited for to become 'clear' too (not just 'set').

This library is implemented with [`Tokio`](https://tokio.rs/)'s [`Notify` synchronization tool](https://docs.rs/tokio/1.32.0/tokio/sync/struct.Notify.html).

I also developed [`async-gate`](https://github.com/babichjacob/async-gate) right before making `awaitable-bool`. That breaks down changing the value of the bool and waiting for value changes into two different types (`Lever` and `Gate` respectively). It is more complex.

_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
