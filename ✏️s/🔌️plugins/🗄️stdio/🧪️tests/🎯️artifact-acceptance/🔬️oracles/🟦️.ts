/** 🔬️ Third-party oracles of the artifact acceptance matrix: a format's bytes are judged by an independent parser (JSON.parse, fast-xml-parser, pngjs, fflate) or by the published magic of the format, never by our own codecs.
 * @see https://www.w3.org/TR/PNG/#5PNG-file-signature
 * @see https://www.rfc-editor.org/rfc/rfc1950 */
import { XMLValidator } from "fast-xml-parser";
import { PNG } from "pngjs";
import { unzlibSync, unzipSync } from "fflate";
import { createHash } from "node:crypto";

/** 🧿️ Fingerprints persisted content, preserving values while ignoring JSON ordering and ZIP container metadata. */
export function persistedContent(bytes: Uint8Array, comparison: "bytes" | "json" | "zip-members"): string {
  const compare = (a: string, b: string): number => a < b ? -1 : a > b ? 1 : 0;
  const ordered = (value: unknown): unknown => Array.isArray(value) ? value.map(ordered) : value !== null && typeof value === "object" ? Object.fromEntries(Object.entries(value).sort(([a], [b]) => compare(a, b)).map(([key, entry]) => [key, ordered(entry)])) : value;
  const canonical = comparison === "json" ? JSON.stringify(ordered(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)))) : comparison === "zip-members" ? JSON.stringify(Object.entries(unzipSync(bytes)).sort(([a], [b]) => compare(a, b)).map(([name, content]) => [name, Buffer.from(content).toString("base64")])) : bytes;
  return createHash("sha256").update(canonical).digest("hex");
}

/** ⚖️ One oracle verdict. */
export type OracleVerdict = Readonly<{ ok: boolean; detail: string }>;

const ascii = (bytes: Uint8Array, from: number, length: number): string => String.fromCharCode(...bytes.slice(from, from + length));
const startsWith = (bytes: Uint8Array, signature: readonly number[]): boolean => signature.every((byte, index) => bytes[index] === byte);
const localHeader = (bytes: Uint8Array): boolean => startsWith(bytes, [0x50, 0x4b, 0x03, 0x04]);
const text = (bytes: Uint8Array): string => new TextDecoder("utf-8", { fatal: true }).decode(bytes);
const verdict = (ok: boolean, detail: string): OracleVerdict => ({ ok, detail });
const guard = (run: () => OracleVerdict): OracleVerdict => {
  try {
    return run();
  } catch (error) {
    return verdict(false, String(error).split("\n")[0]!.slice(0, 160));
  }
};
const head = (bytes: Uint8Array, pattern: RegExp): boolean => pattern.test(text(bytes.slice(0, 256)));

/** 🧾️ Oracles keyed by the fixture's `oracle` name. */
export const ORACLES: Readonly<Record<string, (bytes: Uint8Array) => OracleVerdict>> = {
  json: (bytes) => guard(() => (JSON.parse(text(bytes)), verdict(true, "JSON.parse"))),
  xml: (bytes) => guard(() => { const result = XMLValidator.validate(text(bytes)); return verdict(result === true, result === true ? "fast-xml-parser" : result.err.msg); }),
  svg: (bytes) => guard(() => { const source = text(bytes); const result = XMLValidator.validate(source); return verdict(result === true && /<svg[\s>]/u.test(source), result === true ? "fast-xml-parser, svg root" : result.err.msg); }),
  html: (bytes) => guard(() => verdict(/<[a-z!][^>]*>/iu.test(text(bytes)), "markup present")),
  text: (bytes) => guard(() => (text(bytes), verdict(true, "UTF-8 decodes"))),
  csv: (bytes) => guard(() => (text(bytes), verdict(true, "UTF-8 decodes"))),
  tsv: (bytes) => guard(() => (text(bytes), verdict(true, "UTF-8 decodes"))),
  zip: (bytes) => guard(() => verdict(localHeader(bytes) && Object.keys(unzipSync(bytes)).length >= 0, "PK local header, fflate unzip")),
  ooxml: (bytes) => guard(() => verdict(localHeader(bytes) && "[Content_Types].xml" in unzipSync(bytes), "PK local header, fflate unzip, [Content_Types].xml")),
  bcf: (bytes) => guard(() => verdict(localHeader(bytes) && Object.keys(unzipSync(bytes)).some((name) => name.endsWith("bcf.version")), "PK local header, fflate unzip, bcf.version")),
  zlib: (bytes) => guard(() => verdict(unzlibSync(bytes).length >= 0, "fflate unzlib")),
  png: (bytes) => guard(() => { const image = PNG.sync.read(Buffer.from(bytes)); return verdict(image.width > 0 && image.height > 0, `pngjs ${image.width}x${image.height}`); }),
  gif: (bytes) => verdict(ascii(bytes, 0, 3) === "GIF", "GIF signature"),
  bmp: (bytes) => verdict(ascii(bytes, 0, 2) === "BM", "BM signature"),
  jpeg: (bytes) => verdict(startsWith(bytes, [0xff, 0xd8, 0xff]), "SOI marker"),
  tiff: (bytes) => verdict(startsWith(bytes, [0x49, 0x49, 0x2a, 0]) || startsWith(bytes, [0x4d, 0x4d, 0, 0x2a]), "byte order mark and 42"),
  pdf: (bytes) => verdict(ascii(bytes, 0, 5) === "%PDF-", "%PDF- header"),
  wav: (bytes) => verdict(ascii(bytes, 0, 4) === "RIFF" && ascii(bytes, 8, 4) === "WAVE", "RIFF WAVE"),
  avi: (bytes) => verdict(ascii(bytes, 0, 4) === "RIFF" && ascii(bytes, 8, 4) === "AVI ", "RIFF AVI"),
  mp3: (bytes) => verdict(ascii(bytes, 0, 3) === "ID3" || (bytes[0] === 0xff && ((bytes[1] ?? 0) & 0xfe) === 0xfa), "ID3 tag or MPEG-1 Layer III frame sync"),
  mp4: (bytes) => { const size = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getUint32(0); return verdict(ascii(bytes, 4, 4) === "ftyp" && size >= 16 && size <= bytes.length, "ftyp box with a sane size"); },
  stl: (bytes) => guard(() => verdict(head(bytes, /^\s*solid/u) || bytes.length >= 84, "ASCII solid or binary header")),
  obj: (bytes) => guard(() => verdict(/^\s*(v|vn|vt|f|o|g|mtllib|usemtl|#)\b/mu.test(text(bytes)), "OBJ statements")),
  ply: (bytes) => verdict(ascii(bytes, 0, 3) === "ply", "ply magic"),
  gltf: (bytes) => guard(() => verdict(ascii(bytes, 0, 4) === "glTF" || (JSON.parse(text(bytes)) as { asset?: { version?: string } }).asset?.version === "2.0", "glb magic or asset.version 2.0")),
  las: (bytes) => verdict(ascii(bytes, 0, 4) === "LASF", "LASF signature"),
  step: (bytes) => guard(() => verdict(head(bytes, /ISO-10303-21;/u), "ISO-10303-21 header")),
  ifc: (bytes) => guard(() => verdict(head(bytes, /ISO-10303-21;/u), "ISO-10303-21 header")),
  dxf: (bytes) => guard(() => verdict(/^\s*0\s*\r?\n\s*SECTION/u.test(text(bytes.slice(0, 256))), "group 0 SECTION")),
  dwg: (bytes) => verdict(ascii(bytes, 0, 2) === "AC", "AC version magic"),
  epw: (bytes) => guard(() => verdict(head(bytes, /^LOCATION,/u), "LOCATION header")),
  binary: () => verdict(true, "any octets"),
  semio: (bytes) => guard(() => (startsWith(bytes, [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]) ? verdict(true, "semio pack magic") : (text(bytes), verdict(true, "UTF-8 decodes")))),
  none: () => verdict(true, "no format"),
};

/** 🚫️ Whether an oracle exists for every name in the schema; used by the contract test. */
export const judgeBytes = (oracle: string, bytes: Uint8Array): OracleVerdict => (ORACLES[oracle] ?? (() => verdict(false, `no oracle ${oracle}`)))(bytes);
