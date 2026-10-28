/** 🫳️ Actual glTF artifact snapshots and raw GLB bytes share private source custody. */
import type {GltfSnapshot} from "../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import {IfcEditError} from "../🧬️schema/🔺️diff/🟦️.ts";
import type {IfcExchangeOutcome} from "./geometry.ts";
import {captureIfcExternalSource} from "./external-source.ts";
import {captureIfcIoControl,ifcCheckpoint,IfcStepError,type IfcIoControl} from "./control.ts";

const families=["scenes","nodes","meshes","accessors","bufferViews","buffers","materials","textures","images","samplers","skins","animations","cameras"] as const;
const record=(value:unknown):Record<string,unknown>=>{
 if(value===null||typeof value!=="object"||Array.isArray(value))throw new IfcEditError("invalidValue","glTF source records require objects");
 return value as Record<string,unknown>;
};
const array=(value:unknown):unknown[]=>{
 if(!Array.isArray(value))throw new IfcEditError("invalidValue","glTF source collections require arrays");
 return value;
};
function* records(source:GltfSnapshot):Generator<unknown> {
 const root=record(source),document=record(root.document);
 if(root.schema!=="stdio.gltf")throw new IfcEditError("invalidValue","Actual glTF artifact schema required");
 yield record(document.asset);
 for(const key of families)for(const value of array(document[key])){
  const row=record(value);yield row;
  if(key==="meshes")for(const primitive of array(row.primitives))yield record(primitive);
 }
 for(const buffer of array(root.buffers))array(buffer);
}

/** 🧊 Capture exact artifact data before interpreting metadata or reading accessor bytes. */
export async function captureIfcGltfSource(source:GltfSnapshot,control:IfcIoControl):Promise<IfcExchangeOutcome<GltfSnapshot>> {
 control=captureIfcIoControl(control);
 const captured=await captureIfcExternalSource(source,control,records);if(!captured.artifact)return captured;
 const owned=captured.artifact,batch=control.batchSize!;
 for(const buffer of owned.buffers){
  for(let index=0;index<buffer.length;index++){
   if(index%batch===0)await ifcCheckpoint(control,"gltf-source-bytes",index,buffer.length);
   const byte=buffer[index];
   if(typeof byte!=="number"||!Number.isInteger(byte)||byte<0||byte>255)return{fidelity:"unsupported",diagnostics:[{code:"gltf.bytes",severity:"error",message:"glTF source buffers require unsigned byte values",entityIds:[]}]};
  }
  await ifcCheckpoint(control,"gltf-source-bytes",buffer.length,buffer.length);
 }
 return captured;
}

const typedArray=Object.getPrototypeOf(Uint8Array.prototype);
const slots={
 kind:Object.getOwnPropertyDescriptor(typedArray,Symbol.toStringTag)!.get!,
 length:Object.getOwnPropertyDescriptor(typedArray,"length")!.get!,
 buffer:Object.getOwnPropertyDescriptor(typedArray,"buffer")!.get!,
 offset:Object.getOwnPropertyDescriptor(typedArray,"byteOffset")!.get!
};

/** 📦 Borrow genuine unsigned-byte internal slots, then complete cooperative private capture before parsing. */
export async function captureIfcGlbBytes(source:Uint8Array,control:IfcIoControl):Promise<Uint8Array> {
 control=captureIfcIoControl(control);
 let view:Uint8Array,size:number;
 try{
  if(slots.kind.call(source)!=="Uint8Array")throw new IfcStepError("glb.source","A genuine unsigned-byte view is required");
  size=slots.length.call(source);
  if(size>control.maxValues!)throw new IfcStepError("glb.limit","GLB source bytes exceed caller limit");
  view=new Uint8Array(slots.buffer.call(source),slots.offset.call(source),size);
 }catch(error){if(error instanceof IfcStepError)throw error;throw new IfcStepError("glb.source","An attached unsigned-byte view is required");}
 await ifcCheckpoint(control,"capture-glb-bytes",0,size);
 const owned=new Uint8Array(size),batch=control.batchSize!;
 for(let index=0;index<size;index++){
  if(index>0&&index%batch===0)await ifcCheckpoint(control,"capture-glb-bytes",index,size);
  const byte=view[index];if(byte===undefined)throw new IfcStepError("glb.source","GLB source byte view changed extent during capture");owned[index]=byte;
 }
 await ifcCheckpoint(control,"capture-glb-bytes",size,size);
 return owned;
}
