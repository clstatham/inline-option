# inline-option

Provides the `Nullable` trait for describing nullable values, and `IOption`, an optional type that wraps a `Nullable` value inline without extra space overhead.

In certain performance-intensive applications, the standard library's `Option` enum type can be wasteful because Rust enums are tagged unions with a discriminant. The discriminant takes up extra space in memory, which can be undesirable when working with small, primitive types. For instance, `Option<f32>` has a size of 8 bytes on 64-bit systems, while `f32` has a size of only 4 bytes. This can be unoptimal when working with large arrays of optional values, in terms of both memory usage and cache efficiency.

`IOption` is a thin wrapper around a nullable value that provides a similar API to `Option`, but without the extra space overhead. It is a zero-cost abstraction that is as efficient as working with the nullable value directly.

## Example

```rust
use inline_option::{IOption, Nullable};

// Define a struct that can be null.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Value(i32);

// Implement the Nullable trait for the struct.
impl Nullable for Value {
    const NULL: Self = Value(i32::MAX);

    fn is_null(&self) -> bool {
        self.0 == i32::MAX
    }
}

fn main() {
    // Create an inline option that can hold a Value.
    let mut opt = IOption::<Value>::none();
    assert!(opt.is_none());

    // Replace the value in the inline option.
    opt.replace(Value(42));
    assert!(opt.is_some());

    // Get the value from the inline option.
    let value = opt.unwrap();
    assert_eq!(value.0, 42);

    // Or, convert it to a standard option.
    let std_opt = opt.as_ref();
    assert_eq!(std_opt, Some(&Value(42)));
}

```

## Benchmarks

Based on the simple benchmark in [benches/bench.rs](benches/bench.rs), `Vec<IOption<T>>` can be iterated over 3 times as fast as `Vec<Option<T>>`:

![benchmark violin plot](benchmark.svg)

## Cargo Features

- `serde`: Enables Serde 1 support for `IOption`.
- `nullable-core-floats`: Implements `Nullable` for `f32` and `f64`, using `NAN` as the null value.
- `nullable-core-ints`: Implements `Nullable` for `i8` through `i128`, `u8` through `u128`, `isize`, and `usize` using their maximum values as their null values.

## License

MIT OR Apache-2.0
