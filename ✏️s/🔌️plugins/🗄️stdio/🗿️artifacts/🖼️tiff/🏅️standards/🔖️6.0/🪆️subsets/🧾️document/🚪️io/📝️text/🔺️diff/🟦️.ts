/** 📝️ Owned TIFF diff text framing. */
import {parseTiffDiff,type TiffDiff} from "../../../🧬️schema/🔺️diff/🟦️.ts";
export type TiffDiffText=string;
export function printTiffDiff(diff:TiffDiff):string{const bytes=new TextEncoder().encode(JSON.stringify(parseTiffDiff(diff)));return "tiff-diff payload="+Array.from(bytes,b=>b.toString(16).padStart(2,'0')).join('');}
export function parseTiffDiffText(text:unknown):TiffDiff{if(typeof text!=='string'||!/^tiff-diff payload=(?:[a-fA-F0-9]{2})*$/.test(text))throw Error('tiff: invalid owned diff framing');const hex=text.slice(18),bytes=Uint8Array.from({length:hex.length/2},(_,i)=>parseInt(hex.slice(i*2,i*2+2),16));return parseTiffDiff(JSON.parse(new TextDecoder('utf-8',{fatal:true}).decode(bytes)));}
