# Deque

A fixed size `VecDeque` for Rust to match
the [Python Deque](https://docs.python.org/3/library/collections.html#collections.deque).

Implemented as a thin wrapper around `std::collections::VecDeque` with custom handling for `push_back`
that prevents the VecDeque from growing past the set maximum length.

Once the deque is full, when a new item is added to the back, the front element is popped and returned.

```rust
use fixed_deque::Deque;

let mut deque: Deque<i32> = Deque::new(3);
deque.push_back(1);
deque.push_back(2);
deque.push_back(3);
deque.push_back(4);
assert_eq!(deque.len(), 3);
assert_eq!(deque.get(0), Some(&2));

deque = Deque::new_from_vec(vec![1, 2, 3, 4, 5], 5);
assert_eq!(deque.len(), 5);
let overflow = deque.push_back(6);
assert_eq!(overflow, Some(1));
assert_eq!(deque.len(), 5);
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

## License

MIT
