pub fn hello() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::hello();
    }
}
