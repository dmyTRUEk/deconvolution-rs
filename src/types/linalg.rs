//! Linear Algebra Types.

use nalgebra::{DVector, DVectorView};

use crate::types::float::float;


pub type DVect = DVector<float>;


pub trait Reversed {
    fn reversed(self) -> DVect;
}
impl Reversed for DVectorView<'_, float> {
    // #[inline(never)]
    fn reversed(self) -> DVect {
        // DVect::from_iterator(self.len(), self.iter().rev().copied())
        let len = self.len();
        let mut v = DVect::zeros(len);
        // TODO(optim): make zipped with reversed i loop
        for i in 0..len {
            v[i] = self[len - i - 1];
        }
        v
    }
}


#[cfg(test)]
mod reversed {
    use crate::types::linalg::{DVect, Reversed};

    #[test]
    fn len_0() {
        assert_eq!(
            DVect::from_vec(vec![]),
            DVect::from_vec(vec![]).rows(0, 0).reversed(),
        );
    }

    #[test]
    fn len_1() {
        assert_eq!(
            DVect::from_vec(vec![1.]),
            DVect::from_vec(vec![1.]).rows(0, 1).reversed(),
        );
    }

    #[test]
    fn len_2() {
        assert_eq!(
            DVect::from_vec(vec![2., 1.]),
            DVect::from_vec(vec![1., 2.]).rows(0, 2).reversed(),
        );
    }

    #[test]
    fn len_3() {
        assert_eq!(
            DVect::from_vec(vec![3., 2., 1.]),
            DVect::from_vec(vec![1., 2., 3.]).rows(0, 3).reversed(),
        );
    }

    #[test]
    fn len_4() {
        assert_eq!(
            DVect::from_vec(vec![4., 3., 2., 1.]),
            DVect::from_vec(vec![1., 2., 3., 4.]).rows(0, 4).reversed(),
        );
    }
}

