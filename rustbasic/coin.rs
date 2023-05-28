//1. 거스름돈 계산
// 어느 가계의 계산 카운터에 500원짜리 10개, 100원짜리 3개, 50원짜리 10개가 있다.
// 잔돈으로 3950원을 거슬러 줘야할 경우 나올 수 있는 모든 조합을 구하라.

fn main() {

    //타입추론을 이용한 계산
    let price = 3950;

    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i50 * 50 + i100 * 100 + i500 * 500;
                if total == price {
                    println!{"500원 {}개, 100원 {}개, 50원 {}개",i500,i100,i50};
                }
            }
        }
    }

    println!{"============이용가능한 정수 범위=============="}

    println!{"--- 부호가 있는 정수 ---"}
    println!("i8={}~{}",i8::MIN, i8::MAX);
    println!("i16={}~{}",i16::MIN, i16::MAX);
    println!("i32={}~{}",i32::MIN, i32::MAX);
    println!("i64={}~{}",i64::MIN, i64::MAX);

    println!{"--- 부호가 없는 정수 ---"}
    println!("u8={}~{}",u8::MIN, u8::MAX);
    println!("u16={}~{}",u16::MIN, u16::MAX);
    println!("u32={}~{}",u32::MIN, u32::MAX);
    println!("u64={}~{}",u64::MIN, u64::MAX);

    println!{"--- OS 비트에 따라 달라지는 정수 ---"}
    println!("isize={}~{}",isize::MIN, isize::MAX);
    println!("usize={}~{}",usize::MIN, usize::MAX);
    println!("usize=u{}",usize::BITS);

    println!{"=========================================="}
    
    //타입추론을 이용하지 않은 계산
    // let 변수명: 타입 = 값;
    let price2: i64 = 3950;

    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64 = 10;

    for i500 in 0..(count500 + 1) {
        for i100 in 0..(count100 + 1) {
            for i50 in 0..(count50 + 1) {
                let total: i64 = i50 * 50 + i100 * 100 + i500 * 500;
                if price2 == total {
                    println!{"500원 {}개, 100원 {}개, 50원 {}개",i500,i100,i50};
                }
            }
        }
    }
}