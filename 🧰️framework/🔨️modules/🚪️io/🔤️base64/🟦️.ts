/** 🔤️ TypeScript twin of this module's Rust codec (`🦀️.rs`): strict RFC 4648 §4 standard-alphabet,
 * padded base64 (plus §5 unpadded base64url below), implemented here rather than borrowed from `atob`/`Buffer` so both halves of the
 * repo refuse exactly the same malformed input. `atob` silently accepts unpadded groups and
 * non-canonical trailing bits that `base64_standard_decode` rejects, and a boundary whose two
 * implementations disagree on what a valid export is has no law at all.
 *
 * Both halves drive the same vectors: `🧫️fixtures/🔣️rfc4648-base64-vectors.json`.
 * @see https://www.rfc-editor.org/rfc/rfc4648#section-4
 */

const BASE64_STANDARD_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/** 🔤️ Strict RFC 4648 standard-base64 decoding failure — the TS mirror of Rust's `Base64Error`. */
export type Base64Error =
  | { readonly kind: "invalidLength" }
  | { readonly kind: "invalidByte"; readonly index: number; readonly byte: number }
  | { readonly kind: "invalidPadding" }
  | { readonly kind: "nonCanonicalTrailingBits" };

/** 🚨️ The thrown carrier of a {@link Base64Error} — TS has no `Result`, and a silent fallback is
 * exactly the failure mode this module exists to remove. */
export class Base64DecodeError extends Error {
  readonly detail: Base64Error;
  constructor(detail: Base64Error) {
    super(base64ErrorMessage(detail));
    this.name = "Base64DecodeError";
    this.detail = detail;
  }
}

/** 🗣️ One message per failure shape, worded exactly as the Rust `Display` impl words it. */
export function base64ErrorMessage(error: Base64Error): string {
  switch (error.kind) {
    case "invalidLength":
      return "base64 length must be a multiple of four";
    case "invalidByte":
      return `invalid base64 byte 0x${error.byte.toString(16).padStart(2, "0")} at index ${error.index}`;
    case "invalidPadding":
      return "invalid base64 padding";
    case "nonCanonicalTrailingBits":
      return "non-canonical base64 trailing bits";
  }
}

/** 🔤️ Encodes bytes with the padded RFC 4648 standard alphabet. */
export function base64StandardEncode(bytes: Uint8Array): string {
  let encoded = "";
  for (let offset = 0; offset < bytes.length; offset += 3) {
    const remaining = bytes.length - offset;
    const first = bytes[offset] as number;
    const second = remaining >= 2 ? (bytes[offset + 1] as number) : 0;
    const third = remaining >= 3 ? (bytes[offset + 2] as number) : 0;
    encoded += BASE64_STANDARD_ALPHABET[first >> 2];
    encoded += BASE64_STANDARD_ALPHABET[((first & 0x03) << 4) | (second >> 4)];
    encoded += remaining >= 2 ? BASE64_STANDARD_ALPHABET[((second & 0x0f) << 2) | (third >> 6)] : "=";
    encoded += remaining >= 3 ? BASE64_STANDARD_ALPHABET[third & 0x3f] : "=";
  }
  return encoded;
}

function sextet(byte: number, index: number): number {
  if (byte >= 0x41 && byte <= 0x5a) return byte - 0x41;
  if (byte >= 0x61 && byte <= 0x7a) return byte - 0x61 + 26;
  if (byte >= 0x30 && byte <= 0x39) return byte - 0x30 + 52;
  if (byte === 0x2b) return 62;
  if (byte === 0x2f) return 63;
  throw new Base64DecodeError({ kind: "invalidByte", index, byte });
}

/** 🔤️ Decodes padded RFC 4648 standard base64 and rejects whitespace, misplaced padding and
 * non-canonical unused bits — throwing {@link Base64DecodeError}, never returning partial bytes. */
export function base64StandardDecode(encoded: string): Uint8Array {
  const source = new Uint8Array(encoded.length);
  for (let index = 0; index < encoded.length; index += 1) source[index] = encoded.charCodeAt(index) & 0xff;
  if (source.length % 4 !== 0) throw new Base64DecodeError({ kind: "invalidLength" });
  const decoded: number[] = [];
  for (let offset = 0; offset < source.length; offset += 4) {
    const last = offset + 4 === source.length;
    const chunk = [source[offset] as number, source[offset + 1] as number, source[offset + 2] as number, source[offset + 3] as number];
    if (chunk[0] === 0x3d || chunk[1] === 0x3d) throw new Base64DecodeError({ kind: "invalidPadding" });
    const first = sextet(chunk[0] as number, offset);
    const second = sextet(chunk[1] as number, offset + 1);
    decoded.push(((first << 2) | (second >> 4)) & 0xff);
    if (chunk[2] === 0x3d) {
      if (!last || chunk[3] !== 0x3d) throw new Base64DecodeError({ kind: "invalidPadding" });
      if ((second & 0x0f) !== 0) throw new Base64DecodeError({ kind: "nonCanonicalTrailingBits" });
      continue;
    }
    const third = sextet(chunk[2] as number, offset + 2);
    decoded.push(((second << 4) | (third >> 2)) & 0xff);
    if (chunk[3] === 0x3d) {
      if (!last) throw new Base64DecodeError({ kind: "invalidPadding" });
      if ((third & 0x03) !== 0) throw new Base64DecodeError({ kind: "nonCanonicalTrailingBits" });
      continue;
    }
    const fourth = sextet(chunk[3] as number, offset + 3);
    decoded.push(((third << 6) | fourth) & 0xff);
  }
  return Uint8Array.from(decoded);
}

const BASE64_URL_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/** 🔗️ Encodes bytes with the unpadded RFC 4648 §5 URL-safe alphabet — the twin of Rust `base64_url_encode`,
 * pinned by `🧫️fixtures/🔣️rfc4648-base64url-vectors.json`.
 * @see https://www.rfc-editor.org/rfc/rfc4648#section-5 */
export function base64UrlEncode(bytes: Uint8Array): string {
  let encoded = "";
  for (let offset = 0; offset < bytes.length; offset += 3) {
    const remaining = bytes.length - offset;
    const first = bytes[offset] as number;
    const second = remaining >= 2 ? (bytes[offset + 1] as number) : 0;
    const third = remaining >= 3 ? (bytes[offset + 2] as number) : 0;
    encoded += BASE64_URL_ALPHABET[first >> 2];
    encoded += BASE64_URL_ALPHABET[((first & 0x03) << 4) | (second >> 4)];
    if (remaining >= 2) encoded += BASE64_URL_ALPHABET[((second & 0x0f) << 2) | (third >> 6)];
    if (remaining >= 3) encoded += BASE64_URL_ALPHABET[third & 0x3f];
  }
  return encoded;
}

function urlSextet(byte: number, index: number): number {
  if (byte >= 0x41 && byte <= 0x5a) return byte - 0x41;
  if (byte >= 0x61 && byte <= 0x7a) return byte - 0x61 + 26;
  if (byte >= 0x30 && byte <= 0x39) return byte - 0x30 + 52;
  if (byte === 0x2d) return 62;
  if (byte === 0x5f) return 63;
  throw new Base64DecodeError({ kind: "invalidByte", index, byte });
}

/** 🔗️ Decodes unpadded RFC 4648 §5 base64url, rejecting padding, the standard-only `+`/`/`, a dangling
 * sextet and non-canonical unused bits — the twin of Rust `base64_url_decode`. */
export function base64UrlDecode(encoded: string): Uint8Array {
  if (encoded.length % 4 === 1) throw new Base64DecodeError({ kind: "invalidLength" });
  const decoded = new Uint8Array(Math.floor((encoded.length * 3) / 4));
  let written = 0;
  for (let offset = 0; offset < encoded.length; offset += 4) {
    const width = Math.min(4, encoded.length - offset);
    const a = urlSextet(encoded.charCodeAt(offset), offset);
    const b = urlSextet(encoded.charCodeAt(offset + 1), offset + 1);
    decoded[written++] = ((a << 2) | (b >> 4)) & 0xff;
    if (width === 2) {
      if ((b & 0x0f) !== 0) throw new Base64DecodeError({ kind: "nonCanonicalTrailingBits" });
      continue;
    }
    const c = urlSextet(encoded.charCodeAt(offset + 2), offset + 2);
    decoded[written++] = ((b << 4) | (c >> 2)) & 0xff;
    if (width === 3) {
      if ((c & 0x03) !== 0) throw new Base64DecodeError({ kind: "nonCanonicalTrailingBits" });
      continue;
    }
    const d = urlSextet(encoded.charCodeAt(offset + 3), offset + 3);
    decoded[written++] = ((c << 6) | d) & 0xff;
  }
  return decoded;
}
