/** 🎧 Canonical ID3 and MPEG frame snapshot domain. */
export interface Id3Frame{id:string;flags:number;data:number[]}
export interface Id3v2Tag{majorVersion:number;minorVersion:number;flags:number;frames:Id3Frame[]}
export interface Id3v1Tag{raw:number[]}
export interface Mp3FrameHeader{mpegVersionId:number;layer:number;protectionBit:boolean;bitrateIndex:number;sampleRateIndex:number;padding:boolean;privateBit:boolean;channelMode:number;modeExtension:number;copyright:boolean;original:boolean;emphasis:number}
export interface Mp3Frame{header:Mp3FrameHeader;payload:number[]}
export interface Mp3Snapshot{
 /** 🪪️ @state artifact */ schema:string;
 /** 🏷️ @state artifact */ id3v2:Id3v2Tag|null;
 /** 🎼️ @state artifact */ frames:Mp3Frame[];
 /** 📜️ @state artifact */ id3v1:Id3v1Tag|null;
}
/** 🚫️ An owned snapshot field fails its declared schema. */
export class Mp3SnapshotRefusal extends Error{constructor(readonly at:string,readonly why:string){super(`${at}: ${why}`)}}
const refuse=(at:string,why:string):never=>{throw new Mp3SnapshotRefusal(at,why)};
const object=(value:unknown,at:string):Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as Record<string,unknown>:refuse(at,"expected an object");
const text=(value:unknown,at:string):string=>typeof value==="string"?value:refuse(at,"expected text");
const integer=(value:unknown,at:string,max:number):number=>typeof value==="number"&&Number.isSafeInteger(value)&&value>=0&&value<=max?value:refuse(at,`expected an integer in 0..${max}`);
const flag=(value:unknown,at:string):boolean=>typeof value==="boolean"?value:refuse(at,"expected boolean");
const array=<T>(value:unknown,at:string,read:(value:unknown,at:string)=>T):T[]=>{if(value===undefined)return [];if(!Array.isArray(value))return refuse(at,"expected an array");return value.map((item,index)=>read(item,`${at}[${index}]`))};
const octets=(value:unknown,at:string):number[]=>array(value,at,(value,at)=>integer(value,at,255));
/** 🏷️ Reads an ID3 frame without interpreting its intrinsic encoded octets. */
export function parseId3Frame(value:unknown,at="$"):Id3Frame{const row=object(value,at);return{id:text(row.id,`${at}.id`),flags:integer(row.flags,`${at}.flags`,65535),data:octets(row.data,`${at}.data`)}}
/** 🗂️ Reads a complete optional ID3v2 tag body. */
export function parseId3v2Tag(value:unknown,at="$"):Id3v2Tag{const row=object(value,at);return{majorVersion:integer(row.majorVersion,`${at}.majorVersion`,255),minorVersion:integer(row.minorVersion,`${at}.minorVersion`,255),flags:integer(row.flags,`${at}.flags`,255),frames:array(row.frames,`${at}.frames`,parseId3Frame)}}
/** 🧾️ Reads the intrinsic ID3v1 trailer octets. */
export function parseId3v1Tag(value:unknown,at="$"):Id3v1Tag{const row=object(value,at);return{raw:octets(row.raw,`${at}.raw`)}}
/** 🔊️ Reads every MPEG header field at its canonical native width. */
export function parseMp3FrameHeader(value:unknown,at="$"):Mp3FrameHeader{const row=object(value,at);return{mpegVersionId:integer(row.mpegVersionId,`${at}.mpegVersionId`,255),layer:integer(row.layer,`${at}.layer`,255),protectionBit:flag(row.protectionBit,`${at}.protectionBit`),bitrateIndex:integer(row.bitrateIndex,`${at}.bitrateIndex`,255),sampleRateIndex:integer(row.sampleRateIndex,`${at}.sampleRateIndex`,255),padding:flag(row.padding,`${at}.padding`),privateBit:flag(row.privateBit,`${at}.privateBit`),channelMode:integer(row.channelMode,`${at}.channelMode`,255),modeExtension:integer(row.modeExtension,`${at}.modeExtension`,255),copyright:flag(row.copyright,`${at}.copyright`),original:flag(row.original,`${at}.original`),emphasis:integer(row.emphasis,`${at}.emphasis`,255)}}
/** 🎵️ Reads an ordered MPEG audio frame body. */
export function parseMp3Frame(value:unknown,at="$"):Mp3Frame{const row=object(value,at);return{header:parseMp3FrameHeader(row.header,`${at}.header`),payload:octets(row.payload,`${at}.payload`)}}
/** 📸️ Reads the complete owned container snapshot with native optional defaults. */
export function parseMp3Snapshot(value:unknown,at="$"):Mp3Snapshot{const row=object(value,at);return{schema:text(row.schema,`${at}.schema`),id3v2:row.id3v2==null?null:parseId3v2Tag(row.id3v2,`${at}.id3v2`),frames:array(row.frames,`${at}.frames`,parseMp3Frame),id3v1:row.id3v1==null?null:parseId3v1Tag(row.id3v1,`${at}.id3v1`)}}
