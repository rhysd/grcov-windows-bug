use anyhow::Result;

pub fn hello() -> Result<()> {
    println!("hello");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::hello().unwrap();
    }
}
