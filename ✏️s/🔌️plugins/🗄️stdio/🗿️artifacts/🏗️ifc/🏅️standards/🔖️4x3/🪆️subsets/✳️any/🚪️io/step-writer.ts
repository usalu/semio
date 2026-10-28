/** 📤️ Owned document export emits bounded chunks through the central graph admission. */
import type {IfcSnapshot,IfcValue,IfcComplexType} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {captureIfcGraphSteps,runIfcEditAsync,IfcEditError} from "../🧬️schema/🔺️diff/🟦️.ts";
import {captureIfcIoControlScope,ifcCheckpoint,ifcIoLimit,IfcStepError,type IfcIoControl} from "./control.ts";
import {writeIfcStepValue} from "./step-values.ts";
import {validateIfcStepHeader} from "./step-header.ts";

function* text(value:string):Generator<string>{
 yield"'";let chunk="";for(const char of value){const code=char.codePointAt(0)!;if(code>=0xd800&&code<=0xdfff)throw new IfcStepError("step.string","Unpaired Unicode surrogate");chunk+=char==="'"?"''":char==="\\"?"\\\\":code>=32&&code<=126?char:`\\X${code<=0xffff?"2":"4"}\\${code.toString(16).toUpperCase().padStart(code<=0xffff?4:8,"0")}\\X0\\`;if(chunk.length>=2048){yield chunk;chunk="";}}if(chunk)yield chunk;yield"'";
}
function* name(value:string):Generator<string>{for(let at=0;at<value.length;at+=2048)yield value.slice(at,at+2048);}
function* args(items:readonly IfcValue[],depth:number,maximum:number):Generator<string>{if(depth>maximum)throw new IfcStepError("step.depth","Argument nesting limit exceeded");yield"(";for(let i=0;i<items.length;i++){if(i)yield",";yield* value(items[i]!,depth+1,maximum);}yield")";}
function* value(item:IfcValue,depth:number,maximum:number):Generator<string>{if(depth>maximum)throw new IfcStepError("step.depth","Argument nesting limit exceeded");switch(item.kind){case"string":yield* text(item.value);break;case"binary":yield'"';for(let at=0;at<item.value.length;at+=2048)yield item.value.slice(at,at+2048);yield'"';break;case"enum":yield".";yield* name(item.value);yield".";break;case"aggregate":yield* args(item.value,depth,maximum);break;case"typedValue":yield* name(item.value.name);yield* args(item.value.items,depth,maximum);break;default:yield writeIfcStepValue(item);}}
function* member(item:IfcComplexType,maximum:number):Generator<string>{yield* name(item.name);yield* args(item.args,-1,maximum);}
function* document(snapshot:IfcSnapshot,maximum:number):Generator<string>{
 yield"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION";yield* args(snapshot.header.fileDescription,-1,maximum);yield";\nFILE_NAME";yield* args(snapshot.header.fileName,-1,maximum);yield";\nFILE_SCHEMA";yield* args(snapshot.header.fileSchema,-1,maximum);yield";\nENDSEC;\nDATA;\n";
 for(const entity of snapshot.entities){yield`#${entity.id}=`;if(entity.complex?.length)yield"(";yield* member(entity,maximum);for(const part of entity.complex??[]){yield" ";yield* member(part,maximum);}if(entity.complex?.length)yield")";yield";\n";}yield"ENDSEC;\nEND-ISO-10303-21;\n";
}

function* roots(snapshot:IfcSnapshot):Generator<IfcValue>{for(const lane of[snapshot.header.fileDescription,snapshot.header.fileName,snapshot.header.fileSchema])yield* lane;for(const entity of snapshot.entities){yield* entity.args;for(const part of entity.complex??[])yield* part.args;}}
async function validateText(snapshot:IfcSnapshot,control:IfcIoControl):Promise<void>{
 const stack:Iterator<IfcValue>[]=[roots(snapshot)];let visited=0,scanned=0,last=0;const batch=ifcIoLimit(control.batchSize,256,"batchSize");
 while(stack.length){const next=stack.at(-1)!.next();if(next.done){stack.pop();continue;}const item=next.value;if(item.kind==="aggregate"||item.kind==="typedValue")stack.push((item.kind==="aggregate"?item.value:item.value.items).values());
  if(item.kind==="string")for(let at=0;at<item.value.length;at++){const code=item.value.charCodeAt(at);scanned++;if(code>=0xd800&&code<=0xdbff){const low=item.value.charCodeAt(++at);scanned++;if(!(low>=0xdc00&&low<=0xdfff))throw new IfcStepError("step.string","Unpaired Unicode surrogate");}else if(code>=0xdc00&&code<=0xdfff)throw new IfcStepError("step.string","Unpaired Unicode surrogate");if(scanned-last>=4096){last=scanned;await ifcCheckpoint(control,"validate-text",scanned,0);}}
  if(++visited%batch===0)await ifcCheckpoint(control,"validate-text-values",visited,0);
 }
}

/** 🧵️ Emit at most 4096 UTF-16 code units per chunk; text progress uses total zero for unknown output length. */
export async function* writeIfc4x3StepChunks(source:IfcSnapshot,control:IfcIoControl={}):AsyncGenerator<string>{
 const scope=captureIfcIoControlScope(control);control=scope.control;try{
 const depth=ifcIoLimit(control.maxDepth,256,"maxDepth"),batch=ifcIoLimit(control.batchSize,256,"batchSize");let snapshot:IfcSnapshot;
 try{snapshot=await runIfcEditAsync(captureIfcGraphSteps(source,"validateBase",{signal:control.signal,maxEntities:control.maxEntities,maxValues:control.maxValues,maxDepth:depth,yieldEvery:batch,onProgress:progress=>control.progress?.({...progress,phase:`capture-${progress.phase}`})}),{signal:control.signal,yieldEvery:batch,onProgress:progress=>control.progress?.({...progress,phase:`capture-${progress.phase}`})});}
 catch(error){if(error instanceof IfcEditError)throw new IfcStepError(error.code==="cancelled"?"io.cancelled":error.code==="invalidSchema"?"step.schema":"step.graph",error.message);throw error;}
 await validateIfcStepHeader(snapshot.header,control);await validateText(snapshot,control);await ifcCheckpoint(control,"write",0,snapshot.entities.length);let pending="",completed=0;
 for(const part of document(snapshot,depth)){let at=0;while(at<part.length){const count=Math.min(4096-pending.length,part.length-at);pending+=part.slice(at,at+count);at+=count;if(pending.length===4096){completed+=pending.length;await ifcCheckpoint(control,"write-text",completed,0);yield pending;pending="";}}}
 if(pending){completed+=pending.length;await ifcCheckpoint(control,"write-text",completed,0);yield pending;}await ifcCheckpoint(control,"write",snapshot.entities.length,snapshot.entities.length);scope.rethrowCallerFailure();
 }catch(error){scope.rethrowCallerFailure();throw error;}
}

/** 🧾️ Collect the same admitted chunk stream when the caller requires one immutable string. */
export async function writeIfc4x3Step(snapshot:IfcSnapshot,control:IfcIoControl={}):Promise<string>{let result="";for await(const chunk of writeIfc4x3StepChunks(snapshot,control))result+=chunk;return result;}
