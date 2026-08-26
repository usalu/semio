/** 🧬️ TiffSnapshot schema facet — mirrors 🦀️component.rs field-for-field. Complete TIFF 6.0
 * semantic model: `byteOrder` + index-keyed `ifds`, each holding tag-id-keyed typed tag/type/
 * value entries — TIFF's own generic model ("unknown tags" are just tags this codec doesn't
 * specially interpret, but whose typed value is still stored losslessly via this same
 * triple), plus decoded `pixels`. */

export type TiffByteOrder = 'littleEndian' | 'bigEndian';

/** 🏷️ TIFF6 §2 Table 2 — the 12 real IFD entry field types. */
export type TiffFieldType =
  | 'byte' | 'ascii' | 'short' | 'long' | 'rational' | 'sByte'
  | 'undefined' | 'sShort' | 'sLong' | 'sRational' | 'float' | 'double';

/** 📦️ Typed union over every TIFF 6.0 field type's decoded value — adjacently tagged
 * (`kind`/`value`), mirroring the Rust enum's `#[serde(tag = "kind", content = "value")]`. */
export type TiffValues =
  | { kind: 'byte'; value: number[] }
  | { kind: 'ascii'; value: string }
  | { kind: 'short'; value: number[] }
  | { kind: 'long'; value: number[] }
  | { kind: 'rational'; value: [number, number][] }
  | { kind: 'sByte'; value: number[] }
  | { kind: 'undefined'; value: number[] }
  | { kind: 'sShort'; value: number[] }
  | { kind: 'sLong'; value: number[] }
  | { kind: 'sRational'; value: [number, number][] }
  | { kind: 'float'; value: number[] }
  | { kind: 'double'; value: number[] };

/** 🏷️ One IFD entry — a weak value (whole-value replaced in diffs: `kind`/`values` move
 * together atomically). */
export interface TiffTag {
  tag: number;
  kind: TiffFieldType;
  values: TiffValues;
}

/** 🗂️ One Image File Directory — tag-id-keyed `entries` (TIFF requires ascending-tag-order). */
export interface TiffIfd {
  entries: TiffTag[];
  /** This directory's own raster as RAW STRIP BYTES; empty for IFD 0 (whose raster is the
   * snapshot's own canonical RGBA `pixels`) and for a metadata-only directory. */
  pixels: number[];
}

/** 📸️ Complete `stdio.tiff` 6.0 semantic snapshot. `schema` is an identity field, never
 * diffed. `pixels` is the decoded raster payload (canonical 8-bit RGBA, decoded from IFD 0
 * only — see the Rust engine's doc for the full completeness accounting). */
export interface TiffSnapshot {
  schema: string;
  byteOrder: TiffByteOrder;
  ifds: TiffIfd[];
  pixels: number[];
}
