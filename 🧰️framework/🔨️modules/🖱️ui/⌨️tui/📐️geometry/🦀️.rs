/// 📍️ A cell coordinate on the terminal grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

/// 📏️ A cell-grid size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// 🔲️ An axis-aligned cell-grid rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// 🧭️ Whether `pos` lies within this rect.
    pub fn contains(&self, pos: Pos) -> bool {
        pos.x >= self.x && pos.x < self.x + self.width && pos.y >= self.y && pos.y < self.y + self.height
    }

    /// ✂️ The overlap between two rects (an empty rect on miss).
    pub fn intersect(&self, other: Rect) -> Rect {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.width).min(other.x + other.width);
        let y1 = (self.y + self.height).min(other.y + other.height);
        if x1 <= x0 || y1 <= y0 {
            Rect::default()
        } else {
            Rect::new(x0, y0, x1 - x0, y1 - y0)
        }
    }

    /// 🧊️ Shrinks the rect by `margin` cells on every side.
    pub fn inset(&self, margin: u16) -> Rect {
        self.inset_sides(margin, margin, margin, margin)
    }

    /// 🔳 Shrinks the rect by `top`/`right`/`bottom`/`left` cells.
    pub fn inset_sides(&self, top: u16, right: u16, bottom: u16, left: u16) -> Rect {
        let width = self.width.saturating_sub(left + right);
        let height = self.height.saturating_sub(top + bottom);
        Rect::new(self.x + left.min(self.width), self.y + top.min(self.height), width, height)
    }

    /// 🔝 Splits off `rows` rows from the top, returning `(top, rest)`.
    pub fn split_top(&self, rows: u16) -> (Rect, Rect) {
        let rows = rows.min(self.height);
        let top = Rect::new(self.x, self.y, self.width, rows);
        let rest = Rect::new(self.x, self.y + rows, self.width, self.height - rows);
        (top, rest)
    }

    /// 🔚 Splits off `rows` rows from the bottom, returning `(rest, bottom)`.
    pub fn split_bottom(&self, rows: u16) -> (Rect, Rect) {
        let rows = rows.min(self.height);
        let bottom = Rect::new(self.x, self.y + self.height - rows, self.width, rows);
        let rest = Rect::new(self.x, self.y, self.width, self.height - rows);
        (rest, bottom)
    }
}
