/** 📋️ Clipboard envelope admission counts UTF8 source bytes and exact JSON string escaping. */
export const CLIPBOARD_TEXT_MAX_BYTES=1_048_576;
export const CLIPBOARD_METADATA_MAX_WIRE_BYTES=16_384;
export const CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES=CLIPBOARD_TEXT_MAX_BYTES*6+CLIPBOARD_METADATA_MAX_WIRE_BYTES;
export const CLIPBOARD_PASTE_MAX_WIRE_BYTES=CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES+CLIPBOARD_METADATA_MAX_WIRE_BYTES;
export function clipboardJsonStringBytes(text:string):number {
 let bytes=2;
 for(const scalar of text){const code=scalar.codePointAt(0)!;bytes+=code===34||code===92||[8,9,10,12,13].includes(code)?2:code<32||code>=0xd800&&code<=0xdfff?6:code<128?1:code<2048?2:code<65536?3:4;}
 return bytes;
}
