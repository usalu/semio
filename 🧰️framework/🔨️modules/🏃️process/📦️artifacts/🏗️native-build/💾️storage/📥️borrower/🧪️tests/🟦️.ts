import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,rmSync,linkSync,unlinkSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as pause} from "node:timers/promises";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import * as leases from "../../../../../🔒️leases/🟦️.ts";
import * as borrower from "../🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
declareBuildTestEnvelopeV1({owner:"native-borrower-terminal",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
test("terminal release neutral laws agree with independent SQLite",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{expect(validate(corpus)).toBe(true);for(const row of corpus.cases){const found=db.query("select not (?1 or ?2) and ?3 and ?4 body, ?3 and ?4 release").get(Number(row.expired),Number(row.cancelled),Number(row.sameLease),Number(row.samePin)) as {body:number;release:number};expect({body:Boolean(found.body),release:Boolean(found.release)}).toEqual(row.expected);}}finally{db.close();}
});
test("accepted real borrower releases expired and cancelled guards without renewing body authority",()=>withBuildTestInvocationV1(async original=>{
 const api=borrower as unknown as Record<string,unknown>;
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"borrower-"));
 try{for(const kind of ["expired","cancelled"] as const){
  const stop=new AbortController(),parent=createIssuedResourceControlV1(original,"native:terminal-"+kind,kind==="expired"?2000:0),control=createRequestedIssuedResourceControlV1(parent,0,stop.signal),resource="native-fixed-image:"+join(directory,kind),lease=await leases.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control}),owner=borrower.acceptNativeImageBorrowerV1({lease,leaseDirectory:directory,resource,control,leaseControl:control}),db=new Database(leases.resourceLeasePathV1(directory,resource));
  try{db.exec("pragma busy_timeout=0");expect(()=>db.exec("begin exclusive")).toThrow();if(kind==="expired")await pause((control.remainingMilliseconds()??0)+20);else stop.abort(Error("original body cancelled"));expect(()=>borrower.withNativeImageBorrowerOperationV1(owner,control,async()=>0)).toThrow();await borrower.closeNativeImageBorrowerV1(owner);expect((api.nativeImageBorrowerReleasedV1 as (owner:borrower.NativeImageBorrowerV1)=>boolean)(owner)).toBe(true);expect(()=>borrower.withNativeImageBorrowerOperationV1(owner,control,async()=>0)).toThrow();db.exec("begin exclusive;rollback");}finally{db.close();}
 }}finally{rmSync(directory,{recursive:true,force:true});}
}),10000);
test("terminal close drains accepted descendants while preventing new births",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"borrower-drain-")),control=createIssuedResourceControlV1(original,"native:terminal-drain",0),resource="native-fixed-image:"+directory,lease=await leases.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control}),owner=borrower.acceptNativeImageBorrowerV1({lease,leaseDirectory:directory,resource,control,leaseControl:control}),db=new Database(leases.resourceLeasePathV1(directory,resource));
 try{let done!:()=>void,entered!:()=>void;const gate=new Promise<void>(resolve=>done=resolve),start=new Promise<void>(resolve=>entered=resolve),work=borrower.withNativeImageBorrowerOperationV1(owner,control,async actual=>{entered();await gate;return borrower.withNativeImageBorrowerOperationV1(owner,actual,async()=>7);});await start;const closing=borrower.closeNativeImageBorrowerV1(owner);let closed=false;void closing.then(()=>closed=true);await Promise.resolve();expect(closed).toBe(false);expect(()=>borrower.withNativeImageBorrowerOperationV1(owner,control,async()=>0)).toThrow();db.exec("pragma busy_timeout=0");expect(()=>db.exec("begin exclusive")).toThrow();done();expect(await work).toBe(7);await closing;db.exec("begin exclusive;rollback");}finally{db.close();rmSync(directory,{recursive:true,force:true});}
}));


const retainedTerminalFixtures:unknown[]=[];
/** 🧫️ Observes the same namespace attack on independent SQLite connections without lease or borrower calls. */
function sqliteNamespaceControl(directory:string):void{
 const path=join(directory,"sqlite-control"),alias=join(directory,"sqlite-alias"),guard=new Database(path),observer=new Database(path);let blocked:unknown,after:unknown;
 try{try{guard.exec("create table payload(n)");guard.exec("begin");guard.query("select count(*) from payload").get();linkSync(path,alias);try{observer.exec("begin exclusive");blocked={error:null,active:observer.inTransaction};if(observer.inTransaction)observer.exec("rollback");}catch(error){blocked={error:(error as {code?:string}).code,errno:(error as {errno?:number}).errno,active:observer.inTransaction};}unlinkSync(alias);}finally{guard.close();}try{observer.exec("begin exclusive");after={error:null,active:observer.inTransaction};if(observer.inTransaction)observer.exec("rollback");}catch(error){after={error:(error as {code?:string}).code,errno:(error as {errno?:number}).errno,active:observer.inTransaction};}}finally{observer.close();}
}
test("terminal token clean release and hostile physical pin retain exact custody",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const clean=mkdtempSync(join(base,"terminal-clean-"));sqliteNamespaceControl(clean);
 const control=createIssuedResourceControlV1(original,"native:terminal-pin",0),resource="native-fixed-image:"+clean,lease=await leases.acquireResourceLease({directory:clean,resource,mode:"shared",signal:control.signal,control}),db=new Database(leases.resourceLeasePathV1(clean,resource)),release=lease.release;let calls=0;lease.release=()=>{calls++;release.call(lease);};const token=leases.acceptResourceLeaseTerminalV1(lease,clean,resource,control);
 try{db.exec("pragma busy_timeout=0");expect(()=>db.exec("begin exclusive")).toThrow();expect(()=>leases.releaseResourceLeaseTerminalV1({...token})).toThrow();expect(Object.keys(token)).toEqual([]);expect(()=>leases.acceptResourceLeaseTerminalV1(lease,clean,resource,control)).toThrow();expect(calls).toBe(0);leases.releaseResourceLeaseTerminalV1(token);leases.releaseResourceLeaseTerminalV1(token);expect(calls).toBe(1);db.exec("begin exclusive");expect(db.inTransaction).toBe(true);db.exec("rollback");expect(db.inTransaction).toBe(false);}finally{db.close();rmSync(clean,{recursive:true,force:true});}
 const directory=mkdtempSync(join(base,"terminal-pin-")),hostileResource="native-fixed-image:"+directory,held=await leases.acquireResourceLease({directory,resource:hostileResource,mode:"shared",signal:control.signal,control}),path=leases.resourceLeasePathV1(directory,hostileResource),observer=new Database(path),actual=held.release;let attempts=0;held.release=()=>{attempts++;actual.call(held);};const captured=leases.acceptResourceLeaseTerminalV1(held,directory,hostileResource,control),alias=join(directory,"alias");
 observer.exec("pragma busy_timeout=0");expect(()=>observer.exec("begin exclusive")).toThrow();linkSync(path,alias);expect(()=>leases.releaseResourceLeaseTerminalV1(captured)).toThrow("pin");expect(attempts).toBe(0);expect(()=>leases.requireResourceLeaseReadCustodyV1(held,directory,hostileResource,control)).not.toThrow();retainedTerminalFixtures.push(Object.freeze({held,captured,observer,path,alias}));
}));

test("failed actual terminal release retains unresolved borrower ownership and its kernel guard",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"terminal-unresolved-")),control=createIssuedResourceControlV1(original,"native:terminal-unresolved",0),resource="native-fixed-image:"+directory,lease=await leases.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control}),failure=Error("fixture actual wrapper release failed");
 lease.release=()=>{throw failure;};
 const owner=borrower.acceptNativeImageBorrowerV1({lease,leaseDirectory:directory,resource,control,leaseControl:control}),db=new Database(leases.resourceLeasePathV1(directory,resource));
 try{db.exec("pragma busy_timeout=0");const closing=borrower.closeNativeImageBorrowerV1(owner);await expect(closing).rejects.toBe(failure);expect(borrower.nativeImageBorrowerReleasedV1(owner)).toBe(false);expect(borrower.closeNativeImageBorrowerV1(owner)).toBe(closing);expect(()=>db.exec("begin exclusive")).toThrow();expect(()=>borrower.withNativeImageBorrowerOperationV1(owner,control,async()=>0)).toThrow();}finally{db.close();}
}));
