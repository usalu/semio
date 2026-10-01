/** 🧬️ AviSnapshot — RIFF/AVI 1.0. Mirrors 🦀️.rs field-for-field. */
export interface AviMainHeader {
  microSecPerFrame: number; maxBytesPerSec: number; paddingGranularity: number; flags: number;
  totalFrames: number; initialFrames: number; streams: number; suggestedBufferSize: number;
  width: number; height: number; reserved: number[];
}
export interface AviStreamHeader {
  fccType: string; fccHandler: string; flags: number; priority: number; language: number;
  initialFrames: number; scale: number; rate: number; start: number; length: number;
  suggestedBufferSize: number; quality: number; sampleSize: number;
  rcFrameLeft: number; rcFrameTop: number; rcFrameRight: number; rcFrameBottom: number;
  rcFrameWidth: number; strhExtra: number[];
}
export type AviStreamFormat =
  | { format: "bitmapInfo"; size: number; width: number; height: number; planes: number; bitCount: number; compression: string; sizeImage: number; xPelsPerMeter: number; yPelsPerMeter: number; colorsUsed: number; colorsImportant: number }
  | { format: "waveFormat"; formatTag: number; channels: number; samplesPerSec: number; avgBytesPerSec: number; blockAlign: number; bitsPerSample: number; extra: number[] }
  | { format: "raw"; data: number[] };
export interface AviChunk { fourcc: string; data: number[]; keyframe: boolean; }
export interface AviStream { strh: AviStreamHeader; strf: AviStreamFormat; chunks: AviChunk[]; strlExtra:RiffChunk[]; }
export interface RiffChunk { fourcc: string; data: number[]; }
export interface AviSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ mainHeader: AviMainHeader;
  /** @state artifact */ streams: AviStream[];
  /** @state artifact */ idx1Present: boolean;
  /** @state artifact */ unknownChunks: RiffChunk[];
  /** 📦️ @state artifact */ hdrlExtra: RiffChunk[];
}

function object(value:unknown):Readonly<Record<string,unknown>>{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("AVI requires an object");return value as Readonly<Record<string,unknown>>}
function text(value:unknown):string{if(typeof value!=="string")throw Error("AVI requires text");return value}
function integer(value:unknown,min:number,max:number):number{if(typeof value!=="number"||!Number.isInteger(value)||value<min||value>max)throw Error("AVI integer exceeds its native width");return value}
function u32(value:unknown):number{return integer(value,0,4294967295)}
function u16(value:unknown):number{return integer(value,0,65535)}
function i32(value:unknown):number{return integer(value,-2147483648,2147483647)}
function boolean(value:unknown):boolean{if(typeof value!=="boolean")throw Error("AVI requires boolean state");return value}
function list<T>(value:unknown,parse:(value:unknown)=>T):T[]{if(!Array.isArray(value))throw Error("AVI requires an ordered list");return value.map(parse)}
function bytes(value:unknown):number[]{return list(value,byte=>integer(byte,0,255))}
/** 🏷️ Admits every main-header field and ordered reserved DWORD. */
export function parseAviMainHeader(value:unknown):AviMainHeader{const r=object(value);return{microSecPerFrame:u32(r.microSecPerFrame),maxBytesPerSec:u32(r.maxBytesPerSec),paddingGranularity:u32(r.paddingGranularity),flags:u32(r.flags),totalFrames:u32(r.totalFrames),initialFrames:u32(r.initialFrames),streams:u32(r.streams),suggestedBufferSize:u32(r.suggestedBufferSize),width:u32(r.width),height:u32(r.height),reserved:list(r.reserved,u32)}}
/** 📏️ Admits the complete native stream header and retained tail. */
export function parseAviStreamHeader(value:unknown):AviStreamHeader{const r=object(value);return{fccType:text(r.fccType),fccHandler:text(r.fccHandler),flags:u32(r.flags),priority:u16(r.priority),language:u16(r.language),initialFrames:u32(r.initialFrames),scale:u32(r.scale),rate:u32(r.rate),start:u32(r.start),length:u32(r.length),suggestedBufferSize:u32(r.suggestedBufferSize),quality:i32(r.quality),sampleSize:u32(r.sampleSize),rcFrameLeft:i32(r.rcFrameLeft),rcFrameTop:i32(r.rcFrameTop),rcFrameRight:i32(r.rcFrameRight),rcFrameBottom:i32(r.rcFrameBottom),rcFrameWidth:integer(r.rcFrameWidth,0,255),strhExtra:bytes(r.strhExtra)}}
/** 🎨️ Admits each native stream-format choice without wire restrictions. */
export function parseAviStreamFormat(value:unknown):AviStreamFormat{const r=object(value);switch(r.format){case"bitmapInfo":return{format:"bitmapInfo",size:u32(r.size),width:i32(r.width),height:i32(r.height),planes:u16(r.planes),bitCount:u16(r.bitCount),compression:text(r.compression),sizeImage:u32(r.sizeImage),xPelsPerMeter:i32(r.xPelsPerMeter),yPelsPerMeter:i32(r.yPelsPerMeter),colorsUsed:u32(r.colorsUsed),colorsImportant:u32(r.colorsImportant)};case"waveFormat":return{format:"waveFormat",formatTag:u16(r.formatTag),channels:u16(r.channels),samplesPerSec:u32(r.samplesPerSec),avgBytesPerSec:u32(r.avgBytesPerSec),blockAlign:u16(r.blockAlign),bitsPerSample:u16(r.bitsPerSample),extra:bytes(r.extra)};case"raw":return{format:"raw",data:bytes(r.data)};default:throw Error("AVI stream format is unknown")}}
/** 🎞️ Admits retained chunk bytes and exact keyframe state. */
export function parseAviChunk(value:unknown):AviChunk{const r=object(value);return{fourcc:text(r.fourcc),data:bytes(r.data),keyframe:boolean(r.keyframe)}}
/** 📎️ Admits one intrinsic RIFF child without source-format restrictions. */
export function parseRiffChunk(value:unknown):RiffChunk{const r=object(value);return{fourcc:text(r.fourcc),data:bytes(r.data)}}
/** 🎬️ Admits one full stream and its ordered retained children. */
export function parseAviStream(value:unknown):AviStream{const r=object(value);return{strh:parseAviStreamHeader(r.strh),strf:parseAviStreamFormat(r.strf),chunks:list(r.chunks,parseAviChunk),strlExtra:list(r.strlExtra,parseRiffChunk)}}
/** 🚪️ Admits the full canonical AVI snapshot domain. */
export function parseAviSnapshot(value:unknown):AviSnapshot{const r=object(value);return{schema:text(r.schema),mainHeader:parseAviMainHeader(r.mainHeader),streams:list(r.streams,parseAviStream),idx1Present:boolean(r.idx1Present),unknownChunks:list(r.unknownChunks,parseRiffChunk),hdrlExtra:list(r.hdrlExtra,parseRiffChunk)}}
