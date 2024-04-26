use super::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Intersection {
    None,
    Single(Num),
    Double(Num, Num),
}

impl Intersection {
    fn iter(&self) -> IntersectionIter {
        IntersectionIter::new(*self)
    }
}

impl std::cmp::PartialEq<Intersection> for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        match (self, other) {
            (Intersection::None, Intersection::None) => true,
            (Intersection::Single(n1), Intersection::Single(n2)) => nums_equal(*n1, *n2),
            (Intersection::Double(fn1, fn2), Intersection::Double(sn1, sn2)) => {
                nums_equal(*fn1, *sn1) && nums_equal(*fn2, *sn2)
            }
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<Vec<Num>> for Intersection {
    fn eq(&self, other: &Vec<Num>) -> bool {
        let nums = self.into_iter().collect_vec();
        if nums.len() != other.len() {
            return false;
        }
        nums.iter()
            .zip(other.into_iter())
            .all(|(n1, n2)| nums_equal(*n1, *n2))
    }
}

impl IntoIterator for Intersection {
    type IntoIter = IntersectionIter;
    type Item = Num;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IntersectionIter {
    xs: Intersection,
    idx: usize,
}

impl IntersectionIter {
    fn new(xs: Intersection) -> Self {
        Self { xs, idx: 0 }
    }
}

impl Iterator for IntersectionIter {
    type Item = Num;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.xs {
            Intersection::None => None,
            Intersection::Single(n) => {
                if self.idx == 0 {
                    Some(n)
                } else {
                    None
                }
            }
            Intersection::Double(n1, n2) => match self.idx {
                0 => Some(n1),
                1 => Some(n2),
                _ => None,
            },
        };
        self.idx += 1;
        res
    }
}
