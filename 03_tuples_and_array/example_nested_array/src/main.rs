fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // let mut result = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]; // 빈 3x3 행렬 생성
    let mut result = [[0; 3]; 3]; // 빈 3x3 행렬 생성

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- 주석으로 rustfmt가 줄바꿈을 추가합니다.
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("행렬: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("전치행렬: {:#?}", transposed);
}