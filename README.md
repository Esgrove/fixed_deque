# Deque

[![Crates.io Version](https://img.shields.io/crates/v/fixed_deque)](https://crates.io/crates/fixed_deque)
[![Documentation](https://docs.rs/fixed_deque/badge.svg)](https://docs.rs/fixed_deque/)

A fixed size `VecDeque` for Rust to match
the [Python Deque](https://docs.python.org/3/library/collections.html#collections.deque) functionality.

Implemented as a thin wrapper around `std::collections::VecDeque` with custom handling for `push_back` and `push_front`
that prevents the `VecDeque` from growing past the set maximum length.

Once the deque is full, when a new item is pushed to the deque,
an element from the opposite end is popped and returned.

```rust
use fixed_deque::Deque;

let mut deque: Deque<i32> = Deque::new(3);
deque.push_back(1);
deque.push_back(2);
deque.push_back(3);
deque.push_back(4);
assert_eq!(deque.len(), 3);
assert_eq!(deque.maxlen(), 3);
assert_eq!(deque.get(0), Some(&2));

let mut deque = Deque::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0], 5);
assert_eq!(deque.len(), 5);
let popped = deque.push_back(6.0);
assert_eq!(popped, Some(1.0));
assert_eq!(deque.len(), 5);

let mut deque: Deque<&str> = (vec!["a", "b", "c"], 5).into();
assert_eq!(deque.len(), 3);
assert_eq!(deque.maxlen(), 5);
assert_eq!(deque.front(), Some(&"a"));
deque.push_front("1");
deque.push_front("2");
deque.push_front("3");
assert_eq!(deque.front(), Some(&"3"));
assert_eq!(deque.back(), Some(&"b"));
```

## Features

Optional `serde` feature that adds support for (de)serializing the Deque.

```shell
cargo add fixed_deque --features serde
```

## Contribution

Contributions are very welcome.
If you feel something could be added or improved,
please do open a PR.

- Always add test cases for new functionality
- Ensure that code is formatted with `cargo fmt` and `cargo clippy --all-features` passes

## License

MIT
