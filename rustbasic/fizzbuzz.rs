//1.FIZZ BUZZ문제
// 1~100까지 수를 순서대로 출력하는 프로그램 작성
// 단, 3의배수 일때는 'FIZZ' 5의배수 일때는 'BUZZ' 3과5의 공배수 일때는 'FIZZBUZZ' 출력

//2. 1~50까지 수를 순서대로 화면에 출력한다
// 단, 3의배수와 3이 포함된 숫자가 올떄는 원래 숫자 대신 'A'를 출력한다

fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FIZZBUZZ");
        } else if i % 3 == 0 {
            println!("FIZZ");
        } else if i % 5 == 0 {
            println!("BUZZ");
        } else {
            println!("{}",i);
        }
    }

    println!("=================================");

    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 {
            println!("A");
            continue;
        }

        if i >= 30 && i <= 39 {
            println!("A");
            continue
        }

        println!("{}",i);
    }
}