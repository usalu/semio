/** 🎧 Canonical ID3 and MPEG frame snapshot domain. */
export type Id3Content=
 |{kind:"text";values:string[]}|{kind:"userText";description:string;values:string[]}
 |{kind:"comment"|"lyrics";language:string;description:string;text:string}
 |{kind:"url";url:string}|{kind:"userUrl";description:string;url:string}
 |{kind:"picture";mime:string;pictureType:number;description:string;payload:number[]}
 |{kind:"opaque";bytes:number[]};
export interface Id3Frame{id:string;content:Id3Content}
export interface Id3v2Tag{frames:Id3Frame[]}
export interface Id3v1Tag{title:string;artist:string;album:string;year:string;comment:string;track:number|null;genre:number|null}
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
const exact=(row:Record<string,unknown>,keys:string[],at:string):void=>{if(Object.keys(row).some(key=>!keys.includes(key)))refuse(at,"undeclared field")};
const clean=(value:unknown,at:string):string=>{const result=text(value,at);if(result.includes("\0"))refuse(at,"embedded NUL");return result};
export function id3ContentKind(id:string):string{if(id==="TXXX")return "userText";if(id==="COMM")return "comment";if(id==="USLT")return "lyrics";if(id==="WXXX")return "userUrl";if(id==="APIC")return "picture";if(["CHAP","CTOC","AENC","ASPI","COMR","ENCR","EQU2","EQUA","ETCO","GEOB","GRID","IPLS","LINK","MCDI","MLLT","OWNE","PCNT","POPM","POSS","PRIV","RBUF","RVAD","RVA2","RVRB","SEEK","SIGN","SYLT","SYTC","UFID","USER"].includes(id))return "unsupported";return id.startsWith("T")?"text":id.startsWith("W")?"url":"opaque"}
/** 🏷️ Admits an ID3 content variant bound to its semantic frame identifier. */
export function parseId3Frame(value:unknown,at="$"):Id3Frame{
 const row=object(value,at);exact(row,["id","content"],at);const id=text(row.id,at+".id");if(!/^[A-Z0-9]{4}$/.test(id))refuse(at,"frame identifier");const c=object(row.content,at+".content");const kind=text(c.kind,at+".content.kind");if(kind!==id3ContentKind(id))refuse(at,"identifier/content mismatch");
 const t=(key:string)=>clean(c[key],at+".content."+key);const values=()=>{if(!Array.isArray(c.values)||c.values.length===0)refuse(at,"nonempty text values required");return array(c.values,at+".content.values",clean)};
 let content:Id3Content;
 if(kind==="text"){exact(c,["kind","values"],at);content={kind,values:values()}}
 else if(kind==="userText"){exact(c,["kind","description","values"],at);content={kind,description:t("description"),values:values()}}
 else if(kind==="comment"||kind==="lyrics"){exact(c,["kind","language","description","text"],at);const language=t("language");if(!/^[a-z]{3}$/.test(language))refuse(at,"language code");content={kind,language,description:t("description"),text:t("text")}}
 else if(kind==="url"||kind==="userUrl"){exact(c,kind==="url"?["kind","url"]:["kind","url","description"],at);const url=t("url");if([...url].some(c=>c.codePointAt(0)!>255))refuse(at,"URL character");content=kind==="url"?{kind,url}:{kind,url,description:t("description")}}
 else if(kind==="picture"){exact(c,["kind","mime","pictureType","description","payload"],at);const mime=t("mime");if(!mime||!/^[\x01-\x7f]+$/.test(mime))refuse(at,"MIME");content={kind,mime,pictureType:integer(c.pictureType,at,20),description:t("description"),payload:Array.isArray(c.payload)?octets(c.payload,at):refuse(at,"required picture payload")}}
 else if(kind==="opaque"){exact(c,["kind","bytes"],at);content={kind,bytes:Array.isArray(c.bytes)?octets(c.bytes,at):refuse(at,"required opaque content")}}else return refuse(at,"unsupported known frame");return{id,content};
}
/** 🗂️ Admits ordered semantic ID3v2 contents independently of native version/flags. */
export function parseId3v2Tag(value:unknown,at="$"):Id3v2Tag{const row=object(value,at);exact(row,["frames"],at);if(!Array.isArray(row.frames))refuse(at,"required frames");return{frames:array(row.frames,at+".frames",parseId3Frame)}}
/** 🧾️ Admits named ID3v1 metadata independently of markers and fixed-width padding. */
export function parseId3v1Tag(value:unknown,at="$"):Id3v1Tag{
 const row=object(value,at);exact(row,["title","artist","album","year","comment","track","genre"],at);const track=row.track===null?null:integer(row.track,at+".track",255);if(track===0)refuse(at,"track ordinal");const genre=row.genre===null?null:integer(row.genre,at+".genre",254);
 const read=(key:string,width:number)=>{const value=clean(row[key],at+"."+key);if([...value].length>width||value.endsWith(" ")||[...value].some(c=>c.codePointAt(0)!>255))refuse(at,"ID3v1 field capacity");return value};return{title:read("title",30),artist:read("artist",30),album:read("album",30),year:read("year",4),comment:read("comment",track===null?30:28),track,genre};
}
/** 🔊️ Reads every MPEG header field at its canonical native width. */
export function parseMp3FrameHeader(value:unknown,at="$"):Mp3FrameHeader{const row=object(value,at);return{mpegVersionId:integer(row.mpegVersionId,`${at}.mpegVersionId`,255),layer:integer(row.layer,`${at}.layer`,255),protectionBit:flag(row.protectionBit,`${at}.protectionBit`),bitrateIndex:integer(row.bitrateIndex,`${at}.bitrateIndex`,255),sampleRateIndex:integer(row.sampleRateIndex,`${at}.sampleRateIndex`,255),padding:flag(row.padding,`${at}.padding`),privateBit:flag(row.privateBit,`${at}.privateBit`),channelMode:integer(row.channelMode,`${at}.channelMode`,255),modeExtension:integer(row.modeExtension,`${at}.modeExtension`,255),copyright:flag(row.copyright,`${at}.copyright`),original:flag(row.original,`${at}.original`),emphasis:integer(row.emphasis,`${at}.emphasis`,255)}}
/** 🎵️ Reads an ordered MPEG audio frame body. */
export function parseMp3Frame(value:unknown,at="$"):Mp3Frame{const row=object(value,at);return{header:parseMp3FrameHeader(row.header,`${at}.header`),payload:octets(row.payload,`${at}.payload`)}}
/** 📸️ Reads the complete owned container snapshot with native optional defaults. */
export function parseMp3Snapshot(value:unknown,at="$"):Mp3Snapshot{const row=object(value,at);return{schema:text(row.schema,`${at}.schema`),id3v2:row.id3v2==null?null:parseId3v2Tag(row.id3v2,`${at}.id3v2`),frames:array(row.frames,`${at}.frames`,parseMp3Frame),id3v1:row.id3v1==null?null:parseId3v1Tag(row.id3v1,`${at}.id3v1`)}}
