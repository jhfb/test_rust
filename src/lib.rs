pub mod hello;
pub use hello::say_hello;

pub fn hello_world(){
    say_hello::hello_world();
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
