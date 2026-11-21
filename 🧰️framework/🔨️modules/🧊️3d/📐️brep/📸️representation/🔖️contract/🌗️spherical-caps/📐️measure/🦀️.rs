//! 📐️ Stable spherical cap measures about the common rim plane.

use std::f64::consts::PI;

fn normalized(rim: f64, height: [f64; 2]) -> (f64, f64, [f64; 2]) {
    let scale = rim.max(height[0]).max(height[1]);
    (scale, (rim / scale).powi(2), height.map(|value| value / scale))
}

/// 🫙️ Measures the signed material enclosed by the two caps.
pub(super) fn volume(rim: f64, height: [f64; 2], inward: bool) -> f64 {
    let (scale, radial, [a, b]) = normalized(rim, height);
    let local = if inward {
        ((height[0] - height[1]) / scale) * (3.0 * radial + a * a + a * b + b * b)
    } else {
        a * (3.0 * radial + a * a) + b * (3.0 * radial + b * b)
    };
    ((local * (PI / 6.0) * scale) * scale) * scale
}

/// ⚖️ Locates the material centroid along the common axis, measured from the rim.
pub(super) fn centroid(rim: f64, height: [f64; 2], positive: [bool; 2], inward: bool) -> f64 {
    let (scale, radial, [a, b]) = normalized(rim, height);
    let [a2, b2] = [a * a, b * b];
    let [sign_a, sign_b] = positive.map(|positive| if positive { 1.0 } else { -1.0 });
    let local = if inward {
        sign_a * (a + b) * (2.0 * radial + a2 + b2) / (2.0 * (3.0 * radial + a2 + a * b + b2))
    } else {
        let moment = (sign_a * a2).mul_add(2.0 * radial + a2, sign_b * b2 * (2.0 * radial + b2));
        moment / (2.0 * (a * (3.0 * radial + a2) + b * (3.0 * radial + b2)))
    };
    local * scale
}

/// 🎚️ Integrates disk area and first moment across a bounded spherical band.
pub(super) fn band(radius:f64,[low,high]:[f64;2],[a,b]:[f64;2])->(f64,f64,f64){
    let height=high-low;
    let middle=low+height*0.5;
    let disk_sum=a*a+b*b;
    let height_term=height*height/6.;
    let mean=disk_sum*0.5+height_term;
    (PI*height*mean,PI*(2.*radius*height+disk_sum),middle*(1.-height_term/mean))
}
