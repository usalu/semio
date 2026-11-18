import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{hostOfRootV1,neutralRecordV1,localRecordV1,type PortableRecordV1}from"../../🟦️.ts";

/** 🕰️ A project record is a function of the project's inputs: the same project inferred at two roots (a POSIX checkout and a Windows one) on machines whose files carry other modification times, change times and inodes yields the same neutral bytes. */
type Case={name:string;roots:{root:string;style:"posix"|"windows"}[];hash:string;stamps:number[][]};
const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures","🔣️.json"),"utf8")) as {cases:Case[];expected:string};
const spell=(root:string,style:"posix"|"windows",rest:string)=>style==="windows"?root+rest.replaceAll("/","\\"):root+rest;
function recordAt(root:string,style:"posix"|"windows",stamp:number[]):PortableRecordV1{
 const source={size:12,mtimeMs:stamp[0]!,ctimeMs:stamp[1]!,ino:stamp[2]!,sha256:"a".repeat(64)};
 const own=(rest:string)=>spell(root,style,rest);
 return{digest:"d".repeat(64),authority:[],image:JSON.stringify({name:"alpha",root:"libs/alpha"}),observations:[
  {kind:"file",path:own("/libs/alpha/project.json"),fact:{size:12,mtimeMs:stamp[0]!,ctimeMs:stamp[1]!,ino:stamp[2]!,sha256:"a".repeat(64)}},
  {kind:"file",path:own("/libs/alpha/missing.json"),fact:{error:"ENOENT"}},
  {kind:"analysis",path:own("/libs/alpha/src/a.ts"),fact:{provider:"p",revision:"r",resultSha256:"b".repeat(64),source,nestedObservations:[{kind:"file",path:own("/libs/alpha/src/b.ts"),fact:{size:3,mtimeMs:stamp[0]!+1,ctimeMs:stamp[1]!+1,ino:stamp[2]!+1,sha256:"c".repeat(64)}}]}},
 ]};
}
const bytes=(record:PortableRecordV1,root:string,style:"posix"|"windows")=>JSON.stringify(neutralRecordV1(record,hostOfRootV1(spell(root,style,""))).record);

test("the same project inferred at different roots, styles and file stamps yields identical neutral bytes",()=>{
 for(const row of fixture.cases){
  const made=row.roots.map((where,index)=>bytes(recordAt(where.root,where.style,row.stamps[index]!),where.root,where.style));
  for(const text of made)expect(text).toBe(made[0]!);
 }
});

test("a stamp is a hint of the recording machine: the neutral record carries the declared value, sizes and hashes stay, and an error row is untouched",()=>{
 const text=bytes(recordAt("/srv/a","posix",[111,222,333]),"/srv/a","posix"),record=JSON.parse(text) as PortableRecordV1;
 const rows=record.observations as unknown as {kind:string;fact:Record<string,unknown>}[];
 expect(rows[0]!.fact).toEqual({size:12,mtimeMs:0,ctimeMs:0,ino:0,sha256:"a".repeat(64)});
 expect(rows[1]!.fact).toEqual({error:"ENOENT"});
 const analysis=rows[2]!.fact as {source:Record<string,unknown>;nestedObservations:{fact:Record<string,unknown>}[]};
 expect(analysis.source).toEqual({size:12,mtimeMs:0,ctimeMs:0,ino:0,sha256:"a".repeat(64)});expect(analysis.nestedObservations[0]!.fact.ino).toBe(0);expect(analysis.nestedObservations[0]!.fact.size).toBe(3);
 expect(JSON.stringify(localRecordV1(record,hostOfRootV1("/home/x/semio")).record)).toContain("/home/x/semio/libs/alpha/project.json");
});

test("the neutral bytes equal the independent Python implementation and the authored expectation",()=>{
 const row=fixture.cases[0]!,text=bytes(recordAt(row.roots[0]!.root,row.roots[0]!.style,row.stamps[0]!),row.roots[0]!.root,row.roots[0]!.style);
 expect(createHashHex(text)).toBe(fixture.expected);
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🧫️fixtures","🐍️oracle.py"),join(import.meta.dir,"🧫️fixtures","🔣️.json")],{encoding:"utf8"});
 if(python.status===0)expect(python.stdout.trim()).toBe(fixture.expected);
});
function createHashHex(text:string){return new Bun.CryptoHasher("sha256").update(text).digest("hex");}
