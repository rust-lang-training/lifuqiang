use base64::{prelude::BASE64_STANDARD, Engine};
use wasm_bindgen::prelude::*;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// #[wasm_bindgen(start)]
// fn run() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("No window found");
//     let document = window.document().expect("No document found");
//     let body = document.body().expect("No body found");

//     let val = document.create_element("p")?;
//     val.set_text_content(Some("Hello from Rust"));

//     body.append_child(&val)?;

//     Ok({})
// }

use rsa::{pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt};

const PRIVATE: &'static str = include_str!("../private.pem");
const PUBLIC: &'static str = include_str!("../public.pem");

// const PRIVATE: &'static str = "-----BEGIN PRIVATE KEY-----
// MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDJg8oCU371X30F
// clg+4rKyEPGHxvfRNjHbtYqHEiKgFW+VaInLR/Iu5yd/Q1lKaUbsvQSfwHmtfoG1
// YgEWf8/RqtqxpdbHjp1hpw+dVhp5KmeTXJq9uvtnRKoIOWhO+XeBnEpn4c1r8vgc
// 1aSLUz9WdKZjzekzGS1XKqJRANSctXMBVvvw0npEcWT0JI0QdarIHF+hMZ5vHcQj
// Yf/WEydjnEQ0JMRjTeK8XEWBdcgs9Q97uS21J+yA0oJshoXAEGoFIJQGkf8gM7Id
// gBD91whTd+s4Fb7bzuHGSM5POcHXgtuUoJadSGYjKGUdHcSL3sKGbFt8qxBXceZi
// UIw1tSGnAgMBAAECggEAQO2RnQZCFdVadvLMtBZe9Ti4WcMl/ea0UdJAfBy/w0yk
// 00MHgtP/mFUqSLrufidFjumoH8Doc5obONIG/7TWxPuD01FXzpyI4Pnu8gbYKMoc
// 7UIdKZyhfDiHgHbewPNoKf6tiqMaiPmUL9Y2Wh3MfpK/OdESnkAyd69o9SiLWKIb
// UROQkGiElSDwg/+7p5ypLnyorubslr3dsrZGzlVq0YzmwvaKrcLbAjVBptGLLAjF
// OpLFv0J6D97ktms4+cCxT9RAAd69biDKmCuOGXtXqpAXX3EL743+dz1p1I+JPG1R
// V3AleXF3hGQcJ2TogT88AUwDQ/Hlz+BIdeoBkTAG4QKBgQD013cwEGim+KszQxg4
// gAIJjoX/jrCoVdPhzrzXnhPjvC6W/yQiCciLCTcbhkxVynzqcR4refEH9fkPPoKW
// E06SaX9vHCMQ/c0k+3WeZjqjKMMSWS2uxQhtZBRxciwvm93lQoVsQ2Olp2hQ4XUc
// FEdaI80PCavfzLbSVo2TzTKIVQKBgQDSstW8/UEUEGYn7tOYtNGBLkQXSb5DknII
// JiPBseiOGtj9d2AAhXyB4G91Y28PY6JFTEQzVMSf8X/sQOFeCRHR+e5yF7GFYHIu
// XS1uzBBhBZAdG4TaAsWz5VV4oGs9ati7fQFbBRcgI7z4A/mMHtm7f2EGhXV+Zt5H
// 6G2UIeIuCwKBgGVf3SKSeLJBnajw69Ng9Pb+Au9IJk7ckokZL0lbU0LkxKmSZGSP
// hnBC1YVY+rw+xa/QPDymFa1FI+Uy7aCzx8GJWtEnZkF/xHO+hfmavEaX9wTe1bJa
// NF4OpQQ/0Jm5wItPGq0zzctl4CQ05WdF4jEpR5slVvun6D54xc/fkz+tAoGAeC51
// 6FT0uAMaohCCNYp+Lhc706TGmDiCPs+yeZuHMPdzajCbW/DIMjTYnrbW9Io6VxjH
// lhf2B91ywEnJwx+dQI41SnLlWVTIDFYtwhM4DH9l0pSN1OTb7F8Vv7aRNPkrItaM
// 3WQr6rM36KgrwZcfTum7aELb3/vBMncLegFRyD0CgYEA25HJkrLuIneO2wbA+yqc
// evxmHj8ReCahu7Fv8x+zQMQiCFJnSpLiW37+PMESrkvDEIzC4MCZVCeM5SIYeF5J
// 5OFrXzKxGGd2omqugyJgHFAa3YZ0EygwzpzXkNCFHUmhtwFUFax2iVP+1DsSW4L4
// eRTok+Kj6sOa+suD8lVvM9c=
// -----END PRIVATE KEY-----";
// const PUBLIC: &'static str = "-----BEGIN PUBLIC KEY-----
// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAyYPKAlN+9V99BXJYPuKy
// shDxh8b30TYx27WKhxIioBVvlWiJy0fyLucnf0NZSmlG7L0En8B5rX6BtWIBFn/P
// 0arasaXWx46dYacPnVYaeSpnk1yavbr7Z0SqCDloTvl3gZxKZ+HNa/L4HNWki1M/
// VnSmY83pMxktVyqiUQDUnLVzAVb78NJ6RHFk9CSNEHWqyBxfoTGebx3EI2H/1hMn
// Y5xENCTEY03ivFxFgXXILPUPe7kttSfsgNKCbIaFwBBqBSCUBpH/IDOyHYAQ/dcI
// U3frOBW+287hxkjOTznB14LblKCWnUhmIyhlHR3Ei97ChmxbfKsQV3HmYlCMNbUh
// pwIDAQAB
// -----END PUBLIC KEY-----
// ";

#[wasm_bindgen]
pub fn encrypty_message(msg: &str) -> String {
    // let public_key = rsa::RsaPublicKey::from_public_key_pem(PUBLIC).expect("failed to parse public key");
    // let mut rng = rand::thread_rng();
    // let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes()).unwrap();

    // let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // let pub_key = RsaPublicKey::from(&priv_key);
    // let data = msg;
    // let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");

    let public_key = rsa::RsaPublicKey::from_public_key_pem(PUBLIC).unwrap();
    let mut rng = rand::thread_rng();
    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes()).unwrap();
    // enc_data

    BASE64_STANDARD.encode(enc_data)
}

// #[wasm_bindgen]
// pub fn decrypty_message(enc_data: &str) -> String {
//     let priv_key = PRIVATE;
    
//     let priv_key = DecryptingKey::from_pem(priv_key).expect("failed to parse private key");
//     let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");

//     String::from_utf8(dec_data).unwrap()
// }
#[wasm_bindgen]
pub fn decrypt_message(msg: &str) -> String {
    // let private_key = RsaPrivateKey::from_pkcs1_pem(PRIVATE).expect("Failed to parse private key");
    // let decrypted_bytes = private_key.decrypt(Pkcs1v15Encrypt, encrypted_msg.as_bytes()).expect("failed to decrypt");
    // let padding = PaddingScheme::new_pkcs1v15_encrypt();

    // let encrypted_bytes = base64::decode(encrypted_msg).expect("Failed to decode encrypted message");
    // let decrypted_bytes = private_key.decrypt(padding, &encrypted_bytes).expect("Failed to decrypt message");

    // String::from_utf8(decrypted_bytes).expect("Failed to convert decrypted bytes to string")
    let private_key = rsa::RsaPrivateKey::from_pkcs1_pem(PRIVATE).unwrap();
    let data = BASE64_STANDARD.decode(msg).unwrap();
    let plain_data = private_key.decrypt(Pkcs1v15Encrypt, &data[..]).unwrap();

    String::from_utf8(plain_data).unwrap()
}

use comrak::{markdown_to_html, Options};

#[wasm_bindgen]
pub fn markdown_2_html(markdown: &str) -> String {
    // "Hello, **世界**!"
    markdown_to_html(markdown, &Options::default())
}


