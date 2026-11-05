//! 🧪️ Private entropy primitives are tested beside the codec that owns them.
use super::*;

#[test]
fn huffman_round_trips_all_dc_luma_symbols() {
    let table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let mut bw = BitWriter::new();
    for v in 0u8..=11 {
        let (l, c) = *table.encode.get(&v).unwrap();
        bw.put_bits(c, l);
    }
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    for v in 0u8..=11 {
        assert_eq!(br.decode_symbol(&table).unwrap(), v);
    }
}

#[test]
fn single_block_round_trips_through_huffman() {
    let dc_table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let ac_table = build_huffman(&AC_LUMA_BITS, &ac_luma_values()).unwrap();
    let mut zz = [0i32; 64];
    zz[0] = 120;
    zz[1] = 5;
    zz[2] = -3;
    zz[20] = 1;
    let mut bw = BitWriter::new();
    let mut dc_pred = 0i32;
    encode_block(&mut bw, &zz, &mut dc_pred, &dc_table, &ac_table).unwrap();
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    let mut dc_pred2 = 0i32;
    let decoded = decode_block(&mut br, &mut dc_pred2, &dc_table, &ac_table).unwrap();
    assert_eq!(decoded, zz);
}

