//! This code is copied from https://github.com/ethereum/py_ecc/blob/master/py_ecc/bn128/bn128_curve.py
//! Author is Vitalik Buterin.
//! Unfortunately the field modulus is not generic in this implementation, hence we had to copy the file.
//! All changes from our side are denoted with #CHANGE.


use zokrates_field::Field;

// BabyJubJubParams 구조체 정의
pub struct BabyJubJubParams<T: Field> {
    pub jubjub_c: T,
    pub jubjub_a: T,
    pub jubjub_d: T,
    pub mont_a: T,
    pub mont_b: T,
    pub infinity: [T; 2],
    pub gu: T,
    pub gv: T,
}

// Eddsa 서명 생성 함수
fn generate_eddsa_signature<T: Field>(
    message: &[u8],
    private_key: T,
    params: &BabyJubJubParams<T>,
) -> ([T; 2], T) {
    // 임시로 구현
    ([T::zero(), T::zero()], T::zero())
}

// Eddsa 서명 검증 함수
fn verify_eddsa<T: Field>(
    signature: ([T; 2], T),
    public_key: [T; 2],
    message: &[u8],
    params: &BabyJubJubParams<T>,
) -> bool {
    // 임시로 구현
    true
}

// BabyJubJubParams를 사용하여 타원곡선 상에 있는지 확인하는 함수
fn on_curve<T: Field>(pt: [T; 2], params: &BabyJubJubParams<T>) -> bool {
    let a = params.jubjub_a;
    let d = params.jubjub_d;
    let uu = pt[0] * pt[0];
    let vv = pt[1] * pt[1];
    let uuvv = uu * vv;
    a * uu + vv == T::one() + d * uuvv
}

// BabyJubJubParams를 사용하여 점의 부분 그룹을 확인하는 함수
fn order_check<T: Field>(pt: [T; 2], params: &BabyJubJubParams<T>) -> bool {
    let cofactor = params.jubjub_c;
    assert!(cofactor == T::from(8));

    let mut pt_exp = add(pt, pt, params); // 2 * pt
    pt_exp = add(pt_exp, pt_exp, params); // 4 * pt
    pt_exp = add(pt_exp, pt_exp, params); // 8 * pt

    !(pt_exp[0] == T::zero() && pt_exp[1] == T::one())
}

// sha256 함수 구현 (임시)
fn sha256<T: Field>(r: [T; 2], a: [T; 2], m0: [T; 8], m1: [T; 8]) -> [bool; 256] {
    // 임시 구현
    [false; 256]
}

// scalarMult 함수 구현 (임시)
fn scalar_mult<T: Field>(exponent: [bool; 256], pt: [T; 2], params: &BabyJubJubParams<T>) -> [T; 2] {
    // 임시 구현
    [T::zero(), T::zero()]
}

// add 함수 구현 (임시)
fn add<T: Field>(pt1: [T; 2], pt2: [T; 2], params: &BabyJubJubParams<T>) -> [T; 2] {
    // 임시 구현
    [T::zero(), T::zero()]
}