//! 🗜️ The AC1024 section compression: an LZ77 stream whose only matches are runs of one repeated byte (distance 1), which is what the zero padding of a section page is; everything else is carried as literals. The stream is valid for every reader of the format, no match search is needed, and a page of padding shrinks to a few bytes.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (4.7 Compression)

const MIN_RUN: usize = 6;

fn length(out: &mut Vec<u8>, mut value: usize) {
    while value > 0xFF {
        out.push(0);
        value -= 0xFF;
    }
    out.push(value as u8);
}

fn literal_run(out: &mut Vec<u8>, bytes: &[u8]) {
    if bytes.len() <= 18 {
        out.push((bytes.len() - 3) as u8);
    } else {
        out.push(0);
        length(out, bytes.len() - 18);
    }
    out.extend_from_slice(bytes);
}

fn run_match(out: &mut Vec<u8>, run: usize, trailing: usize) {
    if run >= 0x22 {
        out.push(0x20);
        length(out, run - 0x21);
    } else {
        out.push((run + 0x1E) as u8);
    }
    out.push(trailing as u8);
    out.push(0);
}

/// 🧳️ Encodes `data` (at least four bytes) as a literal prefix, runs of a repeated byte, the literals between them, and the terminator opcode.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 {
        return Err(format!("a compressed page needs at least 4 bytes, got {}", data.len()));
    }
    let mut parts: Vec<(Vec<u8>, usize)> = Vec::new();
    let mut literals = data[..4].to_vec();
    let mut at = 4;
    while at < data.len() {
        let byte = data[at];
        let end = data[at..].iter().position(|next| *next != byte).map_or(data.len(), |count| at + count);
        if end - at >= MIN_RUN {
            literals.push(byte);
            parts.push((std::mem::take(&mut literals), end - at - 1));
        } else {
            literals.extend_from_slice(&data[at..end]);
        }
        at = end;
    }
    parts.push((literals, 0));
    let mut out = Vec::with_capacity(data.len() / 2 + 16);
    let mut following = parts.iter();
    let (first, mut run) = following.next().map(|(bytes, run)| (bytes.as_slice(), *run)).unwrap_or((&[], 0));
    literal_run(&mut out, first);
    for (bytes, next) in following {
        if bytes.len() <= 3 {
            run_match(&mut out, run, bytes.len());
            out.extend_from_slice(bytes);
        } else {
            run_match(&mut out, run, 0);
            literal_run(&mut out, bytes);
        }
        run = *next;
    }
    out.extend_from_slice(&[0x11, 0, 0]);
    Ok(out)
}
