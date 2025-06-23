use encoding_rs;

// gbk to utf-8
pub fn decode_gbk(bytes: &[u8]) -> String {
    let (result, _, _) = encoding_rs::GBK.decode(bytes);
    result.to_string()
}
