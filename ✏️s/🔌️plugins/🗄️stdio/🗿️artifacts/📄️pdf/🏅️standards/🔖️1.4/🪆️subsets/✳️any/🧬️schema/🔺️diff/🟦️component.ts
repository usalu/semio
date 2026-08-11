/** 🔺️ PdfDiff (1.4) — handcrafted sparse diff over `PageDoc{width,height,text}`. No
 *  `snapshot?: PdfSnapshot` full-replace slot; `PageDoc` has no collections so this is a flat
 *  3-field patch (mirrors the Rust `PdfDiff` shape 1:1). */
export interface PdfDiff {
  width?: number;
  height?: number;
  text?: string;
}
