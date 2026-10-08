/** 🧬️ Checks independently authored content fields against their physical words. */
import {expect,test} from "bun:test";
import content from "../../🧫️fixtures/🧱️content/🔣️.json";
type Chunk={ordinal:number;field:string;fieldTag:number|null;elementType:string;values:number[]|string;floatBits:number[]|null;sourceOctets:number[];elementCount:number};

test("Remodeling content witnesses obey strict independent field and word laws",()=>{
 const sources:readonly{chunks:readonly Chunk[]}[]=[content.sparse,content.mesh];
 for(const source of sources)for(const[ordinal,chunk]of source.chunks.entries()){
  const bytes=Uint8Array.from(chunk.sourceOctets),view=new DataView(bytes.buffer),offset=chunk.fieldTag===null?0:1;if(chunk.fieldTag!==null)expect(bytes[0]).toBe(chunk.fieldTag);
  expect(chunk.ordinal).toBe(ordinal);
  if(chunk.elementType==="f32"||chunk.elementType==="u32"){if(!Array.isArray(chunk.values))throw Error("Numeric witness requires values");expect(bytes.length-offset).toBe(chunk.elementCount*4);for(let i=0;i<chunk.elementCount;i++)expect(view.getUint32(offset+i*4,true)).toBe(chunk.elementType==="f32"?chunk.floatBits![i]:chunk.values[i]);}
  else if(chunk.elementType==="u8"){if(!Array.isArray(chunk.values))throw Error("Byte witness requires values");expect(Array.from(bytes.subarray(offset))).toEqual(chunk.values);}
  else{if(typeof chunk.values!=="string")throw Error("Text witness requires content");expect(new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(bytes.subarray(offset))).toBe(chunk.values);}
 }
 console.log("[DEBUG] Remodeling all twelve independent field octet laws matched");
});

