/// Foo function
///
/// Has something to do with [bar](./fn.bar.html).
/// Links to the [crate root](./index.html#functions),
/// but also has a [broken link](./index.html#a)!
pub fn foo() {}

/// Bar function
pub fn bar() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
