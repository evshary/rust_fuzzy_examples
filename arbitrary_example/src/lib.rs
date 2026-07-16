use arbitrary::{Arbitrary, Result, Unstructured};

// `Arbitrary` teaches the crate how to build this type from raw bytes.
// In real fuzzing, a fuzzer would mutate bytes first, then `arbitrary`
// would decode those bytes into a structured Rust value like this.
#[derive(Debug, Arbitrary, PartialEq, Eq)]
pub struct EndpointInput {
    pub ip: [u8; 4],
    pub port: u16,
}

// Turn the structured input back into the string form our parser expects.
pub fn render_endpoint(input: &EndpointInput) -> String {
    let [a, b, c, d] = input.ip;
    format!("{a}.{b}.{c}.{d}:{}", input.port)
}

// The target cares only about the port portion of the endpoint.
pub fn parse_port_from_endpoint(endpoint: &str) -> u16 {
    let Some((_, port_text)) = endpoint.rsplit_once(':') else {
        return 0;
    };

    if port_text == "0" {
        panic!("bug: port 0 is handled incorrectly");
    }

    port_text.parse::<u16>().unwrap_or_default()
}

// `Unstructured` is the core `arbitrary` API. It wraps raw bytes and lets
// us ask for typed values from them with `input.arbitrary::<T>()`.
pub fn parse_port_from_bytes(data: &[u8]) -> Result<u16> {
    let mut input = Unstructured::new(data);
    let endpoint: EndpointInput = input.arbitrary()?;
    Ok(parse_port_from_endpoint(&render_endpoint(&endpoint)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_port_from_endpoint() {
        assert_eq!(parse_port_from_endpoint("192.168.1.10:8080"), 8080);
    }

    #[test]
    fn ignores_the_ip_portion() {
        assert_eq!(parse_port_from_endpoint("not-an-ip:8080"), 8080);
    }

    #[test]
    fn invalid_port_falls_back_to_zero() {
        assert_eq!(parse_port_from_endpoint("127.0.0.1:not-a-port"), 0);
    }

    #[test]
    fn renders_a_structured_endpoint() {
        let input = EndpointInput {
            ip: [192, 168, 1, 10],
            port: 8080,
        };

        assert_eq!(render_endpoint(&input), "192.168.1.10:8080");
    }

    #[test]
    fn arbitrary_can_build_a_structured_input() {
        let mut input = Unstructured::new(&[1; 32]);
        // Request one typed value from the byte buffer.
        let endpoint = input.arbitrary::<EndpointInput>().unwrap();
        let rendered = render_endpoint(&endpoint);

        assert!(!rendered.is_empty());
    }

    #[test]
    fn raw_bytes_can_drive_the_target() {
        let parsed = parse_port_from_bytes(&[1; 32]).unwrap();
        assert!(parsed <= u16::MAX);
    }
}
