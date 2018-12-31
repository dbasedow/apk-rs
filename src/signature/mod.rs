use sha2::{Sha256, Digest};
use nom::*;

mod asn1;

pub fn get_key_fingerprint_sha256(pkcs7: &[u8]) -> Result<Vec<u8>, u32> {
    if let IResult::Done(_, cert) = get_cert(pkcs7) {
        let mut hasher = Sha256::new();
        hasher.input(cert);
        return Ok(hasher.result().to_vec());
    }
    Err(0)
}

fn get_cert(pkcs7: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (_, r) = try_parse!(pkcs7, asn1::parse_data_element);
    let (_, r) = r.parse_data().unwrap();
    let (_, r) = r[1].parse_data().unwrap();
    let (_, r) = r[0].parse_data().unwrap();

    IResult::Done(&[], r[3].data().to_vec())
}