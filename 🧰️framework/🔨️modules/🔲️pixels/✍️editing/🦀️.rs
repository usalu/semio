//! 🎨️ Native RGBA8 pixel editing with owned, cancellable candidates and deterministic sampling.
use crate::RasterImage;

pub const MAX_IMAGE_SIDE: u32 = 16_384;
pub const MAX_IMAGE_PIXELS: usize = 16 * 1024 * 1024;
pub type PixelColor = [u8; 4];

#[derive(Clone, Debug, PartialEq)]
pub struct PixelBrush {
    pub points: Vec<[f64; 2]>,
    pub size: f64,
    pub opacity: f64,
    pub hardness: f64,
    pub color: PixelColor,
    pub erase: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PixelAlphaBrush {
    pub points: Vec<[f64; 2]>,
    pub size: f64,
    pub opacity: f64,
    pub hardness: f64,
    pub alpha: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PixelOperation {
    Invert,
    Grayscale,
    Clear,
    FlipHorizontal,
    FlipVertical,
    RotateClockwise,
    RotateCounterclockwise,
    Brightness(f64),
    Contrast(f64),
    Saturation(f64),
    Gamma(f64),
    Threshold(f64),
    Posterize(u16),
    Blur(u8),
    Sharpen(f64),
    Resize { width: u32, height: u32, bilinear: bool },
    Crop { x: u32, y: u32, width: u32, height: u32 },
    Fill(PixelColor),
    Stroke(PixelBrush),
    AlphaStroke(PixelAlphaBrush),
    AlphaFill { alpha: u8, opacity: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectionShape {
    Box { ellipse: bool, x: f64, y: f64, width: f64, height: f64 },
    Polygon(Vec<[f64; 2]>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMerge {
    Replace,
    Add,
    Subtract,
    Intersect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelProgress {
    pub completed: usize,
    pub total: usize,
    pub done: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PixelEditError {
    Invalid(&'static str),
    Incomplete,
    Cancelled,
}

impl std::fmt::Display for PixelEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => f.write_str(message),
            Self::Incomplete => f.write_str("Pixel operation is incomplete"),
            Self::Cancelled => f.write_str("Pixel operation cancelled"),
        }
    }
}

impl std::error::Error for PixelEditError {}

pub fn validate_extent(width: u32, height: u32) -> Result<usize, PixelEditError> {
    let count = u64::from(width) * u64::from(height);
    if width == 0 || height == 0 || width > MAX_IMAGE_SIDE || height > MAX_IMAGE_SIDE || count > MAX_IMAGE_PIXELS as u64 {
        return Err(PixelEditError::Invalid("Image extent exceeds the pixel budget"));
    }
    Ok(count as usize)
}

pub fn validate_image(image: &RasterImage) -> Result<(), PixelEditError> {
    if image.pixels.len() != validate_extent(image.width, image.height)? * 4 {
        return Err(PixelEditError::Invalid("RGBA8 length does not match image extent"));
    }
    Ok(())
}

fn byte(value: f64) -> u8 {
    value.clamp(0.0, 255.0).round() as u8
}

fn sample(image: &RasterImage, x: i64, y: i64) -> PixelColor {
    let x = x.clamp(0, i64::from(image.width) - 1) as usize;
    let y = y.clamp(0, i64::from(image.height) - 1) as usize;
    let i = (y * image.width as usize + x) * 4;
    [image.pixels[i], image.pixels[i + 1], image.pixels[i + 2], image.pixels[i + 3]]
}

pub fn source_over(destination: PixelColor, source: PixelColor, opacity: f64) -> PixelColor {
    let sa = f64::from(source[3]) / 255.0 * opacity.clamp(0.0, 1.0);
    let da = f64::from(destination[3]) / 255.0;
    let alpha = sa + da * (1.0 - sa);
    if alpha == 0.0 {
        return [0; 4];
    }
    let mut result = [0; 4];
    for c in 0..3 {
        result[c] = byte((f64::from(source[c]) * sa + f64::from(destination[c]) * da * (1.0 - sa)) / alpha);
    }
    result[3] = byte(alpha * 255.0);
    result
}

fn weighted_sample(image: &RasterImage, x: f64, y: f64) -> PixelColor {
    let (x0, y0) = (x.floor() as i64, y.floor() as i64);
    let (fx, fy) = (x - x.floor(), y - y.floor());
    let samples = [sample(image, x0, y0), sample(image, x0 + 1, y0), sample(image, x0, y0 + 1), sample(image, x0 + 1, y0 + 1)];
    let weights = [(1.0 - fx) * (1.0 - fy), fx * (1.0 - fy), (1.0 - fx) * fy, fx * fy];
    let mut alpha = 0.0;
    let mut rgb = [0.0; 3];
    for i in 0..4 {
        let a = f64::from(samples[i][3]) * weights[i];
        alpha += a;
        for c in 0..3 {
            rgb[c] += f64::from(samples[i][c]) * a;
        }
    }
    if alpha > 0.0 { [byte(rgb[0] / alpha), byte(rgb[1] / alpha), byte(rgb[2] / alpha), byte(alpha)] } else { [0; 4] }
}

impl PixelOperation {
    fn stroke(&self) -> Option<(&[[f64;2]], f64, f64, f64)> {
        match self {
            Self::Stroke(brush)=>Some((&brush.points,brush.size,brush.opacity,brush.hardness)),
            Self::AlphaStroke(brush)=>Some((&brush.points,brush.size,brush.opacity,brush.hardness)),
            _=>None,
        }
    }

    fn extent(&self, source: &RasterImage) -> (u32, u32) {
        match self {
            Self::RotateClockwise | Self::RotateCounterclockwise => (source.height, source.width),
            Self::Crop { width, height, .. } | Self::Resize { width, height, .. } => (*width, *height),
            _ => (source.width, source.height),
        }
    }

    fn validate(&self, source: &RasterImage, selected: bool) -> Result<(), PixelEditError> {
        let finite_range = |value: f64, min: f64, max: f64| value.is_finite() && value >= min && value <= max;
        if let Some((points,size,opacity,hardness)) = self.stroke() {
            if !(1..=2048).contains(&points.len()) || !points.iter().flatten().all(|p| p.is_finite())
                || !finite_range(size, 0.1, 4096.0) || !finite_range(opacity, 0.0, 1.0) || !finite_range(hardness, 0.0, 1.0) {
                return Err(PixelEditError::Invalid("Invalid brush stroke"));
            }
        }
        let valid = match *self {
            Self::AlphaFill {opacity,..} => finite_range(opacity,0.0,1.0),
            Self::Brightness(v) | Self::Contrast(v) | Self::Saturation(v) => finite_range(v, -1.0, 1.0),
            Self::Gamma(v) => finite_range(v, 0.01, 10.0),
            Self::Threshold(v) => finite_range(v, 0.0, 255.0),
            Self::Posterize(v) => (2..=256).contains(&v),
            Self::Blur(v) => v <= 16,
            Self::Sharpen(v) => finite_range(v, 0.0, 5.0),
            Self::Crop { x, y, width, height } => u64::from(x) + u64::from(width) <= u64::from(source.width) && u64::from(y) + u64::from(height) <= u64::from(source.height),
            _ => true,
        };
        if !valid {
            return Err(PixelEditError::Invalid("Invalid operation parameter"));
        }
        let (width, height) = self.extent(source);
        validate_extent(width, height)?;
        if selected && matches!(self, Self::Crop { .. } | Self::Resize { .. } | Self::FlipHorizontal | Self::FlipVertical | Self::RotateClockwise | Self::RotateCounterclockwise) {
            return Err(PixelEditError::Invalid("Geometry operations require an unrestricted image"));
        }
        Ok(())
    }

    fn filtered(&self, image: &RasterImage, x: u32, y: u32, selection_coverage: f64, stroke_segments: &[usize]) -> PixelColor {
        let original = sample(image, x.into(), y.into());
        if let Some((points,size,opacity,hardness)) = self.stroke() {
            let radius = size / 2.0;
            let mut coverage = 0.0_f64;
            for &index in stroke_segments {
                let to = &points[index];
                let from = points[index.saturating_sub(1)];
                let (px, py) = (f64::from(x) + 0.5, f64::from(y) + 0.5);
                if px < from[0].min(to[0]) - radius || px > from[0].max(to[0]) + radius || py < from[1].min(to[1]) - radius || py > from[1].max(to[1]) + radius {
                    continue;
                }
                let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
                let length = dx * dx + dy * dy;
                let t = if length == 0.0 { 0.0 } else { ((px - from[0]) * dx + (py - from[1]) * dy) / length }.clamp(0.0, 1.0);
                let distance = (px - from[0] - t * dx).hypot(py - from[1] - t * dy);
                let core = radius * hardness;
                let amount = if distance > radius { 0 } else if distance <= core { 255 } else { byte(255.0 * (radius - distance) / (radius - core)) };
                coverage = coverage.max(f64::from(amount) / 255.0);
            }
            let opacity = coverage * opacity * selection_coverage;
            if opacity == 0.0 { return original; }
            if let Self::AlphaStroke(brush)=self {
                return [original[0],original[1],original[2],byte(f64::from(original[3])+(f64::from(brush.alpha)-f64::from(original[3]))*opacity)];
            }
            let Self::Stroke(brush)=self else {unreachable!()};
            if brush.erase {
                let alpha = byte(f64::from(original[3]) * (1.0 - opacity));
                return if alpha > 0 { [original[0], original[1], original[2], alpha] } else { [0; 4] };
            }
            return source_over(original, brush.color, opacity);
        }
        match *self {
            Self::Clear => return [0; 4],
            Self::AlphaFill {alpha,opacity} => return [original[0],original[1],original[2],byte(f64::from(original[3])+(f64::from(alpha)-f64::from(original[3]))*opacity*selection_coverage)],
            Self::Fill(color) => return source_over(original, color, selection_coverage),
            Self::FlipHorizontal => return sample(image, i64::from(image.width) - i64::from(x) - 1, y.into()),
            Self::FlipVertical => return sample(image, x.into(), i64::from(image.height) - i64::from(y) - 1),
            Self::RotateClockwise => return sample(image, y.into(), i64::from(image.height) - i64::from(x) - 1),
            Self::RotateCounterclockwise => return sample(image, i64::from(image.width) - i64::from(y) - 1, x.into()),
            Self::Crop { x: left, y: top, .. } => return sample(image, i64::from(x + left), i64::from(y + top)),
            Self::Resize { width, height, bilinear } => {
                let sx = (f64::from(x) + 0.5) * f64::from(image.width) / f64::from(width);
                let sy = (f64::from(y) + 0.5) * f64::from(image.height) / f64::from(height);
                return if bilinear { weighted_sample(image, sx - 0.5, sy - 0.5) } else { sample(image, sx.floor() as i64, sy.floor() as i64) };
            }
            Self::Blur(0) | Self::Sharpen(0.0) => return original,
            Self::Blur(_) | Self::Sharpen(_) => {
                let radius = if let Self::Blur(radius) = *self { i64::from(radius) } else { 1 };
                let mut alpha = 0.0;
                let mut rgb = [0.0; 3];
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let neighbor = sample(image, i64::from(x) + dx, i64::from(y) + dy);
                        alpha += f64::from(neighbor[3]);
                        for c in 0..3 {
                            rgb[c] += f64::from(neighbor[c]) * f64::from(neighbor[3]);
                        }
                    }
                }
                let mut result = original;
                for c in 0..3 {
                    let average = if alpha > 0.0 { rgb[c] / alpha } else { 0.0 };
                    result[c] = byte(if let Self::Sharpen(amount) = *self { f64::from(original[c]) + amount * (f64::from(original[c]) - average) } else { average });
                }
                if matches!(self, Self::Blur(_)) {
                    result[3] = byte(alpha / ((radius * 2 + 1) * (radius * 2 + 1)) as f64);
                }
                return result;
            }
            _ => {}
        }
        let luma = 0.2126 * f64::from(original[0]) + 0.7152 * f64::from(original[1]) + 0.0722 * f64::from(original[2]);
        let mut result = original;
        for c in 0..3 {
            let value = f64::from(original[c]);
            result[c] = byte(match *self {
                Self::Invert => 255.0 - value,
                Self::Grayscale => luma,
                Self::Brightness(amount) => value + amount * 255.0,
                Self::Contrast(amount) => (value - 127.5) * 2.0_f64.powf(amount * 4.0) + 127.5,
                Self::Saturation(amount) => luma + (value - luma) * (amount + 1.0),
                Self::Gamma(amount) => 255.0 * (value / 255.0).powf(1.0 / amount),
                Self::Threshold(amount) => if luma >= amount { 255.0 } else { 0.0 },
                Self::Posterize(levels) => (value / 255.0 * f64::from(levels - 1)).round() * 255.0 / f64::from(levels - 1),
                _ => value,
            });
        }
        result
    }
}

/// 🧵️ An unpublished result whose work is bounded by an explicit pixel grant.
pub struct PixelEditJob {
    source: RasterImage,
    output: RasterImage,
    operation: PixelOperation,
    selection: Option<Vec<u8>>,
    cursor: usize,
    cancelled: bool,
    stroke_segments: Vec<usize>,
}

impl PixelEditJob {
    pub fn new(source: RasterImage, operation: PixelOperation, selection: Option<Vec<u8>>) -> Result<Self, PixelEditError> {
        validate_image(&source)?;
        if selection.as_ref().is_some_and(|mask| mask.len() != source.pixels.len() / 4) {
            return Err(PixelEditError::Invalid("Selection extent does not match image"));
        }
        operation.validate(&source, selection.is_some())?;
        let (width, height) = operation.extent(&source);
        Ok(Self { source, output: RasterImage {width,height,pixels:Vec::with_capacity(width as usize*height as usize*4)}, operation, selection, cursor: 0, cancelled: false, stroke_segments: Vec::new() })
    }

    pub fn recommended_grant(&self) -> usize {
        match self.operation { PixelOperation::Blur(_) => 128, PixelOperation::Stroke(_) | PixelOperation::AlphaStroke(_) => 256, _ => 4096 }
    }

    pub fn advance(&mut self, pixel_budget: usize) -> Result<PixelProgress, PixelEditError> {
        if self.cancelled {
            return Err(PixelEditError::Cancelled);
        }
        if !(1..=65536).contains(&pixel_budget) {
            return Err(PixelEditError::Invalid("Pixel budget must be 1–65536"));
        }
        let total = self.output.width as usize * self.output.height as usize;
        let end = total.min(self.cursor + pixel_budget);
        self.output.pixels.resize(end*4,0);
        while self.cursor < end {
            let x = self.cursor % self.output.width as usize;
            let y = self.cursor / self.output.width as usize;
            if x == 0 {
                if let Some((points,size,..)) = self.operation.stroke() {
                    self.stroke_segments.clear();
                    let py = y as f64 + 0.5;
                    for (index, to) in points.iter().enumerate() {
                        let from = points[index.saturating_sub(1)];
                        if py >= from[1].min(to[1]) - size / 2.0 && py <= from[1].max(to[1]) + size / 2.0 {
                            self.stroke_segments.push(index);
                        }
                    }
                }
            }
            let coverage = f64::from(self.selection.as_ref().map_or(255, |mask| mask[self.cursor])) / 255.0;
            let before = sample(&self.source, x as i64, y as i64);
            let after = if coverage == 0.0 { before } else { self.operation.filtered(&self.source, x as u32, y as u32, coverage, &self.stroke_segments) };
            let mix = if matches!(self.operation, PixelOperation::Fill(_) | PixelOperation::Stroke(_) | PixelOperation::AlphaStroke(_) | PixelOperation::AlphaFill {..}) { 1.0 } else { coverage };
            let target = &mut self.output.pixels[self.cursor * 4..self.cursor * 4 + 4];
            for c in 0..4 {
                target[c] = byte(f64::from(before[c]) + (f64::from(after[c]) - f64::from(before[c])) * mix);
            }
            if self.operation == PixelOperation::Clear && coverage > 0.0 && coverage < 1.0 {
                for c in 0..3 {
                    target[c] = if target[3] > 0 { before[c] } else { 0 };
                }
            }
            self.cursor += 1;
        }
        Ok(PixelProgress { completed: self.cursor, total, done: self.cursor == total })
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
        self.source.pixels = Vec::new();
        self.output.pixels = Vec::new();
        self.selection = None;
        self.stroke_segments = Vec::new();
    }

    pub fn result(&self) -> Result<&RasterImage, PixelEditError> {
        if self.cancelled {
            return Err(PixelEditError::Cancelled);
        }
        if self.cursor != self.output.width as usize * self.output.height as usize {
            return Err(PixelEditError::Incomplete);
        }
        Ok(&self.output)
    }

    pub fn into_result(self) -> Result<RasterImage, PixelEditError> {
        self.result()?;
        Ok(self.output)
    }
}

/// 🎯️ Owned scanline selection work with row-granted progress and atomic cancellation.
pub struct PixelSelectionJob {
    width:u32,
    height:u32,
    shape:SelectionShape,
    mask:Vec<u8>,
    row:u32,
    intersections:Vec<f64>,
    cancelled:bool,
}

impl PixelSelectionJob {
    pub fn new(width:u32,height:u32,shape:SelectionShape)->Result<Self,PixelEditError>{
        let count=validate_extent(width,height)?;
        match &shape{
            SelectionShape::Box{x,y,width,height,..} if ![x,y,width,height].iter().all(|v|v.is_finite())=>return Err(PixelEditError::Invalid("Invalid selection box")),
            SelectionShape::Polygon(points) if !(3..=4096).contains(&points.len())||!points.iter().flatten().all(|v|v.is_finite())=>return Err(PixelEditError::Invalid("Invalid selection polygon")),
            _=>{}
        }
        let edges=match &shape{SelectionShape::Polygon(points)=>points.len(),_=>0};
        Ok(Self{width,height,shape,mask:Vec::with_capacity(count),row:0,intersections:Vec::with_capacity(edges),cancelled:false})
    }

    fn fill(mask:&mut [u8],left:f64,right:f64,inclusive:bool){
        if left.is_nan()||right.is_nan(){return;}
        let width=mask.len() as f64;let from=(left-0.5).ceil().clamp(0.0,width) as usize;
        let to=(if inclusive{(right-0.5).floor()+1.0}else{(right-0.5).ceil()}).clamp(from as f64,width) as usize;
        mask[from..to].fill(255);
    }

    pub fn advance(&mut self,rows:usize)->Result<PixelProgress,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if !(1..=64).contains(&rows){return Err(PixelEditError::Invalid("Selection grant must be 1–64 rows"));}
        let end=self.height.min(self.row+rows as u32);
        while self.row<end{
            let start=self.mask.len();self.mask.resize(start+self.width as usize,0);let mask=&mut self.mask[start..];let y=f64::from(self.row)+0.5;
            match &self.shape{
                SelectionShape::Polygon(points)=>{
                    self.intersections.clear();let mut previous=points.len()-1;
                    for (index,a) in points.iter().enumerate(){let b=points[previous];if (a[1]>y)!=(b[1]>y){self.intersections.push((b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]);}previous=index;}
                    self.intersections.sort_unstable_by(f64::total_cmp);
                    for pair in self.intersections.chunks_exact(2){Self::fill(mask,pair[0],pair[1],false);}
                }
                SelectionShape::Box{ellipse,x,y:top,width,height}=>{
                    let (left,top,w,h)=(x.min(x+width),top.min(top+height),width.abs(),height.abs());
                    if w>0.0&&h>0.0&&y>=top&&y<=top+h{
                        if *ellipse{let radius=w/2.0*(1.0-((y-top-h/2.0)/(h/2.0)).powi(2)).max(0.0).sqrt();Self::fill(mask,left+w/2.0-radius,left+w/2.0+radius,true);}
                        else if y<top+h{Self::fill(mask,left,left+w,false);}
                    }
                }
            }
            self.row+=1;
        }
        Ok(PixelProgress{completed:self.row as usize,total:self.height as usize,done:self.row==self.height})
    }

    pub fn result(&self)->Result<&[u8],PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if self.row!=self.height{return Err(PixelEditError::Incomplete);}
        Ok(&self.mask)
    }

    pub fn into_result(self)->Result<Vec<u8>,PixelEditError>{self.result()?;Ok(self.mask)}
    pub fn cancel(&mut self){self.cancelled=true;self.mask=Vec::new();self.intersections=Vec::new();}
}

pub fn selection_mask(width:u32,height:u32,shape:&SelectionShape)->Result<Vec<u8>,PixelEditError>{
    let mut job=PixelSelectionJob::new(width,height,shape.clone())?;
    while !job.advance(64)?.done{}
    job.into_result()
}

pub struct SelectionCombineJob<'a> {
    current:Option<&'a [u8]>,
    next:&'a [u8],
    mode:SelectionMerge,
    output:Vec<u8>,
    cancelled:bool,
}
impl<'a> SelectionCombineJob<'a> {
    pub fn new(current:Option<&'a [u8]>,next:&'a [u8],mode:SelectionMerge)->Result<Self,PixelEditError> {
        if next.is_empty()||next.len()>MAX_IMAGE_PIXELS||current.is_some_and(|source|source.len()!=next.len()) {return Err(PixelEditError::Invalid("Selection dimensions are invalid"));}
        Ok(Self {current,next,mode,output:Vec::with_capacity(next.len()),cancelled:false})
    }
    pub fn advance(&mut self,pixel_budget:usize)->Result<PixelProgress,PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        let end=self.next.len().min(self.output.len()+pixel_budget.min(32768));
        for index in self.output.len()..end {
            let a=self.current.map_or(0,|source|source[index]);let b=self.next[index];
            self.output.push(match self.mode {SelectionMerge::Replace=>b,SelectionMerge::Add=>a.max(b),SelectionMerge::Subtract=>a.saturating_sub(b),SelectionMerge::Intersect=>a.min(b)});
        }
        Ok(PixelProgress {completed:self.output.len(),total:self.next.len(),done:self.output.len()==self.next.len()})
    }
    pub fn cancel(&mut self) {self.cancelled=true;self.output=Vec::new();}
    pub fn result(&self)->Result<&[u8],PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        if self.output.len()!=self.next.len() {return Err(PixelEditError::Incomplete);}
        Ok(&self.output)
    }
    pub fn into_result(self)->Result<Vec<u8>,PixelEditError> {self.result()?;Ok(self.output)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
