import {NativeDecodeControl} from "../../../../🔨️modules/🌱️value/🛬️decode/🟦️.ts";

/** 📨 Identity coordinates of an OS semio container. */
export interface SemioEnvelope {
  plugin: string;
  artifact: string;
  component: "dsl" | "pack" | "op" | "spr" | "cmd";
  version: number;
}

/** 🪪️ Matches all envelope coordinates before a domain codec interprets its body. */
export function matchesSemioEnvelope(envelope: SemioEnvelope, envelopeId: string, component: SemioEnvelope["component"], version: number): boolean {
  return envelopeId === `${envelope.plugin}.${envelope.artifact}` && envelope.component === component && envelope.version === version;
}

const BINARY_MAGIC=[0x89,0x53,0x45,0x4d,0x0d,0x0a,0x1a,0x0a] as const;
function* utf8Octets(text:string):Generator<number>{for(const char of text){const point=char.codePointAt(0)!;if(point<128)yield point;else if(point<2048){yield 0xc0|(point>>6);yield 0x80|(point&63);}else if(point<65536){yield 0xe0|(point>>12);yield 0x80|((point>>6)&63);yield 0x80|(point&63);}else{yield 0xf0|(point>>18);yield 0x80|((point>>12)&63);yield 0x80|((point>>6)&63);yield 0x80|(point&63);}}}
async function matchesDeclaredToken(token:Uint8Array,id:string,component:SemioEnvelope["component"],version:number,control:NativeDecodeControl):Promise<boolean>{
  if(!Number.isInteger(version)||version<0||version>65535)throw new Error("invalid envelope version");
  const suffix=`.${component} v${version}`;let expected=suffix.length;
  await control.scopedStage(async child=>{await child.beginStage(id.length);for(const char of id){const point=char.codePointAt(0)!;expected+=point<128?1:point<2048?2:point<65536?3:4;await child.advance(char.length);}});
  if(token.length!==expected)return false;
  return control.scopedStage(async child=>{await child.beginStage(expected);let position=0,pending=0;for(const piece of [id,suffix])for(const byte of utf8Octets(piece)){if(token[position++]!==byte)return false;if(++pending===256){await child.advance(pending);pending=0;}}if(pending)await child.advance(pending);return true;});
}

/** 🚦️ Admits the declared owner and returns a view into the original binary body storage. */
export async function unwrapBinaryControlled(bytes:Uint8Array,id:string,component:SemioEnvelope["component"],version:number,control:NativeDecodeControl):Promise<Uint8Array>{
  await control.checkpoint();if(bytes.length<12||BINARY_MAGIC.some((byte,index)=>byte!==bytes[index]))throw new Error("invalid binary envelope prefix");
  const length=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getUint32(8,true),end=12+length;
  if(end>bytes.length)throw new Error("truncated envelope token");
  if(!await matchesDeclaredToken(bytes.subarray(12,end),id,component,version,control))throw new Error("declared envelope identity mismatch");return bytes.subarray(end);
}

/** 🛂️ Returns the canonical body offset without owning a second text buffer. */
export async function admitTextPreambleControlled(text:string,id:string,component:SemioEnvelope["component"],version:number,control:NativeDecodeControl):Promise<number>{
  await control.checkpoint();if(!Number.isInteger(version)||version<0||version>65535)throw new Error("invalid envelope version");if(!text.startsWith("semio "))throw new Error("missing canonical envelope prefix");
  const suffix=`.${component} v${version}`,end=6+id.length+suffix.length;if(end>text.length)throw new Error("truncated envelope token");
  await control.scopedStage(async child=>{await child.beginStage(id.length+suffix.length);let position=6;for(const piece of [id,suffix])for(let start=0;start<piece.length;start+=256){const count=Math.min(256,piece.length-start);if(text.slice(position,position+count)!==piece.slice(start,start+count))throw new Error("declared envelope identity mismatch");position+=count;await child.advance(count);}});
  if(end===text.length)return end;if(text[end]==="\n")return end+1;if(text.slice(end,end+2)==="\r\n")return end+2;throw new Error("canonical preamble requires a line boundary");
}
