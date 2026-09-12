
use super::{PcmWav, PcmWavFormat, decode_pcm16_wav, encode_pcm16_wav};

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("ascii hex"), 16).expect("hex byte")).collect()
}

#[test]
fn owned_pcm16_codec_matches_the_frozen_hound_golden() {
    let wav = PcmWav { format: PcmWavFormat { channels: 1, sample_rate: 8000, bits_per_sample: 16 }, samples: vec![-32768, -1, 0, 1, 32767], other_chunks: Vec::new() };
    let expected = hex("524946462e00000057415645666d74201000000001000100401f0000803e000002001000646174610a0000000080ffff00000100ff7f");
    let encoded = encode_pcm16_wav(&wav).expect("encode hostile lanes");
    assert_eq!(encoded, expected);
    let decoded = decode_pcm16_wav(&encoded).expect("decode hostile lanes");
    assert_eq!(decoded.format.channels, 1);
    assert_eq!(decoded.format.sample_rate, 8000);
    assert_eq!(decoded.format.bits_per_sample, 16);
    assert_eq!(decoded.samples, wav.samples);
}

#[test]
fn owned_pcm16_codec_preserves_ordered_odd_auxiliary_chunks() {
    let wav = PcmWav { format: PcmWavFormat { channels: 2, sample_rate: 44_100, bits_per_sample: 16 }, samples: vec![-32768, 32767, -1, 1], other_chunks: vec![("JUNK".to_string(), vec![0, 127, 255]), ("fact".to_string(), vec![2, 0, 0, 0])] };
    let decoded = decode_pcm16_wav(&encode_pcm16_wav(&wav).expect("encode auxiliary chunks")).expect("decode auxiliary chunks");
    assert_eq!(decoded.samples, wav.samples);
    assert_eq!(decoded.other_chunks, wav.other_chunks);
}

#[test]
fn owned_pcm16_decoder_rejects_hostile_framing_and_format_fields() {
    let hostile = [
        "000000000000000000000000",
        "524946461000000057415645666d74200400000001000100",
        "524946462400000057415645666d74201000000003000100401f0000803e0000020010006461746100000000",
        "524946462400000057415645666d74201000000001000000401f000000000000000010006461746100000000",
        "524946462400000057415645666d74201000000001000100401f000001000000020010006461746100000000",
        "524946462400000057415645666d74201000000001000100401f0000803e0000040010006461746100000000",
        "524946462500000057415645666d74201000000001000100401f0000803e000002001000646174610100000000",
        "524946462400000057415645666d74201000000001000100401f0000803e00000200100064617461040000000000",
    ];
    for bytes in hostile {
        assert!(decode_pcm16_wav(&hex(bytes)).is_err(), "accepted hostile WAVE {bytes}");
    }
}

#[test]
fn owned_pcm16_encoder_rejects_incomplete_frames_and_invalid_chunk_ids() {
    let format = PcmWavFormat { channels: 2, sample_rate: 44_100, bits_per_sample: 16 };
    assert!(encode_pcm16_wav(&PcmWav { format, samples: vec![1], other_chunks: Vec::new() }).is_err());
    assert!(encode_pcm16_wav(&PcmWav { format, samples: vec![1, 2], other_chunks: vec![("wide!".to_string(), Vec::new())] }).is_err());
}
