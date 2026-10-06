/** 🌈️ Native pass rows retain filters, packed remainders and unused inflated octets. */
export interface PngHeader { readonly width:number; readonly height:number; readonly bitDepth:number; readonly colorType:number; readonly compression:number; readonly filter:number; readonly interlace:number }
export interface PngScanline { readonly pass:number; readonly row:number; readonly width:number; readonly channels:number; readonly bitDepth:number; readonly filter:number; readonly samples:readonly number[]; readonly remainder:number; readonly remainderBits:number }
export interface PngScanlines { readonly rows:readonly PngScanline[]; readonly tail:readonly number[] }
export class PngSyntaxError extends Error {}
function invalid(reason:string):never{throw new PngSyntaxError("PNG "+reason)}
export function channels(colorType:number):number{switch(colorType){case 0:case 3:return 1;case 2:return 3;case 4:return 2;case 6:return 4;default:return invalid("color type")}}
export function validateHeader(header:PngHeader):void{if(!Number.isInteger(header.width)||header.width<=0||header.width>4294967295||!Number.isInteger(header.height)||header.height<=0||header.height>4294967295)invalid("dimensions");const depths=header.colorType===0?[1,2,4,8,16]:header.colorType===3?[1,2,4,8]:[8,16];channels(header.colorType);if(!depths.includes(header.bitDepth)||header.compression!==0||header.filter!==0||![0,1].includes(header.interlace))invalid("native profile")}
const passes=[[0,0,8,8],[4,0,8,8],[0,4,4,8],[2,0,4,4],[0,2,2,4],[1,0,2,2],[0,1,1,2]];
function extent(size:number,start:number,step:number):number{return size<=start?0:Math.ceil((size-start)/step)}
function paeth(left:number,above:number,corner:number):number{const p=left+above-corner,a=Math.abs(p-left),b=Math.abs(p-above),c=Math.abs(p-corner);return a<=b&&a<=c?left:b<=c?above:corner}
function predictor(filter:number,left:number,above:number,corner:number):number{switch(filter){case 0:return 0;case 1:return left;case 2:return above;case 3:return Math.floor((left+above)/2);case 4:return paeth(left,above,corner);default:return invalid("filter type")}}
export async function parseScanlines(raw:readonly number[],header:PngHeader,checkpoint:(completed:number)=>Promise<void>):Promise<PngScanlines>{
 validateHeader(header);const count=channels(header.colorType),rows:PngScanline[]=[];let cursor=0,work=0;
 for(const[pass,geometry]of(header.interlace?passes:[[0,0,1,1]]).entries()){
  const width=extent(header.width,geometry[0]!,geometry[2]!),height=extent(header.height,geometry[1]!,geometry[3]!);if(width===0||height===0)continue;
  const bits=width*count*header.bitDepth,bytes=Math.ceil(bits/8),bpp=Math.max(1,Math.ceil(count*header.bitDepth/8));if(!Number.isSafeInteger(bytes)||bytes>raw.length)invalid("scanline extent");let previous:number[]=[];
  for(let row=0;row<height;row++){
   if(cursor+bytes+1>raw.length)invalid("short inflated scanline");const filter=raw[cursor++]!,unfiltered:number[]=[],samples:number[]=[];
   for(let at=0;at<bytes;at++){const left=at<bpp?0:unfiltered[at-bpp]!,above=previous[at]??0,corner=at<bpp?0:previous[at-bpp]??0;unfiltered.push((raw[cursor++]!+predictor(filter,left,above,corner))&255);if(++work%256===0)await checkpoint(work)}
   for(let at=0;at<width*count;at++){const bit=at*header.bitDepth;if(header.bitDepth===16)samples.push(unfiltered[bit>>>3]!*256+unfiltered[(bit>>>3)+1]!);else samples.push((unfiltered[bit>>>3]!>>>(8-header.bitDepth-bit%8))&((1<<header.bitDepth)-1));if(++work%256===0)await checkpoint(work)}
   const remainderBits=(8-bits%8)%8,remainder=remainderBits?unfiltered.at(-1)!&((1<<remainderBits)-1):0;rows.push({pass,row,width,channels:count,bitDepth:header.bitDepth,filter,samples,remainder,remainderBits});previous=unfiltered;
  }
 }
 const tail:number[]=[];while(cursor<raw.length){tail.push(raw[cursor++]!);if(++work%256===0)await checkpoint(work)}await checkpoint(work);return{rows,tail};
}
export async function encodeScanlines(source:PngScanlines,checkpoint:(completed:number)=>Promise<void>):Promise<number[]>{
 const raw:number[]=[];let work=0,previous:number[]=[],pass=-1,next=0;
 for(const row of source.rows){
  if(!Number.isInteger(row.pass)||row.pass<0||row.pass>6||row.pass<pass||row.row!==(row.pass===pass?next:0))invalid("scanline pass or row order");if(row.pass!==pass){previous=[];next=0;pass=row.pass}next++;
  if(!Number.isInteger(row.width)||row.width<=0||![1,2,3,4].includes(row.channels)||![1,2,4,8,16].includes(row.bitDepth)||row.samples.length!==row.width*row.channels)invalid("scanline sample extent");
  const bits=row.samples.length*row.bitDepth,remainderBits=(8-bits%8)%8;if(row.remainderBits!==remainderBits||!Number.isInteger(row.remainder)||row.remainder<0||row.remainder>=2**remainderBits)invalid("packed remainder");const bytes=new Array<number>(Math.ceil(bits/8)).fill(0);
  for(let at=0;at<row.samples.length;at++){const value=row.samples[at]!;if(!Number.isInteger(value)||value<0||value>=2**row.bitDepth)invalid("sample range");const bit=at*row.bitDepth;if(row.bitDepth===16){bytes[bit>>>3]=value>>>8;bytes[(bit>>>3)+1]=value&255}else bytes[bit>>>3]!|=value<<(8-row.bitDepth-bit%8);if(++work%256===0)await checkpoint(work)}if(remainderBits)bytes[bytes.length-1]!|=row.remainder;
  const bpp=Math.max(1,Math.ceil(row.channels*row.bitDepth/8));raw.push(row.filter);for(let at=0;at<bytes.length;at++){const left=at<bpp?0:bytes[at-bpp]!,above=previous[at]??0,corner=at<bpp?0:previous[at-bpp]??0;raw.push((bytes[at]!-predictor(row.filter,left,above,corner))&255);if(++work%256===0)await checkpoint(work)}previous=bytes;
 }
 for(const byte of source.tail){if(!Number.isInteger(byte)||byte<0||byte>255)invalid("inflated tail octet");raw.push(byte);if(++work%256===0)await checkpoint(work)}await checkpoint(work);return raw;
}
