use linalgo::{SquareMatrix, Vector};

fn main() {
    let mut vector: Vector<i32> = Vector::new(5);
    vector.set_all_to(&0);

    for i in 0..vector.size() {
        if let Some(entry) = vector.get(i) {
            println!("{entry}");
        }
    }

    let mut matrix: SquareMatrix<i32> = SquareMatrix::new(5);
    matrix.set_all_to(&0);

    for i in 0..matrix.size() {
        for j in 0..matrix.size() {
            if let Some(entry) = matrix.get(i, j) {
                print!("{entry} ");
            }
        }

        println!();
    }
}
