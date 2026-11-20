/** 🔌️ Contract: the attempt of the load module runs through the fleet: a complete generation is installed (graph, receipt in this machine's spelling, admission facts in the memo) and served by the receipt without inference; a partial one seeds only the owners that hold; the sealed result of an attempt is published; an attempt that throws propagates.
 * @see ../../🔌️attempt/🟦️.ts */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdtempSync,readFileSync,writeFileSync,existsSync,mkdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{decodeAuthorityObservationsV1,encodeAuthorityObservationsV1}from"../../../../../👁️observation/🔢️row-table/🟨️.mjs";
import authoritySchema from"../../../../../🧬️schema/📥️authority.json";
import{loadSelectedThroughFleetV1}from"../../🔌️attempt/🟦️.ts";
import{makeFleet,makeMachine,makeRecords,sha,type FleetV1,type MachineV1}from"../🌍️world/🟦️.ts";

const scope=sha("adapter-scope"),module="the-implementation-of-this-machine";
const store=(machine:MachineV1)=>{const directory=join(mkdtempSync(join(tmpdir(),"store-")),"selected");mkdirSync(directory,{recursive:true});return{path:join(directory,scope+".json"),authorityPath:join(directory,scope+".authority.json")};};
const projectsOf=(machine:MachineV1,count:number)=>Object.fromEntries(Array.from({length:count},(_,index)=>[`o${index}`,{root:`libs/o${index}`}]));

function port(fleet:FleetV1,machine:MachineV1,count:number,memo:Map<string,{facts:unknown;rows:unknown[]}>,counts:{attempts:number;inferred:number}){
 const files=store(machine);
 const attempt=async()=>{
  counts.attempts++;
  try{
   const authority=JSON.parse(readFileSync(files.authorityPath,"utf8")),source=readFileSync(files.path,"utf8");
   if(authority.graph===createHash("sha256").update(source).digest("hex")&&authority.module===module&&await machine.holds(decodeAuthorityObservationsV1(authority.observations,authoritySchema.definitions.ObservationKind.enum)))return"served";
  }catch{}
  counts.inferred++;
  const sealed=await machine.read(undefined);
  const source=JSON.stringify({projects:projectsOf(machine,count),graph:sealed.graph});
  writeFileSync(files.path,source);
  writeFileSync(files.authorityPath,JSON.stringify({version:authoritySchema.properties.version.const,module,membership:sealed.membership,graph:createHash("sha256").update(source).digest("hex"),seal:{},observations:JSON.parse(encodeAuthorityObservationsV1(sealed.rows,1<<20))}));
  for(const entry of sealed.admission)memo.set(entry.suffix,{facts:entry.facts,rows:[...entry.rows]});
  return"inferred";
 };
 return{...files,root:machine.root,scope,attempt,sealed:(value:string|undefined)=>value==="inferred",holds:machine.holds,module,verdict:(input:{rows:readonly unknown[]})=>({mode:"quiet",owners:input.rows.length}),
  facts:{entries:()=>[...memo].map(([suffix,value])=>({suffix,facts:value.facts,rows:value.rows})),seed:(_scope:string,suffix:string,facts:unknown,rows:readonly unknown[])=>{memo.set(suffix,{facts,rows:[...rows]});machine.seeded.add(suffix.split("\0")[0]!);}},
  client:fleet.client(machine.name),environment:{},say:()=>undefined};
}

test("the first attempt publishes, a second machine serves the installed generation from its receipt without inference and with the facts in its memo",async()=>{
 const fleet=makeFleet(),records=makeRecords(),a=makeMachine({name:"a",count:6,records}),b=makeMachine({name:"b",count:6,records});
 const memoA=new Map(),memoB=new Map(),countsA={attempts:0,inferred:0},countsB={attempts:0,inferred:0};
 expect(await loadSelectedThroughFleetV1(port(fleet,a,6,memoA,countsA))).toBe("inferred");
 expect(countsA).toEqual({attempts:1,inferred:1});expect(fleet.results.size).toBe(1);
 const portB=port(fleet,b,6,memoB,countsB);
 expect(await loadSelectedThroughFleetV1(portB)).toBe("served");
 expect(countsB).toEqual({attempts:1,inferred:0});expect(b.counters).toEqual({observations:0,inferences:0,takes:0,reads:0});
 expect(memoB.size).toBe(memoA.size);
 const installed=readFileSync(portB.path,"utf8"),receipt=JSON.parse(readFileSync(portB.authorityPath,"utf8"));
 expect(installed.includes(b.root)).toBe(true);expect(installed.includes(a.root)).toBe(false);
 expect(JSON.stringify(receipt.observations).includes(a.root)).toBe(false);expect(receipt.module).toBe(module);
});

test("a partial generation seeds the owners that hold and the attempt runs for the rest, then publishes the repaired head",async()=>{
 const fleet=makeFleet(),records=makeRecords(),a=makeMachine({name:"a",count:6,records}),b=makeMachine({name:"b",count:6,records});
 await loadSelectedThroughFleetV1(port(fleet,a,6,new Map(),{attempts:0,inferred:0}));
 b.edit(3,"changed on b");
 const memoB=new Map(),counts={attempts:0,inferred:0},portB=port(fleet,b,6,memoB,counts);
 expect(await loadSelectedThroughFleetV1(portB)).toBe("inferred");
 expect(counts).toEqual({attempts:1,inferred:1});
 expect(b.counters.observations).toBe(1);expect(b.counters.inferences).toBe(2);
 expect(existsSync(portB.path)).toBe(true);
 const c=makeMachine({name:"c",count:6,records});c.edit(3,"changed on b");
 const countsC={attempts:0,inferred:0};
 expect(await loadSelectedThroughFleetV1(port(fleet,c,6,new Map(),countsC))).toBe("served");
 expect(countsC).toEqual({attempts:1,inferred:0});
});

test("an attempt that throws propagates and a failing fleet leaves the attempt alone",async()=>{
 const fleet=makeFleet(),records=makeRecords(),a=makeMachine({name:"a",count:3,records}),counts={attempts:0,inferred:0};
 const base=port(fleet,a,3,new Map(),counts);
 await expect(loadSelectedThroughFleetV1({...base,attempt:async()=>{throw Object.assign(Error("authority moved"),{code:"BUILD_AUTHORITY_CHANGED"});}})).rejects.toThrow("authority moved");
 fleet.down=true;
 expect(await loadSelectedThroughFleetV1(base)).toBe("inferred");
 expect(await loadSelectedThroughFleetV1({...base,client:undefined,environment:{SEMIO_FLEET_GRAPH:"off"}})).toBe("served");
});
