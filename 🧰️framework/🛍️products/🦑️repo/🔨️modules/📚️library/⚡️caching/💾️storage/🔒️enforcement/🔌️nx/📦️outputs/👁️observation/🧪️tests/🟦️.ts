import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import rawCorpus from "./🔣️.json";
import {planOutputObservationV1,createRepositoryOutputObservationV1,type OutputObservationV1,type OutputObservationPortsV1,type OutputObservationEventV1,type OutputObservationScopeV1} from "../🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

type ScopeCase={name:string;root:string;expressions:string[];entries:Record<string,OutputObservationV1>;scopes:OutputObservationScopeV1[];refused:boolean};
const validateCase=new Ajv({strict:false}).compile<ScopeCase>({$defs:schema.$defs,$ref:"#/$defs/Case"});
const corpus:readonly ScopeCase[]=Object.freeze(rawCorpus.map(row=>{if(!validateCase(row))throw Error("Closed output observation fixture required: "+JSON.stringify(validateCase.errors));const accepted:ScopeCase=row;return accepted;}));
const issued=<T,>(body:(control:ReturnType<typeof createIssuedResourceControlV1>)=>Promise<T>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:output-observation",maximumElapsedMilliseconds:30000},{},Date.now()),original=>body(createIssuedResourceControlV1(original,"output-observation:law",20000)));
const backend=(index=0)=>{
 const entries=new Map<string,OutputObservationV1>(Object.entries(corpus[index]!.entries)),records:{scope:OutputObservationScopeV1;receive:(error:unknown,events:readonly OutputObservationEventV1[])=>void;live:boolean;buffer:OutputObservationEventV1[]}[]=[],order:string[]=[];
 let start:((scope:OutputObservationScopeV1)=>void|Promise<void>)|undefined;
 const ports:OutputObservationPortsV1={observe:async path=>entries.get(path)??{kind:"missing",identity:""},watch:async(scope,receive)=>{await start?.(scope);const row={scope,receive,live:true,buffer:[] as OutputObservationEventV1[]};records.push(row);order.push("start:"+scope.path+":"+scope.identity);return{flush:()=>row.buffer.splice(0),stop:()=>{row.live=false;order.push("stop:"+scope.path+":"+scope.identity);}};}};
 return{entries,records,order,ports,onStart(value:typeof start){start=value;}};
};

test("closed output scope vectors retain membership and agree with SQLite ancestors",async()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Case"}),db=new Database(":memory:");
 try{for(const row of corpus){expect(validate(row),JSON.stringify(validate.errors)).toBe(true);const ask=()=>planOutputObservationV1(row.root,row.expressions,async path=>(row.entries as Record<string,OutputObservationV1>)[path]??{kind:"missing",identity:""},()=>{});if(row.refused){await expect(ask(),row.name).rejects.toThrow();continue;}const actual=await ask();expect(actual.scopes,row.name).toEqual(row.scopes);
 if(row.name==="source receipt"){const oracle=db.query("SELECT key path,'directory' mode,json_extract(value,'$.identity') identity FROM json_each(?2) WHERE json_extract(value,'$.kind')='directory' AND instr(?1,key||'/')=1 ORDER BY key").all(row.root+"/"+row.expressions[0],JSON.stringify(row.entries));expect<unknown>(actual.scopes).toEqual(oracle);}
 }}finally{db.close();}
});

test("noncanonical roots and overlong missing coordinates are refused before watching",async()=>{
 const observe=async():Promise<OutputObservationV1>=>({kind:"directory",identity:"r"});
 for(const root of ["/repo/../outside","/repo//other","C:/repo/./other"])await expect(planOutputObservationV1(root,["file"],observe,()=>{})).rejects.toThrow("Canonical");
 await expect(planOutputObservationV1("/repo",["a/"+"x".repeat(250)],async path=>path==="/repo"?{kind:"directory",identity:"r"}:{kind:"missing",identity:""},()=>{})).rejects.toThrow("Bounded");
});

test("missing components descend without a recursive workspace fallback",()=>issued(async control=>{
 const model=backend(2),events:OutputObservationEventV1[]=[],observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{events.push(...rows);}});
 await observer.register(corpus[2]!.expressions);model.entries.set("/repo/app/new",{kind:"directory",identity:"n"});model.entries.set("/repo/app/new/deep",{kind:"directory",identity:"d"});model.entries.set("/repo/app/new/deep/result.json",{kind:"file",identity:"f"});observer.receiveSource([{path:"app/new",type:"create"}]);await observer.flush();
 expect(model.records.filter(row=>row.live).map(row=>row.scope.path)).toEqual(["/repo","/repo/app","/repo/app/new","/repo/app/new/deep"]);expect(model.records.every(row=>row.scope.mode==="directory")).toBe(true);expect(events.some(event=>event.path==="app/new")).toBe(true);
 const count=model.records.length;model.entries.set("/repo/.cache/unrelated",{kind:"directory",identity:"u"});observer.receiveSource([{path:"unrelated.json",type:"update"}]);await observer.flush();expect(model.records.length).toBe(count);await observer.stop();
}));

test("link retargeting activates new physical coverage before old coverage closes",()=>issued(async control=>{
 const model=backend(5),events:OutputObservationEventV1[]=[],observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{events.push(...rows);}});
 await observer.register(corpus[5]!.expressions);const old=model.records.find(row=>row.scope.path==="/repo/.cache/unit")!;model.entries.set("/repo/app/output",{kind:"link",identity:"l2",target:"../.cache/next"});model.entries.set("/repo/.cache/next",{kind:"directory",identity:"n"});observer.receiveSource([{path:"app/output",type:"update"}]);await observer.flush();
 expect(model.order.indexOf("start:/repo/.cache/next:n")).toBeLessThan(model.order.indexOf("stop:/repo/.cache/unit:u"));expect(model.records.filter(row=>row.live&&row.scope.mode==="recursive").map(row=>row.scope.path)).toEqual(["/repo/.cache/next"]);const count=events.length;old.receive(null,[{path:".cache/unit/late",type:"update"}]);await observer.flush();expect(events.length).toBe(count);await observer.stop();
}));

test("an inode replacement during start is reobserved before coverage is accepted",()=>issued(async control=>{
 const model=backend(1),events:OutputObservationEventV1[]=[];let changed=false;model.onStart(scope=>{if(scope.path==="/repo/.cache/unit"&&!changed){changed=true;model.entries.set(scope.path,{kind:"directory",identity:"u2"});}});
 const observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{events.push(...rows);}});await observer.register(corpus[1]!.expressions);expect(model.records.filter(row=>row.live&&row.scope.mode==="recursive").map(row=>row.scope.identity)).toEqual(["u2"]);expect(model.records.filter(row=>row.scope.identity==="u").every(row=>!row.live)).toBe(true);expect(events.some(row=>row.path===".cache/unit")).toBe(true);await observer.stop();
}));

test("a failed extension rolls back only new handles and fails future responses closed",()=>issued(async control=>{
 const model=backend(),primary=Object.freeze({start:"refused"}),observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:()=>{}});await observer.register(["app/receipt.json"]);const original=model.records.slice();model.entries.set("/repo/.cache",{kind:"directory",identity:"c"});model.entries.set("/repo/.cache/unit",{kind:"directory",identity:"u"});model.onStart(scope=>{if(scope.mode==="recursive")throw primary;});
 await expect(observer.register([".cache/unit"])).rejects.toBe(primary);expect(original.every(row=>row.live)).toBe(true);expect(model.records.filter(row=>!original.includes(row)).every(row=>!row.live)).toBe(true);await expect(observer.flush()).rejects.toBe(primary);await expect(observer.stop()).rejects.toBe(primary);expect(model.records.every(row=>!row.live)).toBe(true);
}));

test("flush drains events enqueued by delivery and rejects reentrant awaited admission",()=>issued(async control=>{
 const model=backend(),events:OutputObservationEventV1[]=[];let injected=false;const observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:async(error,rows)=>{if(error)return;events.push(...rows);if(rows.some(row=>row.path==="app/first.json")&&!injected){injected=true;observer.receiveSource([{path:"app/second.json",type:"update"}]);await expect(observer.register(["app/third.json"])).rejects.toThrow("own admission");}}});
 await observer.register(["app/receipt.json"]);model.records[0]!.buffer.push({path:"app/first.json",type:"update"});await observer.flush();expect(events.map(row=>row.path)).toEqual(["app/first.json","app/second.json"]);await observer.stop();
}));

test("physical target changes invalidate the logical alias recorded by Nx",()=>issued(async control=>{
 const model=backend(5),events:OutputObservationEventV1[]=[],observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{events.push(...rows);}});await observer.register(corpus[5]!.expressions);model.records.find(row=>row.scope.mode==="recursive")!.buffer.push({path:".cache/unit/surface.bin",type:"update"});await observer.flush();expect(events.some(row=>row.path==="app/output/surface.bin")).toBe(true);await observer.stop();
}));

test("ambiguous root membership invalidates recorded endpoints and raw event names stay literal",()=>issued(async control=>{
 const model=backend(),events:OutputObservationEventV1[]=[],observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{events.push(...rows);}});await observer.register(["app/receipt.json"]);observer.receiveSource([{path:".",type:"update"},{path:"!source[raw].ts",type:"update"}]);await observer.flush();expect(events.some(row=>row.path==="app/receipt.json")).toBe(true);expect(events.some(row=>row.path==="!source[raw].ts")).toBe(true);await observer.stop();
}));

test("queued event batches own their rows and length before a backend reuses them",()=>issued(async control=>{
 const model=backend(),received:OutputObservationEventV1[]=[],observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:(_error,rows)=>{received.push(...rows);}});await observer.register(["app/receipt.json"]);const row={path:"app/first.json",type:"update" as const},batch=[row];observer.receiveSource(batch);row.path="app/reused.json";batch.push({path:"app/extra.json",type:"update"});await observer.flush();expect(received.map(row=>row.path)).toEqual(["app/first.json"]);await observer.stop();
}));

test("a settled delivery does not leave reentrant authority on later scheduled work",()=>issued(async control=>{
 const model=backend();let followup:Promise<void>|undefined;const observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:model.ports,receive:()=>{followup=new Promise<void>((accept,reject)=>setImmediate(()=>{void observer.register(["app/later.json"]).then(accept,reject);}));}});await observer.register(["app/receipt.json"]);observer.receiveSource([{path:"app/first.json",type:"update"}]);await observer.flush();await followup;await observer.stop();
}));

test("cancellation during subscription closes accepted handles with the exact Original reason",()=>issued(async control=>{
 const stop=new AbortController(),phase=createRequestedIssuedResourceControlV1(control,0,stop.signal),model=backend();let release!:()=>void,enter!:()=>void;const gate=new Promise<void>(done=>release=done),entered=new Promise<void>(done=>enter=done),primary=Object.freeze({cancel:"output-subscription"});model.onStart(async()=>{enter();await gate;});const observer=createRepositoryOutputObservationV1({root:"/repo",control:phase,ports:model.ports,receive:()=>{}}),work=observer.register(["app/receipt.json"]);await entered;stop.abort(primary);release();await expect(work).rejects.toBe(primary);await expect(observer.stop()).rejects.toBe(primary);expect(model.records.every(row=>!row.live)).toBe(true);
}));

test("large event expansion yields to cancellation before delivering a partial batch",()=>issued(async control=>{
 const stop=new AbortController(),phase=createRequestedIssuedResourceControlV1(control,0,stop.signal),model=backend();let delivered=0;const primary=Object.freeze({cancel:"output-event-expansion"}),observer=createRepositoryOutputObservationV1({root:"/repo",control:phase,ports:model.ports,receive:(error,rows)=>{if(!error)delivered+=rows.length;}});await observer.register(["app/receipt.json"]);setImmediate(()=>stop.abort(primary));observer.receiveSource(Array.from({length:10000},(_,index)=>({path:"app/file"+index,type:"update" as const})));await expect(observer.flush()).rejects.toBe(primary);expect(delivered).toBe(0);await expect(observer.stop()).rejects.toBe(primary);
}));

test("stop closes new admission while draining an accepted registration",()=>issued(async control=>{
 const entries=corpus[0]!.entries as Record<string,OutputObservationV1>;let enter!:()=>void,release!:()=>void,stops=0;
 const entered=new Promise<void>(done=>enter=done),gate=new Promise<void>(done=>release=done),ports:OutputObservationPortsV1={observe:async path=>entries[path]??{kind:"missing",identity:""},watch:async()=>{enter();await gate;return{flush:()=>[],stop:()=>{stops++;}};}};
 const observer=createRepositoryOutputObservationV1({root:"/repo",control,ports,receive:()=>{}}),accepted=observer.register(["app/receipt.json"]);await entered;const stopped=observer.stop();release();await accepted;await stopped;expect(stops).toBe(2);await expect(observer.register(["app/other.json"])).rejects.toThrow("admission closed");
}));

test("terminal backend error retains its identity and drains asynchronous error delivery",()=>issued(async control=>{
 let receive!:(error:unknown,events:readonly any[])=>void,release!:()=>void,entered!:()=>void,delivered=false;
 const gate=new Promise<void>(done=>release=done),ready=new Promise<void>(done=>entered=done),primary=Object.freeze({backend:"failed"}),entries=corpus[0]!.entries as Record<string,OutputObservationV1>;
 const observer=createRepositoryOutputObservationV1({root:"/repo",control,ports:{observe:async path=>entries[path]??{kind:"missing",identity:""},watch:async(_scope,callback)=>{receive=callback;return{flush:()=>[],stop:()=>{}};}},receive:async error=>{if(error){expect(error).toBe(primary);entered();await gate;delivered=true;}}});
 await observer.register(["app/receipt.json"]);receive(primary,[]);await ready;let ended=false;const stopped=observer.stop().catch(error=>{expect(error).toBe(primary);ended=true;});await new Promise<void>(done=>setImmediate(done));expect(ended).toBe(false);release();await stopped;expect(delivered).toBe(true);
}));
