pub mod hello;

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        hello::hello_world::hello_world();
    }
}
