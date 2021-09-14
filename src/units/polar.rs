use cgmath::Vector4;

pub struct Polar {
    pub theta: f32,
    pub phi: f32,
    pub radius: f32,
}

impl Polar {
    pub fn to_cart(&self) -> Vector4<f32> {
        let x = self.radius * self.theta.sin() * self.phi.cos();
        let y = self.radius * self.theta.cos();
        let z = self.radius * self.theta.sin() * self.phi.sin();
        Vector4::new(x, y, z, 1.)
    }
}
