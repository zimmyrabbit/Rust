fn main() {
    let mut height;

    loop {
        println!("키(cm) : ");
        height = input_f(0.0);
        if height > 0.0 {break;}
        println!("숫자만 입력 가능 합니다.");
    }

    let weight = 22.0 * (height/100.0).powf(2.0);
    println!("표준체중은 {:1}kg 입니다.", weight);

    let s ="365";

    let i: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };
    println!("{}",i);
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("입력 에러");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    // match 외에 unwrap(), expect() 사용 가능
    // 단 프로그램이 바로 종료되고, Panic 코드 출력
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}
