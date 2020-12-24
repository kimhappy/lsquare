#![allow(non_snake_case)]
#![feature(min_const_generics)]

use std::ops::*;
use std::convert::TryInto;
use num_traits::*;
use nalgebra::*;

pub trait Domain: Scalar + Num + AddAssign + MulAssign + ComplexField {}
impl< T > Domain for T where T: Scalar + Num + AddAssign + MulAssign + ComplexField {}

pub trait Function< T, const N: usize > where T: Domain {
    fn var(x: T) -> [T; N];

    fn function(coefficients: [T; N]) -> impl Fn(T) -> T {
        move |x: T| {
            let lhs = DVector::from(coefficients.to_vec());
            let rhs = DVector::from(Self::var(x).to_vec());
            Matrix::dot(&lhs, &rhs)
        }
    }

    fn eval(coefficients: [T; N], x: T) -> T {
        let lhs = DVector::from(coefficients.to_vec());
        let rhs = DVector::from(Self::var(x).to_vec());
        Matrix::dot(&lhs, &rhs)
    }

    fn least_square< 'a >(xs: impl IntoIterator< Item = &'a T >, ys: impl IntoIterator< Item = &'a T >) -> [T; N] {
        let flattened = xs.into_iter().flat_map(|x| Self::var(*x).to_vec()).collect::< Vec< _ > >();
        let M = flattened.len() / N;
        let mt = DMatrix::from_iterator(N, M, flattened.into_iter());
        let m = mt.transpose();
        let mtm = &mt * m;
        let mtmi = mtm.try_inverse().unwrap();
        let mtmimt = mtmi * mt;
        let y = DVector::from_iterator(M, ys.into_iter().map(|&x| x));
        let v = mtmimt * y;
        v.as_slice().try_into().unwrap()
    }
}
