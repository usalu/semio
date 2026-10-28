/** 🏗️ IFC4X3_ADD2 STEP entry points retain exact graph values and original cancellation. */
import type {IfcSnapshot} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {validateIfcGraphSteps,runIfcEditAsync,IfcEditError} from "../🧬️schema/🔺️diff/🟦️.ts";
import {captureIfcIoControlScope,ifcCheckpoint,ifcIoLimit,IfcStepError,type IfcIoControl} from "./control.ts";
import {IfcStepReader} from "./step-reader.ts";
import {validateIfcStepHeader} from "./step-header.ts";
import {prepareIfcStepSource} from "./step-source.ts";
export {ifcCheckpoint,IfcStepError} from "./control.ts";
export type {IfcIoControl} from "./control.ts";
export {ifcReal,ifcNumber,validIfcStepBinary,writeIfcStepValue} from "./step-values.ts";
export {writeIfc4x3Step,writeIfc4x3StepChunks} from "./step-writer.ts";

/** 📥️ Decode one private graph and admit references before returning it to the caller. */
export async function parseIfc4x3Step(text:string,control:IfcIoControl={}):Promise<IfcSnapshot>{
 const scope=captureIfcIoControlScope(control);control=scope.control;return scope.run(async()=>{
 await ifcCheckpoint(control,"parse",0,text.length);const reader=new IfcStepReader(await prepareIfcStepSource(text,control),control),batch=reader.batch,maximum=ifcIoLimit(control.maxEntities,1_000_000,"maxEntities");
 await reader.take("ISO-10303-21");await reader.take(";");await reader.take("HEADER");await reader.take(";");
 const records=[];for(const expected of ["FILE_DESCRIPTION","FILE_NAME","FILE_SCHEMA"]){const name=await reader.word();if(name!==expected)reader.fail("step.header","Expected exactly one "+expected+" in mandatory order");records.push(await reader.args());await reader.take(";");}
 if(!await reader.is("ENDSEC"))reader.fail("step.header","Unsupported additional header record");await reader.take("ENDSEC");await reader.take(";");
 const header={fileDescription:records[0]!,fileName:records[1]!,fileSchema:records[2]!};await validateIfcStepHeader(header,control,reader.origin());await reader.take("DATA");await reader.take(";");const entities=[],ids=new Set<bigint>();
 while(!await reader.is("ENDSEC")){if(entities.length===maximum)reader.fail("step.entityLimit","Entity limit exceeded");const entity=await reader.entity();if(ids.has(entity.id))reader.fail("step.duplicate-id","Duplicate #"+entity.id);ids.add(entity.id);entities.push(entity);if(entities.length%batch===0)await ifcCheckpoint(control,"parse",reader.origin(),text.length);}
 await reader.take("ENDSEC");await reader.take(";");await reader.take("END-ISO-10303-21");await reader.take(";");await reader.skip();if(reader.at!==reader.text.length)reader.fail("step.trailing","Unexpected trailing data");
 const snapshot:IfcSnapshot={schema:"stdio.ifc",header,entities};try{await runIfcEditAsync(validateIfcGraphSteps(snapshot,"validateResult",{signal:control.signal,maxEntities:maximum,maxValues:control.maxValues,maxDepth:reader.depth}),{signal:control.signal,yieldEvery:batch,onProgress:progress=>control.progress?.({...progress,phase:"references"})});}catch(error){if(error instanceof IfcEditError)throw new IfcStepError(error.code==="cancelled"?"io.cancelled":error.code==="danglingReference"?"step.reference":"step.graph",error.message,reader.origin());throw error;}
 await ifcCheckpoint(control,"parse",text.length,text.length);return snapshot;
 });
}
