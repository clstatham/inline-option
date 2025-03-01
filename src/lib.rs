#![doc = include_str!("../README.md")]

/// A trait for defining nullable values.
pub trait Nullable {
    /// The null value for the type.
    const NULL: Self;

    /// Returns `true` if the value is null. Typically, this is a comparison with the null value.
    fn is_null(&self) -> bool;
}

impl<T> Nullable for Option<T> {
    const NULL: Self = None;

    #[inline]
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

impl<T> Nullable for *const T {
    const NULL: Self = core::ptr::null();

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}

impl<T> Nullable for *mut T {
    const NULL: Self = core::ptr::null_mut();

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}

#[cfg(feature = "nullable-core-floats")]
impl Nullable for f32 {
    const NULL: Self = core::f32::NAN;

    #[inline]
    fn is_null(&self) -> bool {
        self.is_nan()
    }
}

#[cfg(feature = "nullable-core-floats")]
impl Nullable for f64 {
    const NULL: Self = core::f64::NAN;

    #[inline]
    fn is_null(&self) -> bool {
        self.is_nan()
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for i8 {
    const NULL: Self = i8::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == i8::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for i16 {
    const NULL: Self = i16::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == i16::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for i32 {
    const NULL: Self = i32::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == i32::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for i64 {
    const NULL: Self = i64::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == i64::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for i128 {
    const NULL: Self = i128::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == i128::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for isize {
    const NULL: Self = isize::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == isize::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for u8 {
    const NULL: Self = u8::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == u8::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for u16 {
    const NULL: Self = u16::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == u16::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for u32 {
    const NULL: Self = u32::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == u32::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for u64 {
    const NULL: Self = u64::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == u64::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for u128 {
    const NULL: Self = u128::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == u128::MAX
    }
}

#[cfg(feature = "nullable-core-ints")]
impl Nullable for usize {
    const NULL: Self = usize::MAX;

    #[inline]
    fn is_null(&self) -> bool {
        *self == usize::MAX
    }
}

/// The `IOption` type, a transparent newtype wrapper around a [`Nullable`] value that provides a similar API to [`Option`][core::option::Option].
///
/// See the [module-level documentation](crate) for more information.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct IOption<T: Nullable>(T);

impl<T: Nullable> Default for IOption<T> {
    #[inline]
    fn default() -> Self {
        Self::new(T::NULL)
    }
}

impl<T: Nullable> IOption<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn none() -> Self {
        Self(T::NULL)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        if self.is_none() { None } else { Some(&self.0) }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_none() {
            None
        } else {
            Some(&mut self.0)
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> IOption<U>
    where
        U: Nullable,
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            IOption::new(U::NULL)
        } else {
            IOption::new(f(self.into_inner()))
        }
    }

    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        U: Nullable,
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            default
        } else {
            f(self.into_inner())
        }
    }

    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        U: Nullable,
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            default()
        } else {
            f(self.into_inner())
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        if self.is_none() {
            Err(err)
        } else {
            Ok(self.into_inner())
        }
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        if self.is_none() {
            Err(err())
        } else {
            Ok(self.into_inner())
        }
    }

    #[inline]
    pub fn and<U>(self, other: IOption<U>) -> IOption<U>
    where
        U: Nullable,
    {
        if self.is_none() {
            IOption::new(U::NULL)
        } else {
            other
        }
    }

    #[inline]
    pub fn and_then<U, F>(self, f: F) -> IOption<U>
    where
        U: Nullable,
        F: FnOnce(T) -> IOption<U>,
    {
        if self.is_none() {
            IOption::new(U::NULL)
        } else {
            f(self.into_inner())
        }
    }

    #[inline]
    pub fn or(self, other: IOption<T>) -> IOption<T>
    where
        T: Nullable,
    {
        if self.is_none() { other } else { self }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> IOption<T>
    where
        T: Nullable,
        F: FnOnce() -> IOption<T>,
    {
        if self.is_none() { f() } else { self }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        if self.is_none() {
            panic!("called `IOption::unwrap()` on a `None` value")
        } else {
            self.into_inner()
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        if self.is_none() {
            default
        } else {
            self.into_inner()
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if self.is_none() {
            f()
        } else {
            self.into_inner()
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        if self.is_none() {
            panic!("{}", msg)
        } else {
            self.into_inner()
        }
    }

    #[inline]
    pub fn iter(&self) -> core::option::IntoIter<&T> {
        self.as_ref().into_iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::option::IntoIter<&mut T> {
        self.as_mut().into_iter()
    }

    #[inline]
    pub fn filter<P>(self, predicate: P) -> IOption<T>
    where
        P: FnOnce(&T) -> bool,
    {
        let inner = self.into_inner();
        if inner.is_null() || !predicate(&inner) {
            IOption::new(T::NULL)
        } else {
            IOption::new(inner)
        }
    }

    #[inline]
    pub fn take(&mut self) -> IOption<T> {
        core::mem::take(self)
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> IOption<T> {
        core::mem::replace(self, IOption::new(value))
    }

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if self.is_none() {
            *self = IOption::new(value);
        }
        self.as_mut().unwrap()
    }

    #[inline]
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        if self.is_none() {
            *self = IOption::new(f());
        }
        self.as_mut().unwrap()
    }

    #[inline]
    pub fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default,
    {
        if self.is_none() {
            *self = IOption::new(T::default());
        }
        self.as_mut().unwrap()
    }
}

impl<T: Nullable> From<Option<T>> for IOption<T> {
    #[inline]
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => IOption::new(value),
            None => IOption::new(T::NULL),
        }
    }
}

impl<T: Nullable> From<IOption<T>> for Option<T> {
    #[inline]
    fn from(ioption: IOption<T>) -> Self {
        if ioption.is_none() {
            None
        } else {
            Some(ioption.into_inner())
        }
    }
}

impl<T: Nullable> From<T> for IOption<T> {
    #[inline]
    fn from(value: T) -> Self {
        IOption::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "nullable-core-ints"))]
    impl Nullable for i32 {
        const NULL: Self = i32::MAX;

        fn is_null(&self) -> bool {
            *self == i32::MAX
        }
    }

    #[test]
    fn test_nullable() {
        assert_eq!(i32::NULL, i32::MAX);
        assert!(i32::MAX.is_null());
        assert!(!42.is_null());
    }

    #[test]
    fn test_size() {
        assert_eq!(
            core::mem::size_of::<IOption<i32>>(),
            core::mem::size_of::<i32>()
        );
    }

    #[test]
    fn test_new() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.into_inner(), 42);
    }

    #[test]
    fn test_is_none() {
        let ioption = IOption::new(42);
        assert!(!ioption.is_none());

        let ioption = IOption::new(i32::NULL);
        assert!(ioption.is_none());
    }

    #[test]
    fn test_is_some() {
        let ioption = IOption::new(42);
        assert!(ioption.is_some());

        let ioption = IOption::new(i32::NULL);
        assert!(!ioption.is_some());
    }

    #[test]
    fn test_into_inner() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.into_inner(), 42);
    }

    #[test]
    fn test_as_ref() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.as_ref(), Some(&42));

        let ioption = IOption::new(i32::NULL);
        assert_eq!(ioption.as_ref(), None);
    }

    #[test]
    fn test_as_mut() {
        let mut ioption = IOption::new(42);
        assert_eq!(ioption.as_mut(), Some(&mut 42));

        let mut ioption = IOption::new(i32::NULL);
        assert_eq!(ioption.as_mut(), None);
    }

    #[test]
    fn test_map() {
        let ioption = IOption::new(42);
        let ioption = ioption.map(|value| value * 2);
        assert_eq!(ioption.into_inner(), 84);

        let ioption = IOption::new(i32::NULL);
        let ioption = ioption.map(|value| value * 2);
        assert_eq!(ioption.into_inner(), i32::NULL);
    }

    #[test]
    fn test_map_or() {
        let ioption = IOption::new(42);
        let value = ioption.map_or(0, |value| value * 2);
        assert_eq!(value, 84);

        let ioption = IOption::new(i32::NULL);
        let value = ioption.map_or(0, |value| value * 2);
        assert_eq!(value, 0);
    }

    #[test]
    fn test_map_or_else() {
        let ioption = IOption::new(42);
        let value = ioption.map_or_else(|| 0, |value| value * 2);
        assert_eq!(value, 84);

        let ioption = IOption::new(i32::NULL);
        let value = ioption.map_or_else(|| 0, |value| value * 2);
        assert_eq!(value, 0);
    }

    #[test]
    fn test_ok_or() {
        let ioption = IOption::new(42);
        let result = ioption.ok_or("error");
        assert_eq!(result, Ok(42));

        let ioption = IOption::new(i32::NULL);
        let result = ioption.ok_or("error");
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_ok_or_else() {
        let ioption = IOption::new(42);
        let result = ioption.ok_or_else(|| "error");
        assert_eq!(result, Ok(42));

        let ioption = IOption::new(i32::NULL);
        let result = ioption.ok_or_else(|| "error");
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_and() {
        let ioption = IOption::new(42);
        let other = IOption::new(84);
        let result = ioption.and(other);
        assert_eq!(result.into_inner(), 84);

        let ioption = IOption::new(i32::NULL);
        let other = IOption::new(84);
        let result = ioption.and(other);
        assert_eq!(result.into_inner(), i32::NULL);
    }

    #[test]
    fn test_and_then() {
        let ioption = IOption::new(42);
        let result = ioption.and_then(|value| IOption::new(value * 2));
        assert_eq!(result.into_inner(), 84);

        let ioption = IOption::new(i32::NULL);
        let result = ioption.and_then(|value| IOption::new(value * 2));
        assert_eq!(result.into_inner(), i32::NULL);
    }

    #[test]
    fn test_or() {
        let ioption = IOption::new(42);
        let other = IOption::new(84);
        let result = ioption.or(other);
        assert_eq!(result.into_inner(), 42);

        let ioption = IOption::new(i32::NULL);
        let other = IOption::new(84);
        let result = ioption.or(other);
        assert_eq!(result.into_inner(), 84);
    }

    #[test]
    fn test_or_else() {
        let ioption = IOption::new(42);
        let result = ioption.or_else(|| IOption::new(84));
        assert_eq!(result.into_inner(), 42);

        let ioption = IOption::new(i32::NULL);
        let result = ioption.or_else(|| IOption::new(84));
        assert_eq!(result.into_inner(), 84);
    }

    #[test]
    fn test_unwrap() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.unwrap(), 42);
    }

    #[test]
    #[should_panic]
    fn test_unwrap_panic() {
        let ioption = IOption::new(i32::NULL);
        ioption.unwrap();
    }

    #[test]
    fn test_unwrap_or() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.unwrap_or(0), 42);

        let ioption = IOption::new(i32::NULL);
        assert_eq!(ioption.unwrap_or(0), 0);
    }

    #[test]
    fn test_unwrap_or_else() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.unwrap_or_else(|| 0), 42);

        let ioption = IOption::new(i32::NULL);
        assert_eq!(ioption.unwrap_or_else(|| 0), 0);
    }

    #[test]
    fn test_expect() {
        let ioption = IOption::new(42);
        assert_eq!(ioption.expect("error"), 42);
    }

    #[test]
    #[should_panic]
    fn test_expect_panic() {
        let ioption = IOption::new(i32::NULL);
        ioption.expect("error");
    }

    #[test]
    fn test_iter() {
        let ioption = IOption::new(42);
        let mut iter = ioption.iter();
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut ioption = IOption::new(42);
        let mut iter = ioption.iter_mut();
        assert_eq!(iter.next(), Some(&mut 42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_filter() {
        let ioption = IOption::new(42);
        let result = ioption.filter(|value| *value % 2 == 0);
        assert_eq!(result.into_inner(), 42);

        let ioption = IOption::new(43);
        let result = ioption.filter(|value| *value % 2 == 0);
        assert_eq!(result.into_inner(), i32::NULL);
    }

    #[test]
    fn test_take() {
        let mut ioption = IOption::new(42);
        let result = ioption.take();
        assert_eq!(result.into_inner(), 42);
        assert!(ioption.is_none());
    }

    #[test]
    fn test_replace() {
        let mut ioption = IOption::new(42);
        let result = ioption.replace(84);
        assert_eq!(result.into_inner(), 42);
        assert_eq!(ioption.into_inner(), 84);
    }

    #[test]
    fn test_get_or_insert() {
        let mut ioption = IOption::new(42);
        let value = ioption.get_or_insert(84);
        assert_eq!(value, &42);
        assert_eq!(ioption.into_inner(), 42);

        let mut ioption = IOption::new(i32::NULL);
        let value = ioption.get_or_insert(84);
        assert_eq!(value, &84);
        assert_eq!(ioption.into_inner(), 84);
    }

    #[test]
    fn test_get_or_insert_with() {
        let mut ioption = IOption::new(42);
        let value = ioption.get_or_insert_with(|| 84);
        assert_eq!(value, &42);
        assert_eq!(ioption.into_inner(), 42);

        let mut ioption = IOption::new(i32::NULL);
        let value = ioption.get_or_insert_with(|| 84);
        assert_eq!(value, &84);
        assert_eq!(ioption.into_inner(), 84);
    }

    #[test]
    fn test_get_or_insert_default() {
        let mut ioption = IOption::new(42);
        let value = ioption.get_or_insert_default();
        assert_eq!(value, &42);
        assert_eq!(ioption.into_inner(), 42);

        let mut ioption = IOption::new(i32::NULL);
        let value = ioption.get_or_insert_default();
        assert_eq!(value, &Default::default());
        assert_eq!(ioption.into_inner(), Default::default());
    }

    #[test]
    fn test_from_option() {
        let option = Some(42);
        let ioption = IOption::<i32>::from(option);
        assert_eq!(ioption.into_inner(), 42);

        let option = None;
        let ioption = IOption::<i32>::from(option);
        assert_eq!(ioption.into_inner(), i32::NULL);
    }

    #[test]
    fn test_from_ioption() {
        let ioption = IOption::new(42);
        let option = Option::from(ioption);
        assert_eq!(option, Some(42));

        let ioption = IOption::new(i32::NULL);
        let option = Option::<i32>::from(ioption);
        assert_eq!(option, None);
    }

    #[test]
    fn test_from_value() {
        let value = 42;
        let ioption = IOption::from(value);
        assert_eq!(ioption.into_inner(), 42);
    }

    #[test]
    fn test_default() {
        let ioption = IOption::<i32>::default();
        assert!(ioption.is_none());
    }
}
