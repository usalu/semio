//! ✍️ Encoder of the JPEG codec: the box downsampler, the JFIF APP0 writer, the frame components and the baseline encoder with its Huffman table writer. See [`super`].

use super::*;

//#region Encode
/// 🧩 Downsamples a full-res plane by box-averaging `fx`×`fy` pixel blocks —
/// used to build the (subsampled) chroma planes at 4:2:0/4:2:2 from 4:4:4 source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_downsample(src: &[f64], sw: usize, sh: usize, fx: usize, fy: usize) -> (Vec<f64>, usize, usize) {
    let dw = sw / fx;
    let dh = sh / fy;
    let mut out = vec![0f64; dw * dh];
    for y in 0..dh {
        for x in 0..dw {
            let mut sum = 0f64;
            for dy in 0..fy {
                for dx in 0..fx {
                    sum += src[(y * fy + dy) * sw + (x * fx + dx)];
                }
            }
            out[y * dw + x] = sum / (fx * fy) as f64;
        }
    }
    (out, dw, dh)
}

/// 🏷️ Builds a real `APP0`/`JFIF\0` segment (ITU-T T.871 §) from `snap.jfif_*`, including an
/// embedded thumbnail when present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_jfif_app0(snap: &JpgSnapshot) -> Vec<u8> {
    let thumb = snap.image.jfif_thumbnail.as_ref();
    let (tw, th, tdata): (u8, u8, &[u8]) = match thumb {
        Some(t) => (t.width, t.height, &t.rgb_data),
        None => (0, 0, &[]),
    };
    let len = 2 + 5 + 2 + 1 + 2 + 2 + 1 + 1 + tdata.len();
    let mut out = vec![0xFFu8, 0xE0, (len >> 8) as u8, (len & 0xFF) as u8];
    out.extend_from_slice(b"JFIF\0");
    out.push(snap.image.jfif_version.0);
    out.push(snap.image.jfif_version.1);
    out.push(snap.image.jfif_density_units.to_u8());
    out.push((snap.image.jfif_x_density >> 8) as u8);
    out.push((snap.image.jfif_x_density & 0xFF) as u8);
    out.push((snap.image.jfif_y_density >> 8) as u8);
    out.push((snap.image.jfif_y_density & 0xFF) as u8);
    out.push(tw);
    out.push(th);
    out.extend_from_slice(tdata);
    out
}

/// 📐️ Resolves explicit native export component identities and sampling factors.
/// Empty options select the encoder's 4:2:0 default. Table selectors are allocated for the
/// encoder's luma/chroma tables; native sampling must divide the maximum component extent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frame_components_of(options: &JpgEncodeOptions) -> Result<Vec<JpgFrameComponent>, JpgError> {
    let declared = options.components.iter().map(|component|JpgFrameComponent{id:component.id,h_sampling:component.h_sampling,v_sampling:component.v_sampling,quant_table_id:0}).collect::<Vec<_>>();
    if declared.is_empty() {
        return Ok(vec![
            JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
            JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
            JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
        ]);
    }
    if declared.len() != 1 && declared.len() != 3 {
        return Err(JpgError::Unsupported(format!("this encoder writes a single-component grayscale frame or a three-component Y/Cb/Cr frame; the export request declares {}", declared.len())));
    }
    for component in &declared {
        if !(1..=4).contains(&component.h_sampling) || !(1..=4).contains(&component.v_sampling) {
            return Err(JpgError::Unsupported(format!("T.81 B.2.2 confines a component's sampling factors to 1..=4; component {} declares {}x{}", component.id, component.h_sampling, component.v_sampling)));
        }
    }
    let hmax = declared.iter().map(|component| component.h_sampling).max().unwrap_or(1);
    let vmax = declared.iter().map(|component| component.v_sampling).max().unwrap_or(1);
    for component in &declared {
        if hmax % component.h_sampling != 0 || vmax % component.v_sampling != 0 {
            return Err(JpgError::Unsupported(format!("component {}'s {}x{} sampling does not divide the frame maximum {hmax}x{vmax}, so its plane has no integral resolution", component.id, component.h_sampling, component.v_sampling)));
        }
    }
    Ok(declared.into_iter().enumerate().map(|(index, component)| JpgFrameComponent { quant_table_id: if index == 0 { 0 } else { 1 }, ..component }).collect())
}

/// 🖨️ Encodes owned RGBA pixels and JFIF/APP/COM metadata as baseline JPEG.
/// Explicit IO options select physical quality and component sampling. Fresh DQT/DHT and SOF0
/// records are derived from those options; entropy observations never enter authored content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_jpg(snap: &JpgSnapshot, options:&JpgEncodeOptions) -> Result<Vec<u8>, JpgError> {
    if snap.image.width == 0 || snap.image.height == 0 {
        return Err(JpgError::Malformed("empty image".into()));
    }
    if snap.image.pixels.len() != (snap.image.width as usize) * (snap.image.height as usize) * 4 {
        return Err(JpgError::Malformed("pixels length mismatch".into()));
    }
    if snap.image.pixels.chunks_exact(4).any(|pixel|pixel[3]!=255){return Err(JpgError::Unsupported("JPEG raster output cannot represent transparent alpha samples".into()));}
    if let Some(thumbnail)=&snap.image.jfif_thumbnail{
        if thumbnail.rgb_data.len()!=usize::from(thumbnail.width)*usize::from(thumbnail.height)*3||thumbnail.rgb_data.len()>u16::MAX as usize-16{return Err(JpgError::Malformed("JFIF thumbnail exceeds its exact RGB extent or APP0 segment width".into()));}
    }
    if snap.image.other_segments.iter().any(|segment|!matches!(segment.marker,0xe0..=0xef|0xfe)||segment.data.len()>u16::MAX as usize-2||segment.marker==0xe0&&segment.data.starts_with(b"JFIF\0")){return Err(JpgError::Malformed("opaque metadata requires bounded APP/COM content outside the owned JFIF header".into()));}
    if snap.image.width > u16::MAX as u32 || snap.image.height > u16::MAX as u32 {
        return Err(JpgError::Unsupported("image dimensions exceed JPEG's 16-bit SOF0 width/height field".into()));
    }
    let (width, height): (u16, u16) = (snap.image.width as u16, snap.image.height as u16);
    if !(1..=100).contains(&options.quality) {return Err(JpgError::Malformed("JPEG export quality must be 1..=100".into()));}
    let quality = options.quality as i32;
    let comps = frame_components_of(options)?;
    let hmax = comps.iter().map(|c| c.h_sampling as usize).max().unwrap_or(1);
    let vmax = comps.iter().map(|c| c.v_sampling as usize).max().unwrap_or(1);
    let mcu_w = 8 * hmax;
    let mcu_h = 8 * vmax;
    let mcus_x = (width as usize).div_ceil(mcu_w);
    let mcus_y = (height as usize).div_ceil(mcu_h);
    let pw = mcus_x * mcu_w;
    let ph = mcus_y * mcu_h;

    let mut full: Vec<Vec<f64>> = vec![vec![0f64; pw * ph]; comps.len()];
    for y in 0..ph {
        let sy = y.min(height as usize - 1);
        for x in 0..pw {
            let sx = x.min(width as usize - 1);
            let idx = (sy * width as usize + sx) * 4;
            let (yy, cb, cr) = rgb_to_ycbcr(snap.image.pixels[idx], snap.image.pixels[idx + 1], snap.image.pixels[idx + 2]);
            let channels = [yy, cb, cr];
            for (plane, value) in full.iter_mut().zip(channels) {
                plane[y * pw + x] = value;
            }
        }
    }
    let planes: Vec<(Vec<f64>, usize)> = comps
        .iter()
        .zip(full)
        .map(|(component, source)| {
            let (fx, fy) = (hmax / component.h_sampling as usize, vmax / component.v_sampling as usize);
            if fx == 1 && fy == 1 {
                return (source, pw);
            }
            let (plane, plane_width, _) = box_downsample(&source, pw, ph, fx, fy);
            (plane, plane_width)
        })
        .collect();

    let frame = JpgFrameHeader { precision: 8, width, height, components: comps.clone() };

    let luma_q = quant_zigzag(&scale_quality(&STD_LUMA_Q, quality));
    let chroma_q = quant_zigzag(&scale_quality(&STD_CHROMA_Q, quality));
    let dc_luma = build_huffman(&DC_LUMA_BITS, &dc_luma_values())?;
    let ac_luma = build_huffman(&AC_LUMA_BITS, &ac_luma_values())?;
    let dc_chroma = build_huffman(&DC_CHROMA_BITS, &dc_chroma_values())?;
    let ac_chroma = build_huffman(&AC_CHROMA_BITS, &ac_chroma_values())?;

    let mut out = Vec::new();
    out.extend_from_slice(&[0xFF, 0xD8]);
    out.extend_from_slice(&encode_jfif_app0(snap));
    for seg in &snap.image.other_segments {
        out.push(0xFF);
        out.push(seg.marker);
        let len = seg.data.len() + 2;
        out.push((len >> 8) as u8);
        out.push((len & 0xFF) as u8);
        out.extend_from_slice(&seg.data);
    }

    let mut dqt0 = vec![0xFFu8, 0xDB, 0x00, 0x43, 0x00];
    dqt0.extend(luma_q.iter().map(|&v| v as u8));
    out.extend_from_slice(&dqt0);
    let mut dqt1 = vec![0xFFu8, 0xDB, 0x00, 0x43, 0x01];
    dqt1.extend(chroma_q.iter().map(|&v| v as u8));
    out.extend_from_slice(&dqt1);

    let mut sof = vec![0xFFu8, 0xC0];
    let sof_len = 8 + 3 * frame.components.len();
    sof.push((sof_len >> 8) as u8);
    sof.push((sof_len & 0xFF) as u8);
    sof.push(frame.precision);
    sof.push((frame.height >> 8) as u8);
    sof.push((frame.height & 0xFF) as u8);
    sof.push((frame.width >> 8) as u8);
    sof.push((frame.width & 0xFF) as u8);
    sof.push(frame.components.len() as u8);
    for c in frame.components.iter() {
        sof.push(c.id);
        sof.push((c.h_sampling << 4) | c.v_sampling);
        sof.push(c.quant_table_id);
    }
    out.extend_from_slice(&sof);

    write_dht(&mut out, 0, 0, &DC_LUMA_BITS, &dc_luma_values());
    write_dht(&mut out, 1, 0, &AC_LUMA_BITS, &ac_luma_values());
    write_dht(&mut out, 0, 1, &DC_CHROMA_BITS, &dc_chroma_values());
    write_dht(&mut out, 1, 1, &AC_CHROMA_BITS, &ac_chroma_values());

    let scan_comps: Vec<JpgScanComponent> = comps.iter().enumerate().map(|(index, component)| JpgScanComponent { id: component.id, dc_table_id: if index == 0 { 0 } else { 1 }, ac_table_id: if index == 0 { 0 } else { 1 } }).collect();
    let mut sos = vec![0xFFu8, 0xDA];
    let sos_len = 6 + 2 * scan_comps.len();
    sos.push((sos_len >> 8) as u8);
    sos.push((sos_len & 0xFF) as u8);
    sos.push(scan_comps.len() as u8);
    for c in scan_comps.iter() {
        sos.push(c.id);
        sos.push((c.dc_table_id << 4) | c.ac_table_id);
    }
    sos.push(0);
    sos.push(63);
    sos.push(0);
    out.extend_from_slice(&sos);

    let mut bw = BitWriter::new();
    let mut dc_pred = vec![0i32; comps.len()];
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            for (index, component) in comps.iter().enumerate() {
                let (plane, plane_width) = &planes[index];
                let (table_q, dc_table, ac_table) = if index == 0 { (&luma_q, &dc_luma, &ac_luma) } else { (&chroma_q, &dc_chroma, &ac_chroma) };
                for by in 0..component.v_sampling as usize {
                    for bx in 0..component.h_sampling as usize {
                        let ox = (mx * component.h_sampling as usize + bx) * 8;
                        let oy = (my * component.v_sampling as usize + by) * 8;
                        let mut block = [0f64; 64];
                        for r in 0..8 {
                            for c in 0..8 {
                                block[r * 8 + c] = plane[(oy + r) * plane_width + (ox + c)] - 128.0;
                            }
                        }
                        let coeff = fdct_8x8(&block);
                        let mut zz = [0i32; 64];
                        for z in 0..64 {
                            zz[z] = (coeff[ZIGZAG_TO_NATURAL[z]] / table_q[z] as f64).round() as i32;
                        }
                        encode_block(&mut bw, &zz, &mut dc_pred[index], dc_table, ac_table)?;
                    }
                }
            }
        }
    }
    bw.flush();
    out.extend_from_slice(&bw.bytes);
    out.extend_from_slice(&[0xFF, 0xD9]);
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_dht(out: &mut Vec<u8>, class: u8, id: u8, bits: &[u8; 16], values: &[u8]) {
    out.push(0xFF);
    out.push(0xC4);
    let len = 2 + 1 + 16 + values.len();
    out.push((len >> 8) as u8);
    out.push((len & 0xFF) as u8);
    out.push((class << 4) | id);
    out.extend_from_slice(bits);
    out.extend_from_slice(values);
}
//#endregion Encode
