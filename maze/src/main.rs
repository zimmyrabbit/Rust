use rand::Rng;

//상수 선언
// const 상수명: 타입 = 값;
const MAP_N: usize = 25;

fn main() {
    //난수 생성기
    let mut rng = rand::thread_rng();
    //2차원 배열 초기화
    let mut maze = [[0; MAP_N]; MAP_N]; // 배열 변수 선언

    //둘레를 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = 1; //위
        maze[0][n] = 1; //왼쪽
        maze[n][MAP_N-1] = 1; //아래
        maze[MAP_N-1][n] = 1; //오른쪽
    }

    //2칸마다 1개의 벽을 설치
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 {continue;}
            maze[y][x] = 1;

            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1,
                1 => maze[y+1][x] = 1,
                2 => maze[y][x-1] = 1,
                3 => maze[y][x+1] = 1,
                _ => {},
            }
        }
    }

    //미로 출력
    // 0과 1을 각각 흰색타일 (U+2B1C) 과 검은색타일 (U+2B1B) 로 치환
    let tiles = ["⬜","⬛"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}",tiles[maze[y][x]])
        }
        println!("");
    }
}
