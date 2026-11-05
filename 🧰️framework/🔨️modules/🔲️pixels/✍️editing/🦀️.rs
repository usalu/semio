//! ✏️ Native RGBA8 pixel editing with owned, cancellable candidates and deterministic sampling.
use crate::RasterImage;
use std::mem::size_of;

pub use semio_framework_pixels_codecs::{MAX_IMAGE_SIDE,MAX_IMAGE_PIXELS,PixelProgress,PixelEditError,validate_extent,validate_image};
pub type PixelColor = [u8; 4];

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PixelBrush {
    pub points: Vec<[f64; 2]>,
    pub size: f64,
    pub opacity: f64,
    pub hardness: f64,
    pub color: PixelColor,
    pub erase: bool,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PixelAlphaBrush {
    pub points: Vec<[f64; 2]>,
    pub size: f64,
    pub opacity: f64,
    pub hardness: f64,
    pub alpha: u8,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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

fn stroke_segment_coverage(points:&[[f64;2]],size:f64,hardness:f64,x:u32,y:u32,index:usize)->f64{
    let to=points[index];let from=points[index.saturating_sub(1)];let radius=size/2.0;let(px,py)=(f64::from(x)+0.5,f64::from(y)+0.5);
    if px<from[0].min(to[0])-radius||px>from[0].max(to[0])+radius||py<from[1].min(to[1])-radius||py>from[1].max(to[1])+radius{return 0.0;}
    let(dx,dy)=(to[0]-from[0],to[1]-from[1]);let length=dx*dx+dy*dy;let t=if length==0.0{0.0}else{((px-from[0])*dx+(py-from[1])*dy)/length}.clamp(0.0,1.0);let distance=(px-from[0]-t*dx).hypot(py-from[1]-t*dy);let core=radius*hardness;
    let amount=if distance>radius{0}else if distance<=core{255}else{byte(255.0*(radius-distance)/(radius-core))};f64::from(amount)/255.0
}

fn kernel_accumulate(alpha:&mut f64,rgb:&mut [f64;3],neighbor:PixelColor){*alpha+=f64::from(neighbor[3]);for c in 0..3{rgb[c]+=f64::from(neighbor[c])*f64::from(neighbor[3]);}}

fn kernel_color(operation:&PixelOperation,original:PixelColor,alpha:f64,rgb:[f64;3],radius:i64)->PixelColor{
    let mut result=original;for c in 0..3{let average=if alpha>0.0{rgb[c]/alpha}else{0.0};result[c]=byte(if let PixelOperation::Sharpen(amount)=*operation{f64::from(original[c])+amount*(f64::from(original[c])-average)}else{average});}
    if matches!(operation,PixelOperation::Blur(_)){result[3]=byte(alpha/((radius*2+1)*(radius*2+1))as f64);}result
}

impl PixelOperation {
    fn stroke(&self) -> Option<(&[[f64;2]], f64, f64, f64)> {
        match self {
            Self::Stroke(brush)=>Some((&brush.points,brush.size,brush.opacity,brush.hardness)),
            Self::AlphaStroke(brush)=>Some((&brush.points,brush.size,brush.opacity,brush.hardness)),
            _=>None,
        }
    }

    pub fn extent(&self, source: &RasterImage) -> (u32, u32) {
        match self {
            Self::RotateClockwise | Self::RotateCounterclockwise => (source.height, source.width),
            Self::Crop { width, height, .. } | Self::Resize { width, height, .. } => (*width, *height),
            _ => (source.width, source.height),
        }
    }

    pub fn validate(&self, source: &RasterImage, selected: bool) -> Result<(), PixelEditError> {
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
        if self.stroke().is_some() {
            return self.stroke_pixel(original, x, y, selection_coverage, stroke_segments);
        }
        self.filtered_color(image, original, x, y, selection_coverage)
    }

    /// 🖊️ One pixel under a brush or alpha brush: the strongest coverage any of `stroke_segments` (the segments whose
    /// band holds the pixel's row) lays at its centre, scaled by opacity and selection coverage. The one brush law
    /// both the whole-image job and the in-place painter use.
    fn stroke_pixel(&self,original:PixelColor,x:u32,y:u32,selection_coverage:f64,stroke_segments:&[usize])->PixelColor{
        let Some((points,size,_,hardness))=self.stroke()else{return original;};
        let coverage=stroke_segments.iter().fold(0.0_f64,|coverage,&index|coverage.max(stroke_segment_coverage(points,size,hardness,x,y,index)));
        self.stroke_color(original,coverage,selection_coverage)
    }

    fn stroke_color(&self,original:PixelColor,coverage:f64,selection_coverage:f64)->PixelColor{
        let Some((_,_,opacity,_))=self.stroke()else{return original;};let opacity=coverage*opacity*selection_coverage;
        if opacity==0.0{return original;}
        if let Self::AlphaStroke(brush)=self{return [original[0],original[1],original[2],byte(f64::from(original[3])+(f64::from(brush.alpha)-f64::from(original[3]))*opacity)];}
        let Self::Stroke(brush)=self else{unreachable!()};
        if brush.erase{let alpha=byte(f64::from(original[3])*(1.0-opacity));return if alpha>0{[original[0],original[1],original[2],alpha]}else{[0;4]};}
        source_over(original,brush.color,opacity)
    }

    /// 🎨️ One pixel under every operation that is not a brush: fills, geometry, resampling, kernels and tone maps.
    fn filtered_color(&self, image: &RasterImage, original: PixelColor, x: u32, y: u32, selection_coverage: f64) -> PixelColor {
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
                        kernel_accumulate(&mut alpha,&mut rgb,neighbor);
                    }
                }
                return kernel_color(self,original,alpha,rgb,radius);
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

/// 🔲️ A pixel-aligned rectangle of an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// 📏️ Every pixel a brush of `size` along `points` can reach on a `width × height` image: the union of each segment's
/// box widened by the radius and clipped to the image — the walk the TypeScript twin's `paintStroke` takes. `None`
/// when the stroke lies wholly outside the image.
pub fn stroke_bounds(points: &[[f64; 2]], size: f64, width: u32, height: u32) -> Option<PixelRect> {
    if width == 0 || height == 0 {
        return None;
    }
    let radius = size / 2.0;
    let (last_x, last_y) = (f64::from(width - 1), f64::from(height - 1));
    let mut bounds: Option<(f64, f64, f64, f64)> = None;
    for (index, to) in points.iter().enumerate() {
        let from = points[index.saturating_sub(1)];
        let left = (from[0].min(to[0]) - radius).floor().max(0.0);
        let right = (from[0].max(to[0]) + radius).ceil().min(last_x);
        let top = (from[1].min(to[1]) - radius).floor().max(0.0);
        let bottom = (from[1].max(to[1]) + radius).ceil().min(last_y);
        if left > right || top > bottom {
            continue;
        }
        bounds = Some(match bounds {
            None => (left, top, right, bottom),
            Some((l, t, r, b)) => (l.min(left), t.min(top), r.max(right), b.max(bottom)),
        });
    }
    bounds.map(|(left, top, right, bottom)| PixelRect { x: left as u32, y: top as u32, width: (right - left) as u32 + 1, height: (bottom - top) as u32 + 1 })
}

/// 🖌️ Paints a `Stroke` or `AlphaStroke` into `image` in place over [`stroke_bounds`] only, clipped by an optional
/// per-pixel `selection` coverage: byte-identical to a whole-image [`PixelEditJob`] of the same operation and selection,
/// at a cost bounded by the stroke instead of the image — the deterministic rasterizer a replayed stroke leaf runs.
/// Answers the rectangle walked (`None`: the stroke reaches no pixel).
pub fn paint_stroke_in_place(image: &mut RasterImage, operation: &PixelOperation, selection: Option<&[u8]>) -> Result<Option<PixelRect>, PixelEditError> {
    validate_image(image)?;
    let Some((points, size, ..)) = operation.stroke() else { return Err(PixelEditError::Invalid("Only a brush stroke paints in place")) };
    if selection.is_some_and(|mask| mask.len() != image.pixels.len() / 4) {
        return Err(PixelEditError::Invalid("Selection extent does not match image"));
    }
    operation.validate(image, false)?;
    let Some(rect) = stroke_bounds(points, size, image.width, image.height) else { return Ok(None) };
    let mut segments = Vec::with_capacity(points.len());
    for y in rect.y..rect.y + rect.height {
        segments.clear();
        let py = f64::from(y) + 0.5;
        for (index, to) in points.iter().enumerate() {
            let from = points[index.saturating_sub(1)];
            if py >= from[1].min(to[1]) - size / 2.0 && py <= from[1].max(to[1]) + size / 2.0 {
                segments.push(index);
            }
        }
        for x in rect.x..rect.x + rect.width {
            let index = y as usize * image.width as usize + x as usize;
            let coverage = f64::from(selection.map_or(255, |mask| mask[index])) / 255.0;
            if coverage == 0.0 {
                continue;
            }
            let offset = index * 4;
            let before = [image.pixels[offset], image.pixels[offset + 1], image.pixels[offset + 2], image.pixels[offset + 3]];
            let after = operation.stroke_pixel(before, x, y, coverage, &segments);
            image.pixels[offset..offset + 4].copy_from_slice(&after);
        }
    }
    Ok(Some(rect))
}

/// 🪣️ The 4-connected region of `image` around the seed `(x, y)` whose RGBA channels all lie within `tolerance` of the
/// seed's, as per-pixel coverage: 255, or the optional `selection`'s coverage — an unselected pixel is never taken and
/// blocks the region behind it. The region a bucket fills and a magic wand selects, byte-equal to the TypeScript twin
/// `floodSelection` (shared corpus section `floodSelections`).
#[derive(semio_framework_value::RetireOwned)]
pub struct PixelFloodSelectionJob {
    source:Option<std::sync::Arc<RasterImage>>,
    width:usize,
    count:usize,
    seed:usize,
    target:[u8;4],
    tolerance:u8,
    mask:Vec<u8>,
    seen:Vec<bool>,
    queue:Vec<u32>,
    head:usize,
    done:bool,
    cancelled:bool,
    write_bytes:usize,
    work_units:usize,
}

/// 🌊️ Every allocation retained by an unfinished flood, transferred without discarding its source.
pub struct PixelFloodStorage {
    pub source: Option<std::sync::Arc<RasterImage>>,
    pub mask: Vec<u8>,
    pub seen: Vec<bool>,
    pub queue: Vec<u32>,
}

impl PixelFloodSelectionJob {
    pub fn last_write_bytes(&self) -> usize { self.write_bytes }
    pub fn last_work_units(&self)->usize{self.work_units}
    pub fn storage_capacity_bytes(&self) -> usize { self.mask.capacity() + self.seen.capacity() + self.queue.capacity() * size_of::<u32>() }
    pub fn initialized_bytes(&self) -> usize { self.mask.len() + self.seen.len() + self.queue.len() * size_of::<u32>() }
    pub fn into_storage(self) -> PixelFloodStorage { PixelFloodStorage { source: self.source, mask: self.mask, seen: self.seen, queue: self.queue } }
    pub fn queue_reservation_bytes(&self) -> usize { self.count * size_of::<u32>() }
    pub fn reserve_queue(&mut self) -> Result<usize, PixelEditError> {
        if self.head != 0 || !self.queue.is_empty() { return Err(PixelEditError::Invalid("Flood queue reservation must precede computation")); }
        self.queue.try_reserve_exact(self.count).map_err(|_| PixelEditError::Invalid("Flood queue reservation failed"))?;
        Ok(self.queue.capacity() * size_of::<u32>())
    }
    pub fn new(source:std::sync::Arc<RasterImage>,x:u32,y:u32,tolerance:u8)->Result<Self,PixelEditError>{
        validate_image(&source)?;if x>=source.width||y>=source.height{return Err(PixelEditError::Invalid("Invalid flood selection seed or tolerance"));}
        let seed=(y*source.width+x) as usize;let target=source.pixels[seed*4..seed*4+4].try_into().map_err(|_|PixelEditError::Invalid("Invalid flood source"))?;
        let mut job=Self::uniform(source.width,source.height,[0;4],x,y,tolerance)?;job.target=target;job.source=Some(source);Ok(job)
    }
    pub fn uniform(width:u32,height:u32,color:[u8;4],x:u32,y:u32,tolerance:u8)->Result<Self,PixelEditError>{
        let count=validate_extent(width,height)?;if x>=width||y>=height{return Err(PixelEditError::Invalid("Invalid flood selection seed or tolerance"));}
        Ok(Self {source:None,width:width as usize,count,seed:(y*width+x) as usize,target:color,tolerance,mask:Vec::with_capacity(count),seen:Vec::with_capacity(count),queue:Vec::new(),head:0,done:false,cancelled:false,write_bytes:0,work_units:0})
    }
    pub fn advance(&mut self,budget:usize)->Result<PixelProgress,PixelEditError>{self.advance_selected(budget,None)}
    pub fn advance_selected(&mut self,budget:usize,selection:Option<&[u8]>)->Result<PixelProgress,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}if budget==0||budget>32768{return Err(PixelEditError::Invalid("Invalid flood selection grant"));}
        if selection.is_some_and(|mask| mask.len()!=self.count){return Err(PixelEditError::Invalid("Selection extent does not match flood source"));}
        self.write_bytes=0;self.work_units=0;
        if self.mask.len()<self.count{
            let end=self.count.min(self.mask.len()+budget);self.work_units=end-self.mask.len();self.write_bytes=self.work_units*2;self.mask.resize(end,0);self.seen.resize(end,false);
            if end==self.count{self.queue.push(self.seed as u32);self.seen[self.seed]=true;self.write_bytes+=5;}
            return Ok(PixelProgress {completed:end,total:self.count*2,done:false});
        }
        for _ in 0..budget{
            let Some(&index)=self.queue.get(self.head)else{self.done=true;break;};self.head+=1;self.work_units+=1;let index=index as usize;
            let coverage=selection.map_or(255,|mask|mask[index]);if coverage==0{continue;}
            if self.source.as_ref().is_some_and(|source|source.pixels[index*4..index*4+4].iter().zip(self.target).any(|(a,b)|a.abs_diff(b)>self.tolerance)){continue;}
            self.mask[index]=coverage;self.write_bytes+=1;let x=index%self.width;
            for (next,inside) in [(index.wrapping_sub(1),x>0),(index+1,x+1<self.width),(index.wrapping_sub(self.width),index>=self.width),(index+self.width,index+self.width<self.count)]{
                if inside&&!self.seen[next]{self.seen[next]=true;self.queue.push(next as u32);self.write_bytes+=5;}
            }
        }
        if self.head==self.queue.len(){self.done=true;}
        Ok(PixelProgress {completed:if self.done{self.count*2}else{self.count+self.head},total:self.count*2,done:self.done})
    }
    pub fn result(&self)->Result<&[u8],PixelEditError>{if self.cancelled{return Err(PixelEditError::Cancelled);}if !self.done{return Err(PixelEditError::Incomplete);}Ok(&self.mask)}
    pub fn cancel(&mut self){self.cancelled=true;self.mask=Vec::new();self.seen=Vec::new();self.queue=Vec::new();}
}

pub fn flood_selection(image: &RasterImage, x: u32, y: u32, tolerance: u8, selection: Option<&[u8]>) -> Result<Vec<u8>, PixelEditError> {
    validate_image(image)?;
    let count = image.pixels.len() / 4;
    if selection.is_some_and(|mask| mask.len() != count) {
        return Err(PixelEditError::Invalid("Selection extent does not match image"));
    }
    if x >= image.width || y >= image.height {
        return Err(PixelEditError::Invalid("Invalid flood selection seed or tolerance"));
    }
    let (width, height) = (image.width as usize, image.height as usize);
    let seed = y as usize * width + x as usize;
    let target: PixelColor = [image.pixels[seed * 4], image.pixels[seed * 4 + 1], image.pixels[seed * 4 + 2], image.pixels[seed * 4 + 3]];
    let mut mask = vec![0_u8; count];
    let mut seen = vec![false; count];
    let mut queue = vec![seed as u32];
    seen[seed] = true;
    let mut head = 0;
    while let Some(&next) = queue.get(head) {
        head += 1;
        let index = next as usize;
        let coverage = selection.map_or(255, |selection| selection[index]);
        if coverage == 0 || image.pixels[index * 4..index * 4 + 4].iter().zip(target).any(|(channel, seed)| channel.abs_diff(seed) > tolerance) {
            continue;
        }
        mask[index] = coverage;
        let (px, py) = (index % width, index / width);
        for (neighbour, inside) in [(index.wrapping_sub(1), px > 0), (index + 1, px + 1 < width), (index.wrapping_sub(width), py > 0), (index + width, py + 1 < height)] {
            if inside && !seen[neighbour] {
                seen[neighbour] = true;
                queue.push(neighbour as u32);
            }
        }
    }
    Ok(mask)
}

/// 🖼️ Applies a `Fill` or `AlphaFill` to `image` in place under the per-pixel `coverage` of a region (a
/// [`flood_selection`]): byte-identical to a whole-image [`PixelEditJob`] of the same operation selected by `coverage`,
/// touching only the covered pixels — the deterministic filler a replayed region-fill leaf runs. Answers whether a pixel
/// changed.
pub fn fill_in_place(image: &mut RasterImage, operation: &PixelOperation, coverage: &[u8]) -> Result<bool, PixelEditError> {
    validate_image(image)?;
    if !matches!(operation, PixelOperation::Fill(_) | PixelOperation::AlphaFill { .. }) {
        return Err(PixelEditError::Invalid("Only a fill fills a region in place"));
    }
    if coverage.len() != image.pixels.len() / 4 {
        return Err(PixelEditError::Invalid("Selection extent does not match image"));
    }
    operation.validate(image, true)?;
    let width = image.width as usize;
    let mut changed = false;
    for (index, amount) in coverage.iter().enumerate().filter(|(_, amount)| **amount > 0) {
        let offset = index * 4;
        let before = [image.pixels[offset], image.pixels[offset + 1], image.pixels[offset + 2], image.pixels[offset + 3]];
        let after = operation.filtered_color(image, before, (index % width) as u32, (index / width) as u32, f64::from(*amount) / 255.0);
        if after != before {
            image.pixels[offset..offset + 4].copy_from_slice(&after);
            changed = true;
        }
    }
    Ok(changed)
}

/// 🧵️ An unpublished result whose work is bounded by an explicit pixel grant.
#[derive(semio_framework_value::RetireOwned)]
pub struct PixelEditJob {
    #[doc(hidden)] pub source: RasterImage,
    #[doc(hidden)] pub output: RasterImage,
    operation: PixelOperation,
    selection: Option<Vec<u8>>,
    cursor: usize,
    cancelled: bool,
    stroke_segments: Vec<usize>,
    stroke_row: Option<usize>,
    stroke_scan: usize,
    pending_pixel:Option<usize>,
    pixel_scan:usize,
    pixel_coverage:f64,
    kernel_alpha:f64,
    kernel_rgb:[f64;3],
    write_bytes: usize,
    work_units:usize,
}

/// 📦️ Exact owned buffers returned to a retained caller for result handoff or granted retirement.
pub struct PixelEditStorage {
    pub source: RasterImage,
    pub output: RasterImage,
    pub selection: Option<Vec<u8>>,
    pub points: Vec<[f64; 2]>,
    pub stroke_segments: Vec<usize>,
}

impl PixelEditJob {
    pub fn last_write_bytes(&self) -> usize { self.write_bytes }
    pub fn last_work_units(&self)->usize{self.work_units}
    pub fn admit(source: &mut Option<RasterImage>, operation: &mut Option<PixelOperation>, selection: &mut Option<Vec<u8>>) -> Result<Self,PixelEditError> {
        let image = source.as_ref().ok_or(PixelEditError::Invalid("Pixel source owner is absent"))?;
        let editing = operation.as_ref().ok_or(PixelEditError::Invalid("Pixel operation owner is absent"))?;
        validate_image(image)?;
        if selection.as_ref().is_some_and(|mask|mask.len()!=image.pixels.len()/4) { return Err(PixelEditError::Invalid("Selection extent does not match image")); }
        editing.validate(image,selection.is_some())?;
        let (width,height) = editing.extent(image);
        let mut pixels = Vec::new();
        pixels.try_reserve_exact(width as usize*height as usize*4).map_err(|_|PixelEditError::Invalid("Pixel result reservation failed"))?;
        Ok(Self { source:source.take().expect("pixel source was validated"),output:RasterImage {width,height,pixels},operation:operation.take().expect("pixel operation was validated"),selection:selection.take(),cursor:0,cancelled:false,stroke_segments:Vec::new(),stroke_row:None,stroke_scan:0,pending_pixel:None,pixel_scan:0,pixel_coverage:0.0,kernel_alpha:0.0,kernel_rgb:[0.0;3],write_bytes:0,work_units:0 })
    }

    pub fn storage_capacity_bytes(&self) -> usize { self.source.pixels.capacity() + self.output.pixels.capacity() + self.selection.as_ref().map_or(0,Vec::capacity) + self.stroke_segments.capacity() * size_of::<usize>() + match &self.operation { PixelOperation::Stroke(brush)=>brush.points.capacity()*size_of::<[f64;2]>(),PixelOperation::AlphaStroke(brush)=>brush.points.capacity()*size_of::<[f64;2]>(),_=>0 } }
    pub fn scratch_reservation_bytes(&self) -> usize { self.operation.stroke().map_or(0,|(points,..)|points.len() * size_of::<usize>()) }
    pub fn reserve_scratch(&mut self) -> Result<usize,PixelEditError> {
        let count = self.operation.stroke().map_or(0,|(points,..)|points.len());
        self.stroke_segments.try_reserve_exact(count).map_err(|_|PixelEditError::Invalid("Stroke scratch reservation failed"))?;
        Ok(self.stroke_segments.capacity() * size_of::<usize>())
    }
    pub fn into_storage(self) -> PixelEditStorage {
        let points = match self.operation { PixelOperation::Stroke(brush) => brush.points, PixelOperation::AlphaStroke(brush) => brush.points, _ => Vec::new() };
        PixelEditStorage { source: self.source, output: self.output, selection: self.selection, points, stroke_segments: self.stroke_segments }
    }

    pub fn new(source: RasterImage, operation: PixelOperation, mut selection: Option<Vec<u8>>) -> Result<Self, PixelEditError> {
        Self::admit(&mut Some(source),&mut Some(operation),&mut selection)
    }

    pub fn recommended_grant(&self) -> usize {
        match self.operation { PixelOperation::Blur(_) => 128, PixelOperation::Stroke(_) | PixelOperation::AlphaStroke(_) => 256, _ => 4096 }
    }

    pub fn advance(&mut self, pixel_budget: usize) -> Result<PixelProgress, PixelEditError> {
        self.advance_inner(pixel_budget,usize::MAX,usize::MAX,||false)
    }

    /// 🧵️ Retains unfinished row indexing when either scratch writes or its point scan reaches the supplied grant.
    pub fn advance_bounded(&mut self,pixel_budget:usize,write_budget:usize)->Result<PixelProgress,PixelEditError>{
        self.advance_controlled(pixel_budget,write_budget,1024,||false)
    }

    /// ⏱️ One row test, segment distance, kernel sample or output pixel consumes one primitive and polls the original stop probe.
    pub fn advance_controlled(&mut self,pixel_budget:usize,write_budget:usize,work_budget:usize,should_stop:impl FnMut()->bool)->Result<PixelProgress,PixelEditError>{
        if write_budget<4{return Err(PixelEditError::Invalid("Pixel write budget must hold one RGBA pixel"));}
        if !(1..=65536).contains(&work_budget){return Err(PixelEditError::Invalid("Pixel work budget must be 1–65536"));}
        self.advance_inner(pixel_budget,write_budget,work_budget,should_stop)
    }

    fn advance_inner(&mut self,pixel_budget:usize,write_budget:usize,work_budget:usize,mut should_stop:impl FnMut()->bool)->Result<PixelProgress,PixelEditError>{
        if self.cancelled {
            return Err(PixelEditError::Cancelled);
        }
        if !(1..=65536).contains(&pixel_budget) {
            return Err(PixelEditError::Invalid("Pixel budget must be 1–65536"));
        }
        let total = self.output.width as usize * self.output.height as usize;
        let end = total.min(self.cursor + pixel_budget);
        self.write_bytes = 0;
        self.work_units=0;
        while self.cursor < end {
            if self.work_units==work_budget||should_stop(){break;}
            let x = self.cursor % self.output.width as usize;
            let y = self.cursor / self.output.width as usize;
            if x == 0 {
                if let Some((points,size,..)) = self.operation.stroke() {
                    if self.stroke_row!=Some(y){self.stroke_segments.clear();self.stroke_scan=0;self.stroke_row=Some(y);}
                    let py = y as f64 + 0.5;
                    let end=points.len();
                    while self.stroke_scan<end {
                        if self.work_units==work_budget||should_stop(){break;}
                        let index=self.stroke_scan;let to=&points[index];
                        let from = points[index.saturating_sub(1)];
                        if py >= from[1].min(to[1]) - size / 2.0 && py <= from[1].max(to[1]) + size / 2.0 {
                            if write_budget-self.write_bytes<size_of::<usize>(){break;}
                            self.stroke_segments.push(index);
                            self.write_bytes += size_of::<usize>();
                        }
                        self.stroke_scan+=1;
                        self.work_units+=1;
                    }
                    if self.stroke_scan<points.len(){break;}
                }
            }
            if write_budget-self.write_bytes<4{break;}
            let coverage = f64::from(self.selection.as_ref().map_or(255, |mask| mask[self.cursor])) / 255.0;
            let before = sample(&self.source, x as i64, y as i64);
            let after=if coverage==0.0{before}else if let Some((points,size,_,hardness))=self.operation.stroke(){
                if self.pending_pixel!=Some(self.cursor){self.pending_pixel=Some(self.cursor);self.pixel_scan=0;self.pixel_coverage=0.0;}
                while self.pixel_scan<self.stroke_segments.len(){if self.work_units==work_budget||should_stop(){break;}let index=self.stroke_segments[self.pixel_scan];self.pixel_coverage=self.pixel_coverage.max(stroke_segment_coverage(points,size,hardness,x as u32,y as u32,index));self.pixel_scan+=1;self.work_units+=1;}
                if self.pixel_scan<self.stroke_segments.len(){break;}
                self.operation.stroke_color(before,self.pixel_coverage,coverage)
            }else if matches!(self.operation,PixelOperation::Blur(radius)if radius>0)||matches!(self.operation,PixelOperation::Sharpen(amount)if amount>0.0){
                if self.pending_pixel!=Some(self.cursor){self.pending_pixel=Some(self.cursor);self.pixel_scan=0;self.kernel_alpha=0.0;self.kernel_rgb=[0.0;3];}
                let radius=if let PixelOperation::Blur(radius)=self.operation{i64::from(radius)}else{1};let side=(radius*2+1)as usize;let count=side*side;
                while self.pixel_scan<count{if self.work_units==work_budget||should_stop(){break;}let(dx,dy)=((self.pixel_scan%side)as i64-radius,(self.pixel_scan/side)as i64-radius);let neighbor=sample(&self.source,x as i64+dx,y as i64+dy);kernel_accumulate(&mut self.kernel_alpha,&mut self.kernel_rgb,neighbor);self.pixel_scan+=1;self.work_units+=1;}
                if self.pixel_scan<count{break;}kernel_color(&self.operation,before,self.kernel_alpha,self.kernel_rgb,radius)
            }else{self.operation.filtered(&self.source,x as u32,y as u32,coverage,&self.stroke_segments)};
            if self.work_units==work_budget||should_stop(){break;}
            let mix = if matches!(self.operation, PixelOperation::Fill(_) | PixelOperation::Stroke(_) | PixelOperation::AlphaStroke(_) | PixelOperation::AlphaFill {..}) { 1.0 } else { coverage };
            let mut target = [0;4];
            for c in 0..4 {
                target[c] = byte(f64::from(before[c]) + (f64::from(after[c]) - f64::from(before[c])) * mix);
            }
            if self.operation == PixelOperation::Clear && coverage > 0.0 && coverage < 1.0 {
                for c in 0..3 {
                    target[c] = if target[3] > 0 { before[c] } else { 0 };
                }
            }
            self.output.pixels.extend_from_slice(&target);
            self.write_bytes += 4;
            self.work_units+=1;
            self.cursor += 1;
            self.pending_pixel=None;
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
#[derive(semio_framework_value::RetireOwned)]
pub struct PixelSelectionJob {
    #[doc(hidden)] pub width:u32,
    #[doc(hidden)] pub height:u32,
    shape:SelectionShape,
    #[doc(hidden)] pub mask:Vec<u8>,
    row:u32,
    intersections:Vec<f64>,
    cancelled:bool,
    phase:u8,
    cursor:usize,
    sort_index:usize,
    sort_cursor:usize,
    heap_end:usize,
    heap_active:bool,
    pair:usize,
    fill_end:usize,
    write_bytes:usize,
    work_units:usize,
}

/// 🎯️ Shape coverage and scanline scratch transferred to a retained retirement owner.
pub struct PixelSelectionStorage {
    pub mask: Vec<u8>,
    pub intersections: Vec<f64>,
    pub points: Vec<[f64; 2]>,
}

impl PixelSelectionJob {
    pub fn storage_capacity_bytes(&self) -> usize { self.mask.capacity() + self.intersections.capacity() * size_of::<f64>() + match &self.shape { SelectionShape::Polygon(points) => points.capacity() * size_of::<[f64;2]>(), _ => 0 } }
    pub fn initialized_bytes(&self) -> usize { self.mask.len() + self.intersections.len() * size_of::<f64>() + match &self.shape { SelectionShape::Polygon(points) => points.len() * size_of::<[f64;2]>(), _ => 0 } }
    pub fn into_storage(self) -> PixelSelectionStorage {
        let points = match self.shape { SelectionShape::Polygon(points) => points, _ => Vec::new() };
        PixelSelectionStorage { mask: self.mask, intersections: self.intersections, points }
    }
    pub fn new(width:u32,height:u32,shape:SelectionShape)->Result<Self,PixelEditError>{
        validate_extent(width,height)?;
        if matches!(&shape,SelectionShape::Polygon(points)if !points.iter().flatten().all(|value|value.is_finite())){return Err(PixelEditError::Invalid("Invalid selection polygon"));}
        let mut job=Self::begin(width,height,shape)?;if job.phase==6{job.phase=0;}Ok(job)
    }
    /// 🛬️ Polygon coordinates remain owned until finite validation advances under the caller's work and stop grants.
    pub fn begin(width:u32,height:u32,shape:SelectionShape)->Result<Self,PixelEditError>{
        let count=validate_extent(width,height)?;
        match &shape{
            SelectionShape::Box{x,y,width,height,..} if ![x,y,width,height].iter().all(|v|v.is_finite())=>return Err(PixelEditError::Invalid("Invalid selection box")),
            SelectionShape::Polygon(points) if !(3..=4096).contains(&points.len())=>return Err(PixelEditError::Invalid("Invalid selection polygon")),
            _=>{}
        }
        let edges=match &shape{SelectionShape::Polygon(points)=>points.len(),_=>0};
        Ok(Self{width,height,shape,mask:Vec::with_capacity(count),row:0,intersections:Vec::with_capacity(edges),cancelled:false,phase:if edges==0{0}else{6},cursor:0,sort_index:0,sort_cursor:0,heap_end:0,heap_active:false,pair:0,fill_end:0,write_bytes:0,work_units:0})
    }

    pub fn last_write_bytes(&self)->usize{self.write_bytes}
    pub fn last_work_units(&self)->usize{self.work_units}

    /// ⏳️ Coordinate validation, initialized samples, polygon edges, sorting and fill samples consume bounded work units.
    pub fn advance_budget(&mut self,maximum_work:usize,maximum_writes:usize)->Result<PixelProgress,PixelEditError>{
        self.advance_budget_with_control(maximum_work,maximum_writes,||true)
    }

    pub fn advance_budget_with_control(&mut self,maximum_work:usize,maximum_writes:usize,mut continue_work:impl FnMut()->bool)->Result<PixelProgress,PixelEditError>{
        self.advance_range(maximum_work,maximum_writes,self.height,&mut continue_work)
    }

    fn advance_range(&mut self,maximum_work:usize,maximum_writes:usize,stop_row:u32,continue_work:&mut dyn FnMut()->bool)->Result<PixelProgress,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        self.write_bytes=0;self.work_units=0;
        while self.row<stop_row&&self.work_units<maximum_work&&continue_work(){
            let y=f64::from(self.row)+0.5;
            match self.phase{
                6=>{let SelectionShape::Polygon(points)=&self.shape else{return Err(PixelEditError::Invalid("Invalid selection validation phase"));};if self.cursor==points.len(){self.cursor=0;self.phase=0;}else{let point=points[self.cursor];self.cursor+=1;if !point.iter().all(|value|value.is_finite()){self.work_units+=1;return Err(PixelEditError::Invalid("Invalid selection polygon"));}}},
                0=>{
                    if self.cursor==self.width as usize{self.cursor=0;self.intersections.clear();self.phase=1;}
                    else{if self.write_bytes==maximum_writes{break;}self.mask.push(0);self.cursor+=1;self.write_bytes+=1;}
                }
                1=>match &self.shape{
                    SelectionShape::Polygon(points)=>{
                        if self.cursor==points.len(){self.heap_end=self.intersections.len();self.sort_index=self.heap_end/2;self.heap_active=false;self.phase=2;}
                        else{let a=points[self.cursor];let b=points[if self.cursor==0{points.len()-1}else{self.cursor-1}];if (a[1]>y)!=(b[1]>y){if maximum_writes.saturating_sub(self.write_bytes)<size_of::<f64>(){break;}self.intersections.push((b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]);self.write_bytes+=size_of::<f64>();}self.cursor+=1;}
                    }
                    SelectionShape::Box{ellipse,x,y:top,width,height}=>{
                        let(left,top,w,h)=(x.min(x+width),top.min(top+height),width.abs(),height.abs());
                        let range=if w>0.0&&h>0.0&&y>=top&&y<=top+h{if *ellipse{let radius=w/2.0*(1.0-((y-top-h/2.0)/(h/2.0)).powi(2)).max(0.0).sqrt();Some((left+w/2.0-radius,left+w/2.0+radius,true))}else if y<top+h{Some((left,left+w,false))}else{None}}else{None};
                        if let Some((left,right,inclusive))=range{self.set_fill_range(left,right,inclusive);self.phase=4;}else{self.phase=5;}
                    }
                },
                2=>{
                    if self.heap_active{
                        let child=self.sort_cursor*2+1;
                        if child>=self.heap_end{self.heap_active=false;}
                        else{let larger=if child+1<self.heap_end&&self.intersections[child].total_cmp(&self.intersections[child+1]).is_lt(){child+1}else{child};if self.intersections[self.sort_cursor].total_cmp(&self.intersections[larger]).is_lt(){if maximum_writes.saturating_sub(self.write_bytes)<2*size_of::<f64>(){break;}self.intersections.swap(self.sort_cursor,larger);self.sort_cursor=larger;self.write_bytes+=2*size_of::<f64>();}else{self.heap_active=false;}}
                    }else if self.sort_index>0{self.sort_index-=1;self.sort_cursor=self.sort_index;self.heap_active=true;}
                    else if self.heap_end>1{if maximum_writes.saturating_sub(self.write_bytes)<2*size_of::<f64>(){break;}self.heap_end-=1;self.intersections.swap(0,self.heap_end);self.write_bytes+=2*size_of::<f64>();self.sort_cursor=0;self.heap_active=true;}
                    else{self.pair=0;self.phase=3;}
                }
                3=>{
                    if self.pair+1>=self.intersections.len(){self.phase=5;}
                    else{let(left,right)=(self.intersections[self.pair],self.intersections[self.pair+1]);self.set_fill_range(left,right,false);self.pair+=2;self.phase=4;}
                }
                4=>{
                    if self.cursor==self.fill_end{self.phase=if matches!(&self.shape,SelectionShape::Polygon(_)){3}else{5};}
                    else{if self.write_bytes==maximum_writes{break;}self.mask[self.row as usize*self.width as usize+self.cursor]=255;self.cursor+=1;self.write_bytes+=1;}
                }
                _=>{self.row+=1;self.cursor=0;self.phase=0;}
            }
            self.work_units+=1;
        }
        Ok(PixelProgress{completed:self.row as usize,total:self.height as usize,done:self.row==self.height})
    }

    fn set_fill_range(&mut self,left:f64,right:f64,inclusive:bool){
        if left.is_nan()||right.is_nan(){self.cursor=0;self.fill_end=0;return;}
        let width=f64::from(self.width);self.cursor=(left-0.5).ceil().clamp(0.0,width)as usize;self.fill_end=(if inclusive{(right-0.5).floor()+1.0}else{(right-0.5).ceil()}).clamp(self.cursor as f64,width)as usize;
    }

    pub fn advance(&mut self,rows:usize)->Result<PixelProgress,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if !(1..=64).contains(&rows){return Err(PixelEditError::Invalid("Selection grant must be 1–64 rows"));}
        let end=self.height.min(self.row+rows as u32);
        self.advance_range(usize::MAX,usize::MAX,end,&mut||true)
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
    #[doc(hidden)] pub output:Vec<u8>,
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


#[path="🪣️flood/🦀️.rs"]
pub mod flood;
pub use flood::{FloodRegionCursor,FloodSource,FloodProgress,FloodMask,FLOOD_PAGE_PIXELS};
