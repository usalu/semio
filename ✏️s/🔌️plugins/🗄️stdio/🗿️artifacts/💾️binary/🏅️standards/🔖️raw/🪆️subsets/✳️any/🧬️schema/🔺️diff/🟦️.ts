/** ✂️ One byte-range edit against the BASE array. */
export interface ByteSplice {
  offset: number;
  removeLen: number;
  insert: number[];
}

/** 🔺️ BinaryDiff schema — an ordered splice list (processed descending by offset on apply). */
export interface BinaryDiff {
  splices: ByteSplice[];
}
