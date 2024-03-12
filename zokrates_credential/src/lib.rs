

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
    use rand::rngs::OsRng;
    use std::io::{BufReader, Read, Write};
    use std::path::Path;
    use std::fs::File;
    use zokrates_field::Field;
    use std::io;
    
    // fn read_witness<T: Field> (witness_path : &String) -> Witness<T> {
    //     let witness_file = File::open(witness_path).unwrap();
    //     let witness_reader = BufReader::new(witness_file);
    //     let witness = ir::Witness::read(witness_reader)
    //     .map_err(|why| format!("Could not load witness: {:?}", why));
    //     witness.unwrap()
    // }

    fn read_witness(witness_path: &str) -> io::Result<([u32; 8], [u32; 8])> {
        // Witness 파일 읽기 로직 구현...
        // 예제에서는 더미 데이터를 반환합니다.
        Ok((
            [0, 0, 0, 0, 1154505464, 2643875965, 3678618428, 1130793931],
            [0, 0, 0, 0, 3208641210, 2043857794, 3394837354, 3218078181],
        ))
    }

    #[test]
    fn read_witness_file() -> io::Result<()> {
        // Witness 파일의 경로 설정
        let path = Path::new("./zok/case1/issuer/witness");

        // 파일 열기
        let mut file = File::open(path).unwrap();

        // 파일 내용을 담을 바이트 벡터
        let mut contents = Vec::new();

        // 파일 내용을 바이트 배열로 읽기
        file.read_to_end(&mut contents)?;

        // 바이트 배열의 내용 출력 (옵션: 디버깅을 위해 바이트를 출력하거나, 가능하다면 문자열로 변환하여 출력)
        println!("File contents: {:?}", contents);

        Ok(())
    }

    fn get_or_create_keypair(path: &Path) -> io::Result<Keypair> {
        if path.exists() {
            let mut file = File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Keypair::from_bytes(&bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            let mut csprng = OsRng{};
            let keypair = Keypair::generate(&mut csprng);
    
            let bytes = keypair.to_bytes();
            let mut file = File::create(path)?;
            file.write_all(&bytes)?;
    
            Ok(keypair)
        }
    }
    

    #[test]
    fn usecase1() {
        // 키 페어 생성
        let keypair_path = Path::new("./keypair.bin");
        let keypair = get_or_create_keypair(keypair_path).unwrap();
        
        // 메시지 읽기
        // let witness_path = "./zok/case1/issuer/witness".to_string();
        // let mut witness_file = File::open(witness_path).unwrap();
        // let mut message = Vec::new();
        // witness_file.read_to_end(&mut message).unwrap();
        let message = vec![0,0,0,0,1154505464,2643875965,3678618428,1130793931,0,0,0,0,3208641210,2043857794,3394837354,3218078181];

        // 메시지에 서명
        let signature: Signature = keypair.sign(&message);

        // write_signature_for_zokrates_cli 함수를 사용하여 ZoKrates 호환 파일 생성
        let path = "zokrates_cli_input.txt";
        write_signature_for_zokrates_cli(&keypair.public, &signature, &message, path).unwrap();

        // 서명 검증 (예시)
        assert!(keypair.public.verify(&message, &signature).is_ok(), "Verification failed!");
        println!("Signature verified successfully!");
    }


    fn write_signature_for_zokrates_cli(pk: &PublicKey, sig: &Signature, msg: &[u8], path: &str) -> std::io::Result<()> {
        // 서명의 R과 S 구성 요소 추출
        let signature_bytes = sig.to_bytes();
        let (sig_r, sig_s) = signature_bytes.split_at(32);
    
        // 공개키 구성 요소 추출
        let pk_bytes = pk.to_bytes();
        // 공개키의 x, y 좌표 추출은 특정 구현에 따라 달라질 수 있으며, 여기서는 단순화를 위해 전체 바이트 배열을 사용합니다.
    
        // 메시지 해시를 16진수 문자열로 변환
        let msg_hex = hex::encode(msg);
        let (m0, m1) = msg_hex.split_at(64);
        
        // 파일에 쓸 문자열 생성
        let args = format!(
            "{} {} {} {} {}",
            hex::encode(sig_r),
            hex::encode(sig_s),
            pk_bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(" "),
            split_into_chunks(m0),
            split_into_chunks(m1)
        );
    
        // 파일 쓰기
        let mut file = File::create(path)?;
        writeln!(file, "{}", args)
    }
    
    // 16진수 문자열을 8자리 정수로 분할하는 함수
    fn split_into_chunks(s: &str) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chunk| {
                let chunk_str: String = chunk.iter().collect();
                u64::from_str_radix(&chunk_str, 16).unwrap_or(0).to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
