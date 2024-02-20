

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
    use rand::rngs::OsRng;
    use sha2::Sha512;
    use std::path::Path;
    use std::fs;

    fn save_to_file<P: AsRef<Path>>(path: P, data: &[u8]) {
        fs::write(path, data).expect("Failed to write to file");
    }
    #[test]
    fn usecase1() {
        // 키 페어 생성
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        // 메시지 읽기
        let message_path = "./zok/case1/witness";
        let message = fs::read(message_path).expect("Failed to read witness file");

        // 메시지에 서명
        let signature: Signature = keypair.sign(&message);

        // 결과 저장
        // 공개키 (A) 저장
        save_to_file("public_key.pem", keypair.public.as_bytes());
        
        // 서명 (R, S) 저장
        // 서명은 바이트 배열이므로, R과 S를 분리하여 저장할 수 없습니다.
        // 대신 서명 전체를 저장합니다.
        save_to_file("signature.sig", &signature.to_bytes());

        // 서명의 R 부분과 S 부분은 별도로 분리하여 저장해야 하나, 이 예제에서는
        // ed25519의 서명 구조상 직접 분리가 어렵습니다.
        // ZoKrates에서 사용하기 위해서는 추가 처리가 필요할 수 있습니다.

        // 서명 검증 (예시)
        let public_key: PublicKey = keypair.public;
        assert!(public_key.verify(&message, &signature).is_ok(), "Verification failed!");
        println!("Signature verified successfully!");
    }
}
