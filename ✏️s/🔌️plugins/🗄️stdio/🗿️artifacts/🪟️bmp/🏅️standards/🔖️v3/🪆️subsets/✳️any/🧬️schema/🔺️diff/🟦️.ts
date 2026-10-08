/** 🔺️ Sparse Bmp diff algebra: a whole-image replacement and ordered pixel rectangles. */
import {parseBmpImage,parseBmpSnapshot,type BmpImage,type BmpNativeSample,type BmpRegion,type BmpSnapshot} from "../📸️snapshot/🟦️.ts";
export interface BmpSampleRect { region:BmpRegion; indices?:number[]; samples?:BmpNativeSample[]; }
export interface BmpDiff { image?:BmpImage; rects?:BmpSampleRect[]; }
const equal=(a:unknown,b:unknown):boolean=>{if(a===b)return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object")return false;const left=Object.keys(a),right=Object.keys(b);return left.length===right.length&&left.every(key=>Object.hasOwn(b,key)&&equal((a as Record<string,unknown>)[key],(b as Record<string,unknown>)[key]));};
const fail=(at:string,why:string):never=>{throw new TypeError(`${at}: ${why}`);};
const isRecord=(value:unknown):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const integer=(value:unknown,at:string,maximum:number):number=>Number.isSafeInteger(value)&&Number(value)>=0&&Number(value)<=maximum?Number(value):fail(at,"integer outside owned range");
function parseSample(value:unknown,at:string):BmpNativeSample{
  if(!isRecord(value)||Object.keys(value).some(key=>!["red","green","blue","alpha","reserved"].includes(key)))return fail(at,"invalid native sample");
  return {red:integer(value.red,`${at}.red`,4294967295),green:integer(value.green,`${at}.green`,4294967295),blue:integer(value.blue,`${at}.blue`,4294967295),alpha:integer(value.alpha,`${at}.alpha`,4294967295),reserved:integer(value.reserved,`${at}.reserved`,4294967295)};
}
function parseRect(value:unknown,at:string):BmpSampleRect{
  if(!isRecord(value)||Object.keys(value).some(key=>!["region","indices","samples"].includes(key))||!isRecord(value.region))return fail(at,"invalid sample rectangle");
  const region=value.region;if(Object.keys(region).some(key=>!["x","y","width","height"].includes(key)))return fail(at,"invalid region");
  const rect:BmpSampleRect={region:{x:integer(region.x,`${at}.region.x`,4294967295),y:integer(region.y,`${at}.region.y`,4294967295),width:integer(region.width,`${at}.region.width`,4294967295),height:integer(region.height,`${at}.region.height`,4294967295)}};
  if(value.indices!==undefined){if(!Array.isArray(value.indices))return fail(at,"invalid indices");if(value.indices.length>0)rect.indices=value.indices.map((index,at2)=>integer(index,`${at}.indices[${at2}]`,255));}
  if(value.samples!==undefined){if(!Array.isArray(value.samples))return fail(at,"invalid samples");if(value.samples.length>0)rect.samples=value.samples.map((sample,at2)=>parseSample(sample,`${at}.samples[${at2}]`));}
  return rect;
}
export function parseBmpDiff(value:unknown,at="$"):BmpDiff{
  if(!isRecord(value)||Object.keys(value).some(key=>key!=="image"&&key!=="rects"))return fail(at,"invalid image diff");
  const diff:BmpDiff={};
  if(value.image!==undefined)diff.image=parseBmpImage(value.image,`${at}.image`);
  if(value.rects!==undefined){if(!Array.isArray(value.rects))return fail(at,"invalid sample rectangles");if(value.rects.length>0)diff.rects=value.rects.map((rect,index)=>parseRect(rect,`${at}.rects[${index}]`));}
  return diff;
}
const rowStart=(image:BmpImage,region:BmpRegion):number|undefined=>region.width>0&&region.height>0&&region.x+region.width<=image.width&&region.y+region.height<=image.height?region.y*image.width+region.x:undefined;
export function bmpRegionRect(image:BmpImage,region:BmpRegion):BmpSampleRect{
  const start=rowStart(image,region);if(start===undefined)return {region:{...region}};
  const out:BmpSampleRect={region:{...region}};
  if(image.pixels.storage==="indexed"){out.indices=[];for(let line=0;line<region.height;line+=1)out.indices.push(...image.pixels.indices.slice(start+line*image.width,start+line*image.width+region.width));}
  else{out.samples=[];for(let line=0;line<region.height;line+=1)out.samples.push(...image.pixels.samples.slice(start+line*image.width,start+line*image.width+region.width).map(sample=>({...sample})));}
  return out;
}
function writeRect(image:BmpImage,rect:BmpSampleRect):void{
  const start=rowStart(image,rect.region);if(start===undefined)return fail("$.rects","sample rectangle exceeds the owned image or is empty");
  const row=rect.region.width,count=row*rect.region.height;
  if(image.pixels.storage==="indexed"){
    if((rect.indices??[]).length!==count||(rect.samples??[]).length!==0)return fail("$.rects","sample rectangle storage or cardinality differs from its region");
    for(let line=0;line<rect.region.height;line+=1)image.pixels.indices.splice(start+line*image.width,row,...rect.indices!.slice(line*row,(line+1)*row));
  }else{
    if((rect.samples??[]).length!==count||(rect.indices??[]).length!==0)return fail("$.rects","sample rectangle storage or cardinality differs from its region");
    for(let line=0;line<rect.region.height;line+=1)image.pixels.samples.splice(start+line*image.width,row,...rect.samples!.slice(line*row,(line+1)*row).map(sample=>({...sample})));
  }
}
export function applyBmpDiff(base:BmpSnapshot,diff:BmpDiff):BmpSnapshot{
  const image:BmpImage=structuredClone(diff.image??base.image);
  (diff.rects??[]).forEach(rect=>writeRect(image,rect));
  return parseBmpSnapshot({schema:base.schema,image});
}
export function inverseBmpDiff(base:BmpSnapshot,diff:BmpDiff):BmpDiff{
  if(diff.image!==undefined)return {image:parseBmpImage(base.image)};
  const rects=diff.rects??[];
  if(rects.length===0)return {};
  const running:BmpImage=structuredClone(base.image);
  return {rects:rects.map(rect=>{const restore=bmpRegionRect(running,rect.region);writeRect(running,rect);return restore;}).reverse()};
}
export function betweenBmpSnapshots(base:BmpSnapshot,next:BmpSnapshot):BmpDiff{return equal(base.image,next.image)?{}:{image:parseBmpImage(next.image)};}
export function absorbBmpDiff(base:BmpDiff,next:BmpDiff):BmpDiff{
  if(next.image!==undefined)return parseBmpDiff(next);
  const out:BmpDiff={...base};
  if(next.rects!==undefined&&next.rects.length>0)out.rects=[...(base.rects??[]),...next.rects];
  return parseBmpDiff(out);
}
