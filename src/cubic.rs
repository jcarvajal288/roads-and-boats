use bevy::math::Vec2;

#[derive(PartialEq, Eq, Hash)]
pub struct Cubic {
    coords: [i16; 3]
}

impl Cubic {
    pub fn q(&self) -> i16 {
        return self.coords[0];
    }

    pub fn r(&self) -> i16 {
        return self.coords[1];
    }

    pub fn s(&self) -> i16 {
        return self.coords[2];
    }

    pub fn to_pixel(&self, hex_size: f32) -> Vec2 {
        let q = f32::from(self.q());
        let r = f32::from(self.r());
        let x: f32 = hex_size * (3./2. * q);
        let y: f32 = -1. * hex_size * (f32::sqrt(3.)/2. * q + f32::sqrt(3.) * r);
        return Vec2 { x, y };
    }
}

pub fn create_axial(c: [i16; 2]) -> Cubic {
    return create_cubic([c[0], c[1], -(c[0])-c[1]]);
}

pub fn create_cubic(c: [i16; 3]) -> Cubic {
    assert_eq!(c[0] + c[1] + c[2], 0);
    return Cubic { coords: c };
}
