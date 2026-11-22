import {brotliDecompressSync} from "node:zlib";

/** 🔬️ One side of a compared output: its digest and size, and whether its bytes were kept. */
export type EvidenceSideV1=Readonly<{sha256:string;bytes:number;stored:boolean}>;

/** 🔬️ The comparison of one output that differs between the kept and the refused result: the first differing byte and how many bytes differ (both sides stored), and the lines only one side has (`-` kept, `+` refused, at most 20, each cut to 200 characters) when both decode as text (Brotli and JSON are unpacked and JSON is indented first). */
export type EvidenceDiffV1=Readonly<{path:string;kept:EvidenceSideV1|null;refused:EvidenceSideV1|null;firstDifferenceByte:number|null;differingBytes:number|null;lines:readonly string[]}>;

const LINES=20,WIDTH=200;

/** 📜️ The text of a blob, or undefined when it is binary: Brotli is unpacked when it decodes, JSON is indented so that every field has its own line. */
export function evidenceTextV1(bytes:Uint8Array):string|undefined{
 let data=Buffer.from(bytes);
 try{data=brotliDecompressSync(data);}catch{}
 if(data.includes(0))return undefined;
 const text=data.toString("utf8");
 try{return JSON.stringify(JSON.parse(text),null,1);}catch{return text;}
}

/** 🔬️ The lines one text has more often than the other: `-` for the kept, `+` for the refused, in order of appearance, at most 20. */
export function evidenceLinesV1(kept:string,refused:string):readonly string[]{
 const count=(text:string)=>{const map=new Map<string,number>();for(const line of text.split("\n"))map.set(line,(map.get(line)??0)+1);return map;};
 const left=count(kept),right=count(refused),rows:string[]=[];
 const take=(text:string,mark:string,own:Map<string,number>,other:Map<string,number>):void=>{
  const used=new Map<string,number>();
  for(const line of text.split("\n")){
   const index=(used.get(line)??0)+1;used.set(line,index);
   if(index>(other.get(line)??0)&&own.has(line))rows.push(mark+line.slice(0,WIDTH));
  }
 };
 take(kept,"-",left,right);take(refused,"+",right,left);
 return rows.slice(0,LINES);
}

/** 🔬️ Compares the two sides of one differing output. A side that is absent, or whose bytes were not kept, is named by its digest only. */
export function diffEvidenceV1(path:string,kept:Readonly<{sha256:string;bytes:Uint8Array|undefined;size:number}>|undefined,refused:Readonly<{sha256:string;bytes:Uint8Array|undefined;size:number}>|undefined):EvidenceDiffV1{
 const side=(row:typeof kept):EvidenceSideV1|null=>row===undefined?null:{sha256:row.sha256,bytes:row.size,stored:row.bytes!==undefined};
 const left=kept?.bytes,right=refused?.bytes;
 if(left===undefined||right===undefined)return{path,kept:side(kept),refused:side(refused),firstDifferenceByte:null,differingBytes:null,lines:[]};
 const shortest=Math.min(left.length,right.length);let first=-1,different=Math.abs(left.length-right.length);
 for(let index=0;index<shortest;index++)if(left[index]!==right[index]){if(first<0)first=index;different++;}
 if(first<0&&left.length!==right.length)first=shortest;
 const a=evidenceTextV1(left),b=evidenceTextV1(right);
 return{path,kept:side(kept),refused:side(refused),firstDifferenceByte:first<0?null:first,differingBytes:different,lines:a===undefined||b===undefined?[]:evidenceLinesV1(a,b)};
}
