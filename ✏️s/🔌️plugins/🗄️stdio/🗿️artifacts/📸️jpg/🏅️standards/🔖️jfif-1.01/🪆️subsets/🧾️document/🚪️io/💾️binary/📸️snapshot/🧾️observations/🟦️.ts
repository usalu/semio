/** 🧾️ Native JPEG observations kept independently from logical content. */
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


export interface JpgScanComponent {id:number;dcTableId:number;acTableId:number}
export interface JpgArithmeticConditioning {selector:number;value:number}
export interface JpgNativeObservations {frame:JpgFrameHeader;sofMarker:number;arithmetic:boolean;arithmeticConditioning:JpgArithmeticConditioning[];scanComponents:JpgScanComponent[];scanParameters:[number,number,number];quantTables:JpgQuantTable[];huffmanTables:JpgHuffmanTable[];restartInterval?:number}
