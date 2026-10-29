/** 🗜️ A first-party ZIP writer (stored entries, deterministic timestamps) and PNG text-chunk embedding; both need the CRC-32 below.
 *
 * @see https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT — ZIP file format
 * @see https://www.w3.org/TR/png/#11tEXt — PNG textual information
 */

const TABLE = Uint32Array.from({ length: 256 }, (_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});

/** 🔢️ The CRC-32 (IEEE 802.3) of bytes, continuing from `seed`. */
export function crc32(bytes: Uint8Array, seed = 0): number {
  let c = ~seed >>> 0;
  for (const byte of bytes) c = TABLE[(c ^ byte) & 0xff]! ^ (c >>> 8);
  return ~c >>> 0;
}

const encoder = new TextEncoder();
const DOS_DATE = ((2026 - 1980) << 9) | (1 << 5) | 1;

/** 📦️ One file of an archive. */
export type ZipEntry = { readonly name: string; readonly bytes: Uint8Array };

/** 🗜️ Packs entries into a ZIP archive with stored (uncompressed) members; PNG frames are already compressed. */
export function writeZip(entries: readonly ZipEntry[], comment = ""): Uint8Array {
  const parts: Uint8Array[] = [];
  const central: Uint8Array[] = [];
  let offset = 0;
  const header = (size: number): { view: DataView; bytes: Uint8Array } => {
    const bytes = new Uint8Array(size);
    return { view: new DataView(bytes.buffer), bytes };
  };
  for (const entry of entries) {
    const name = encoder.encode(entry.name);
    const crc = crc32(entry.bytes);
    const local = header(30 + name.length);
    local.view.setUint32(0, 0x04034b50, true);
    local.view.setUint16(4, 20, true);
    local.view.setUint16(6, 0x0800, true);
    local.view.setUint16(10, 0, true);
    local.view.setUint16(12, DOS_DATE, true);
    local.view.setUint32(14, crc, true);
    local.view.setUint32(18, entry.bytes.length, true);
    local.view.setUint32(22, entry.bytes.length, true);
    local.view.setUint16(26, name.length, true);
    local.bytes.set(name, 30);
    const directory = header(46 + name.length);
    directory.view.setUint32(0, 0x02014b50, true);
    directory.view.setUint16(4, 20, true);
    directory.view.setUint16(6, 20, true);
    directory.view.setUint16(8, 0x0800, true);
    directory.view.setUint16(14, DOS_DATE, true);
    directory.view.setUint32(16, crc, true);
    directory.view.setUint32(20, entry.bytes.length, true);
    directory.view.setUint32(24, entry.bytes.length, true);
    directory.view.setUint16(28, name.length, true);
    directory.view.setUint32(42, offset, true);
    directory.bytes.set(name, 46);
    parts.push(local.bytes, entry.bytes);
    central.push(directory.bytes);
    offset += local.bytes.length + entry.bytes.length;
  }
  const size = central.reduce((sum, part) => sum + part.length, 0);
  const text = encoder.encode(comment);
  const end = header(22 + text.length);
  end.view.setUint32(0, 0x06054b50, true);
  end.view.setUint16(8, entries.length, true);
  end.view.setUint16(10, entries.length, true);
  end.view.setUint32(12, size, true);
  end.view.setUint32(16, offset, true);
  end.view.setUint16(20, text.length, true);
  end.bytes.set(text, 22);
  const all = [...parts, ...central, end.bytes];
  const out = new Uint8Array(all.reduce((sum, part) => sum + part.length, 0));
  let at = 0;
  for (const part of all) {
    out.set(part, at);
    at += part.length;
  }
  return out;
}

const latin1 = (text: string): Uint8Array => Uint8Array.from(text, (c) => c.charCodeAt(0) & 0xff);

/** 🏷️ Inserts `tEXt` chunks (Latin-1 keyword and text, non-Latin characters degrade to `?`) after the IHDR chunk of a PNG. */
export function withPngText(png: Uint8Array, entries: Readonly<Record<string, string>>): Uint8Array {
  const chunks = Object.entries(entries).map(([keyword, text]) => {
    const body = new Uint8Array([...latin1(keyword), 0, ...latin1(text.replace(/[^\x20-\xff]/gu, "?"))]);
    const chunk = new Uint8Array(12 + body.length);
    const view = new DataView(chunk.buffer);
    view.setUint32(0, body.length);
    chunk.set(latin1("tEXt"), 4);
    chunk.set(body, 8);
    view.setUint32(8 + body.length, crc32(chunk.subarray(4, 8 + body.length)));
    return chunk;
  });
  const afterIhdr = 8 + 12 + new DataView(png.buffer, png.byteOffset).getUint32(8);
  const out = new Uint8Array(png.length + chunks.reduce((sum, chunk) => sum + chunk.length, 0));
  out.set(png.subarray(0, afterIhdr), 0);
  let at = afterIhdr;
  for (const chunk of chunks) {
    out.set(chunk, at);
    at += chunk.length;
  }
  out.set(png.subarray(afterIhdr), at);
  return out;
}
