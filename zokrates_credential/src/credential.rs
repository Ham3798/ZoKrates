use serde::{Serialize, Deserialize};
    use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
    use std::fs;
    use chrono::{DateTime, Utc, Duration};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct Claims {
        // 필드명을 스네이크 케이스로 변경
        #[serde(rename = "@context")]
        context: Vec<String>,
        id: String,
        #[serde(rename = "type")]
        credential_type: Vec<String>,
        issuer: CredentialIssuer,
        issuance_date: DateTime<Utc>, // 변경됨
        credential_subject: CredentialSubject, // 변경됨
        exp: i64, // `exp` 클레임 추가
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct CredentialIssuer {
        id: String,
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct CredentialSubject {
        id: String,
        name: String,
        age: u8,
        student_number: String,
        alumni_of: AlumniOf,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct AlumniOf {
        id: String,
        name: String,
        department: String,
    }

    #[test]
    fn encode_and_decode_jwt_rs256() {
        let my_claims = Claims {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_owned(),
                "https://www.example.org/examples/v1".to_owned(),
            ],
            id: "http://chungnam.ac.kr/credentials/3732".to_owned(),
            credential_type: vec![
                "VerifiableCredential".to_owned(),
                "AlumniCredential".to_owned(),
            ],
            issuer: CredentialIssuer {
                id: "https://infosec.chungnam.ac.kr".to_owned(),
                name: "Chungnam National University Information Security Lab".to_owned(),
            },
            issuance_date: Utc::now(),
            credential_subject: CredentialSubject {
                id: "did:example:abcdef1234567890".to_owned(),
                name: "Socrates".to_owned(),
                age: 30,
                student_number: "201812345".to_owned(),
                alumni_of: AlumniOf {
                    id: "did:example:c34fb4561237890".to_owned(),
                    name: "Chungnam National University".to_owned(),
                    department: "Information Security".to_owned(),
                },
            },
            exp: (Utc::now() + Duration::days(90)).timestamp(), // 90일 후 만료
        };


        // RSA 개인 키 로드
        let private_key = fs::read("./private_key.pem").expect("Failed to read private key");
        let public_key = fs::read("./public_key.pem").expect("Failed to read public key");

        // JWT 인코딩
        let token = encode(&Header::new(Algorithm::RS256), &my_claims, &EncodingKey::from_rsa_pem(&private_key).expect("Failed to create encoding key")).expect("Failed to encode");
        println!("Encoded JWT: {}", token);

        // JWT 디코딩
        let validation = Validation::new(Algorithm::RS256);
        let token_data = decode::<Claims>(&token, &DecodingKey::from_rsa_pem(&public_key).expect("Failed to create decoding key"), &validation).expect("Failed to decode");

        // Assertions
        assert_eq!(token_data.claims.credential_subject.name, "Socrates");
        assert_eq!(token_data.claims.credential_subject.student_number, "201812345");
    }