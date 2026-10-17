//! 🪜️ Stepwise decoder of the JPEG codec: the resumable decoder state and its scan, restart and output steps. See [`super`].

use super::*;

//#region Decode
/// 🎞️ Resumable baseline decoder: [`JpgStepDecoder::new`] parses the marker segments up to `SOS`,
/// then every [`JpgStepDecoder::step`] decodes at most `unit_budget` MCUs of the entropy-coded
/// scan (nearest-neighbor chroma upsampling for subsampled components) and, once the scan has
/// met its `EOI`, converts at most `unit_budget` output rows to RGBA — so a worker can hold one
/// fixed-cost microstep per call however large the admitted frame is. The bit reader's cursor
/// (`pos`/`acc`/`nbits`) and the DC predictors persist between steps, so a stepped decode reads
/// every source byte exactly as often as a one-shot decode and yields the identical raster.
pub struct JpgStepDecoder {
    pub(crate) observations:Option<JpgNativeObservations>,
    pub(crate) header: Option<Box<JpgHeader>>,
    pub(crate) planes: Vec<Vec<f64>>,
    pub(crate) plane_dims: Vec<(usize, usize)>,
    pub(crate) hmax: usize,
    pub(crate) vmax: usize,
    pub(crate) mcus_x: usize,
    pub(crate) mcus_y: usize,
    pub(crate) mcu: usize,
    mcus_since_restart: u32,
    pub(crate) dc_pred: Vec<i32>,
    pub(crate) pos: usize,
    pub(crate) acc: u32,
    pub(crate) nbits: u32,
    pub(crate) entropy_done: bool,
    rgba: Vec<u8>,
    pub(crate) row: usize,
}

impl JpgStepDecoder {
    /// 🧾️ Transfers exact native observations after the decoded content completed.
    pub fn take_observations(&mut self)->Option<JpgNativeObservations>{self.observations.take()}
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(data: &dyn JpgByteSource) -> Result<Self, JpgError> {
        Self::from_header(parse_jpg_header(data,false)?, true)
    }

    pub(crate) fn from_header(header: JpgHeader, initialize: bool) -> Result<Self, JpgError> {
        let frame = &header.frame;
        if frame.precision!=8||frame.width==0||frame.height==0||!(1..=4).contains(&frame.components.len()){return Err(JpgError::Unsupported("component decoding requires a nonempty 8-bit baseline frame".into()));}
        if frame.components.iter().any(|component|!(1..=4).contains(&component.h_sampling)||!(1..=4).contains(&component.v_sampling)){return Err(JpgError::Malformed("component sampling exceeds the native 1..=4 domain".into()));}
        let mut ids=std::collections::HashSet::new();
        if frame.components.iter().any(|component|!ids.insert(component.id)){return Err(JpgError::Malformed("duplicate frame component identifier".into()));}
        if initialize&&!matches!(frame.components.len(),1|3){return Err(JpgError::Unsupported("RGBA admission supports grayscale and JFIF YCbCr images".into()));}
        let hmax = frame.components.iter().map(|c| c.h_sampling).max().unwrap_or(1).max(1) as usize;
        let vmax = frame.components.iter().map(|c| c.v_sampling).max().unwrap_or(1).max(1) as usize;
        let (width, height) = (frame.width as usize, frame.height as usize);
        let mcus_x = width.div_ceil(8 * hmax);
        let mcus_y = height.div_ceil(8 * vmax);
        let mut planes: Vec<Vec<f64>> = Vec::with_capacity(frame.components.len());
        let mut plane_dims: Vec<(usize, usize)> = Vec::with_capacity(frame.components.len());
        for c in frame.components.iter() {
            let pwc = mcus_x * c.h_sampling.max(1) as usize * 8;
            let phc = mcus_y * c.v_sampling.max(1) as usize * 8;
            let mut plane=Vec::new();
            plane.try_reserve_exact(pwc*phc).map_err(|error|JpgError::Malformed(format!("JPEG plane allocation refused: {error}")))?;
            if initialize {plane.resize(pwc*phc,0.0);}
            planes.push(plane);
            plane_dims.push((pwc, phc));
        }
        let dc_pred = vec![0i32; frame.components.len()];
        let pos = header.scan_start;
        Ok(Self { observations:None,header: Some(Box::new(header)), planes, plane_dims, hmax, vmax, mcus_x, mcus_y, mcu: 0, mcus_since_restart: 0, dc_pred, pos, acc: 0, nbits: 0, entropy_done: false, rgba: Vec::new(), row: 0 })
    }

    /// ⏱️ One bounded unit of work: `Ok(None)` while decoding continues, `Ok(Some(snapshot))` once.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn step_entropy(&mut self, data: &dyn JpgByteSource, unit_budget: usize) -> Result<bool, JpgError> {
        let budget = unit_budget.max(1);
        let header = self.header.as_deref().ok_or_else(|| JpgError::Malformed("JPEG decoder stepped after completion".into()))?;
        let frame = &header.frame;
        if !self.entropy_done {
            let total = self.mcus_x * self.mcus_y;
            let end = self.mcu.saturating_add(budget).min(total);
            let restart_interval = header.restart_interval_raw;
            let mut br = BitReader { data, pos: self.pos, acc: self.acc, nbits: self.nbits };
            while self.mcu < end {
                let (my, mx) = (self.mcu / self.mcus_x, self.mcu % self.mcus_x);
                if restart_interval > 0 && self.mcus_since_restart == restart_interval as u32 && self.mcu != 0 {
                    br.skip_restart_marker()?;
                    for p in self.dc_pred.iter_mut() {
                        *p = 0;
                    }
                    self.mcus_since_restart = 0;
                }
                for &(ci,dc_id,ac_id) in &header.scan_tabs {
                    let c=&frame.components[ci];
                    let dc_tab = header.dc_tables.get(&dc_id).ok_or_else(|| JpgError::Malformed("missing DC huffman table".into()))?;
                    let ac_tab = header.ac_tables.get(&ac_id).ok_or_else(|| JpgError::Malformed("missing AC huffman table".into()))?;
                    let q = header.quant.get(&c.quant_table_id).ok_or_else(|| JpgError::Malformed("missing quant table".into()))?;
                    let (pwc, _) = self.plane_dims[ci];
                    for by in 0..c.v_sampling.max(1) as usize {
                        for bx in 0..c.h_sampling.max(1) as usize {
                            let zz = decode_block(&mut br, &mut self.dc_pred[ci], dc_tab, ac_tab)?;
                            let mut natural = [0f64; 64];
                            for z in 0..64 {
                                natural[ZIGZAG_TO_NATURAL[z]] = (zz[z] * q[z]) as f64;
                            }
                            let spatial = idct_8x8(&natural);
                            let ox = (mx * c.h_sampling.max(1) as usize + bx) * 8;
                            let oy = (my * c.v_sampling.max(1) as usize + by) * 8;
                            for r in 0..8 {
                                for cc in 0..8 {
                                    self.planes[ci][(oy + r) * pwc + (ox + cc)] = spatial[r * 8 + cc] + 128.0;
                                }
                            }
                        }
                    }
                }
                self.mcus_since_restart += 1;
                self.mcu += 1;
            }
            if self.mcu == total {
                br.expect_end_of_image()?;
                self.entropy_done = true;

            }
            (self.pos, self.acc, self.nbits) = (br.pos, br.acc, br.nbits);
            return Ok(self.entropy_done);
        }
        Ok(true)
    }

    /// 🎞️ Decodes bounded MCU or output-row units into the logical RGBA snapshot.
    pub fn step(&mut self, data: &dyn JpgByteSource, unit_budget: usize) -> Result<Option<JpgSnapshot>, JpgError> {
        let budget=unit_budget.max(1);
        if !self.entropy_done {
            self.step_entropy(data,budget)?;
            if self.entropy_done {
                let frame=&self.header.as_ref().expect("active JPEG header").frame;
                self.rgba=vec![0;frame.width as usize*frame.height as usize*4];
            }
            return Ok(None);
        }
        let header=self.header.as_deref().ok_or_else(||JpgError::Malformed("JPEG decoder stepped after completion".into()))?;
        let frame=&header.frame;
        let (width, height) = (frame.width as usize, frame.height as usize);
        let (hmax, vmax) = (self.hmax, self.vmax);
        let grayscale = frame.components.len() == 1;
        let y_idx = frame.components.iter().position(|c| c.id == 1).unwrap_or(0);
        let (cb_idx, cr_idx) = if grayscale { (None, None) } else { (frame.components.iter().position(|c| c.id == 2), frame.components.iter().position(|c| c.id == 3)) };
        let end = self.row.saturating_add(budget).min(height);
        for y in self.row..end {
            for x in 0..width {
                let yc = frame.components[y_idx];
                let (ypwc, _) = self.plane_dims[y_idx];
                let sy = (y * yc.v_sampling.max(1) as usize) / vmax;
                let sx = (x * yc.h_sampling.max(1) as usize) / hmax;
                let yy = self.planes[y_idx][sy * ypwc + sx];
                let (r, g, b) = if grayscale {
                    let v = yy.round().clamp(0.0, 255.0) as u8;
                    (v, v, v)
                } else {
                    let cb_idx = cb_idx.ok_or_else(|| JpgError::Malformed("missing Cb component".into()))?;
                    let cr_idx = cr_idx.ok_or_else(|| JpgError::Malformed("missing Cr component".into()))?;
                    let cbc = frame.components[cb_idx];
                    let crc = frame.components[cr_idx];
                    let (cbpwc, _) = self.plane_dims[cb_idx];
                    let (crpwc, _) = self.plane_dims[cr_idx];
                    let cby = (y * cbc.v_sampling.max(1) as usize) / vmax;
                    let cbx = (x * cbc.h_sampling.max(1) as usize) / hmax;
                    let cry = (y * crc.v_sampling.max(1) as usize) / vmax;
                    let crx = (x * crc.h_sampling.max(1) as usize) / hmax;
                    ycbcr_to_rgb(yy, self.planes[cb_idx][cby * cbpwc + cbx], self.planes[cr_idx][cry * crpwc + crx])
                };
                let idx = (y * width + x) * 4;
                self.rgba[idx] = r;
                self.rgba[idx + 1] = g;
                self.rgba[idx + 2] = b;
                self.rgba[idx + 3] = 255;
            }
        }
        self.row = end;
        if self.row < height {
            return Ok(None);
        }
        let header = *self.header.take().ok_or_else(|| JpgError::Malformed("JPEG decoder stepped after completion".into()))?;
        self.planes = Vec::new();
        self.observations=Some(JpgNativeObservations{frame:header.frame,sof_marker:header.sof_marker,arithmetic:header.arithmetic,arithmetic_conditioning:header.arithmetic_conditioning,scan_components:header.scan_components,scan_parameters:header.scan_parameters,quant_tables:header.quant_tables,huffman_tables:header.huffman_tables,restart_interval:header.restart_interval});
        Ok(Some(JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: width as u32,height: height as u32,pixels: std::mem::take(&mut self.rgba),jfif_version: header.jfif_version,jfif_density_units: header.jfif_density_units,jfif_x_density: header.jfif_x_density,jfif_y_density: header.jfif_y_density,jfif_thumbnail: header.jfif_thumbnail,other_segments: header.other_segments } }))
    }
}
//#endregion Decode
