//! 🔌️ The WebSocket handshake and frame codec: accept key, SHA-1, base64 and client and server frames.

use super::http_surface::HttpTerminalReason;
use super::http_transport_state::{HTTP_RESPONSE_BYTES, WEBSOCKET_FRAME_BYTES};

//#region 🔖️HttpTransport
pub(crate) fn websocket_accept(key: &str) -> String {
    let mut input = Vec::with_capacity(key.len() + 36);
    input.extend_from_slice(key.as_bytes());
    input.extend_from_slice(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    base64(&sha1(&input))
}

pub(crate) fn websocket_key_nonce(key: &str) -> Option<[u8; 16]> {
    let bytes = key.as_bytes();
    if bytes.len() != 24 || bytes[22..] != *b"==" {
        return None;
    }
    let mut nonce = [0u8; 16];
    for group in 0..5 {
        let index = group * 4;
        let a = base64_value(bytes[index])?;
        let b = base64_value(bytes[index + 1])?;
        let c = base64_value(bytes[index + 2])?;
        let d = base64_value(bytes[index + 3])?;
        nonce[group * 3] = (a << 2) | (b >> 4);
        nonce[group * 3 + 1] = (b << 4) | (c >> 2);
        nonce[group * 3 + 2] = (c << 6) | d;
    }
    let a = base64_value(bytes[20])?;
    let b = base64_value(bytes[21])?;
    if b & 0x0f != 0 {
        return None;
    }
    nonce[15] = (a << 2) | (b >> 4);
    Some(nonce)
}

fn base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn sha1(input: &[u8]) -> [u8; 20] {
    let bit_len = (input.len() as u64) * 8;
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 80];
        for (index, word) in words[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[index * 4..index * 4 + 4].try_into().expect("sha1 word width"));
        }
        for index in 16..80 {
            words[index] = (words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for (index, word) in words.iter().enumerate() {
            let (f, k) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let next = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = next;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut digest = [0u8; 20];
    for (index, word) in h.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let value = ((chunk[0] as u32) << 16) | ((chunk.get(1).copied().unwrap_or(0) as u32) << 8) | chunk.get(2).copied().unwrap_or(0) as u32;
        output.push(TABLE[((value >> 18) & 0x3f) as usize] as char);
        output.push(TABLE[((value >> 12) & 0x3f) as usize] as char);
        output.push(if chunk.len() > 1 { TABLE[((value >> 6) & 0x3f) as usize] as char } else { '=' });
        output.push(if chunk.len() > 2 { TABLE[(value & 0x3f) as usize] as char } else { '=' });
    }
    output
}

#[derive(Clone, Copy)]
pub(super) struct ClientWebSocketFrame {
    pub(crate) opcode: u8,
    payload_start: usize,
    pub(crate) payload_len: usize,
    mask: [u8; 4],
    pub(crate) consumed: usize,
}

impl ClientWebSocketFrame {
    pub(crate) fn payload_byte(self, bytes: &[u8], index: usize) -> Option<u8> {
        if index >= self.payload_len {
            return None;
        }
        bytes.get(self.payload_start + index).map(|byte| byte ^ self.mask[index % 4])
    }

    pub(super) fn copy_control_payload(self, bytes: &[u8], output: &mut [u8; 125]) -> Option<usize> {
        if self.payload_len > output.len() {
            return None;
        }
        for (index, target) in output[..self.payload_len].iter_mut().enumerate() {
            *target = self.payload_byte(bytes, index)?;
        }
        Some(self.payload_len)
    }
}

pub(crate) fn decode_client_websocket_frame(bytes: &[u8]) -> Result<Option<ClientWebSocketFrame>, HttpTerminalReason> {
    if bytes.len() < 2 {
        return Ok(None);
    }
    let fin = bytes[0] & 0x80 != 0;
    let rsv = bytes[0] & 0x70;
    let opcode = bytes[0] & 0x0f;
    let masked = bytes[1] & 0x80 != 0;
    if !fin || rsv != 0 || !masked || !matches!(opcode, 0x2 | 0x8 | 0x9 | 0xA) {
        return Err(HttpTerminalReason::Unsupported);
    }
    let mut cursor = 2;
    let marker = bytes[1] & 0x7f;
    let payload_len = match marker {
        0..=125 => marker as usize,
        126 => {
            if bytes.len() < cursor + 2 {
                return Ok(None);
            }
            let value = u16::from_be_bytes(bytes[cursor..cursor + 2].try_into().expect("websocket u16 width")) as usize;
            cursor += 2;
            value
        }
        _ => {
            if bytes.len() < cursor + 8 {
                return Ok(None);
            }
            let value = u64::from_be_bytes(bytes[cursor..cursor + 8].try_into().expect("websocket u64 width"));
            cursor += 8;
            usize::try_from(value).map_err(|_| HttpTerminalReason::Capacity)?
        }
    };
    if payload_len > WEBSOCKET_FRAME_BYTES || (opcode & 0x8 != 0 && payload_len > 125) {
        return Err(HttpTerminalReason::Capacity);
    }
    if bytes.len() < cursor + 4 {
        return Ok(None);
    }
    let mask: [u8; 4] = bytes[cursor..cursor + 4].try_into().expect("websocket mask width");
    cursor += 4;
    let Some(consumed) = cursor.checked_add(payload_len) else { return Err(HttpTerminalReason::Capacity) };
    if bytes.len() < consumed {
        return Ok(None);
    }
    Ok(Some(ClientWebSocketFrame { opcode, payload_start: cursor, payload_len, mask, consumed }))
}

pub(crate) fn encode_server_websocket_frame(opcode: u8, payload: &[u8]) -> Option<Vec<u8>> {
    let (header, header_bytes) = websocket_server_header(opcode, payload.len())?;
    let total = header_bytes.checked_add(payload.len())?;
    if total > HTTP_RESPONSE_BYTES {
        return None;
    }
    let mut frame = Vec::new();
    frame.try_reserve_exact(total).ok()?;
    frame.extend_from_slice(&header[..header_bytes]);
    frame.extend_from_slice(payload);
    Some(frame)
}

pub(super) fn websocket_server_header(opcode: u8, payload_len: usize) -> Option<([u8; 10], usize)> {
    if payload_len > WEBSOCKET_FRAME_BYTES || (opcode & 0x8 != 0 && payload_len > 125) {
        return None;
    }
    let mut header = [0u8; 10];
    header[0] = 0x80 | opcode;
    let header_len = if payload_len <= 125 {
        header[1] = payload_len as u8;
        2
    } else if payload_len <= u16::MAX as usize {
        header[1] = 126;
        header[2..4].copy_from_slice(&(payload_len as u16).to_be_bytes());
        4
    } else {
        header[1] = 127;
        header[2..10].copy_from_slice(&(payload_len as u64).to_be_bytes());
        10
    };
    Some((header, header_len))
}
//#endregion 🔖️HttpTransport
