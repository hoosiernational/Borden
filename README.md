# Borden

Simple lightweight Base64 encoder/decoder which allows fast RFC 4648-compliant Base64 usage without the engine generation, large dependencies, and unnecessary customization of the base64 crate.

It has no public structs, and only two methods:
 - `encode(input: Vec<u8>) -> String`
 - `decode(input: &str) -> Vec<u8>`