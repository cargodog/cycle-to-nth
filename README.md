# cycle-to-nth
This crate provides a simple iterator and associated adapter to cycle an iterator until the nth element is reached. This is a wrapper around the [Cycle](https://doc.rust-lang.org/std/iter/struct.Cycle.html) iterator in the standard library, with the slight modification to stop cycling at the nth iteration.

This can be helpful in situations where you have shorter (or unknown length) iterator, but you require an iterator over _exactly_ `n` elements. For example, custom initialization of an array/container, extending/truncating a sample set to input into a signal processing algorithm, etc.

# Examples
The example below extends a smaller iterator(only one element) to a longer iterator (5 iterations over the same element):
```
let v = vec![1u32];
assert_eq!(v.iter().cycle_to_nth(5).sum::<u32>(), 5u32);
```

The example below truncates a longer iterator to an iterator over exactly 5 elements.
```
let v = vec![1u32; 10];
assert_eq!(v.iter().cycle_to_nth(5).sum::<u32>(), 5u32);
```

