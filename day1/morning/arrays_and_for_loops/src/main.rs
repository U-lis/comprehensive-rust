fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }
    return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len() {
        if i == 0 {
            print!("⎡");
        } else if i == matrix.len() - 1 {
            print!("⎣");
        } else {
            print!("⎢");
        }
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j]);
            if j < matrix[i].len() - 1 {
                print!(" ");
            }
        }
        if i == 0 {
            print!("⎤");
        } else if i == matrix.len() - 1 {
            print!("⎦");
        } else {
            print!("⎢");
        }
        println!();
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}