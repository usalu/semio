import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import protobuf from "protobufjs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1,parseRemoteCacheTreeV1,validateRemoteCacheActionResultV1,type RemoteCacheBlobsV1,type RemoteCacheDigestV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle","🐍️.py"),sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
type Vector={name:string;kind:"result"|"tree";hex:string;valid:boolean;references?:[string,number][];trees?:[string,number][]};
type Verdict={valid:boolean;reason?:string;references?:[string,number][];trees?:[string,number][]};

function run(args:string[],input?:string):string{
 const result=Bun.spawnSync([python!,"-I",oracle,...args],{stdin:input===undefined?"ignore":new TextEncoder().encode(input),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);return result.stdout.toString();
}
const hex=(value:string)=>Uint8Array.from(value.match(/../gu)??[],byte=>Number.parseInt(byte,16));
const pairs=(rows:readonly RemoteCacheDigestV1[])=>rows.map(row=>[row.hash,row.size] as [string,number]);

function typescript(kind:"result"|"tree",value:string):Verdict{
 try{
  if(kind==="tree")return {valid:true,references:pairs(parseRemoteCacheTreeV1(hex(value))),trees:[]};
  const parsed=parseRemoteCacheActionResultV1(hex(value));return {valid:true,references:pairs(parsed.references),trees:pairs(parsed.trees)};
 }catch(error){return {valid:false,reason:(error as Error).message};}
}

test.skipIf(!python)("the strict decoder answers every named vector like the independent Python decoder and like the encoder that built it",()=>{
 const rows=JSON.parse(run(["vectors"])) as Vector[];
 expect(rows.length).toBeGreaterThan(25);expect(rows.filter(row=>row.valid).length).toBeGreaterThan(8);expect(rows.filter(row=>!row.valid).length).toBeGreaterThan(12);
 const decoded=JSON.parse(run(["decode"],JSON.stringify(rows.map(row=>({kind:row.kind,hex:row.hex}))))) as Verdict[];
 rows.forEach((row,index)=>{
  const ours=typescript(row.kind,row.hex),theirs=decoded[index]!;
  expect(ours.valid,row.name).toBe(row.valid);expect(theirs.valid,row.name+" (python)").toBe(row.valid);
  if(row.valid){
   expect(ours.references,row.name).toEqual(row.references!);expect(ours.trees,row.name).toEqual(row.trees!);
   expect(theirs.references,row.name+" (python)").toEqual(row.references!);expect(theirs.trees,row.name+" (python)").toEqual(row.trees!);
  }
 });
});

test.skipIf(!python)("every truncation and byte flip of the valid vectors gets the same verdict from TypeScript and Python",()=>{
 const rows=(JSON.parse(run(["vectors"])) as Vector[]).filter(row=>row.valid),cases:{kind:"result"|"tree";hex:string}[]=[];
 let seed=0x9e3779b9;const next=()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};
 for(const row of rows){
  const bytes=hex(row.hex);
  for(let length=0;length<bytes.length;length++)cases.push({kind:row.kind,hex:Buffer.from(bytes.subarray(0,length)).toString("hex")});
  for(let round=0;round<40;round++){const copy=bytes.slice();if(copy.length===0)break;const at=Math.floor(next()*copy.length);copy[at]=Math.floor(next()*256);cases.push({kind:row.kind,hex:Buffer.from(copy).toString("hex")});}
 }
 expect(cases.length).toBeGreaterThan(1000);
 const theirs=JSON.parse(run(["decode"],JSON.stringify(cases))) as Verdict[];let accepted=0;
 cases.forEach((row,index)=>{
  const ours=typescript(row.kind,row.hex);
  expect(ours.valid,`${row.kind} ${row.hex}`).toBe(theirs[index]!.valid);
  if(ours.valid){accepted++;expect(ours.references).toEqual(theirs[index]!.references!);expect(ours.trees).toEqual(theirs[index]!.trees!);}
 });
 expect(accepted).toBeGreaterThan(50);expect(accepted).toBeLessThan(cases.length);
});

const proto=`syntax="proto3";
message Digest{string hash=1;int64 size_bytes=2;}
message OutputFile{string path=1;Digest digest=2;bool is_executable=4;bytes contents=5;}
message OutputSymlink{string path=1;string target=2;}
message OutputDirectory{string path=1;Digest tree_digest=3;Digest root_directory_digest=5;}
message ActionResult{repeated OutputFile output_files=2;repeated OutputDirectory output_directories=3;int32 exit_code=4;bytes stdout_raw=5;Digest stdout_digest=6;bytes stderr_raw=7;Digest stderr_digest=8;repeated OutputSymlink output_file_symlinks=10;repeated OutputSymlink output_directory_symlinks=11;repeated OutputSymlink output_symlinks=12;}
message FileNode{string name=1;Digest digest=2;bool is_executable=4;}
message DirectoryNode{string name=1;Digest digest=2;}
message SymlinkNode{string name=1;string target=2;}
message Directory{repeated FileNode files=1;repeated DirectoryNode directories=2;repeated SymlinkNode symlinks=3;}
message Tree{Directory root=1;repeated Directory children=2;}`;
const root=protobuf.parse(proto).root,ActionResult=root.lookupType("ActionResult"),Tree=root.lookupType("Tree");
const digestOf=(value:string)=>({hash:sha(value),sizeBytes:value.length});

test("results and trees encoded by protobufjs, a third-party implementation, are decoded to the same digests",()=>{
 const encoded=ActionResult.encode(ActionResult.create({
  outputFiles:[{path:"a.txt",digest:digestOf("alpha"),isExecutable:true},{path:"b.txt",digest:digestOf("bravo")},{path:"inline.txt",digest:digestOf("tiny"),contents:Buffer.from("tiny")}],
  outputDirectories:[{path:"gen",treeDigest:digestOf("tree-bytes")}],exitCode:-9,stdoutDigest:digestOf("out"),stderrRaw:Buffer.from("warn"),
  outputSymlinks:[{path:"l",target:"a.txt"}]
 })).finish();
 const parsed=parseRemoteCacheActionResultV1(encoded);
 expect(pairs(parsed.references).sort()).toEqual([[sha("alpha"),5],[sha("bravo"),5],[sha("out"),3]].sort() as [string,number][]);expect(pairs(parsed.trees)).toEqual([[sha("tree-bytes"),10]]);
 const decoded=ActionResult.toObject(ActionResult.decode(encoded),{longs:Number}) as {outputFiles:{digest:{hash:string;sizeBytes:number}}[]};
 expect(decoded.outputFiles.map(file=>file.digest.hash)).toEqual([sha("alpha"),sha("bravo"),sha("tiny")]);
 const tree=Tree.encode(Tree.create({root:{files:[{name:"f",digest:digestOf("one")}],directories:[{name:"d",digest:digestOf("dir")}],symlinks:[{name:"s",target:"f"}]},children:[{files:[{name:"g",digest:digestOf("two")}]}]})).finish();
 expect(pairs(parseRemoteCacheTreeV1(tree))).toEqual([[sha("one"),3],[sha("two"),3]]);
 const wrong=ActionResult.encode(ActionResult.create({outputFiles:[{path:"x",digest:{hash:"G".repeat(64),sizeBytes:1}}]})).finish();
 expect(()=>parseRemoteCacheActionResultV1(wrong)).toThrow("lowercase sha256");
 for(const length of[1,3,7])expect(()=>ActionResult.decode(encoded.subarray(0,encoded.length-length)),"protobufjs also refuses a truncated message").toThrow();
 for(const length of[1,3,7])expect(()=>parseRemoteCacheActionResultV1(encoded.subarray(0,encoded.length-length))).toThrow();
});

test("validation levels: off accepts anything, structure only malformed messages, complete needs every named blob and every file of a tree",async()=>{
 const stored=new Set<string>();
 const tree=Tree.encode(Tree.create({root:{files:[{name:"f",digest:digestOf("inside")}]}})).finish(),treeHash=sha(tree);
 const result=ActionResult.encode(ActionResult.create({outputFiles:[{path:"a",digest:digestOf("alpha")}],outputDirectories:[{path:"d",treeDigest:{hash:treeHash,sizeBytes:tree.length}}]})).finish();
 const reader:RemoteCacheBlobsV1={has:hash=>stored.has(hash),read:async hash=>stored.has(hash)&&hash===treeHash?tree:undefined};
 expect(await validateRemoteCacheActionResultV1(new Uint8Array([0xff,0xff]),"off",reader)).toEqual({valid:true});
 expect(await validateRemoteCacheActionResultV1(new Uint8Array([0xff,0xff]),"structure",reader)).toMatchObject({valid:false,kind:"malformed"});
 expect(await validateRemoteCacheActionResultV1(result,"structure",reader)).toEqual({valid:true});
 expect(await validateRemoteCacheActionResultV1(result,"complete",reader)).toMatchObject({valid:false,kind:"missing"});
 stored.add(sha("alpha"));expect(await validateRemoteCacheActionResultV1(result,"complete",reader)).toMatchObject({valid:false,kind:"missing"});
 stored.add(treeHash);expect(await validateRemoteCacheActionResultV1(result,"complete",reader)).toMatchObject({valid:false,kind:"missing",reason:expect.stringContaining("of tree")});
 stored.add(sha("inside"));expect(await validateRemoteCacheActionResultV1(result,"complete",reader)).toEqual({valid:true});
 const bad:RemoteCacheBlobsV1={has:()=>true,read:async()=>new Uint8Array([0x0a,0x05])};
 expect(await validateRemoteCacheActionResultV1(result,"complete",bad)).toMatchObject({valid:false,kind:"malformed"});
});
