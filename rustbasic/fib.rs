// 1. 피보나치 수열

// 2. 어떤 컴퓨터 업체의 PC가격은 98만원이다.
// A 쇼핑몰에서는 배송비가 12000원 이고 20%할인된 가격으로 판매하고 있다.
// B 쇼핑몰에서는 배송비가 무료 이지만 10% 할인된 가격으로 판매하고 있다.
// 둘중 어느 쇼핑몰의 가격이 싼가?

fn main() {
    // mut = 가변변수 정의
    let mut a = 1;
    let mut b = 1;

    println!("{}",a);
    println!("{}",b);

    for _ in 0..30 {
        println!("{}",a+b);
        let tmp = a;
        a = b;
        b = tmp+b;
    }

    println!("==============================================");

    let pc_price = 980000.0;
    let a_deli_fee = 12000.0;
    let a_rate = 0.8;
    let b_deli_fee = 0.0;
    let b_rate = 0.9;

    println!("A쇼핑몰 {} 원",pc_price * a_rate + a_deli_fee);
    println!("B쇼핑몰 {} 원",pc_price * b_rate + b_deli_fee);
}