/** 📥️ Receive private GLB JSON bytes through shared lossless syntax and glTF wire declarations. */
import {decodeJsonSyntaxSegments,JsonSyntaxError,type JsonSyntaxProgress} from '@semio-tech/pack-json-decode-ts';
import {JsonMemberPolicy} from '@semio-tech/framework-pack-json-ts/members';
import {parseGltfDocumentSyntaxSteps,type GltfDocument} from '../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts';
import {captureIfcIoControl,ifcCheckpoint,IfcStepError,type IfcIoControl} from './control.ts';

/** 🧵️ Borrow only original private bytes and publish the typed document after all completion checkpoints. */
export async function decodeIfcGltfJson(bytes:Uint8Array,control:IfcIoControl):Promise<GltfDocument>{
 control=captureIfcIoControl(control);const batch=control.batchSize!,maximum=control.maxValues!,decoder=new TextDecoder('utf-8',{fatal:true}),parts:string[]=[];
 if(bytes.length>maximum)throw new IfcStepError('glb.limit','Raw JSON bytes exceed admission');
 for(let offset=0;offset<bytes.length;offset+=batch){await ifcCheckpoint(control,'glb-json-decode',offset,bytes.length);const part=decoder.decode(bytes.subarray(offset,Math.min(bytes.length,offset+batch)),{stream:true});if(part)parts.push(part);}
 const tail=decoder.decode();if(tail)parts.push(tail);await ifcCheckpoint(control,'glb-json-decode',bytes.length,bytes.length);
 await ifcCheckpoint(control,'glb-json-syntax',0,bytes.length);let progress:JsonSyntaxProgress={bytes:0,nodes:0};
 const syntax=await decodeJsonSyntaxSegments(parts,JsonMemberPolicy.Replace,{maximumBytes:maximum,maximumNodes:maximum,maximumDepth:control.maxDepth!,chunk:batch,cancelled:()=>control.signal?.aborted??false,progress:value=>{progress=value;},yield:()=>ifcCheckpoint(control,'glb-json-syntax',progress.bytes,bytes.length)}).catch(error=>{
  if(!(error instanceof JsonSyntaxError))throw error;
  if(error.code==='cancelled')throw new IfcStepError('io.cancelled','Exchange cancelled',error.offset);
  throw new IfcStepError(['byte-budget','node-budget','depth-budget'].includes(error.code)?'glb.limit':'glb.json',error.message,error.offset);
 });
 await ifcCheckpoint(control,'glb-json-syntax',bytes.length,bytes.length);
 const steps=parseGltfDocumentSyntaxSteps(syntax);let completed=0;
 await ifcCheckpoint(control,'glb-json-project',0,0);
 for(;;){const step=steps.next();if(step.done){await ifcCheckpoint(control,'glb-json-project',completed,completed);return step.value;}
  if(step.value?.kind==='number'&&step.value.codeUnits>control.maxNumericCodeUnits!)throw new IfcStepError('glb.limit','Numeric token extent exceeds admission at '+step.value.at);
  if(++completed%batch===0)await ifcCheckpoint(control,'glb-json-project',completed,0);
 }
}
