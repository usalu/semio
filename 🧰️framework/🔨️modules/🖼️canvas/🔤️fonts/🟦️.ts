export const FONT_ASSET = "🔤️guestslim-typst-fonts.bin";

/** 🔤️ Validates every packed font extent before publication. */
export function validateFontAsset(bytes: Uint8Array): number {
  if (bytes.byteLength < 4) throw new Error("Missing packed font count");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength), count = view.getUint32(0, true);
  if (count < 1 || count > 1024) throw new Error("Invalid packed font count");
  let offset = 4;
  for (let index = 0; index < count; index++) {
    if (offset + 4 > bytes.byteLength) throw new Error("Missing packed font extent");
    const size = view.getUint32(offset, true);
    offset += 4;
    if (size < 12 || offset + size > bytes.byteLength) throw new Error("Invalid packed font extent");
    offset += size;
  }
  if (offset !== bytes.byteLength) throw new Error("Trailing packed font bytes");
  return count;
}
