/** 🧾️ Reads exactly one complete Cargo JSON object; DATA confers no authority. */
export function readCargoObjectCaptureV1(text:string):Record<string,unknown>|undefined {
 try {
  const value:unknown=JSON.parse(text);
  return value!==null&&typeof value==="object"&&!Array.isArray(value)?value as Record<string,unknown>:undefined;
 }catch{return undefined;}
}

/** 📬️ Reads machine data only after its captured child has completed successfully. */
export function readCargoResponseObjectV1(capture:Readonly<{stdout:string;stderr:string;status:number|null;signal:string|null}>):Record<string,unknown>|undefined {
 return capture.status===0&&capture.signal===null?readCargoObjectCaptureV1(capture.stdout):undefined;
}

/** 🧵️ Decodes complete machine stdout after its owned runner has settled successfully. */
export async function readCargoObjectStreamV1(run:(accept:(bytes:Uint8Array,stream:"stdout"|"stderr")=>void)=>Promise<void>):Promise<Record<string,unknown>|undefined> {
 const decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}),chunks:string[]=[];
 await run((bytes,stream)=>{if(stream==="stdout")chunks.push(decoder.decode(bytes,{stream:true}));});
 chunks.push(decoder.decode());
 return readCargoObjectCaptureV1(chunks.join(""));
}
