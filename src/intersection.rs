use super::prelude::*;

pub fn intersection(t: impl Into<Num>, object: impl Into<Intersected>) -> Intersection {
    Intersection::new(t, object)
}

#[derive(Clone, Debug)]
pub struct Intersection {
    t: Num,
    object: Intersected,
}

impl Intersection {
    pub fn new(t: impl Into<Num>, object: impl Into<Intersected>) -> Self {
        Self {
            t: t.into(),
            object: object.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Intersected {
    Sphere(Sphere),
}

impl PartialEq<Sphere> for Intersected {
    #[allow(irrefutable_let_patterns)]
    fn eq(&self, other: &Sphere) -> bool {
        if let Intersected::Sphere(s) = self {
            s == other
        } else {
            false
        }
    }
}

impl From<Sphere> for Intersected {
    fn from(value: Sphere) -> Self {
        Intersected::Sphere(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }
}
