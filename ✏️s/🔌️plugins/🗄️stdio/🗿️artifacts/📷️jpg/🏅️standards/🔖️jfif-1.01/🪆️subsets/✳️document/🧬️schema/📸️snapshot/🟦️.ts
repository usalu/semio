/** 🧬️ JpgSnapshot schema facet — mirrors 🦀️.rs field-for-field. Complete JFIF 1.01
 * semantic model: typed JFIF APP0, typed SOF (frame) + id-keyed DQT/DHT tables, DRI restart
 * interval, verbatim-retained other APPn/COM segments, decoded pixels. */

/** 📏️ JFIF APP0 `units` byte. `aspect` means x/y density are merely a pixel aspect ratio. */
export type JfifDensityUnits = 'aspect' | 'pixelsPerInch' | 'pixelsPerCm';

/** 🖼️ JFIF APP0's optional embedded thumbnail — uncompressed 24-bit RGB, `width * height * 3`
 * bytes, row-major. A weak value (whole-value replaced in diffs). */
export interface JfifThumbnail {
  width: number;
  height: number;
  rgbData: number[];
}

/** 🧩️ One SOF0 frame component descriptor. Id-keyed within `JpgFrameHeader.components`. */
export interface JpgFrameComponent {
  id: number;
  hSampling: number;
  vSampling: number;
  quantTableId: number;
}

/** 🖼️ Baseline (SOF0) frame header. */
export interface JpgFrameHeader {
  precision: number;
  width: number;
  height: number;
  components: JpgFrameComponent[];
}

/** 📊️ One `DQT` table (id-keyed within `JpgSnapshot.quantTables`). `values` is retained in the
 * EXACT zigzag scan order the DQT segment stores on disk. */
export interface JpgQuantTable {
  id: number;
  /** DQT `Pq` nibble: `0` = 8-bit values, `1` = 16-bit values. */
  precision: number;
  values: number[]; // exactly 64 entries
}

/** 🌳️ `DHT` table class. */
export type JpgHuffmanClass = 'dc' | 'ac';

/** 🌳️ One `DHT` table, keyed by `(class, id)` within `JpgSnapshot.huffmanTables`. */
export interface JpgHuffmanTable {
  id: number;
  class: JpgHuffmanClass;
  bits: number[]; // exactly 16 entries
  values: number[];
}

/** 🗃️ An APPn (other than a recognized JFIF APP0)/COM segment retained VERBATIM. Index-keyed
 * within `JpgSnapshot.otherSegments` — duplicate markers are legal. */
export interface JpgSegment {
  marker: number;
  data: number[];
}

/** 📸️ Complete `stdio.jpg` jfif-1.01 semantic snapshot. `schema` is an identity field, never
 * diffed. `frame`/`sofMarker`/`arithmetic` are populated by decode (undefined/0/false only for a
 * snapshot that has never round-tripped through a real JPEG byte stream). `width`/`height`/
 * `pixels` are the canonical raster (8-bit RGBA, `width * height * 4` bytes) — distinct from
 * `frame.width`/`frame.height` (on-disk SOF values). */
export interface JpgSnapshot {
  schema: string;
  width: number;
  height: number;
  pixels: number[];
  reEncodeQuality?: number;
  jfifVersion: [number, number];
  jfifDensityUnits: JfifDensityUnits;
  jfifXDensity: number;
  jfifYDensity: number;
  jfifThumbnail?: JfifThumbnail;
  frame?: JpgFrameHeader;
  sofMarker: number;
  arithmetic: boolean;
  quantTables: JpgQuantTable[];
  huffmanTables: JpgHuffmanTable[];
  restartInterval?: number;
  otherSegments: JpgSegment[];
}
