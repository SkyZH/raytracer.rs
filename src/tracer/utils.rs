use super::Vec3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut vec = Vec3::zero();
    while vec.squared_length() < 1.0 {
        vec = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    vec
}

pub fn gamma_correct(color: Vec3) -> Vec3 {
    Vec3::new(
        ramp(color.x.sqrt()),
        ramp(color.y.sqrt()),
        ramp(color.z.sqrt()),
    )
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - n * Vec3::dot(v, n) * 2.0;
}

pub fn refract(v: Vec3, n: Vec3, ratio: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ratio * ratio * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - n * dt) * ratio - n * discriminant.sqrt());
    } else {
        return None;
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn ramp(color: f32) -> f32 {
    if color > 1.0 {
        return 1.0;
    } else if color < 0.0 {
        return 0.0;
    } else {
        return color;
    }
}

#[cfg(test)]
mod tests {
    use super::random_in_unit_sphere;

    #[test]
    fn test_random_in_unit_shpere() {
        let vec = random_in_unit_sphere();
        assert!(vec.squared_length() >= 1.0);
    }
}
