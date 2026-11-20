import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {mkdirSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {acquireFreshResourceLeaseV1,acquireResourceLease,checkResourceLeaseControl,resourceLeasePathV1,type ResourceLease} from "../../../../../../../../🔒️leases/🟦️.ts";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1} from "../../../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../../../../🧪️testing/📥️invocation/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"test:native-input-journal-admission",maximumElapsedMilliseconds:120000,capabilities:{artifacts:true}});

test("the protected input journal tolerates bounded catalog contention and never waits for a real writer",()=>withBuildTestInvocationV1(async original=>{
 const source=readFileSync(new URL("../../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("native-input-journal.ts",source,ts.ScriptTarget.Latest,true);
 const body=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="withProtectedArtifactInputJournalV1");
 if(!body)throw Error("Actual fixed input journal owner required");
 const journal=ts.transpileModule(body.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const oracle=new Database(":memory:"),scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ij-"));
 const outer=createIssuedResourceControlV1(original,"cache:journal-admission",20000);
 try{
  for(const [index,sample] of fixture.cases.entries()){
   const directory=join(scratch,String(index)),identity="private-input-"+index,resource="native-storage:"+identity,abort=new AbortController(),stop=Error("cancel current protected read");
   mkdirSync(directory);
   const control=createRequestedIssuedResourceControlV1(outer,3000,abort.signal);
   const options={directory,resource,mode:"exclusive" as const,signal:control.signal,control};
   const initial=await acquireResourceLease({...options,timeoutMs:0});
   initial.release();
   const gate=new Database(resourceLeasePathV1(directory,fixture.admissionResource),{readwrite:true,create:false});
   let locked=false,holder:ResourceLease|undefined,reads=0,called=0,success=false,failure:unknown;
   const waits:string[]=[];
   const release=()=>{if(locked){gate.exec("ROLLBACK");locked=false;}};
   try{
    if(sample.lock==="directory"){
     gate.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");locked=true;
     let raw:unknown;
     try{(await acquireResourceLease({...options,timeoutMs:0})).release();}catch(error){raw=error;}
     expect(raw).toBeInstanceOf(Error);
     expect(Reflect.get(raw as Error,"resourceLeaseAdmission")?.operation).toBe("open");
    }else if(sample.lock==="journal"){
     holder=await acquireResourceLease({...options,owner:"fixture-journal-writer",timeoutMs:0});
    }
    const scope={options:{},read:{port:{check:()=>checkResourceLeaseControl(control),recheck:async()=>{checkResourceLeaseControl(control);}}}};
    const held={scope,expected:{policy:{leaseDirectory:directory},backing:{identity}}};
    const ports={
     requireProtectedArtifactPolicyV1:()=>checkResourceLeaseControl(control),
     protectedArtifactInputMappingsV1:new WeakSet(),
     acquireProtectedMaintenanceMappingsV1:async()=>{checkResourceLeaseControl(control);},
     acquireResourceLease,
     acquireFreshResourceLeaseV1:(request:Parameters<typeof acquireFreshResourceLeaseV1>[0])=>acquireFreshResourceLeaseV1({...request,onWait:progress=>{
      waits.push(progress.resource);
      if(sample.action==="release")release();
      if(sample.action==="cancel")abort.abort(stop);
     }}),
     readArtifactRetirementJournalV1:async()=>({stamp:"exact",text:sample.action==="drift"&&++reads===2?"changed":"current"}),
     protectedParentFailureV1:(failures:unknown[],error:unknown)=>failures.push(error)
    };
    const run=new Function(...Object.keys(ports),journal+";return withProtectedArtifactInputJournalV1;")(...Object.values(ports));
    try{
     await run(held,control,async()=>{called++;});
     success=true;
    }catch(error){failure=error;}
    const model=oracle.query("select ? in ('release') accepted, ? in ('release','drift') invoked, ?='directory' waited").get(sample.action,sample.action,sample.lock) as {accepted:number;invoked:number;waited:number};
    expect(success,sample.action).toBe(sample.success);
    expect(Number(success)).toBe(model.accepted);
    expect(called).toBe(model.invoked);
    expect(Number(waits.length>0)).toBe(model.waited);
    expect([...new Set(waits)]).toEqual(sample.lock==="directory"?[fixture.admissionResource]:[]);
    if(sample.action==="cancel")expect(failure).toBe(stop);
    if(sample.action==="held"){
     expect((failure as Error).message).toBe("Resource lease timed out: "+resource);
     expect(Reflect.get(failure as Error,"resourceLeaseAdmission")).toBeUndefined();
    }
    if(sample.action==="drift")expect((failure as Error).message).toContain("journal changed");
   }finally{release();holder?.release();gate.close(true);}
  }
 }finally{oracle.close(true);rmSync(scratch,{recursive:true,force:true});}
}),30000);
