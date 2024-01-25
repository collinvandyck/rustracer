use std::ops;

type Num = f64;

pub struct Point {
    tup: Tuple4,
}

impl ops::Deref for Point {
    type Target = Tuple4;
    fn deref(&self) -> &Self::Target {
        &self.tup
    }
}

impl ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tup
    }
}

impl Point {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Self {
            tup: Tuple4([x, y, z, 0.0]),
        }
    }
}

pub struct Vector {
    tup: Tuple4,
}

impl ops::Deref for Vector {
    type Target = Tuple4;
    fn deref(&self) -> &Self::Target {
        &self.tup
    }
}

impl ops::DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tup
    }
}

impl Vector {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Self {
            tup: Tuple4([x, y, z, 1.0]),
        }
    }
}

pub struct Tuple4([Num; 4]);

impl Tuple4 {
    pub fn x(&self) -> Num {
        self.0[0]
    }
    pub fn y(&self) -> Num {
        self.0[1]
    }
    pub fn z(&self) -> Num {
        self.0[2]
    }
    pub fn set_x(&mut self, num: Num) {
        self.0[0] = num
    }
    pub fn set_y(&mut self, num: Num) {
        self.0[1] = num
    }
    pub fn set_z(&mut self, num: Num) {
        self.0[2] = num
    }
}
