use arbitrary_example::{parse_port_from_endpoint, render_endpoint, EndpointInput};

fn main() {
    let input = EndpointInput {
        ip: [192, 168, 1, 10],
        port: 8080,
    };
    let endpoint = render_endpoint(&input);
    let port = parse_port_from_endpoint(&endpoint);

    println!("Generated input: {input:?}");
    println!("Rendered endpoint: {endpoint}");
    println!("Parsed port: {port}");
}
