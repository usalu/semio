meta:
  id: stdio_pdf_1_7_snapshot
  endian: le
doc: |
  Real (not placeholder) Kaitai Struct sketch of a PDF 1.7 file's byte-level framing (ISO
  32000-1 §7.5) -- the `⚙️engine`'s classic/xref-stream/hybrid/brute-force reader implements this
  shape imperatively rather than via a Kaitai-generated parser, but the field layout matches.
seq:
  - id: magic
    contents: '%PDF-'
  - id: version_major
    type: str
    encoding: ASCII
    terminator: 0x2e # '.'
  - id: version_minor
    type: strz
    encoding: ASCII
    terminator: 0x0a
  - id: body
    type: body_bytes
    size-eos: true
types:
  body_bytes:
    doc: |
      Indirect objects, xref table/stream, and trailer, interleaved per the incremental-update
      structure ISO 32000-1 allows -- byte offsets in the trailing xref table (or `startxref`
      pointer) are the authoritative index, not a fixed record layout, so this leaf is
      necessarily `size-eos` (the `⚙️engine`'s `build_xref`/`resolve_all` walk it via those
      offsets, not sequential Kaitai-style field decoding).
    seq:
      - id: raw
        size-eos: true
