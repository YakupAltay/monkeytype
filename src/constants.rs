use celestia_types::nmt::Namespace;
use anyhow::Result;

/// Namespace bytes must be exactly 10 bytes long for Celestia's v0 namespace format.
/// This represents the ASCII values for the string "monkeytype".
const MONKEYTYPE_NAMESPACE_BYTES: [u8; 10] = [
    109, 111, 110, 107, 101, 121, 116, 121, 112, 101 // "m", "o", "n", "k", "e", "y", "t", "y", "p", "e"
];

/// Returns a fixed-size v0 namespace for the game, constructed from a 10-byte constant.
pub fn get_monkeytype_namespace() -> Result<Namespace> {
    // `const_v0` constructs a namespace without runtime validation
    let namespace = Namespace::const_v0(MONKEYTYPE_NAMESPACE_BYTES);
    Ok(namespace)
}

// Constants for game configuration and networking

/// Local Celestia RPC endpoint (adjustable for different environments or deployments)
pub const CELESTIA_RPC_URL: &str = "http://localhost:10101";

/// Local Avail RPC endpoint (adjustable for different environments or deployments)
pub const AVAIL_RPC_URL: &str = "http://localhost:10102";
