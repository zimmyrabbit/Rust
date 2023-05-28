// 1. 구구단

// 2. 태조 원년(1392)부터 세종 32년(1450)까지의 연호를 서력과 함께 나타낸다.
// 태조는 1392년부터, 정종은 1399년 부터, 태종은 1401년 부터, 세종은 1419년 부터 시작한다.
// 연호의 1년쨰는 '원년'으로 표시한다

fn main() {

    /*
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3}, ",i*j);
        }
        println!("");
    }
    */

    for y in 1..10 {
        let s = (1..10)
                .map(|x| format!("{:3}", x*y))
                .collect::<Vec<String>>().join(",");
            println!("{}",s);
    }


    for y in 1392..1451 {
        print!("서력 {} 년 = ", y);

        if y >= 1419 {
            if y == 1419 {println!("세종 원년");}
            else {println!("세종 {} 년", y-1419+1);}
        } else if y >= 1401 {
            if y == 1401 {println!("태종 원년");}
            else {println!("태종 {} 년", y-1401+1);}
        } else if y >= 1399 {
            if y == 1399 {println!("정종 원년");}
            else {println!("정종 {} 년", y-1399+1);}
        } else if y >= 1392 {
            if y == 1392 {println!("태조 원년");}
            else {println!("태조 {} 년", y-1392+1);}
        }
    }
}