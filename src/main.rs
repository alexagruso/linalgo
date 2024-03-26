use linalgo::SquareMatrix;

fn main() {
    let matrix: SquareMatrix<i32> = SquareMatrix::new(5);
    println!("{:?}", matrix.get(0, 0));
}
