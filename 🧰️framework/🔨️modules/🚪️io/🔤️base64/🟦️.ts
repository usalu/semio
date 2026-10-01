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
export function base64StandardEncode(bytes:Uint8Array):string{return base64StandardEncodeControlled(bytes,{maximumOutputBytes:Number.MAX_SAFE_INTEGER,progress:()=>true});}

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
export function base64StandardDecode(encoded:string):Uint8Array{return base64StandardDecodeControlled(encoded,{maximumOutputBytes:Number.MAX_SAFE_INTEGER,progress:()=>true});}

/** 🧬️ Caller-owned output bounds and cancellation at every bounded base64 chunk. */
export interface Base64Control {
  readonly maximumOutputBytes:number;
  readonly progress:(event:Base64Progress)=>boolean;
}
/** 📍️ Byte counts for encoding and ASCII character counts for validation or decoding. */
export interface Base64Progress {
  readonly phase:"encode"|"validate"|"decode";
  readonly completed:number;
  readonly total:number;
}
const alphabet=BASE64_STANDARD_ALPHABET;
function admit(size:number,control:Base64Control):void{
  if(!Number.isSafeInteger(control.maximumOutputBytes)||control.maximumOutputBytes<0||!Number.isSafeInteger(size)||size>control.maximumOutputBytes)throw Error("intrinsic byte output limit exceeded");
}
function checkpoint(control:Base64Control,phase:Base64Progress["phase"],completed:number,total:number):void{
  if(!control.progress({phase,completed,total}))throw Error("intrinsic bytes cancelled");
}
/** 🔤️ Produces canonical RFC4648 padding with no runtime library dependency. */
export function base64StandardEncodeControlled(bytes:Uint8Array,control:Base64Control):string{
  admit(Math.ceil(bytes.length/3)*4,control);checkpoint(control,"encode",0,bytes.length);
  const chunks:string[]=[];
  for(let offset=0;offset<bytes.length;){
    const end=Math.min(offset+4095,bytes.length);let chunk="";
    while(offset<end){const a=bytes[offset++]!,hasB=offset<bytes.length,b=hasB?bytes[offset++]!:0,hasC=offset<bytes.length,c=hasC?bytes[offset++]!:0;chunk+=alphabet[a>>>2]!+alphabet[((a&3)<<4)|(b>>>4)]!+(hasB?alphabet[((b&15)<<2)|(c>>>6)]!:"=")+(hasC?alphabet[c&63]!:"=");}
    chunks.push(chunk);checkpoint(control,"encode",offset,bytes.length);
  }
  return chunks.join("");
}
/** 🔤️ Validates canonical padding and unused bits before allocating the owned result. */
export function base64StandardDecodeControlled(text:string,control:Base64Control):Uint8Array{
  if(text.length%4)throw new Base64DecodeError({kind:"invalidLength"});
  const padding=text.endsWith("==")?2:text.endsWith("=")?1:0;
  const size=text.length/4*3-padding;admit(size,control);checkpoint(control,"validate",0,text.length);
  for(let offset=0;offset<text.length;offset+=4){
    if(text[offset]==="="||text[offset+1]==="=")throw new Base64DecodeError({kind:"invalidPadding"});
    const a=sextet(text.charCodeAt(offset),offset),b=sextet(text.charCodeAt(offset+1),offset+1),pad2=text[offset+2]==="=",pad1=text[offset+3]==="=",last=offset+4===text.length;
    if(((pad1||pad2)&&!last)||(pad2&&!pad1))throw new Base64DecodeError({kind:"invalidPadding"});
    const c=pad2?0:sextet(text.charCodeAt(offset+2),offset+2);if(!pad1)sextet(text.charCodeAt(offset+3),offset+3);
    if((pad2&&(b&15)!==0)||(pad1&&!pad2&&(c&3)!==0))throw new Base64DecodeError({kind:"nonCanonicalTrailingBits"});
    if((offset+4)%4096===0||last)checkpoint(control,"validate",offset+4,text.length);
  }
  checkpoint(control,"decode",0,text.length);const bytes=new Uint8Array(size);let output=0;
  for(let offset=0;offset<text.length;offset+=4){
    const a=sextet(text.charCodeAt(offset),offset),b=sextet(text.charCodeAt(offset+1),offset+1),c=text[offset+2]==="="?0:sextet(text.charCodeAt(offset+2),offset+2),d=text[offset+3]==="="?0:sextet(text.charCodeAt(offset+3),offset+3);
    bytes[output++]=(a<<2)|(b>>>4);if(output<size)bytes[output++]=(b<<4)|(c>>>2);if(output<size)bytes[output++]=(c<<6)|d;
    if((offset+4)%4096===0||offset+4===text.length)checkpoint(control,"decode",offset+4,text.length);
  }
  return bytes;
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
