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

/** 🧮️ Validates one standard-alphabet quartet and writes at most three bytes without allocation. */
function standardQuadValue(first:number,second:number,third:number,fourth:number,index:number,last:boolean):number {
  if(first===61||second===61)throw new Base64DecodeError({kind:"invalidPadding"});
  const a=sextet(first,index),b=sextet(second,index+1),pad2=third===61,pad1=fourth===61;
  if((pad1||pad2)&&!last||pad2&&!pad1)throw new Base64DecodeError({kind:"invalidPadding"});
  const c=pad2?0:sextet(third,index+2),d=pad1?0:sextet(fourth,index+3);
  if(pad2&&(b&15)!==0||pad1&&!pad2&&(c&3)!==0)throw new Base64DecodeError({kind:"nonCanonicalTrailingBits"});
  const count=pad2?1:pad1?2:3;
  return (count<<24)|((a<<2)|(b>>>4))|((((b<<4)|(c>>>2))&255)<<8)|((((c<<6)|d)&255)<<16);
}

/** 🧮️ Validates a quartet before writing to caller storage, using only one packed scalar cell. */
export function decodeBase64Quad(quad:ArrayLike<number>,index:number,last:boolean,output:Uint8Array,at=0):number {
  if(quad.length!==4)throw new Base64DecodeError({kind:"invalidLength"});
  const value=standardQuadValue(quad[0]!,quad[1]!,quad[2]!,quad[3]!,index,last),count=value>>>24;
  if(!(output instanceof Uint8Array)||!Number.isSafeInteger(at)||at<0||at+count>output.length)throw new RangeError("Base64 quartet output is too small");
  output[at]=value&255;if(count>1)output[at+1]=(value>>>8)&255;if(count>2)output[at+2]=(value>>>16)&255;
  return count;
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
  const cursor=new Base64DecodeCursor(text);
  while(cursor.step(cursor.nextWorkDemand(),control)!=="complete"){}
  return cursor.takeOutput()!;
}

/** 🫴️ Withdrawal transfers the original buffer with its valid prefix and first refusal. */
export interface Base64DecodeParts {
  readonly input:string;
  readonly range:Base64InputRange;
  readonly output:Uint8Array|null;
  readonly written:number;
  readonly outcome:Base64DecodeOutcome;
}

/** 🚪️ The tag preserves even a callback refusal whose original value is null or undefined. */
export type Base64DecodeOutcome={readonly kind:"pending"}|{readonly kind:"complete"}|{readonly kind:"refused";readonly refusal:unknown};

/** 📐️ Half-open UTF-16 storage indices select a payload without slicing original text. */
export interface Base64InputRange {readonly start:number;readonly end:number;}

/** 🪜️ Immutable input and exact phase positions survive bounded event-loop turns without payload copies. */
export class Base64DecodeCursor {
  private output:Uint8Array|null=null;
  private written=0;
  private offset=0;
  private phase:"validate"|"decode"="validate";
  private started=false;
  private complete=false;
  private refused=false;
  private refusal:unknown=null;
  private withdrawn=false;
  private stepping=false;
  private readonly outputLength:number;
  private readonly start:number;
  private readonly end:number;
  private readonly length:number;

  constructor(private text:string,range:Base64InputRange={start:0,end:text.length}){
    this.start=range.start;this.end=range.end;
    const boundary=(at:number)=>at===0||at===text.length||!(text.charCodeAt(at)>=0xdc00&&text.charCodeAt(at)<=0xdfff&&text.charCodeAt(at-1)>=0xd800&&text.charCodeAt(at-1)<=0xdbff);
    const valid=Number.isSafeInteger(this.start)&&Number.isSafeInteger(this.end)&&this.start>=0&&this.start<=this.end&&this.end<=text.length&&boundary(this.start)&&boundary(this.end);
    this.length=valid?this.end-this.start:0;
    if(!valid){this.refused=true;this.refusal=new RangeError("base64 input range is invalid");}
    else if(this.length%4){this.refused=true;this.refusal=new Base64DecodeError({kind:"invalidLength"});}
    const padding=this.length&&text.charCodeAt(this.end-1)===61?(this.length>1&&text.charCodeAt(this.end-2)===61?2:1):0;
    this.outputLength=this.refused?0:this.length/4*3-padding;
  }
  private active():void{if(this.withdrawn)throw Error("base64 cursor ownership withdrawn");}
  private mutable():void{this.active();if(this.stepping)throw Error("base64 cursor is already stepping");}
  private checkpoint(control:Base64Control,completed:number):void{
    checkpoint(control,this.phase,completed,this.length);
    if(this.refused)throw this.refusal;
  }
  /** 📊️ Reports input units already validated or reconstructed in the current phase. */
  progress():Base64Progress{this.active();return{phase:this.phase,completed:this.offset,total:this.length};}
  /** 📏️ Admits both visits of a nonfinal validation chunk or one stage transition. */
  nextWorkDemand():number{
    this.active();if(this.complete||this.refused)return 0;
    if(!this.started||this.offset===this.length)return 1;
    const bytes=Math.min(this.length-this.offset,4096);
    return bytes+(this.phase==="validate"&&this.offset+bytes<this.length?bytes:0);
  }
  private advance(control:Base64Control):void{
    admit(this.outputLength,control);
    if(!this.started){
      this.checkpoint(control,0);
      if(this.phase==="decode")this.output=new Uint8Array(this.outputLength);
      this.started=true;
    }else if(this.offset===this.length){
      if(this.phase==="validate"){this.phase="decode";this.offset=0;this.started=false;}else this.complete=true;
    }else{
      const end=Math.min(this.offset+4096,this.length);
      if(this.phase==="validate"&&end<this.length)for(let at=this.offset;at<end;at++)if(this.text.charCodeAt(this.start+at)===61)throw new Base64DecodeError({kind:"invalidPadding"});
      for(let at=this.offset;at<end;at+=4){
        const value=standardQuadValue(this.text.charCodeAt(this.start+at),this.text.charCodeAt(this.start+at+1),this.text.charCodeAt(this.start+at+2),this.text.charCodeAt(this.start+at+3),at,at+4===this.length),count=value>>>24;
        if(this.phase==="decode"){
          this.output![this.written++]=value&255;
          if(count>1)this.output![this.written++]=(value>>>8)&255;
          if(count>2)this.output![this.written++]=(value>>>16)&255;
        }
      }
      this.offset=end;this.checkpoint(control,end);
    }
  }
  /** 🚦️ Inert undersized grants never call progress; any first thrown refusal freezes later work. */
  step(maximumWorkUnits:number,control:Base64Control):"pending"|"complete"{
    this.mutable();if(this.refused)throw this.refusal;if(this.complete)return"complete";
    if(!Number.isSafeInteger(maximumWorkUnits)||maximumWorkUnits<0){this.refused=true;this.refusal=RangeError("invalid base64 work grant");throw this.refusal;}
    let remaining=maximumWorkUnits;
    this.stepping=true;
    try{
      while(!this.complete){
        const demand=this.nextWorkDemand();if(remaining<demand)return"pending";remaining-=demand;
        try{this.advance(control);}catch(error){if(!this.refused){this.refused=true;this.refusal=error;}throw this.refusal;}
      }
      return"complete";
    }finally{this.stepping=false;}
  }
  /** 🛑️ Cancellation preserves source and partial output for explicit withdrawal. */
  cancel():boolean{this.active();if(this.complete||this.refused)return false;this.refused=true;this.refusal=Error("intrinsic bytes cancelled");return true;}
  /** ✅️ Completion is successful only after both strict passes finish. */
  isComplete():boolean{this.active();return this.complete;}
  /** 📏️ Borrows exact retained typed-array storage without transferring its owner. */
  outputCapacity():number{this.active();return this.output?.byteLength??0;}
  /** 📤️ Moves the original result once after complete reconstruction. */
  takeOutput():Uint8Array|null{this.mutable();if(!this.complete||this.refused)return null;const output=this.output;this.output=null;return output;}
  /** 🫴️ Consumes the handle and transfers the original typed array without slicing its prefix. */
  intoParts():Base64DecodeParts{
    this.mutable();const outcome:Base64DecodeOutcome=this.refused?{kind:"refused",refusal:this.refusal}:this.complete?{kind:"complete"}:{kind:"pending"};
    const parts={input:this.text,range:{start:this.start,end:this.end},output:this.output,written:this.written,outcome};
    this.text="";this.output=null;this.refusal=null;this.withdrawn=true;return parts;
  }
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
