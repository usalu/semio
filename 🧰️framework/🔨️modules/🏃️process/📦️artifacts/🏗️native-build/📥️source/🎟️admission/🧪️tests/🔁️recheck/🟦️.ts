import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{admitCargoSourceBoundariesV1,storageRecheckGateV1}from"../../🟦️.ts";
import{readCargoCompilationGraphV1}from"../../../../🕸️units/🟦️.ts";
import graphCorpus from"../../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json"with{type:"json"};
import type{ResourceLeaseControl}from"../../../../../../🔒️leases/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};

const policy={version:1 as const,maxSourceBytes:65536,maxSourceFiles:64,maxActiveLines:100};
const control:ResourceLeaseControl={signal:new AbortController().signal,remainingMilliseconds:()=>60000,async publish(){},async yieldContinuation(){}};
function graphOf(units:number){const raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!;return readCargoCompilationGraphV1({version:1,units:Array.from({length:units},()=>({...structuredClone(raw),dependencies:[]})),roots:Array.from({length:units},(_,index)=>index)},graphCorpus.metadata,"/repo");}

test("source admission settles storage once per unit boundary and once at completion instead of once per read",async()=>{
 const db=new Database(":memory:");try{for(const row of corpus.sequence.cases){const source=corpus.source+Array.from({length:row.embedded},(_,index)=>`const _: &str = include_str!("data${index}");\n`).join(""),events:string[]=[];await admitCargoSourceBoundariesV1(graphOf(row.units),"/repo",policy,control,{async recheck(received,phase){expect(received).toBe(control);events.push("recheck:"+phase);},async cfg(){events.push("cfg");return[];},async read(path){events.push("read");return new TextEncoder().encode(/data\d+$/u.test(path)?"x":source);}});const expected=JSON.parse((db.query<{value:string},[number,number]>("with recursive u(n) as (select 1 union all select n+1 from u where n<?1), r(k) as (select 1 union all select k+1 from r where k<?2), steps as (select n*1000000 ord,'recheck:unit' v from u union all select n*1000000+1,'cfg' from u union all select n*1000000+1+k,'read' from u,r union all select 2000000000,'recheck:complete') select json_group_array(v) value from (select v from steps order by ord)").get(row.units,row.embedded+1))!.value);expect(events,row.id).toEqual(expected);expect(events.filter(value=>value.startsWith("recheck")).length,row.id).toBe(row.units+1);}}finally{db.close();}
});

test("storage recheck gate follows an independent SQLite throttle oracle and always settles completion",async()=>{
 const ajv=new Ajv({strict:false}),db=new Database(":memory:");expect(ajv.validate({type:"integer",minimum:1},schema.definitions.storageRecheck.const.intervalMilliseconds)).toBe(true);try{for(const row of corpus.gate.cases){let clock=0,rechecks=0;const gate=storageRecheckGateV1(row.interval,()=>clock,async()=>{rechecks++;});db.exec("drop table if exists calls;create table calls(i integer primary key,phase text,at integer)");row.calls.forEach(([phase,at],index)=>db.query("insert into calls values(?,?,?)").run(index+1,phase,at));for(const[phase,at]of row.calls){clock=at as number;await gate(phase as"unit"|"complete");}const oracle=db.query<{n:number},[number]>("with recursive r(i,last,fired) as (select 1,at,1 from calls where i=1 union all select c.i,case when c.phase='complete' or c.at-r.last>=?1 then c.at else r.last end,(c.phase='complete' or c.at-r.last>=?1) from r join calls c on c.i=r.i+1) select sum(fired) n from r").get(row.interval)!;expect(rechecks,row.id).toBe(oracle.n);expect(rechecks,row.id).toBe(row.expected);}}finally{db.close();}
});

test("storage recheck gate surfaces a failed recheck and retries on the next call",async()=>{
 let attempts=0;const gate=storageRecheckGateV1(1000,()=>0,async()=>{attempts++;if(attempts===1)throw Error("allocation retired");});await expect(gate("unit")).rejects.toThrow("allocation retired");await gate("unit");expect(attempts).toBe(2);
});
