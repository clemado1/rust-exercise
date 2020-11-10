use std::ops::AddAssign;

pub trait Rangeable: AddAssign + PartialOrd +Copy {}

impl<T: AddAssign + PartialOrd + Copy> Rangeable for T {}

pub struct GenRangeIterator<T: Rangeable> {
    curr: T,
    stop: T,
    step: T,
}

impl<T: Rangeable> GenRangeIterator<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenRangeIterator {
            curr: start,
            stop,
            step,
        }
    }
}

impl<T: Rangeable> Iterator for GenRangeIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::gen_range::GenRangeIterator;

    #[test]
    fn gen_test1() {
        let mut m = 0.;
        let mut it = GenRangeIterator::new(5., 12., 2.5);
        for s in it {
            m += s;
        }
        assert_eq!(m, 5. + 7.5 + 10., "Test 1");
    }
}
