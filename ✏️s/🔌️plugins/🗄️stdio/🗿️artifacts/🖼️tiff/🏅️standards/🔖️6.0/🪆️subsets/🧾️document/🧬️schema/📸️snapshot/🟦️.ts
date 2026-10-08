/** 🧬️ Exact owned TIFF sample words and typed metadata. */
export type TiffFieldType='byte'|'ascii'|'short'|'long'|'rational'|'sByte'|'undefined'|'sShort'|'sLong'|'sRational'|'float'|'double';
export interface TiffWord64{lo:number;hi:number}
export interface TiffBinary32{bits:number}
export type TiffValues=
 |{kind:'byte'|'short'|'long'|'sByte'|'undefined'|'sShort'|'sLong';value:number[]}
 |{kind:'ascii';value:string[]}
 |{kind:'rational'|'sRational';value:[number,number][]}
 |{kind:'float';value:TiffBinary32[]}
 |{kind:'double';value:TiffWord64[]};
export interface TiffTag{tag:number;values:TiffValues}
export interface TiffSampleBlock{x:number;y:number;width:number;height:number;channels:number;samples:TiffWord64[]}
export interface TiffIfd{entries:TiffTag[];blocks:TiffSampleBlock[]}
export interface TiffSnapshot{schema:'stdio.tiff';ifds:TiffIfd[]}
//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentSnapshotGuardRefusal(at, why);
};

type stdioTiff60DocumentSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentSnapshotGuardReject(at, "value is not an object");
export const stdioTiff60DocumentSnapshotGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentSnapshotGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentSnapshotGuardNumber(value, at, bounds) : stdioTiff60DocumentSnapshotGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers


function exact(value:unknown,keys:readonly string[],at:string):Readonly<Record<string,unknown>>{const row=stdioTiff60DocumentSnapshotGuardObject(value,at);if(Object.keys(row).length!==keys.length||Object.keys(row).some(key=>!keys.includes(key)))return stdioTiff60DocumentSnapshotGuardReject(at,"foreign or missing owned fields");return row}
const integer=(value:unknown,at:string,min=0,max=4294967295)=>stdioTiff60DocumentSnapshotGuardInteger(value,at,{minimum:min,maximum:max});
const sequence=stdioTiff60DocumentSnapshotGuardArray;
export function parseTiffFieldType(value:unknown,at="$"):TiffFieldType{return stdioTiff60DocumentSnapshotGuardMember(value,at,["byte","ascii","short","long","rational","sByte","undefined","sShort","sLong","sRational","float","double"])}
export function parseTiffWord64(value:unknown,at="$"):TiffWord64{const row=exact(value,["lo","hi"],at);return {lo:integer(row.lo,at+".lo"),hi:integer(row.hi,at+".hi")}}
export function parseTiffValues(value:unknown,at="$"):TiffValues{
 const row=exact(value,["kind","value"],at),kind=parseTiffFieldType(row.kind,at+".kind"),values=sequence(row.value,at+".value");
 const ints=(min:number,max:number)=>values.map((value,i)=>integer(value,at+".value["+i+"]",min,max));
 switch(kind){
 case "ascii":return {kind,value:values.map((value,i)=>stdioTiff60DocumentSnapshotGuardString(value,at+".value["+i+"]",{pattern:"^[\\u0001-\\u007f]*$"}))};
 case "float":return {kind,value:values.map((value,i)=>{const v=exact(value,["bits"],at+".value["+i+"]");return {bits:integer(v.bits,at+".value["+i+"].bits")}})};
 case "double":return {kind,value:values.map((value,i)=>parseTiffWord64(value,at+".value["+i+"]"))};
 case "rational":case "sRational":return {kind,value:values.map((value,i)=>{const pair=sequence(value,at+".value["+i+"]",{minItems:2,maxItems:2});const min=kind==="rational"?0:-2147483648,max=kind==="rational"?4294967295:2147483647;return [integer(pair[0],at,min,max),integer(pair[1],at,min,max)]})};
 case "byte":case "undefined":return {kind,value:ints(0,255)};
 case "short":return {kind,value:ints(0,65535)};
 case "long":return {kind,value:ints(0,4294967295)};
 case "sByte":return {kind,value:ints(-128,127)};
 case "sShort":return {kind,value:ints(-32768,32767)};
 case "sLong":return {kind,value:ints(-2147483648,2147483647)};
 }
}
export function parseTiffTag(value:unknown,at="$"):TiffTag{const row=exact(value,["tag","values"],at);const tag=integer(row.tag,at+".tag",0,65535);if([259,266,273,278,279,284,317,322,323,324,325,292,293,347,512,513,514,515,517,518,519,520,521,530].includes(tag))return stdioTiff60DocumentSnapshotGuardReject(at+".tag","native layout policy has no semantic tag");return {tag,values:parseTiffValues(row.values,at+".values")}}
export function parseTiffSampleBlock(value:unknown,at="$"):TiffSampleBlock{const row=exact(value,["x","y","width","height","channels","samples"],at);const block={x:integer(row.x,at+".x"),y:integer(row.y,at+".y"),width:integer(row.width,at+".width",1),height:integer(row.height,at+".height",1),channels:integer(row.channels,at+".channels",1,65535),samples:sequence(row.samples,at+".samples").map((v,i)=>parseTiffWord64(v,at+".samples["+i+"]"))};if(BigInt(block.width)*BigInt(block.height)*BigInt(block.channels)!==BigInt(block.samples.length))return stdioTiff60DocumentSnapshotGuardReject(at,"block sample cardinality differs");return block}
export function tiffIntegers(page:TiffIfd,tag:number):number[]{const value=page.entries.find(v=>v.tag===tag)?.values;return value&&["byte","short","long"].includes(value.kind)?(value.value as number[]):[]}
export function parseTiffIfd(value:unknown,at="$"):TiffIfd{
 const row=exact(value,["entries","blocks"],at),page={entries:sequence(row.entries,at+".entries").map((v,i)=>parseTiffTag(v,at+".entries["+i+"]")),blocks:sequence(row.blocks,at+".blocks").map((v,i)=>parseTiffSampleBlock(v,at+".blocks["+i+"]"))};
 validateTiffIfd(page,at);return page;
}
export function validateTiffIfd(page:TiffIfd,at="$"):void{
 exact(page,["entries","blocks"],at);sequence(page.entries,at+".entries");sequence(page.blocks,at+".blocks");
 for(let i=0;i<page.entries.length;i++){
  const tag=page.entries[i]!,path=at+".entries["+i+"]";exact(tag,["tag","values"],path);integer(tag.tag,path+".tag",0,65535);
  if([259,266,273,278,279,284,317,322,323,324,325,292,293,347,512,513,514,515,517,518,519,520,521,530].includes(tag.tag))return stdioTiff60DocumentSnapshotGuardReject(path,"native layout policy has no semantic tag");
  exact(tag.values,["kind","value"],path+".values");const kind=parseTiffFieldType(tag.values.kind,path+".values.kind"),values=sequence(tag.values.value,path+".values.value");
  for(let j=0;j<values.length;j++){const value=values[j],position=path+".values.value["+j+"]";
   if(kind==='ascii'){if(typeof value!=='string')stdioTiff60DocumentSnapshotGuardReject(position,"ASCII value is not text");for(let n=0;n<(value as string).length;n++){const code=(value as string).charCodeAt(n);if(code===0||code>127)stdioTiff60DocumentSnapshotGuardReject(position,"ASCII text contains a native terminator or non-ASCII code unit");}}
   else if(kind==='float'){const v=exact(value,['bits'],position);integer(v.bits,position+'.bits');}
   else if(kind==='double'){const v=exact(value,['lo','hi'],position);integer(v.lo,position+'.lo');integer(v.hi,position+'.hi');}
   else if(kind==='rational'||kind==='sRational'){const pair=sequence(value,position,{minItems:2,maxItems:2}),min=kind==='rational'?0:-2147483648,max=kind==='rational'?4294967295:2147483647;integer(pair[0],position,min,max);integer(pair[1],position,min,max);}
   else{const min=kind==='sByte'?-128:kind==='sShort'?-32768:kind==='sLong'?-2147483648:0,max=kind==='byte'||kind==='undefined'?255:kind==='short'?65535:kind==='sByte'?127:kind==='sShort'?32767:kind==='sLong'?2147483647:4294967295;integer(value,position,min,max);}
  }
 }
 if(page.entries.some((v,i)=>i>0&&page.entries[i-1]!.tag>=v.tag))return stdioTiff60DocumentSnapshotGuardReject(at,"tags need unique ascending identities");
 if(page.blocks.length===0)return;if(page.blocks.length!==1)return stdioTiff60DocumentSnapshotGuardReject(at,"page owns one logical raster block");
 const block=page.blocks[0]!,width=tiffIntegers(page,256)[0],height=tiffIntegers(page,257)[0],channels=tiffIntegers(page,277)[0]??1;
 exact(block,['x','y','width','height','channels','samples'],at+'.blocks[0]');integer(block.x,at+'.x');integer(block.y,at+'.y');integer(block.width,at+'.width',1);integer(block.height,at+'.height',1);integer(block.channels,at+'.channels',1,65535);sequence(block.samples,at+'.samples');if(BigInt(block.width)*BigInt(block.height)*BigInt(block.channels)!==BigInt(block.samples.length))return stdioTiff60DocumentSnapshotGuardReject(at,"block sample cardinality differs");
 let bits=tiffIntegers(page,258),formats=tiffIntegers(page,339);if(bits.length===0)bits=[1];if(formats.length===0)formats=[1];if(bits.length===1)bits=Array(channels).fill(bits[0]);if(formats.length===1)formats=Array(channels).fill(formats[0]);
 if(block.x!==0||block.y!==0||block.width!==width||block.height!==height||block.channels!==channels||bits.length!==channels||formats.length!==channels||bits.some(v=>v<1||v>64)||formats.some((v,i)=>![1,2,3,4].includes(v)||v===3&&![16,32,64].includes(bits[i]!)))return stdioTiff60DocumentSnapshotGuardReject(at,"invalid exact sample interpretation");
 block.samples.forEach((v,i)=>{const position=at+".samples["+i+"]";exact(v,['lo','hi'],position);integer(v.lo,position+'.lo');integer(v.hi,position+'.hi');const word=BigInt(v.lo)|(BigInt(v.hi)<<32n);if(word>>BigInt(bits[i%channels]!)!==0n)stdioTiff60DocumentSnapshotGuardReject(position,"word exceeds channel precision")});return;
}
export function parseTiffSnapshot(value:unknown,at="$"):TiffSnapshot{const row=exact(value,["schema","ifds"],at);return {schema:stdioTiff60DocumentSnapshotGuardConstant(row.schema,at+".schema","stdio.tiff"),ifds:sequence(row.ifds,at+".ifds").map((v,i)=>parseTiffIfd(v,at+".ifds["+i+"]"))}}

export function validateTiffSnapshot(snapshot:TiffSnapshot):void{exact(snapshot,['schema','ifds'],'$');if(snapshot.schema!=="stdio.tiff")throw Error("tiff: undeclared schema");sequence(snapshot.ifds,'$.ifds');snapshot.ifds.forEach((page,index)=>validateTiffIfd(page,"$.ifds["+index+"]"));}
