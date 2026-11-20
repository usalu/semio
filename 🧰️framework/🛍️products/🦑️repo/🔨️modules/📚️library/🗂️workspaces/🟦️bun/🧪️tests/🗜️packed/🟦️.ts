import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import ts from "typescript";
import corpus from "./🔣️.json";

const harness=fileURLToPath(new URL("./🟨️.cjs",import.meta.url)),ownershipFile=process.env.PACKED_SOURCE??fileURLToPath(new URL("../../🟨️.cjs",import.meta.url));
type Walk=Readonly<{rows:number;foreign:number;listed:number;mismatches:number;rowsSha:string;scopesSha:string;textsSha:string;packages:number;peakFootprintMegabytes:number;seconds:number;trace:unknown[]}>;
/** 🗜️ Walks a virtual store of the given shape in a fresh process with the real ownership module and returns its verdict beside the peak footprint of that process. */
function walk(groups:number,leaves:number):{walk:Walk;peak:number}{
 const timed=process.platform==="darwin",command=timed?["/usr/bin/time","-l",process.execPath,harness]:[process.execPath,harness],run=spawnSync(command[0]!,command.slice(1),{encoding:"utf8",env:{...process.env,PACKED_GROUPS:String(groups),PACKED_LEAVES:String(leaves)},maxBuffer:1<<26}),line=run.stdout.split("\n").find(row=>row.startsWith("RESULT "));
 if(!line)throw Error("Virtual walk failed: "+run.stdout.slice(0,2000)+run.stderr.slice(0,2000));
 const result=JSON.parse(line.slice(7))as Walk,measured=timed?Number(/(\d+)\s+peak memory footprint/u.exec(run.stderr)?.[1]??0)/1048576:0;
 return{walk:result,peak:Math.max(result.peakFootprintMegabytes,measured)};
}
const verdict=(row:typeof corpus.walks[number]|typeof corpus.large,result:Walk)=>{expect(result.mismatches,row.name).toBe(0);expect(result.foreign,row.name).toBe(1);expect(result.rows,row.name).toBe(row.rows);expect(result.listed,row.name).toBe(row.listed);expect(result.packages,row.name).toBe(row.packages);expect(result.rowsSha,row.name).toBe(row.rowsSha);expect(result.scopesSha,row.name).toBe(row.scopesSha);expect(result.textsSha,row.name).toBe(row.textsSha);};

for(const row of corpus.walks)test("packed ownership walk keeps the rows, scopes and manifests of the unpacked walk byte for byte: "+row.name,()=>{
 verdict(row,walk(row.groups,row.leaves).walk);
},120000);

test("a virtual store of five hundred thousand paths is walked under a declared 8 GB and 4 core host within the packed footprint limit and with the unpacked rows",()=>{
 const host=corpus.host,{walk:result,peak}=walk(corpus.large.groups,corpus.large.leaves);
 expect(host.limitMegabytes).toBeLessThanOrEqual(Math.floor(host.treeShare*host.memoryMegabytes));
 verdict(corpus.large,result);
 expect(result.rows).toBeGreaterThan(480000);
 expect(peak,"peak footprint of the walking process in megabytes").toBeLessThan(host.limitMegabytes);
},600000);

const parsed=ts.createSourceFile("ownership.cjs",readFileSync(ownershipFile,"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),declared=["stateKindNames","stateKindCodes","listingKindChars","listingKindNames","internedNameCapacity","internedNames","emptyListing","isListing"],functions=["internName","packListing","listingEntries","listingEquals","PackedObservations","packedRow","observedRows","ownershipChange","retainOwnershipObservationV1","FrontierProof"];
const body=parsed.statements.filter(node=>(ts.isFunctionDeclaration(node)||ts.isClassDeclaration(node))&&functions.includes(node.name!.text)||ts.isVariableStatement(node)&&node.declarationList.declarations.some(declaration=>ts.isIdentifier(declaration.name)&&(declared.includes(declaration.name.text)||declaration.name.text==="physicalObservations"))).map(node=>node.getText(parsed)).join(";\n");
const api=new Function("limits","edited",body+";return{create:physicalObservations,retain:retainOwnershipObservationV1,pack:packListing,entries:listingEntries,equals:listingEquals,Proof:FrontierProof}")({maximumObservationPaths:64,maximumObservationFacts:128},(message:string)=>Error(message));
const state=(index:number)=>["directory",String(16777232+index),String(2000000000000+index*7),16877];

test("packed listings share names, keep their order and kinds, and compare like their rows",()=>{
 const rows=[{name:"a",kind:"file"},{name:"b",kind:"directory"},{name:"c",kind:"symlink"},{name:"d",kind:"other"}],first=api.pack(rows),second=api.pack(rows.map(row=>({...row})));
 expect(api.entries(first)).toEqual(rows);expect(api.equals(first,second)).toBe(true);expect(first.names[1]).toBe(second.names[1]);
 expect(api.equals(first,api.pack(rows.slice(0,3)))).toBe(false);expect(api.equals(first,api.pack([...rows.slice(0,3),{name:"d",kind:"file"}]))).toBe(false);
 expect(api.pack([])).toBe(api.pack([]));expect(()=>api.pack([{name:"x",kind:"socket"}])).toThrow("kind");
});

test("packed observations answer as the array of frozen rows they replace, and keep every row once and unchanged after the store grows",()=>{
 const store=api.create();
 for(let index=0;index<40;index++){api.retain(store,"state","/p"+index,state(index));if(index%3===0)api.retain(store,"list","/p"+index,[{name:"n"+index,kind:"directory"},{name:"package.json",kind:"file"}]);}
 api.retain(store,"state","/gone",["missing"]);
 const rows=store.rows(),expected=Array.from({length:41},(_,index)=>index<40?{path:"/p"+index,state:state(index),...(index%3===0?{list:[{name:"n"+index,kind:"directory"},{name:"package.json",kind:"file"}]}:{})}:{path:"/gone",state:["missing"]});
 expect(Array.isArray(rows)).toBe(true);expect(rows.length).toBe(41);expect(JSON.parse(JSON.stringify(rows))).toEqual(expected);expect([...rows]).toEqual(expected);expect(rows.at(-1)).toEqual(expected[40]);expect(rows[0]).toEqual(expected[0]);expect(rows.map((row:any)=>row.path)).toEqual(expected.map(row=>row.path));expect(rows.filter((row:any)=>row.list).length).toBe(14);expect(rows[41]).toBeUndefined();
 expect(Object.isFrozen(rows[0])).toBe(true);expect(Object.isFrozen(rows[0].state)).toBe(true);expect(Object.isFrozen(rows[0].list)).toBe(true);expect(()=>{(rows as any)[0]=1;}).toThrow();expect(()=>{(rows as any).push(1);}).toThrow();
 for(let index=40;index<60;index++)api.retain(store,"state","/q"+index,state(index));
 expect(rows.length).toBe(41);expect(JSON.parse(JSON.stringify(rows))).toEqual(expected);expect(store.rows().length).toBe(61);
});

test("packed observations refuse a changed state or listing and keep fact order, capacity and the first observation",()=>{
 const store=api.create();api.retain(store,"state","/a",state(1));api.retain(store,"list","/a",[{name:"b",kind:"directory"}]);api.retain(store,"state","/a",state(1));api.retain(store,"list","/a",[{name:"b",kind:"directory"}]);
 expect(store.factRows()).toEqual([["/a","state"],["/a","list"]]);
 expect(()=>api.retain(store,"state","/a",state(2))).toThrow("changed during verification");expect(()=>api.retain(store,"list","/a",[{name:"b",kind:"symlink"}])).toThrow("changed during verification");
 expect(()=>api.retain(store,"list","/nowhere",[])).toThrow("Original Bun directory state observation required");
 api.retain(store,"state","/f",["file","1","2",33188]);expect(()=>api.retain(store,"list","/f",[])).toThrow("Original Bun directory state observation required");
 expect(()=>api.retain(store,"state","/odd",["directory","007","2",16877])).toThrow("Closed Bun ownership observation");expect(()=>api.retain(store,"state","/odd",["socket","1","2",1])).toThrow("Closed Bun ownership observation");
 expect(store.size).toBe(2);expect(store.record("/a").state).toEqual(state(1));expect(store.record("/odd")).toBeUndefined();
});

test("a frontier proof applies only the stamps it collected",()=>{
 const store=api.create();api.retain(store,"state","/a",state(1));api.retain(store,"state","/b",state(2));
 const proof=new api.Proof(store.size);proof.set(1,{mtimeNs:5n,ctimeNs:6n},true);proof.applyTo(store);
 expect([...store.settled.slice(0,2)]).toEqual([0,2]);expect(store.frontierMoved(1,{mtimeNs:5n,ctimeNs:6n})).toBe(false);expect(store.frontierMoved(1,{mtimeNs:5n,ctimeNs:7n})).toBe(true);
});
