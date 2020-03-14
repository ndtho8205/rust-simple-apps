use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "({:5.2} {:5.2})", self.0, self.1)?;
        write!(f, "({:5.2} {:5.2})", self.2, self.3)
    }
}

fn transpose(orig: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = orig;
    Matrix(a, c, b, d)
}
fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:{}", matrix);
    println!("Transpose:{}", transpose(matrix));
}
