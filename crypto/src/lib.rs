use secp256k1::hashes::sha256::Hash;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1, SecretKey, XOnlyPublicKey};
use sha256::digest;
use std::str::FromStr;


pub fn hash(s: String) -> String {
    return digest(s);
}

pub fn create_key_pair() -> (String, String) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    return (
        secret_key.display_secret().to_string(),
        public_key.to_string(),
    );
}

pub fn sign_message(s: String, private_key: String) -> String {
    let secp = Secp256k1::new();
    let message = Message::from_hashed_data::<Hash>(s.as_bytes());
    let decoded = hex::decode(private_key).expect("Decoding failed");
    let secret_key = SecretKey::from_slice(&decoded).expect("32 bytes, within curve order");
    let key_pair = secret_key.keypair(&secp);

    let sig = secp.sign_schnorr(&message, &key_pair);
    return sig.to_string();
}

pub fn get_public_key(private_key: String) -> String {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_str(&private_key).expect("32 bytes, within curve order");
    return secret_key.public_key(&secp).to_string();
}

pub fn verify_message(message: String, signature: String, public_key: String) -> bool {
    let mut pub_key = public_key.clone();
    pub_key.remove(0);
    pub_key.remove(0);

    let secp = Secp256k1::new();
    let message = Message::from_hashed_data::<Hash>(message.as_bytes());
    let xonly = XOnlyPublicKey::from_str(&pub_key).expect("Bad public key");
    let decoded_signature = hex::decode(signature).expect("Decoding failed");
    let sig = secp256k1::schnorr::Signature::from_slice(&decoded_signature).expect("TODO");
    let result = secp.verify_schnorr(&sig, &message, &xonly);
    return result.is_ok();
}
