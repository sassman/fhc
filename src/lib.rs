#[cfg(test)]
mod tests {
    use super::sha256;

    #[test]
    fn empty() -> Result<(), std::io::Error> {
        let r = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(sha256("tests/empty.txt")?, r);
        Ok(())
    }

    #[test]
    fn good() -> Result<(), std::io::Error> {
        let r = "32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5";
        assert_eq!(sha256("tests/lorem.txt")?, r);
        Ok(())
    }
}

fn sha256(path: &str) -> Result<String, std::io::Error> {
    Ok(path.to_string())
}

