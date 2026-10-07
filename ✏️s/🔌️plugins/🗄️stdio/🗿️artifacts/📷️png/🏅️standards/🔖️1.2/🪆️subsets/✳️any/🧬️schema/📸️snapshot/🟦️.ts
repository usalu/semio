/** 📷️ Owned PNG native samples and metadata. */
export type PngColorType="grayscale"|"rgb"|"palette"|"grayscaleAlpha"|"rgba";
export interface PngRgb {r:number;g:number;b:number;}
export type PngTransparency={colorType:"indexed";alpha:number[]}|{colorType:"grayscale";gray:number}|{colorType:"rgb";r:number;g:number;b:number};
export type PngBackground={colorType:"indexed";index:number}|{colorType:"grayscale";gray:number}|{colorType:"rgb";r:number;g:number;b:number};
export interface PngChromaticities {whiteX:number;whiteY:number;redX:number;redY:number;greenX:number;greenY:number;blueX:number;blueY:number;}
export interface PngPhysicalDims {ppuX:number;ppuY:number;unitIsMeter:boolean;}
export interface PngTimestamp {year:number;month:number;day:number;hour:number;minute:number;second:number;}
export type PngSrgbIntent="perceptual"|"relativeColorimetric"|"saturation"|"absoluteColorimetric";
export interface PngTextChunk {keyword:string;value:string;compressed:boolean;kind:"text"|"zText"|"iText";languageTag:string;translatedKeyword:string;}
export interface PngAncillaryChunk {kind:[number,number,number,number];data:number[];afterRaster:boolean;}
export interface PngImage {width:number;height:number;bitDepth:1|2|4|8|16;colorType:PngColorType;interlace:boolean;samples:number[];palette:PngRgb[]|null;transparency:PngTransparency|null;gamma:number|null;chromaticities:PngChromaticities|null;srgb:PngSrgbIntent|null;physicalDims:PngPhysicalDims|null;timestamp:PngTimestamp|null;background:PngBackground|null;textChunks:PngTextChunk[];ancillaryChunks:PngAncillaryChunk[];}
export interface PngSnapshot {schema:"stdio.png";image:PngImage;}
export class stdioPng12AnySnapshotGuardRefusal extends Error {constructor(readonly at:string,readonly why:string){super(`${at}: ${why}`);}}
const reject=(at:string,why:string):never=>{throw new stdioPng12AnySnapshotGuardRefusal(at,why);};
type Guard=(v:unknown,at:string)=>unknown;
const integer=(minimum:number,maximum:number):Guard=>(v,at)=>Number.isSafeInteger(v)&&Number(v)>=minimum&&Number(v)<=maximum?v:reject(at,"integer outside owned range");
const u8=integer(0,255),u16=integer(0,65535),u32=integer(0,4294967295);
const text:Guard=(v,at)=>typeof v==="string"?v:reject(at,"text required"),boolean:Guard=(v,at)=>typeof v==="boolean"?v:reject(at,"boolean required");
const member=(values:readonly unknown[]):Guard=>(v,at)=>values.includes(v)?v:reject(at,"undeclared member");
const nullable=(guard:Guard):Guard=>(v,at)=>v===null?null:guard(v,at);
const array=(guard:Guard,size?:number):Guard=>(v,at)=>Array.isArray(v)&&(size===undefined||v.length===size)?v.map((item,index)=>guard(item,`${at}[${index}]`)):reject(at,"invalid owned array");
const record=(fields:Record<string,Guard>):Guard=>(v,at)=>{if(v===null||typeof v!=="object"||Array.isArray(v))return reject(at,"not an object");const row=v as Record<string,unknown>;if(Object.keys(row).some(key=>!Object.hasOwn(fields,key)))return reject(at,"undeclared field");return Object.fromEntries(Object.entries(fields).map(([key,guard])=>[key,guard(row[key],`${at}.${key}`)]));};
const rgb=record({r:u8,g:u8,b:u8});
const colorRecord=(indexed:Record<string,Guard>):Guard=>(v,at)=>{const type=(v as {colorType?:unknown}|null)?.colorType;return type==="indexed"?record({colorType:member(["indexed"]),...indexed})(v,at):type==="grayscale"?record({colorType:member(["grayscale"]),gray:u16})(v,at):record({colorType:member(["rgb"]),r:u16,g:u16,b:u16})(v,at);};
const image=record({width:integer(1,4294967295),height:integer(1,4294967295),bitDepth:member([1,2,4,8,16]),colorType:member(["grayscale","rgb","palette","grayscaleAlpha","rgba"]),interlace:boolean,samples:array(u16),palette:nullable(array(rgb)),transparency:nullable(colorRecord({alpha:array(u8)})),gamma:nullable(integer(1,4294967295)),chromaticities:nullable(record({whiteX:u32,whiteY:u32,redX:u32,redY:u32,greenX:u32,greenY:u32,blueX:u32,blueY:u32})),srgb:nullable(member(["perceptual","relativeColorimetric","saturation","absoluteColorimetric"])),physicalDims:nullable(record({ppuX:u32,ppuY:u32,unitIsMeter:boolean})),timestamp:nullable(record({year:u16,month:integer(1,12),day:integer(1,31),hour:integer(0,23),minute:integer(0,59),second:integer(0,60)})),background:nullable(colorRecord({index:u8})),textChunks:array(record({keyword:text,value:text,compressed:boolean,kind:member(["text","zText","iText"]),languageTag:text,translatedKeyword:text})),ancillaryChunks:array(record({kind:array(u8,4),data:array(u8),afterRaster:boolean}))});
export function pngSamplesPerPixel(type:PngColorType):number{return{grayscale:1,rgb:3,palette:1,grayscaleAlpha:2,rgba:4}[type];}
export function validatePngImage(value:PngImage,at="$.image"):void {
 const count=value.width*value.height*pngSamplesPerPixel(value.colorType),maximum=2**value.bitDepth-1;if(!Number.isSafeInteger(count)||value.samples.length!==count||value.samples.some(sample=>sample>maximum))reject(at,"sample cardinality or precision differs");
 if(!(value.colorType==="grayscale"?[1,2,4,8,16]:value.colorType==="palette"?[1,2,4,8]:[8,16]).includes(value.bitDepth))reject(at,"invalid color depth");
 if(value.palette&&(value.palette.length===0||value.palette.length>256||["grayscale","grayscaleAlpha"].includes(value.colorType)))reject(at,"invalid palette");if(value.colorType==="palette"&&(!value.palette||value.palette.length>maximum+1||value.samples.some(sample=>sample>=value.palette!.length)))reject(at,"indexed sample has no palette entry");
 const validColor=(color:PngTransparency|PngBackground,background:boolean):boolean=>color.colorType==="indexed"?value.colorType==="palette"&&("alpha"in color?color.alpha.length>0&&color.alpha.length<=value.palette!.length:color.index<value.palette!.length):color.colorType==="grayscale"?(value.colorType==="grayscale"||background&&value.colorType==="grayscaleAlpha")&&color.gray<=maximum:(value.colorType==="rgb"||background&&value.colorType==="rgba")&&[color.r,color.g,color.b].every(v=>v<=maximum);
 if(value.transparency&&!validColor(value.transparency,false)||value.background&&!validColor(value.background,true))reject(at,"metadata differs from color profile");
 for(const t of value.textChunks){if(!t.keyword.length||[...t.keyword].length>79||[...t.keyword].some(c=>c==="\0"||c.codePointAt(0)!>255)||t.kind==="text"&&t.compressed||t.kind==="zText"&&!t.compressed||t.kind!=="iText"&&([...t.value].some(c=>c.codePointAt(0)!>255)||t.languageTag!==""||t.translatedKeyword!=="")||[...t.languageTag].some(c=>c.codePointAt(0)!>127))reject(at,"text profile differs");}
 let after=false;for(const c of value.ancillaryChunks){const kind=String.fromCharCode(...c.kind);if(!/^[a-z][A-Za-z][A-Z][A-Za-z]$/.test(kind)||["tRNS","gAMA","cHRM","sRGB","pHYs","tIME","bKGD","tEXt","zTXt","iTXt"].includes(kind)||after&&!c.afterRaster)reject(at,"invalid opaque ancillary placement");after ||=c.afterRaster;}
}
export function parsePngImage(value:unknown,at="$.image"):PngImage{const owned=image(value,at) as PngImage;validatePngImage(owned,at);return owned;}
export function parsePngSnapshot(value:unknown,at="$"):PngSnapshot{return record({schema:member(["stdio.png"]),image:(v,p)=>parsePngImage(v,p)})(value,at) as PngSnapshot;}
export function defaultPngSnapshot():PngSnapshot{return{schema:"stdio.png",image:{width:1,height:1,bitDepth:8,colorType:"rgba",interlace:false,samples:[255,255,255,255],palette:null,transparency:null,gamma:null,chromaticities:null,srgb:null,physicalDims:null,timestamp:null,background:null,textChunks:[],ancillaryChunks:[]}};}
