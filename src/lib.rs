pub fn ascii_to_binary(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn ascii_to_hex(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:02x}", c as u8))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn binary_to_ascii(input: &str) -> String {
    input
        .split(" ")
        .map(|c| {
            let c = u8::from_str_radix(c, 2).unwrap();
            char::from(c)
        })
        .collect::<String>()
}

pub fn hex_to_ascii(input: &str) -> String {
    input
        .split(" ")
        .map(|c| {
            let c = u8::from_str_radix(c, 16).unwrap();
            char::from(c)
        })
        .collect::<String>()
}

pub fn binary_to_hex(input: &str) -> String {
    input
        .split(" ")
        .map(|c| {
            let c = u8::from_str_radix(c, 2).unwrap();
            format!("{:02x}", c)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn hex_to_binary(input: &str) -> String {
    input
        .split(" ")
        .map(|c| {
            let c = u8::from_str_radix(c, 16).unwrap();
            format!("{:08b}", c)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_binary() {
        assert_eq!(
            ascii_to_binary("Hello, world!"),
            "01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001"
        );
    }

    #[test]
    fn test_ascii_to_hex() {
        assert_eq!(
            ascii_to_hex("Hello, world!"),
            "48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21"
        );
    }

    #[test]
    fn test_binary_to_ascii() {
        assert_eq!(
            binary_to_ascii("01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001"),
            "Hello, world!"
        );
    }

    #[test]
    fn test_hex_to_ascii() {
        assert_eq!(
            hex_to_ascii("48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21"),
            "Hello, world!"
        );
    }

    #[test]
    fn test_binary_to_hex() {
        assert_eq!(
            binary_to_hex("01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001"),
            "48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21"
        );
    }

    #[test]
    fn test_hex_to_binary() {
        assert_eq!(
            hex_to_binary("48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21"),
            "01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001"
        );
    }

    
}