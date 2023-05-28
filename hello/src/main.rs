//BigInt 를 사용하기 위한 선언
use num_bigint::BigInt;

fn main() {
    let msg_list = [
        "모든 것에 대한 열쇠는 인내이다."
        , "계란을 부화시켜야지 깨뜨려서는 닭을 얻을 수 없다."
    ];
    for msg in msg_list {
        println!("{}",msg);
    }

    println!("=====================================");
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678));
}
