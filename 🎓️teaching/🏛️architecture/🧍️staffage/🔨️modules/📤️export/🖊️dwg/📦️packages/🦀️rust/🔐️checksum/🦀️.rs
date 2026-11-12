//! 🔐️ The three checksums of an AC1024 file: CRC-16 (objects, header, classes, handle chunks), CRC-32 (file header data) and the page Adler variant, plus the magic sequence that pads and masks pages.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (2.14 CRCs, 4.1, 4.2, 4.6)

const fn crc16_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u16;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 { (value >> 1) ^ 0xA001 } else { value >> 1 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

const fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 { (value >> 1) ^ 0xEDB8_8320 } else { value >> 1 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

const CRC16: [u16; 256] = crc16_table();
const CRC32: [u32; 256] = crc32_table();

/// 🧾️ The seed of every CRC-16 in the format.
pub const CRC16_SEED: u16 = 0xC0C1;

/// 🔢️ CRC-16 (polynomial 0xA001, reflected) of `data` continuing from `seed`.
pub fn crc16(seed: u16, data: &[u8]) -> u16 {
    data.iter().fold(seed, |crc, byte| (crc >> 8) ^ CRC16[usize::from((crc as u8) ^ byte)])
}

/// 🧪️ Standard CRC-32 (ISO-HDLC) of `data`.
pub fn crc32(data: &[u8]) -> u32 {
    !data.iter().fold(!0u32, |crc, byte| (crc >> 8) ^ CRC32[usize::from((crc as u8) ^ byte)])
}

/// 🧮️ The page checksum: an Adler-32 variant over `data` continuing from `seed`.
pub fn adler(seed: u32, data: &[u8]) -> u32 {
    let (mut sum1, mut sum2) = (seed & 0xFFFF, seed >> 16);
    for chunk in data.chunks(0x15B0) {
        for byte in chunk {
            sum1 += u32::from(*byte);
            sum2 += sum1;
        }
        sum1 %= 0xFFF1;
        sum2 %= 0xFFF1;
    }
    (sum2 << 16) | (sum1 & 0xFFFF)
}

/// 🎲️ The 256 bytes of the linear congruential magic sequence (seed 1) that mask the file header and pad pages.
pub fn magic() -> [u8; 256] {
    let mut seed: u32 = 1;
    let mut bytes = [0u8; 256];
    for byte in &mut bytes {
        seed = seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        *byte = (seed >> 16) as u8;
    }
    bytes
}

/// 🎭️ Masks the four-byte groups of a data page header with `0x4164536B ^ position` (self-inverse).
pub fn mask_page_header(header: &mut [u8], position: u32) {
    let mask = (0x4164_536B ^ position).to_le_bytes();
    for group in header.as_chunks_mut::<4>().0 {
        for (byte, key) in group.iter_mut().zip(mask) {
            *byte ^= key;
        }
    }
}
