meta:
  id: stdio_mp4_snapshot
  endian: be
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a `stdio.mp4`
  payload: the REAL ISO-BMFF file bytes `crate::artifacts::mp4::standards::isobmff::engine::
  {decode_mp4,encode_mp4}` produce/consume — `ftyp` typed, `moov`/`trak`/`stbl` walked for real
  per-sample tables, `mdat` sample bytes copied verbatim, everything else typed-raw.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
    endian: le
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.mp4.pack v1"
  - id: boxes
    type: iso_bmff_box
    repeat: eos
    doc: The real top-level ISO-BMFF box stream (ftyp, free/uuid/..., mdat, moov).
types:
  iso_bmff_box:
    doc: One ISO/IEC 14496-12 box — 32-bit size (or 64-bit largesize when size==1) + 4-byte type.
    seq:
      - id: size32
        type: u4
      - id: fourcc
        type: str
        size: 4
        encoding: ASCII
      - id: largesize
        type: u8
        if: size32 == 1
      - id: body
        size: (size32 == 1 ? largesize : (size32 == 0 ? -1 : size32)) - (size32 == 1 ? 16 : 8)
  ftyp_body:
    doc: "§4.3 FileTypeBox — major_brand + minor_version + compatible_brands*."
    seq:
      - id: major_brand
        type: str
        size: 4
      - id: minor_version
        type: u4
      - id: compatible_brands
        type: str
        size: 4
        repeat: eos
  moov_trak_tkhd:
    doc: "§8.3.2 TrackHeaderBox — this codec reads only track_id (see engine module doc comment
      for the documented normal-form scope: matrix/volume/timestamps are not retained)."
    seq:
      - id: version
        type: u1
      - id: flags
        size: 3
      - id: creation_and_modification_time
        size: "version == 1 ? 16 : 8"
      - id: track_id
        type: u4
  stbl_stsd_avc1_avcc:
    doc: "ISO/IEC 14496-15 AVCDecoderConfigurationRecord — configurationVersion, AVCProfileIndication,
      profile_compatibility, AVCLevelIndication, lengthSizeMinusOne, then SPS*/PPS* each u2-length-prefixed."
    seq:
      - id: configuration_version
        type: u1
      - id: avc_profile_indication
        type: u1
      - id: profile_compatibility
        type: u1
      - id: avc_level_indication
        type: u1
      - id: length_size_minus_one_reserved
        type: u1
      - id: num_sps_reserved
        type: u1
