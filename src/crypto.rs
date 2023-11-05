use base64::engine::{general_purpose, Engine};

pub fn encrypt(plain_text: &str) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(plain_text, &mut buf);
    println!("texto encriptado: {:?}", buf);
    decrypt(&buf);
    buf
}

pub fn decrypt(plain_text: &String) {
    let mut buf = Vec::<u8>::new();
    // with the diefault engine
    general_purpose::STANDARD
        .decode_vec(plain_text, &mut buf)
        .unwrap();
    // Tentando converter os bytes em uma string
    match String::from_utf8(buf) {
        Ok(string) => {
            println!("String legível: {}", string);
        }
        Err(e) => {
            println!("Erro ao converter bytes em string: {}", e);
        }
    }
}

