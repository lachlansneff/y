use glam::Vec3;

use crate::{
    deriv::{Deriv, Deriv3},
    extra::VectorN as _,
};

pub fn sphere(p: Deriv3, r: f32) -> Deriv {
    p.length() - r
}

pub fn rectangular_prism(p: Deriv3, sides: Vec3) -> Deriv {
    let q = p.abs() - sides;
    q.max(Deriv3::zero()).length() + q.y.max(q.z.max(q.x.min(Deriv::new(0.0))))
}

// pub fn cylinder(p: DualVec3, h: f32, r: f32) -> f32 {
//     let d = vec2(p.xz().length(), p.y).abs() - vec2(r, h);
//     d.x.max(d.y).min(0.0) + d.max(Vec2::ZERO).length()
// }

pub fn schwarz_p(p: Deriv3, scale: f32, thickness: f32) -> Deriv {
    let p = p * scale;
    (p.cos().dot(Deriv3::one()).abs() / scale - thickness) * 0.6
}

pub fn union(lhs: Deriv, rhs: Deriv) -> Deriv {
    lhs.min(rhs)
}

pub fn intersect(lhs: Deriv, rhs: Deriv) -> Deriv {
    lhs.max(rhs)
}