//! 🧭️ A fixed planar contour for clipping, distance and prism construction.

/// 🪟️ An oriented planar contour with a fixed maximum vertex count.
#[derive(Clone, Copy)]
pub struct Footprint {
    points: [[f64; 2]; 16],
    count: usize,
}

fn difference(a: [f64; 2], b: [f64; 2]) -> [f64; 2] { [a[0] - b[0], a[1] - b[1]] }

impl Footprint {
    pub fn rectangle(width: f64, depth: f64) -> Self {
        let mut points = [[0.0; 2]; 16];
        points[..4].copy_from_slice(&[[0.0, 0.0], [width, 0.0], [width, depth], [0.0, depth]]);
        Self { count: 4, points }
    }

    fn push(&mut self, point: [f64; 2]) -> Option<()> {
        if self.count == 0 || self.points[self.count - 1] != point {
            *self.points.get_mut(self.count)? = point;
            self.count += 1;
        }
        Some(())
    }

    pub fn clip(self, [x, y]: [f64; 2], limit: f64) -> Option<Self> {
        let mut result = Self { points: [[0.0; 2]; 16], count: 0 };
        if self.count == 0 { return Some(result); }
        let signed = |[u, v]: [f64; 2]| x * u + y * v - limit;
        let mut a = self.points[self.count - 1];
        let mut da = signed(a);
        for &b in &self.points[..self.count] {
            let db = signed(b);
            if !da.is_finite() || !db.is_finite() { return None; }
            if (da < 0.0 && db > 0.0) || (da > 0.0 && db < 0.0) {
                let scale = da.abs().max(db.abs());
                let f = (da.abs() / scale) / (da.abs() / scale + db.abs() / scale);
                result.push(std::array::from_fn(|i| a[i] * (1.0 - f) + b[i] * f))?;
            }
            if db <= 0.0 { result.push(b)?; }
            (a, da) = (b, db);
        }
        if result.count > 1 && result.points[0] == result.points[result.count - 1] { result.count -= 1; }
        Some(result)
    }

    pub fn remove_collinear(&mut self) {
        let mut i = 0;
        while self.count >= 3 && i < self.count {
            let a = difference(self.points[i], self.points[(i + self.count - 1) % self.count]);
            let b = difference(self.points[(i + 1) % self.count], self.points[i]);
            if a[0] * b[1] == a[1] * b[0] && a[0] * b[0] + a[1] * b[1] >= 0.0 {
                self.points.copy_within(i+1..self.count, i);
                self.count -= 1;
                i = i.saturating_sub(1);
            } else { i += 1; }
        }
    }

    pub fn signed_distance(&self, point: [f64; 2]) -> f64 {
        Self::ring_signed_distance(self.points(),point)
    }

    pub fn ring_signed_distance(points: &[[f64;2]], point: [f64;2]) -> f64 {
        let mut distance = f64::INFINITY;
        let mut inside = false;
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i+1)%points.len()];
            let [dx,dy] = difference(b,a);
            let [x,y] = difference(point,a);
            let length = dx.hypot(dy);
            let [u,v] = [dx/length,dy/length];
            if (a[1]>point[1])!=(b[1]>point[1]) && x < (y/dy)*dx { inside = !inside; }
            let along = (x*u + y*v).clamp(0.0,length);
            distance = distance.min((x-u*along).hypot(y-v*along));
        }
        if inside { -distance } else { distance }
    }

    /// 📐️ Copies the assembler's one positively oriented material ring.
    pub fn from_ring(ring: &[[f64;2]]) -> Option<Self> {
        if !(3..=16).contains(&ring.len()) { return None; }
        let mut points = [[0.;2];16];
        points[..ring.len()].copy_from_slice(ring);
        let shape = Self { points, count: ring.len() };
        (shape.twice_area() > 0.).then_some(shape)
    }

    pub fn points(&self) -> &[[f64;2]] { &self.points[..self.count] }

    pub fn twice_area(&self) -> f64 {
        (1..self.count.saturating_sub(1)).map(|i| {
            let [x, y] = difference(self.points[i], self.points[0]);
            let [u, v] = difference(self.points[i + 1], self.points[0]);
            x * v - y * u
        }).sum()
    }

}
