/** 🪟️ Owned BMP native samples and metadata. */
export type BmpRowOrder = "bottomUp" | "topDown";
export type BmpProfile = "indexedRgb1" | "indexedRgb4" | "indexedRgb8" | "directRgb16" | "directRgb24" | "directRgb32" | "directBitfields16" | "directBitfields32";
export interface BmpPaletteEntry { b:number; g:number; r:number; reserved:number; }
export interface BmpNativeSample { red:number; green:number; blue:number; alpha:number; reserved:number; }
export type BmpPixels = { storage:"indexed"; indices:number[] } | { storage:"direct"; samples:BmpNativeSample[] };
export interface BmpRegion { x:number; y:number; width:number; height:number; }
export interface BmpColor { red:number; green:number; blue:number; alpha:number; }
export interface BmpImage { width:number; height:number; rowOrder:BmpRowOrder; profile:BmpProfile; masks:[number,number,number,number]; palette:BmpPaletteEntry[]; pixels:BmpPixels; xPixelsPerMeter:number; yPixelsPerMeter:number; colorsUsed:number; colorsImportant:number; reserved1:number; reserved2:number; opaqueGap:number[]; opaqueTrailer:number[]; }
export interface BmpSnapshot { schema:"stdio.bmp"; image:BmpImage; }
export class stdioBmpV3AnySnapshotGuardRefusal extends Error { constructor(readonly at:string,readonly why:string){super(`${at}: ${why}`);} }
const reject=(at:string,why:string):never=>{throw new stdioBmpV3AnySnapshotGuardRefusal(at,why);};
type Guard=(v:unknown,at:string)=>unknown;
const integer=(minimum:number,maximum:number):Guard=>(v,at)=>Number.isSafeInteger(v)&&Number(v)>=minimum&&Number(v)<=maximum?v:reject(at,"integer outside owned range");
const u8=integer(0,255),u16=integer(0,65535),u32=integer(0,4294967295),i32=integer(-2147483648,2147483647);
const member=(values:readonly string[]):Guard=>(v,at)=>values.includes(v as string)?v:reject(at,"undeclared member");
const array=(guard:Guard,size?:number):Guard=>(v,at)=>Array.isArray(v)&&(size===undefined||v.length===size)?v.map((item,index)=>guard(item,`${at}[${index}]`)):reject(at,"invalid owned array");
const record=(fields:Record<string,Guard>):Guard=>(v,at)=>{if(v===null||typeof v!=="object"||Array.isArray(v))return reject(at,"not an object");const row=v as Record<string,unknown>;if(Object.keys(row).some(key=>!Object.hasOwn(fields,key)))return reject(at,"undeclared field");return Object.fromEntries(Object.entries(fields).map(([key,guard])=>[key,guard(row[key],`${at}.${key}`)]));};
const paletteEntry=record({b:u8,g:u8,r:u8,reserved:u8}),nativeSample=record({red:u32,green:u32,blue:u32,alpha:u32,reserved:u32});
const pixels:Guard=(v,at)=>{const storage=(v as {storage?:unknown}|null)?.storage;return storage==="indexed"?record({storage:member(["indexed"]),indices:array(u8)})(v,at):record({storage:member(["direct"]),samples:array(nativeSample)})(v,at);};
const image=record({width:integer(0,2147483647),height:integer(0,2147483647),rowOrder:member(["bottomUp","topDown"]),profile:member(["indexedRgb1","indexedRgb4","indexedRgb8","directRgb16","directRgb24","directRgb32","directBitfields16","directBitfields32"]),masks:array(u32,4),palette:array(paletteEntry),pixels,xPixelsPerMeter:i32,yPixelsPerMeter:i32,colorsUsed:u32,colorsImportant:u32,reserved1:u16,reserved2:u16,opaqueGap:array(u8),opaqueTrailer:array(u8)});
export function bmpBitsPerPixel(profile:BmpProfile):number{return Number(profile.match(/\d+$/)![0]);}
export function bmpMaskShift(mask:number):number{if(mask===0)return 0;let shift=0;while(mask%2===0){mask/=2;shift++;}return shift;}
export function bmpMaskMaximum(mask:number):number{return mask/2**bmpMaskShift(mask);}
export function validateBmpImage(value:BmpImage,at="$.image"):void {
 if((value.width===0)!==(value.height===0))reject(at,"dimensions disagree");const count=value.width*value.height;if(!Number.isSafeInteger(count))reject(at,"sample count overflow");const depth=bmpBitsPerPixel(value.profile);
 if(value.profile.startsWith("indexed")){if(value.pixels.storage!=="indexed"||value.masks.some(Boolean)||value.palette.length===0||value.palette.length>2**depth||value.pixels.indices.length!==count||value.pixels.indices.some(index=>index>=value.palette.length)||(value.colorsUsed===0?2**depth:value.colorsUsed)!==value.palette.length)reject(at,"invalid indexed ownership");return;}
 if(value.pixels.storage!=="direct")reject(at,"direct samples required");const samples=(value.pixels as {samples:BmpNativeSample[]}).samples;
 if(samples.length!==count||value.palette.length||value.masks[3]!==0)reject(at,"invalid direct ownership");
 const expected=value.profile==="directRgb16"?[31744,992,31,0]:value.profile==="directRgb24"||value.profile==="directRgb32"?[16711680,65280,255,0]:null;if(expected&&expected.some((mask,index)=>mask!==value.masks[index]))reject(at,"profile masks differ");
 const valid=depth===32?4294967295:2**depth-1;let assigned=0;for(const[index,mask]of value.masks.entries()){const maximum=bmpMaskMaximum(mask);if(index<3&&mask===0||((mask&~valid)>>>0)!==0||(mask&assigned)!==0||maximum!==0&&(BigInt(maximum)&BigInt(maximum+1))!==0n)reject(at,"invalid channel mask");assigned=(assigned|mask)>>>0;}
 for(const sample of samples){if([sample.red,sample.green,sample.blue,sample.alpha].some((component,index)=>component>bmpMaskMaximum(value.masks[index]!))||(sample.reserved&assigned)!==0||((sample.reserved&~valid)>>>0)!==0)reject(at,"sample exceeds native precision");}
}
export function parseBmpImage(value:unknown,at="$.image"):BmpImage{const owned=image(value,at) as BmpImage;validateBmpImage(owned,at);return owned;}
export function parseBmpSnapshot(value:unknown,at="$"):BmpSnapshot{return record({schema:member(["stdio.bmp"]),image:(v,p)=>parseBmpImage(v,p)})(value,at) as BmpSnapshot;}
export function defaultBmpSnapshot():BmpSnapshot{return{schema:"stdio.bmp",image:{width:1,height:1,rowOrder:"bottomUp",profile:"directRgb24",masks:[16711680,65280,255,0],palette:[],pixels:{storage:"direct",samples:[{red:255,green:255,blue:255,alpha:0,reserved:0}]},xPixelsPerMeter:0,yPixelsPerMeter:0,colorsUsed:0,colorsImportant:0,reserved1:0,reserved2:0,opaqueGap:[],opaqueTrailer:[]}};}
