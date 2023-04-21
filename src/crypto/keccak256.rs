use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn keccak256(data: &String) -> String {
    let mut hasher = Sha3::keccak256();
    hasher.input_str(data);
    let hex = hasher.result_str();
    hex
}
