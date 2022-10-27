mod hello;

pub fn hello_world_v(){
    hello::say_hello::hello_world();
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
