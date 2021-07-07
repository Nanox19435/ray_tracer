

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vector3{
        Vector3 {x,y,z}
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn normalized(self) -> Vector3 {
        self/self.length()
    }

    pub fn dot(self, v: Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    //This one could be wrong
    pub fn cross(self, v: Vector3) -> Vector3 {
        Vector3 {x: self.y * v.z - self.z * v.y,
                 y: self.z * v.x - self.x * v.z,
                 z: self.x * v.y - self.y * v.x}
    }
}

//implements addition
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
impl std::ops::Add<Vector3> for f64 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {x: self + rhs.x, y: self + rhs.y, z: self + rhs.z}
    }
}

//implements substraction
impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

//implements multiplication
impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}
impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}


//implements division
impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3 {x: self.x/rhs, y: self.y/rhs, z: self.z/rhs}
    }
}

//implements the negative operator
impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        -1.0 * self
    }
}

pub fn write_color(pixel_color: Vector3) {
    println!("{} {} {}", (255.999*pixel_color.x) as i32, (255.999*pixel_color.y) as i32,
                         (255.999*pixel_color.z) as i32);
}
