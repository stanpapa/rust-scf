use crate::math::matrix::{FMatrix, Matrix};
use crate::math::scalar::Scalar;
use crate::math::vector::Vector;

pub fn check_vec_vec<T: Scalar>(s: &str, lhs: &Vector<T>, rhs: &Vector<T>) {
    if lhs.size() != rhs.size() {
        panic!("[{}] Vector dimensions are not the same.", s);
    }

    if !lhs.is_ok() {
        panic!("[{}] lhs is not OK.", s);
    } else if !rhs.is_ok() {
        panic!("[{}] rhs is not OK.", s);
    }
}

pub fn check_mat_vec<T: Scalar>(s: &str, lhs: &Matrix<T>, rhs: &Vector<T>) {
    if lhs.cols() != rhs.size() {
        panic!("[{}] Incompatible dimensions.", s);
    }

    if !lhs.is_ok() {
        panic!("[{}] lhs is not OK.", s);
    } else if !rhs.is_ok() {
        panic!("[{}] rhs is not OK.", s);
    }
}

// C(i,j) = A(i,k) B(k,j)
pub fn check_mat_mat<T: Scalar>(s: &str, c: &FMatrix, a: &Matrix<T>, b: &Matrix<T>) {
    if a.cols() != b.rows() {
        panic!("[{}] Incompatible contraction dimension.", s);
    }

    if c.rows() != a.rows() || c.cols() != b.cols() {
        panic!("[{}] Target not compatible.", s);
    }

    if !c.is_ok() {
        panic!("[{}] C is not OK.", s);
    } else if !a.is_ok() {
        panic!("[{}] A is not OK.", s);
    } else if !b.is_ok() {
        panic!("[{}] B is not OK.", s);
    }
}
