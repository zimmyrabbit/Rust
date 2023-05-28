// 1.소수 100개를 구하여라

//배열
// let mut arr = [초깃값; 배열 길이]
// arr[요소번호] = 값;
// let v = arr[요소번호];

fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    return true
}

fn get_primes(primes: &mut[usize; 100]) {
    let mut i = 2;
    let mut count = 0;

    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut primes = [0; 100];

    get_primes(&mut primes);

    println!("{:?}",primes);
}