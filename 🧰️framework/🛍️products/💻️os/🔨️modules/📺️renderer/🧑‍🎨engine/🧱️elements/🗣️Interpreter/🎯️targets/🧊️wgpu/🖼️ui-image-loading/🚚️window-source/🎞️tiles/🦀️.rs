//! 🧩️ Native window image raster tiles sized for the existing fixed upload credits.

const EDGE: u32 = 510;

#[derive(Clone, Copy)]
pub(super) struct ImageLayout {
    pub width: u32,
    pub height: u32,
}

impl ImageLayout {
    fn columns(self) -> u32 {
        self.width.div_ceil(EDGE)
    }
    pub fn count(self) -> usize {
        (self.columns() * self.height.div_ceil(EDGE)) as usize
    }

    pub fn tile(self, index: usize) -> ImageTile {
        let columns = self.columns();
        let left = index as u32 % columns * EDGE;
        let top = index as u32 / columns * EDGE;
        let width = EDGE.min(self.width - left);
        let height = EDGE.min(self.height - top);
        let padded_left = left.saturating_sub(1);
        let padded_top = top.saturating_sub(1);
        let padded_width = (left + width + 1).min(self.width) - padded_left;
        let padded_height = (top + height + 1).min(self.height) - padded_top;
        ImageTile { left, top, width, height, padded_left, padded_top, padded_width, padded_height }
    }
}

pub(super) struct ImageTile {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub padded_left: u32,
    pub padded_top: u32,
    pub padded_width: u32,
    pub padded_height: u32,
}

impl ImageTile {
    pub fn fill(&self, pixels: &[u8], row_width: u32, target: &mut Vec<u8>) {
        let pitch = row_width as usize * 4;
        let width = self.padded_width as usize * 4;
        target.clear();
        for row in self.padded_top..self.padded_top + self.padded_height {
            let offset = row as usize * pitch + self.padded_left as usize * 4;
            target.extend_from_slice(&pixels[offset..offset + width]);
        }
    }

    pub fn uv(&self) -> [f32; 4] {
        let left = (self.left - self.padded_left) as f32;
        let top = (self.top - self.padded_top) as f32;
        let padded_width = self.padded_width as f32;
        let padded_height = self.padded_height as f32;
        [left / padded_width, top / padded_height, (left + self.width as f32) / padded_width, (top + self.height as f32) / padded_height]
    }
}
