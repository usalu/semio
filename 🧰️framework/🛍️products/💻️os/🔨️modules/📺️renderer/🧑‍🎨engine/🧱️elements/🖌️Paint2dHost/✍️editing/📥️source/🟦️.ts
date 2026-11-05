import {Base64DecodeCursor} from "@semio-tech/framework/io/base64";
import {validateExtent,type PixelImage} from "@semio-tech/framework/pixels/editing";
import schema from "./🧬️schema/🔣️.json";

export type PixelSourceOptions={signal?:AbortSignal;onProgress?:(fraction:number)=>void};
const check=(signal?:AbortSignal):void=>{if(signal?.aborted)throw new DOMException("Cancelled","AbortError");};

/** 📥️ Yields between strict shared-codec grants and never publishes cancelled source bytes. */
export async function preparePixelSourceBytes(encoded:string,options:PixelSourceOptions&{maximumOutputBytes?:number}={}):Promise<Uint8Array<ArrayBuffer>>{
  check(options.signal);
  const cursor=new Base64DecodeCursor(encoded),policy=schema["x-semio-admission"];
  const control={maximumOutputBytes:options.maximumOutputBytes??cursor.expectedOutputBytes(),progress:(event:{phase:string;completed:number;total:number})=>{
    check(options.signal);
    options.onProgress?.(((event.phase==="decode"?event.total:0)+event.completed)/Math.max(1,event.total*2));
    check(options.signal);return true;
  }};
  try{
    while(cursor.step(policy.maximumWorkUnits,control)!=="complete"){
      check(options.signal);await new Promise<void>(resolve=>setTimeout(resolve,0));check(options.signal);
    }
    options.onProgress?.(1);check(options.signal);
    const bytes=cursor.takeOutput();
    if(!bytes||!(bytes.buffer instanceof ArrayBuffer))throw new Error("Pixel source output is unavailable");
    return new Uint8Array(bytes.buffer,bytes.byteOffset,bytes.byteLength);
  }catch(error){cursor.cancel();throw error;}
}

/** 🪟️ Reuses the browser-owned buffer without copying the full image. */
export function pixelReadbackView(source:Uint8ClampedArray<ArrayBuffer>):Uint8Array<ArrayBuffer>{
  return new Uint8Array(source.buffer,source.byteOffset,source.byteLength);
}

/** 🧻️ Initializes authored mask coverage in bounded turns instead of one full-image fill. */
export async function prepareBlankPixelSource(width:number,height:number,target:"pixels"|"mask",options:PixelSourceOptions={}):Promise<PixelImage>{
  check(options.signal);validateExtent(width,height);
  if(target!=="pixels"&&target!=="mask")throw new Error("Invalid pixel source target");
  options.onProgress?.(0);check(options.signal);
  const pixels=new Uint8Array(width*height*4),grant=schema["x-semio-admission"].maximumWorkUnits;
  if(target==="mask")for(let offset=0;offset<pixels.length;offset+=grant){
    check(options.signal);const end=Math.min(offset+grant,pixels.length);pixels.fill(255,offset,end);
    options.onProgress?.(end/pixels.length);check(options.signal);
    if(end<pixels.length)await new Promise<void>(resolve=>setTimeout(resolve,0));
  }
  else{options.onProgress?.(1);check(options.signal);}
  return {width,height,pixels};
}
