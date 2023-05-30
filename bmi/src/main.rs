fn main() {
    //키와 몸무게 입력
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");

    //BMI계산
    // 체중 / 신장^2
    let height = height_cm/100.0;
    let bmi = weight / height.powf(2.0);
    println!("{:.1}", bmi);

    //비만도 진단
    if bmi < 18.5 {println!("저체중");}
    else if bmi < 23.0 {println!("정상");}
    else if bmi < 25.0 {println!("비만전단계");}
    else if bmi < 30.0 {println!("1단계 비만");}
    else if bmi < 35.0 {println!("2단계 비만");}
    else {println!("3단계 비만");}

    str_parse_f();
}

fn input(prompt: &str) -> f64 {
    //메시지 출력
    println!("{}", prompt);

    //입력값 가져옴
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");

    //골백을 제거하고 숫자 값으로 변환
    return s.trim().parse().expect("숫자가 아닙니다.");
}

fn str_parse_f() {
    let s = "3.1415a";

    //f64 타입으로 변환
    let num = s.parse::<f64>();
    //변환한 서식 값을 소수점 2자리 까지 출력
    match num {
        Ok(result) => println!("{:.2}", result),
        Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}",e)
    }
}
