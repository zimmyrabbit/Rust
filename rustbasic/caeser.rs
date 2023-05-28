// 1.시저암호 만들기

//함수 정의
// fn 함수명 (인수 선언) -> 반환 값 타입 {
    // 변수 선언 및 함수의 동작 정의
//}

//클로저 작성 방법
// let 이름 = |인수| 정의;

//문자,문자열
// char -> 문자
// &str, String -> 문자열 

fn encrypt(text: &str, shift: i16) -> String {
    //'A','Z'의 ASCII CODE를 i16타입으로 취득
    //  as -> 강제적인 타입 변환을 위해 이용하는 기능 
    //  let 변수명 = 변수 as 타입
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    //결과 대입할 변수 선언
    let mut result = String::new();

    //한글자씩 치환 처리
    for ch in text.chars() {
        //문자 코드로 변환
        let mut code = ch as i16;

        if code_a <= code && code_z >= code {
            // (79(O) - 65 + 3 + 26) % 26 + 65
            code = (code - code_a + shift + 26) % 26 + code_a
        }

        result.push((code as u8) as char);
    }
    return result;
}

fn encrypt2(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c >= 'Z';
    let conv = |c| (((c-a+shift+26)%26+a)as u8)as char;
    let enc1 = |c| if is_az(c) {conv(c as i16)} else {c};
    text.chars().map(|c| enc1(c)).collect()
}

fn main() {
    let enc = encrypt("I LOVE RUST", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);

    println!("==============================");
    let x2 = |n| n*2;
    println!("{}", x2(2));
    println!("{}", x2(8));

    println!("==============================");
    let enc2 = encrypt2("I LOVE RUST", 3);
    let dec2 = encrypt2(&enc2, -3);
    println!("{} => {}", enc2, dec2);
}