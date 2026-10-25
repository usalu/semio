//! 🔲️ Rect shapes of the geometry primitives: the rectangle, the rounded rectangle with its radii and the corner arc helper. See [`super`].

use super::*;

// #region 🔖️Shapes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub(crate) x0: f64,
    pub(crate) y0: f64,
    pub(crate) x1: f64,
    pub(crate) y1: f64,
}

impl Rect {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { x0, y0, x1, y1 }
    }
    pub fn from_points(p0: Point, p1: Point) -> Self {
        Self { x0: p0.x.min(p1.x), y0: p0.y.min(p1.y), x1: p0.x.max(p1.x), y1: p0.y.max(p1.y) }
    }
    pub fn inflate(self, dx: f64, dy: f64) -> Self {
        Self { x0: self.x0 - dx, y0: self.y0 - dy, x1: self.x1 + dx, y1: self.y1 + dy }
    }
    pub fn x0(self) -> f64 {
        self.x0
    }
    pub fn y0(self) -> f64 {
        self.y0
    }
    pub fn x1(self) -> f64 {
        self.x1
    }
    pub fn y1(self) -> f64 {
        self.y1
    }
    pub fn width(self) -> f64 {
        self.x1 - self.x0
    }
    pub fn height(self) -> f64 {
        self.y1 - self.y0
    }
    /// 📦️ Exact — a rect's own outline needs no curve flattening, so `tolerance` is unused.
    pub fn path_elements(&self, _tolerance: f64) -> Vec<PathEl> {
        vec![
            PathEl::MoveTo(Point::new(self.x0, self.y0)),
            PathEl::LineTo(Point::new(self.x1, self.y0)),
            PathEl::LineTo(Point::new(self.x1, self.y1)),
            PathEl::LineTo(Point::new(self.x0, self.y1)),
            PathEl::ClosePath,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRectRadii {
    pub(crate) top_left: f64,
    pub(crate) top_right: f64,
    pub(crate) bottom_right: f64,
    pub(crate) bottom_left: f64,
}

impl RoundedRectRadii {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }
    /// 🧾️ Clockwise corner radii from top-left, the order `ctx.roundRect` itself takes.
    pub fn as_clockwise(self) -> [f64; 4] {
        [self.top_left, self.top_right, self.bottom_right, self.bottom_left]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRect {
    pub(crate) rect: Rect,
    pub(crate) radii: RoundedRectRadii,
}

impl RoundedRect {
    /// 📤️ The normalized body rectangle, for encoders that emit the primitive itself rather than
    /// its flattening (`canvas::draw_list`, whose `ctx.roundRect` twin needs the exact corners).
    pub fn rect(self) -> Rect {
        self.rect
    }
    /// 🔃️ The four normalized corner radii, clockwise from top-left.
    pub fn radii(self) -> RoundedRectRadii {
        self.radii
    }
    pub fn new(rect: Rect, radii: RoundedRectRadii) -> Self {
        let rect = Rect::new(rect.x0.min(rect.x1), rect.y0.min(rect.y1), rect.x0.max(rect.x1), rect.y0.max(rect.y1));
        let max_radius = rect.width().min(rect.height()) / 2.0;
        let normalize = |radius: f64| radius.abs().min(max_radius);
        let radii = RoundedRectRadii::new(normalize(radii.top_left), normalize(radii.top_right), normalize(radii.bottom_right), normalize(radii.bottom_left));
        Self { rect, radii }
    }
    /// 🍕️ Four straight edges plus four quarter-elliptical corners (circular, one radius per
    /// corner), preserving the `kurbo` oracle's element order and zero-radius cubic elements.
    pub fn path_elements(&self, tolerance: f64) -> Vec<PathEl> {
        let Rect { x0, y0, x1, y1 } = self.rect;
        let tl = self.radii.top_left;
        let tr = self.radii.top_right;
        let br = self.radii.bottom_right;
        let bl = self.radii.bottom_left;
        let quarter = std::f64::consts::FRAC_PI_2;
        let mut out = vec![PathEl::MoveTo(Point::new(x0, y0 + tl))];
        push_corner_arc(&mut out, Point::new(x0 + tl, y0 + tl), tl, 2.0 * quarter, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x1 - tr, y0)));
        push_corner_arc(&mut out, Point::new(x1 - tr, y0 + tr), tr, 3.0 * quarter, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x1, y1 - br)));
        push_corner_arc(&mut out, Point::new(x1 - br, y1 - br), br, 0.0, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x0 + bl, y1)));
        push_corner_arc(&mut out, Point::new(x0 + bl, y1 - bl), bl, quarter, quarter, tolerance);
        out.push(PathEl::ClosePath);
        out
    }
}

fn push_corner_arc(out: &mut Vec<PathEl>, center: Point, radius: f64, start_angle: f64, sweep: f64, tolerance: f64) {
    for seg in elliptical_arc_segments(center, (radius, radius), 0.0, start_angle, sweep, tolerance) {
        out.push(seg.as_path_el());
    }
}
// #endregion 🔖️Shapes
