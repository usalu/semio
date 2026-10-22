import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {Management,captureProjection,pathContains,type EntryObservation,type ManagementPort,type Command} from "../🟦️.ts";

const observed=(identity:string,directory:boolean,nonempty=false):EntryObservation=>({identity,directory,nodes:[{suffix:"",identity,directory},...(nonempty?[{suffix:"a",identity:"child",directory:false}]:[])]});
const prepare=async(command:Command,expected:EntryObservation|null)=>({destination:command.destination??(command.kind==="retain-entry"?".workspace-events/segments/00000000/command.retained":command.path),physicalRootUnits:20,nodes:expected?.nodes??[]});

test("guided file commands match shared neutral outcomes and independent SQLite",async()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){
  let current:EntryObservation|null=row.initial==="missing"?null:observed("original",row.initial!=="file",row.initial==="nonempty-directory");const events:string[]=[],identifiers:string[]=[];let published=0;const controller=new AbortController();
  const port:ManagementPort={prepare,inspect:async path=>path===row.path?current:(row.destinationExists?observed("occupied",false):null),publish:async command=>{published++;if(command.kind==="retain-entry")current=null;else current=observed("published",command.kind==="create-directory"||current?.directory===true);return {recoveryPath:command.kind==="retain-entry"?".workspace-events/segments/00000000/test.retained":undefined};},record:async event=>{events.push(event.kind);identifiers.push(event.commandId);}};
  const manager=new Management(port);let token; if(row.command==="rename-entry"||row.command==="retain-entry")token=await manager.inspect(row.path,controller.signal);
  if(row.stale)current={...current!,identity:"foreign"};let outcome="committed";
  const command:Command={kind:row.command as Command["kind"],path:row.path,...("destination"in row?{destination:row.destination}:{}),...(row.command==="create-file"?{text:""}:{})};
  try{await manager.execute(command,token,controller.signal,phase=>{if(phase==="publishing"&&row.cancel==="before-publication")controller.abort();if(phase==="complete"&&row.cancel==="after-publication")controller.abort();});}catch(error){outcome=(error as Error).message;}
  const oracle=db.query("SELECT CASE WHEN ?1='before-publication' THEN 'cancelled' WHEN ?4 OR ?5 OR (?2 IN ('create-file','create-directory') AND ?3<>'missing') THEN 'conflict' ELSE 'committed' END AS outcome").get(row.cancel,row.command,row.initial,Number(row.destinationExists),Number(row.stale))as{outcome:string};
  expect(outcome,row.name).toBe(row.expected);expect(outcome,row.name).toBe(oracle.outcome);expect(published,row.name).toBe(Number(outcome==="committed"));if(outcome==="committed"){expect(events).toEqual(["intent","committed"]);expect(new Set(identifiers).size).toBe(1);expect(identifiers[0]).toMatch(/^source-command-[1-9][0-9]*$/u);}
 }}finally{db.close();}
});

test("guided entry revisions reject copied and foreign observations before publication",async()=>{
 let publications=0;const port:ManagementPort={prepare,inspect:async()=>observed("one",false),publish:async()=>{publications++;return{};},record:async()=>{}};const signal=new AbortController().signal;const first=new Management(port),second=new Management(port);const token=await first.inspect("a",signal);
 for(const invalid of [{...token},token]){await expect((invalid===token?second:first).execute({kind:"retain-entry",path:"a"},invalid,signal,()=>{})).rejects.toThrow("conflict");}expect(publications).toBe(0);
});


test("guided command schema and actual owner refuse unknown requests before observation",async()=>{
 const validator=new Ajv({strict:false}).compile(schema.$defs.Request);let reads=0;const manager=new Management({prepare,inspect:async()=>{reads++;return null;},publish:async()=>({}),record:async()=>{}});for(const value of [{kind:"remove-tree",path:"a"},{kind:"create-file",path:"a",force:true},{kind:"retain-entry",path:"a",destination:"foreign"}]){expect(validator(value)).toBe(false);await expect(manager.execute(value as Command,undefined,new AbortController().signal,()=>{})).rejects.toThrow("invalid-command");}expect(reads).toBe(0);
});


test("guided request snapshots remain unchanged across asynchronous inspection",async()=>{
 let release!:()=>void;const waiting=new Promise<void>(resolve=>{release=resolve;}),seen:string[]=[];let published:Command|undefined;const manager=new Management({prepare,inspect:async path=>{seen.push(path);await waiting;return null;},publish:async command=>{published=command;return{};},record:async()=>{}});const request={kind:"create-file",path:"original.txt",text:"original"}as Command;const pending=manager.execute(request,undefined,new AbortController().signal,()=>{});Object.assign(request,{path:"../foreign",text:"changed"});release();await pending;expect(seen).toEqual(["original.txt","original.txt"]);expect(published).toEqual({kind:"create-file",path:"original.txt",text:"original"});expect(Object.isFrozen(published)).toBe(true);
});


test("complete tree projections obey shared node, ancestry and physical path limits",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.trees){
  const nodes=Array.from({length:row.nodeCount},(_,i)=>row.nodes[i]??{suffix:String(i).padStart(4,"0"),identity:String(i),directory:false});let outcome="accepted";try{captureProjection({destination:row.destination,physicalRootUnits:row.physicalRootUnits,nodes});}catch(error){outcome=(error as Error).message;}
  const result=db.query(`WITH RECURSIVE positions(n) AS(SELECT 1 UNION ALL SELECT n+1 FROM positions WHERE n<256), nodes AS(SELECT CAST(key AS INT) i,json_extract(value,'$.suffix') suffix,json_extract(value,'$.directory') directory FROM json_each(?1)), ordered AS(SELECT suffix,i,lag(suffix) OVER(ORDER BY i) previous FROM nodes), paths AS(SELECT ?2||CASE WHEN suffix='' THEN '' ELSE '/'||suffix END path FROM nodes), parents AS(SELECT *,COALESCE((SELECT MAX(n) FROM positions WHERE substr(suffix,n,1)='/'),0) slash FROM nodes) SELECT CASE WHEN ?4>4096 THEN 'too-many-entries' WHEN ?2 LIKE '.workspace-events/%' AND (substr(?2,1,27)<>'.workspace-events/segments/' OR substr(?2,28,8) GLOB '*[^0-9a-f]*' OR length(substr(?2,28,8))<>8 OR substr(?2,36,1)<>'/' OR substr(?2,-9)<>'.retained' OR length(?2) NOT BETWEEN 46 AND 109 OR substr(?2,37,length(?2)-45) GLOB '*[^0-9a-z-]*') THEN 'invalid-path' WHEN EXISTS(SELECT 1 FROM nodes WHERE suffix LIKE '../%' OR suffix LIKE '%/../%' OR suffix LIKE '/%' OR suffix LIKE '%//%' OR suffix IN ('.','..')) OR EXISTS(SELECT 1 FROM paths WHERE ?3+1+(SELECT SUM(CASE WHEN unicode(substr(path,n,1))>65535 THEN 2 ELSE 1 END) FROM positions WHERE n<=length(path))>256) THEN 'invalid-path' WHEN (SELECT COUNT(*) FROM nodes)<>(SELECT COUNT(DISTINCT suffix) FROM nodes) OR (SELECT suffix FROM nodes WHERE i=0)<>'' OR EXISTS(SELECT 1 FROM ordered WHERE i>0 AND suffix<=previous) OR EXISTS(SELECT 1 FROM parents child WHERE i>0 AND NOT EXISTS(SELECT 1 FROM nodes parent WHERE parent.directory=1 AND parent.suffix=CASE WHEN child.slash=0 THEN '' ELSE substr(child.suffix,1,child.slash-1) END)) THEN 'conflict' ELSE 'accepted' END outcome`).get(JSON.stringify(nodes),row.destination,row.physicalRootUnits,row.nodeCount)as{outcome:string};
  expect(outcome,row.name).toBe(row.expected);expect(result.outcome,row.name).toBe(row.expected);
 }}finally{db.close();}
});


test("management document footprints use exact path segments",()=>{const db=new Database(":memory:");try{for(const row of fixture.hierarchy){const oracle=db.query("select ?1=?2 or substr(?1,1,length(?2)+1)=?2||'/' result").get(row.path,row.root)as{result:number};expect(pathContains(row.root,row.path)).toBe(row.contains);expect(Boolean(oracle.result)).toBe(row.contains);}}finally{db.close();}});
