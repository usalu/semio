import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {readFileSync} from "node:fs";
import schema from "./🔣️schema.json";
import corpus from "./🔣️.json";
const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
const tree=ts.createSourceFile("actual.ts",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
function actual(names:readonly string[],ports:Record<string,unknown>):any{
 const nodes=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&node.name&&names.includes(node.name.text));
 expect(nodes.length).toBe(names.length);
 return new Function(...Object.keys(ports),ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n")+";return{"+names.join(",")+"};",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(...Object.values(ports));
}
test("closed producer phase vectors agree with independent Ajv and SQLite",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");
 try{for(const row of corpus.cases){let completed=0,refused=false;const events:string[]=[];for(const op of row.steps){const oracle=db.query("select case when ?1='complete' and ?2=0 then 'complete' when ?1='complete' or ?1='mutate' or ?1='wrong-control' then 'refused' when ?1='measure' and ?2=0 then 'artifact-measure' when ?1='measure' then 'inputs-measure' when ?2=0 then 'artifact-read' else 'inputs-read' end value").get(op,completed) as {value:string};if(oracle.value==="refused"){refused=true;break;}if(oracle.value==="complete")completed=1;else events.push(oracle.value);}expect(events,row.id).toEqual(row.events);expect(refused,row.id).toBe(row.refused);}}finally{db.close();}
});
for(const row of corpus.cases)test(row.id,async()=>{
 const control=Object.freeze({id:"actual-phase"}),foreign=Object.freeze({id:"foreign-phase"}),token=Object.freeze({id:"actual-artifact"}),receipt=Object.freeze({id:"actual-completed-inputs"}),events:string[]=[];let consumed=false;
 const requireControl=(value:unknown)=>{if(value!==control)throw Error("control mismatch");},requireUnused=(value:unknown,phase:unknown)=>{requireControl(phase);if(value!==token||consumed)throw Error("unused mutation owner required");},requireInput=(value:unknown,phase:unknown)=>{requireControl(phase);if(value!==receipt||!consumed)throw Error("actual completed receipt required");};
 const native={measureNativePlatformProtectedArtifactFundingV1:async(value:unknown,phase:unknown)=>{requireUnused(value,phase);events.push("artifact-measure");},readNativePlatformProtectedArtifactMaintenanceV1:async(value:unknown,phase:unknown)=>{requireUnused(value,phase);events.push("artifact-read");},measureNativePlatformProtectedArtifactInputGroupFundingV1:async(value:unknown,phase:unknown)=>{requireInput(value,phase);events.push("inputs-measure");},readNativePlatformProtectedArtifactInputGroupV1:async(value:unknown,phase:unknown)=>{requireInput(value,phase);events.push("inputs-read");}};
 const api=actual(["createRepositoryDependencyProducerArtifactPhaseV1"],{dependencyMaintenanceNative:native}),phase=api.createRepositoryDependencyProducerArtifactPhaseV1(token,control,()=>{});let refused=false;
 try{for(const op of row.steps){if(op==="complete"){if(!consumed)consumed=true;phase.accept(receipt);}else if(op==="measure")await phase.measure();else if(op==="recheck")await phase.recheck(control);else if(op==="wrong-control")await phase.recheck(foreign);else requireUnused(token,control);}}catch{refused=true;}
 expect(refused,row.id).toBe(row.refused);expect(events,row.id).toEqual(row.events);expect(consumed,row.id).toBe(row.steps.includes("complete"));
});

test("the actual producer switches staging/readback to completed-receipt funding before the installer phase",async()=>{
 const control=Object.freeze({id:"phase"}),token=Object.freeze({id:"artifact"}),receipt=Object.freeze({id:"inputs"}),stop=Error("next actual phase reached"),events:string[]=[];let consumed=false;
 const native={measureNativePlatformProtectedArtifactFundingV1:async(value:unknown,phase:unknown)=>{if(value!==token||phase!==control||consumed)throw Error("unused owner required");events.push("artifact-measure");},readNativePlatformProtectedArtifactMaintenanceV1:async()=>{if(consumed)throw Error("consumed owner");},stageNativePlatformProtectedArtifactInputGroupV1:async(value:unknown,phase:unknown)=>{if(value!==token||phase!==control||consumed)throw Error("unused mutation owner required");consumed=true;events.push("stage");return receipt;},readNativePlatformProtectedFreshArtifactInputGroupV1:async(value:unknown,phase:unknown)=>{if(value!==receipt||phase!==control||!consumed)throw Error("actual completed inputs required");events.push("inputs-read");},measureNativePlatformProtectedArtifactInputGroupFundingV1:async(value:unknown,phase:unknown)=>{if(value!==receipt||phase!==control||!consumed)throw Error("actual completed funding required");events.push("inputs-measure");}};
 const api=actual(["createRepositoryDependencyProducerArtifactPhaseV1","runRepositoryDependencyProducerCompletionV1"],{dependencyMaintenanceNative:native,readJavascriptDependencyProducerMaterializationV1:()=>({scopes:[],group:{batches:[]}}),prepareRepositoryDependencyProducerEnvironmentV1:async()=>{throw stop;},console:{error:()=>{}}}),phase=api.createRepositoryDependencyProducerArtifactPhaseV1(token,control,()=>{});
 await phase.measure();let failure:unknown;try{await api.runRepositoryDependencyProducerCompletionV1("/workspace",{},control,{},token,phase,"/root",{plans:[]},()=>{},async()=>"binding",async()=>undefined,()=>{});}catch(error){failure=error;}
 expect(failure).toBe(stop);expect(events).toEqual(["artifact-measure","stage","inputs-read","inputs-measure"]);expect(consumed).toBe(true);
});

import readerCases from "./🔎️reader.json";
import readerSchema from "./🔎️reader.schema.json";
test("the caller reader cases agree with independent Ajv and SQLite ordering",()=>{
 expect(new Ajv({strict:true}).validate(readerSchema,readerCases)).toBe(true);const db=new Database(":memory:");
 try{for(const row of readerCases.cases){const expected=db.query("select case when ?1 and ?2 then 'aggregate' when ?1 then 'body' when ?2 then 'after' else 'none' end refusal,json_array('inputs-read','reader-open','body','reader-drained','inputs-read') events").get(Number(row.bodyFailure),Number(row.afterFailure)) as {refusal:string;events:string};expect(expected.refusal,row.id).toBe(row.refusal);expect(JSON.parse(expected.events)).toEqual(["inputs-read","reader-open","body","reader-drained","inputs-read"]);}}finally{db.close();}
});
for(const row of readerCases.cases)test(row.id,async()=>{
 const control=Object.freeze({id:"control"}),parent=Object.freeze({id:"parent"}),token=Object.freeze({id:"mutation"}),receipt=Object.freeze({id:"completed"}),bodyFailure=Error("body failed"),afterFailure=Error("after failed"),events:string[]=[];let pending=0,reads=0,physicalChecks=0,value:unknown,failure:unknown;
 const physical={root:"/root",associations:[],check:()=>{physicalChecks++;}},native={readNativePlatformProtectedArtifactInputGroupV1:async(input:unknown,phase:unknown)=>{if(input!==receipt||phase!==control||pending!==0)throw Error("Quiescent immutable input completion required");events.push("inputs-read");if(++reads===2&&row.afterFailure)throw afterFailure;},withNativePlatformProtectedMaintenanceReadV1:async(selected:unknown,phase:unknown,body:Function)=>{if(selected!==parent||phase!==control)throw Error("Actual reader parent required");pending++;events.push("reader-open");try{return await body(physical,control);}finally{pending--;events.push("reader-drained");}}};
 const api=actual(["createRepositoryDependencyProducerArtifactPhaseV1","withRepositoryDependencyProducerArtifactReadV1"],{dependencyMaintenanceNative:native,requireIssuedResourceControlRefinementV1:(ancestor:unknown,phase:unknown)=>{if(ancestor!==control||phase!==control)throw Error("Actual reader control required");}}),phase=api.createRepositoryDependencyProducerArtifactPhaseV1(token,control,()=>{});phase.accept(receipt);
 try{value=await api.withRepositoryDependencyProducerArtifactReadV1(parent,control,phase,"/root",{},()=>{},async(context:any)=>{events.push("body");expect(pending).toBe(1);await context.recheck(control);if(row.bodyFailure)throw bodyFailure;return "value";},row.sourceOnly);}catch(error){failure=error;}
 expect(events,row.id).toEqual(["inputs-read","reader-open","body","reader-drained","inputs-read"]);expect(pending).toBe(0);expect(physicalChecks).toBeGreaterThanOrEqual(2);
 if(row.refusal==="none"){expect(value).toBe("value");expect(failure).toBeUndefined();}else{expect(value).toBeUndefined();if(row.refusal==="aggregate"){expect(failure).toBeInstanceOf(AggregateError);expect((failure as AggregateError).errors).toEqual([bodyFailure,afterFailure]);}else expect(failure).toBe(row.refusal==="body"?bodyFailure:afterFailure);}
});
