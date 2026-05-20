use std::io::Write;
use crate::vector3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;


fn linear_to_gamma(linear_component: f64) -> f64 { 
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.0, 0.999);
  
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
