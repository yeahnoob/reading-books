#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn hi_from_lib(msg: &str) {
    println!(".. Echo Hi [{}] from library ..", msg);
}
