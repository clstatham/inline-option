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
