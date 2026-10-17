import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{admitCargoSourceBoundariesV1}from"../🟦️.ts";
import{readCargoCompilationGraphV1}from"../../../🕸️units/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import graphSchema from"../../../🕸️units/🧬️schema/🔣️.json"with{type:"json"};
import graphCorpus from"../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};

test("neutral source admission fills every actual compiler unit even when package closure keys coincide",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"cargo:duplicate-source-admission",0),db=new Database(":memory:"),validate=new Ajv({strict:false}).compile(graphSchema);try{for(const row of corpus.cases){const raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!,units=row.flags.map(()=>({...structuredClone(raw),dependencies:[]})),graph=readCargoCompilationGraphV1({version:1,units,roots:units.map((_,index)=>index)},graphCorpus.metadata,"/repo");graph.units.forEach((unit,index)=>{unit.compilerFlags=["--cfg",row.flags[index]!];});db.exec("drop table if exists expected;create table expected(id text primary key,cfg text,bytes integer)");for(const unit of graph.units)db.query("insert into expected values(?,?,?)").run(unit.id,JSON.stringify([unit.compilerFlags[1]]),Buffer.byteLength(corpus.source));let cfgCalls=0;await admitCargoSourceBoundariesV1(graph,"/repo",{version:1,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100},control,{async cfg(unit){cfgCalls++;return[unit.compilerFlags[1]!];},async read(path){expect(path).toBe(graph.units[0]!.entry);return new TextEncoder().encode(corpus.source);}});expect(cfgCalls,row.id).toBe(graph.units.length);expect(validate(graph),JSON.stringify(validate.errors)).toBe(true);const actual=graph.units.map(unit=>({id:unit.id,cfg:JSON.stringify(unit.compilerCfg),bytes:unit.sourceInputs.reduce((sum,input)=>sum+input.bytes,0)})).sort((left,right)=>left.id.localeCompare(right.id)),expected=db.query<{id:string;cfg:string;bytes:number},[]>("select id,cfg,bytes from expected order by id").all();expect(actual,row.id).toEqual(expected);for(const unit of graph.units)expect(unit.sourceInputs.length,row.id).toBe(1);}}finally{db.close();}
}));
