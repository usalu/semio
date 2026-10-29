//! 🛑️ Current platform orchestration must join its authentic physical body before absent or successful settlement.
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{existsSync,lstatSync,mkdtempSync,mkdirSync,readFileSync,rmSync,rmdirSync}from"node:fs";
import{join}from"node:path";
import ts from"typescript";
import * as routing from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifacts)throw Error("Original test artifact directory required");
const path=new URL("../🟦️.ts",import.meta.url),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.ES2022,true),owner=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="retireNativePlatformFundedChildStorageV1");
if(!owner)throw Error("Actual platform child retirement function required");
const program=ts.transpileModule(owner.getText(tree).replace(/^export /,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;

test("neutral current platform stops the accepted physical body before child settlement",async()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const root=mkdtempSync(join(artifacts!,"child-stop-")),namespace=join(root,"namespace"),directory=join(namespace,"child"),events:string[]=[],captureFailure=Error("capture failed"),physicalFailure=Error("physical retirement failed"),db=new Database(":memory:");
  let settlement="none",held=1,bodyStopped=false,bodyGateRefused=false,caught:unknown;
  if(row.mode!=="absent"&&row.mode!=="reappeared"){mkdirSync(namespace);mkdirSync(directory);}
  try{
   await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"fixture:child-stop",maximumElapsedMilliseconds:0},{fixture:"child-stop"},Date.now()),async invocation=>{
    const control=routing.createIssuedResourceControlV1(invocation,"fixture:child-stop",0),parent={root},request={directory},reservation={allocation:{phase:"committed",kind:"compiler-state",child:request,namespace:{directory:namespace}},backing:{root}},cleanup=Object.freeze({}),writer=Object.freeze({});
    const authorize=(enrollment:Parameters<typeof routing.checkAcceptedPhysicalCleanupV1>[0])=>{routing.checkAcceptedPhysicalCleanupV1(enrollment);expect(bodyStopped).toBe(true);};
    const ports={
     requireNativePlatformImageV1:(image:unknown,actual:unknown)=>{expect(image).toBe(parent);expect(actual).toBe(control);},
     requireIssuedResourceControlInvocationV1:routing.requireIssuedResourceControlInvocationV1,
     requireIssuedResourceControlRefinementV1:routing.requireIssuedResourceControlRefinementV1,
     readNativeStorageReservationPolicyV1:()=>({root,leaseDirectory:join(root,"leases")}),
     withNativePlatformImageOperationV1:(_image:unknown,actual:unknown,run:(value:unknown)=>Promise<unknown>)=>run(actual),
     withIssuedResourceOperationV1:routing.withIssuedResourceOperationV1,
     withAcceptedPhysicalCleanupV1:routing.withAcceptedPhysicalCleanupV1,
     beginAcceptedPhysicalCleanupV1:routing.beginAcceptedPhysicalCleanupV1,
     existsSync,
     acceptNativeStorageCleanupV1:async(_reservation:unknown,_original:unknown,actual:typeof control,_body:unknown,enrollment:Parameters<typeof routing.checkAcceptedPhysicalCleanupV1>[0])=>{
      events.push("capture");
      expect(()=>routing.checkAcceptedPhysicalCleanupV1(enrollment)).toThrow("Accepted physical body must stop before cleanup");bodyGateRefused=true;
      if(row.mode==="capture-failed")throw captureFailure;
      if(row.mode==="reappeared"){mkdirSync(namespace);mkdirSync(directory);events.push("namespace-reappeared");}
      void routing.withIssuedResourceOperationV1(actual,async()=>{await new Promise<void>(resolve=>setTimeout(resolve,20));bodyStopped=true;events.push("body-child-stopped");});
      return cleanup;
     },
     requireNativeStorageChildCleanupV1:(actual:unknown)=>{expect(actual).toBe(cleanup);return request;},
     withNativePlatformChildStorageV1:async(_parent:unknown,_options:unknown,run:(image:unknown)=>Promise<unknown>)=>run({withExclusiveMaintenance:async(_request:unknown,body:(port:unknown,custody:unknown)=>Promise<unknown>)=>{await body({},writer);return{acquired:true};}}),
     nativePlatformImageBackendV1:()=>"fixture",
     nativeStorageOperationsV1:()=>({retireFundedChild:async()=>{if(row.mode==="retirement-failed"){events.push("physical-refused");throw physicalFailure;}rmdirSync(directory);events.push("physical-retired");}}),
     settleAbsentNativeStorageChildV1:async(actual:unknown,enrollment:Parameters<typeof routing.checkAcceptedPhysicalCleanupV1>[0])=>{
      expect(actual).toBe(cleanup);authorize(enrollment);
      if(existsSync(namespace)){events.push("absence-refused");throw Error("namespace is still present");}
      expect(()=>lstatSync(namespace)).toThrow();events.push("absent-settled");settlement="absent";held=0;
     },
     settleSuccessfulNativeStorageChildV1:async(actual:unknown,enrollment:Parameters<typeof routing.checkAcceptedPhysicalCleanupV1>[0],custody:unknown)=>{
      expect(actual).toBe(cleanup);expect(custody).toBe(writer);authorize(enrollment);
      if(existsSync(directory)){events.push("success-refused");throw Error("physical child remains");}
      expect(()=>lstatSync(directory)).toThrow();events.push("success-settled");settlement="successful";held=0;
     },
     withNativeStorageCleanupV1:async()=>{throw Error("Unexpected retained-reader branch");},
    };
    const retire=new Function(...Object.keys(ports),program+";return retireNativePlatformFundedChildStorageV1;")(...Object.values(ports))as(...args:unknown[])=>Promise<void>;
    try{await retire(parent,reservation,control,control,invocation);}catch(error){caught=error;}
    if(caught&&row.expectedFailure===null)throw caught;
   });
  }catch(error){caught??=error;}
  try{
   expect(bodyGateRefused).toBe(true);expect(events).toEqual(row.events);expect(settlement).toBe(row.expectedSettlement);
   expect(caught instanceof Error?caught.message:null).toBe(row.expectedFailure);
   if(row.mode==="capture-failed")expect(caught).toBe(captureFailure);
   if(row.mode==="retirement-failed")expect(caught).toBe(physicalFailure);
   db.exec("create table events(position integer primary key,name text)");
   events.forEach((name,position)=>db.query("insert into events values(?,?)").run(position,name));
   const oracle=db.query<{held:number;ordered:number},[]>( "select case when exists(select 1 from events where name in ('absent-settled','success-settled')) then 0 else 1 end held, not exists(select 1 from events c where c.name in ('absent-settled','success-settled') and not exists(select 1 from events s where s.name='body-child-stopped' and s.position<c.position)) ordered").get();
   expect(held).toBe(row.held);expect(oracle?.held).toBe(row.held);expect(oracle?.ordered).toBe(1);
  }finally{db.close();rmSync(root,{recursive:true,force:true});}
 }
},10000);

test("actual platform awaits body stop before either namespace settlement branch",()=>{
 const calls:ts.CallExpression[]=[];function visit(node:ts.Node){if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);}visit(owner!);
 const begin=calls.find(node=>node.expression.getText(tree)==="beginAcceptedPhysicalCleanupV1")!;
 expect(begin).toBeDefined();expect(ts.isAwaitExpression(begin.parent)).toBe(true);
 for(const name of["settleAbsentNativeStorageChildV1","settleSuccessfulNativeStorageChildV1"]){const call=calls.find(node=>node.expression.getText(tree)===name)!;expect(call).toBeDefined();expect(call.pos).toBeGreaterThan(begin.pos);expect(ts.isAwaitExpression(call.parent)).toBe(true);}
});
