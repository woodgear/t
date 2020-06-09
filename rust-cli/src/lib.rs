#![deny(warnings)]
#![allow(clippy::needless_return)]

pub fn ping(name: String) -> String {
    if name == "x" {
        return "x".to_string();
    } else {
        return "no".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ping() {
        assert_eq!(ping("x".to_string()), "x".to_string());
        assert_eq!(ping("xxx".to_string()), "no".to_string());
    }
}
