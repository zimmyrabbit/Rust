fn main() {
    //매크로를 사용해 만든 Vector
    let nums_m = vec![1,2,3];
    println!("{:?}",nums_m);

    //매크로를 사용하지 않고 만든 Vector
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);
    println!("{:?}",nums);

    //Vec<T>
    // u32 타입 vector 생성
    let a_vec: Vec<u32> = vec![100,200,300];
    for i in a_vec {
        println!("{}",i);
    }

    // &str 타입 vector 생성
    let s_vec: Vec<&str> = vec!["개","고양이","닭"];
    for i in s_vec {
        println!("{}",i);
    }
}
