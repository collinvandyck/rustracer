use super::prelude::*;

pub fn intersection(t: impl Into<Num>, object: impl Into<Intersected>) -> Intersection {
    Intersection::new(t, object)
}

pub fn intersections(xs: impl IntoIterator<Item = Intersection>) -> Intersections {
    Intersections(xs.into_iter().collect())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Intersections(Vec<Intersection>);

impl std::ops::Deref for Intersections {
    type Target = Vec<Intersection>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

    #[test]
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1, s);
        let i2 = intersection(2, s);
        let xs = intersections([i1, i2]);
        assert_eq!(xs.iter().map(|i| i.t).collect_vec(), vec![1.0, 2.0]);
    }
}
