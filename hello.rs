//컴파일
// rustc hello.rs

// 실행
// hello

fn main() {
    println!("hello rust");

    let banana = 300;
    println!("바나나 가격 = {}원", banana);

    //지구에서 달까지의 거리는 384000km, 지구에서 달까지 80km/h의 자동차와 300km/h ktx로 간다면 각각 몇일이 걸리는가
    let moon = 384000.0;
    let car = 80.0;
    let train = 300.0;
    println!("달까지 자동차로 : {}일", moon/car/24.0);
    println!("달까지 ktx로 : {}일", moon/train/24.0);
}