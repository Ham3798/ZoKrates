

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
    use rand::rngs::OsRng;
    use sha2::Sha512;
    use std::io::BufReader;
    use std::path::Path;
    use std::fs;
    use zokrates_ast::ir::{self, Witness};
    use std::fs::File;
    use zokrates_field::Field;

    fn save_to_file<P: AsRef<Path>>(path: P, data: &[u8]) {
        fs::write(path, data).expect("Failed to write to file");
    }

    
    fn read_witness<T: Field> (witness_path : &String) -> Witness<T> {
        let witness_file = File::open(witness_path).unwrap();
        let witness_reader = BufReader::new(witness_file);
        let witness = ir::Witness::read(witness_reader)
        .map_err(|why| format!("Could not load witness: {:?}", why));
        witness.unwrap()
    }

    #[test]
    fn usecase1() {
        // 키 페어 생성
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let witness_path = "./zok/case1/witness".to_string();
        
        // 메시지 읽기
        // let witness = read_witness(&witness_path);
        let m0: [u32; 8] = [0,0,0,0,1154505464,2643875965,3678618428,1130793931];
        let m1: [u32; 8] = [0,0,0,0,3208641210,2043857794,3394837354,3218078181];

         // M0과 M1을 바이트 배열로 결합
        let mut message = Vec::new();
        for &val in m0.iter().chain(m1.iter()) {
            message.extend_from_slice(&val.to_be_bytes());
        }
        
        // 메시지에 서명
        let signature: Signature = keypair.sign(&message);

        // 결과 저장
        // 공개키 (A) 저장
        save_to_file("public_key.pem", keypair.public.as_bytes());
        
        // 서명 (R, S) 저장
        // 서명은 바이트 배열이므로, R과 S를 분리하여 저장할 수 없습니다.
        // 대신 서명 전체를 저장합니다.
        
        
        let signature_bytes: [u8; 64] = signature.to_bytes();
        let (r, s) = signature_bytes.split_at(32); 
        save_to_file("signature_r", &signature.to_bytes());
        save_to_file("signature_s", &signature.to_bytes());

        // 서명 검증 (예시)
        let public_key: PublicKey = keypair.public;
        assert!(public_key.verify(&message, &signature).is_ok(), "Verification failed!");
        println!("Signature verified successfully!");

        println!("R : {:?}", r);
        println!("S : {:?}", s);
        println!("A : {:?}", keypair.public.as_bytes());
        println!("m0 : {:?}", m0);
        println!("m1 : {:?}", m1);
    }
}
