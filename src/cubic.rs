use bevy::math::Vec2;
use crate::hex_map::HEX_SIZE;

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

    pub fn to_pixel(&self, hs: f32) -> Vec2 {
        let hex_size = hs - 1.; // avoid 1 pixel space between hexes
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

pub fn cubic_from_pixel(pixel: Vec2) -> Cubic {
    let q = (2./3. * pixel.x) / HEX_SIZE;
    let r = (-1./3. * pixel.x + f32::sqrt(3.)/3. * pixel.y) / HEX_SIZE;
    return cubic_round(q, r, -q - r);
}

fn cubic_round(frac_q: f32, frac_r: f32, frac_s: f32) -> Cubic {
    let mut q = frac_q.round() as f32;
    let mut r = frac_r.round() as f32;
    let mut s = frac_s.round() as f32;
    
    let q_diff = (q - frac_q).abs();
    let r_diff = (r - frac_r).abs();
    let s_diff = (s - frac_s).abs();
    
    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    } else {
        s = -q - r;
    }
    return Cubic { coords: [q as i16, r as i16, s as i16]};
}
