import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {KilledV1,StateCorruptV1,appendWalV1,decodeRecordsV1,frameV1,payloadsV1,readCheckpointV1,scanWalV1,walHeaderV1,writeCheckpointV1,type CheckpointV1,type WalRecordV1} from "../🟦️.ts";
import {checkpointReplicaV1,openReplicaV1,recallV1,rememberV1,type ReplicaV1} from "../../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),python=testPythonV1(),scratch:string[]=[];
const oracle=join(import.meta.dirname,"🐍️.py"),fixturePath=join(import.meta.dirname,"🧫️fixtures","🔣️.json");
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`state-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const hex=(seed:number):string=>Buffer.alloc(32,seed%251).toString("hex");
const origins=["a1","b2","c-3"];
const hlcOf=(index:number):string=>`${String(1_700_000_000_000+index).padStart(13,"0")}-${String(index%7).padStart(6,"0")}-${origins[index%3]}`;

type Row=Readonly<{path:string;entry:{hlc:string;operation:"put"|"delete"|"link";sha256?:string;bytes?:number;executable?:boolean;target?:string;mtimeMs:number;seenMs?:number}}>;

function rows(count:number):Row[]{
 const found:Row[]=[];
 for(let index=0;index<count;index++){
  const kind=index%9,path=index%11===0?`ünï/cödé-é-${index}.txt`:`dir${index%13}/sub${index%5}/file-${index}.txt`,base={hlc:hlcOf(index),mtimeMs:1_700_000_000_000+index+0.5,...(index%2===0?{seenMs:1_700_000_100_000+index}:{})};
  found.push({path,entry:kind===3?{...base,operation:"link",target:`../ziel-✓-${index}`}:kind===6?{...base,operation:"delete"}:{...base,operation:"put",sha256:hex(index),bytes:index*7,executable:index%4===0}});
 }
 found.push({path:".cache/f/index-base",entry:{hlc:hlcOf(1001),operation:"link",target:"tree 0000000000000000000000000000000000000000",mtimeMs:0}});
 found.push({path:".cache/f/index/dir/x",entry:{hlc:hlcOf(1002),operation:"delete",mtimeMs:0}});
 return found;
}

async function populated(count:number,queued:number=0):Promise<Readonly<{replica:ReplicaV1;worktree:string}>>{
 const worktree=temp("tree"),replica=await openReplicaV1({worktree,storeRoot:temp("store"),originId:"a1",groupId:"g1",lock:false});
 for(const {path,entry} of rows(count)){
  replica.files.set(path,entry);
  if(entry.operation!=="delete"&&!path.startsWith(".cache/f/index"))replica.folds.add(replica.book.find(path)!);
 }
 for(const row of rows(queued)){
  const {mtimeMs:_,seenMs:__,...manifest}=row.entry;void _;void __;
  const path=`queued/${row.path}`;
  replica.manifest.set(path,manifest);replica.queue.addId(replica.book.find(path)!);
 }
 for(let index=0;index<20;index++)replica.dirs.set(`dir${index}`,1_700_000_000_000+index);
 return {replica,worktree};
}

const byPath=(left:{path:string},right:{path:string}):number=>Buffer.compare(Buffer.from(left.path),Buffer.from(right.path));
function projectionOf(replica:ReplicaV1){
 const files=[...replica.files.entries()].map(([path,entry])=>({path,...entry})).sort(byPath),queue=[...replica.queue.entries()].map(([path,entry])=>({path,...entry})).sort(byPath);
 return {files,queue,manifest:replica.files.manifestHex(),shared:replica.files.shared};
}

function python3(command:string,file:string):{status:number|null;json:Record<string,unknown>}{
 const result=Bun.spawnSync([python!,"-I",oracle,command,file],{stdout:"pipe",stderr:"pipe"}),text=result.stdout.toString();
 return {status:result.exitCode,json:text.trim()===""?{}:JSON.parse(text) as Record<string,unknown>};
}

const sorted=(value:unknown):unknown=>Array.isArray(value)?value.map(sorted):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value as Record<string,unknown>).sort(([left],[right])=>left<right?-1:1).map(([key,item])=>[key,sorted(item)])):value;

test("a checkpoint written by the replica is read by the Python oracle with the same files, queue, memos and manifest digest, and reloads identically",async()=>{
 const {replica,worktree}=await populated(300,40);
 rememberV1(replica,"view",Buffer.from('{"digest":"x"}'));rememberV1(replica,"dirs",Uint8Array.of(1,2,3,255));
 await checkpointReplicaV1(replica);
 const file=join(worktree,".cache","f","state.ckp"),decoded=python3("checkpoint",file);
 expect(decoded.status).toBe(0);
 expect(validator("Decoded")(decoded.json)).toBe(true);
 const projection=projectionOf(replica),manifest=decoded.json.manifest as {digest:string;shared:number;stored:string;storedShared:number};
 expect(sorted(decoded.json.files)).toEqual(sorted(projection.files));
 expect(sorted(decoded.json.queue)).toEqual(sorted(projection.queue));
 expect(manifest).toEqual({digest:projection.manifest,shared:projection.shared,stored:projection.manifest,storedShared:projection.shared});
 expect(decoded.json.memo).toMatchObject({view:Buffer.from('{"digest":"x"}').toString("base64"),dirs:Buffer.from([1,2,3,255]).toString("base64")});
 expect(Object.keys(decoded.json.memo as object).sort()).toEqual(["clock","cursors","dirs","view"]);
 expect(validator("Header")(decoded.json.header)).toBe(true);
 const again=await openReplicaV1({worktree,storeRoot:temp("store"),originId:"a1",groupId:"g1",lock:false});
 expect(sorted(projectionOf(again))).toEqual(sorted(projection));
 expect([...recallV1(again,"dirs")!]).toEqual([1,2,3,255]);
 const probe=rows(300)[16]!.path;
 expect(again.folds.find(probe.toUpperCase())).toBe(probe);
 expect(again.dirs.get("dir7")).toBe(1_700_000_000_007);expect(again.dirs.size).toBe(20);
});

test("the committed vectors are reproduced by the Python oracle and by the TypeScript codec",async()=>{
 const result=Bun.spawnSync([python!,"-I",oracle,"vectors",fixturePath],{stdout:"pipe",stderr:"pipe"});
 expect(result.stdout.toString()+result.stderr.toString()).toMatch(/^ok \d+ vectors/u);
 expect(result.exitCode).toBe(0);
 expect(validator("Fixture")(fixture)).toBe(true);
 expect(fixture.checkpoints.length).toBeGreaterThan(0);
 for(const [index,item] of fixture.checkpoints.entries()){
  const worktree=temp(`vector${index}`);
  mkdirSync(join(worktree,".cache","f"),{recursive:true});
  writeFileSync(join(worktree,".cache","f","state.ckp"),Buffer.from(item.base64,"base64"));
  const replica=await openReplicaV1({worktree,storeRoot:temp("store"),originId:"a1",groupId:"g1",lock:false}),decoded=item.decoded;
  expect(sorted(projectionOf(replica).files)).toEqual(sorted(decoded.files));
  expect(sorted(projectionOf(replica).queue)).toEqual(sorted(decoded.queue));
  expect(replica.files.manifestHex()).toBe(decoded.manifest.digest);
 }
 for(const item of fixture.logs){
  const scan=scanWalV1(Buffer.from(item.base64,"base64"));
  expect<unknown>(scan.verdict).toBe(item.verdict);expect(scan.good).toBe(item.good);expect(scan.generation??null).toBe(item.generation);
  expect(sorted(scan.payloads.flatMap(payload=>decodeRecordsV1(payload).map(record=>record.kind==="memo"?{...record,value:record.value===null?null:Buffer.from(record.value).toString("base64")}:record)))).toEqual(sorted(item.records));
  for(const record of item.records)expect(validator("WalRecord")(record)).toBe(true);
 }
});

const parts=(seed:number):CheckpointV1=>({generation:seed,paths:3,files:2,queue:1,parts:[{name:"meta",bytes:[Uint8Array.from({length:32},(_,index)=>index+seed)]},{name:"book.pages",bytes:[Uint8Array.from({length:300},(_,index)=>index*3%256),Uint8Array.of(9,9,9)]},{name:"memo:x",bytes:[]}]});
const wanted=(name:string,length:number)=>name==="meta"||name==="book.pages"||name==="memo:x"?new Uint8Array(length):undefined;

test("a checkpoint is closed: any truncation and any flipped byte is detected, never half-read",async()=>{
 const directory=temp("corrupt"),file=join(directory,"state.ckp");
 const total=await writeCheckpointV1(file,join(directory,"tmp"),parts(1)),good=readFileSync(file);
 expect(total).toBe(good.length);
 const header=await readCheckpointV1(file,wanted);
 expect(validator("Header")(header)).toBe(true);
 expect(header).toMatchObject({generation:1,paths:3,files:2,queue:1,parts:3,payloadBytes:good.length-96});
 for(let length=0;length<good.length;length++){
  writeFileSync(file,good.subarray(0,length));
  await expect(readCheckpointV1(file,wanted)).rejects.toBeInstanceOf(StateCorruptV1);
 }
 for(let position=0;position<good.length;position++){
  const flipped=Buffer.from(good);flipped[position]=flipped[position]!^0x5a;
  writeFileSync(file,flipped);
  await expect(readCheckpointV1(file,wanted)).rejects.toBeInstanceOf(StateCorruptV1);
 }
 writeFileSync(file,good);
 expect(await readCheckpointV1(file,wanted)).toEqual(header);
 expect(await readCheckpointV1(join(directory,"missing.ckp"),wanted)).toBeUndefined();
});

test("a kill at any byte of a checkpoint write leaves the previous checkpoint intact and loadable",async()=>{
 const directory=temp("kill"),file=join(directory,"state.ckp"),temporary=join(directory,"tmp");
 await writeCheckpointV1(file,temporary,parts(1));
 const before=readFileSync(file),next=parts(2),size=await writeCheckpointV1(join(directory,"probe.ckp"),temporary,next);
 for(let killAt=0;killAt<size;killAt++){
  await expect(writeCheckpointV1(file,temporary,next,{killAt})).rejects.toBeInstanceOf(KilledV1);
  expect(readFileSync(file).equals(before)).toBe(true);
  expect((await readCheckpointV1(file,wanted))?.generation).toBe(1);
 }
 expect(await writeCheckpointV1(file,temporary,next,{killAt:size})).toBe(size);
 expect((await readCheckpointV1(file,wanted))?.generation).toBe(2);
});

const record=(index:number):WalRecordV1=>index%4===0?{kind:"memo",name:`m${index}`,value:Uint8Array.of(index,index+1)}:index%4===1?{kind:"files",path:`p/${index}`,entry:{hlc:hlcOf(index),operation:"put",sha256:hex(index),bytes:index,executable:index%8===1,mtimeMs:index+0.25,seenMs:index+1}}:index%4===2?{kind:"queue",path:`q/${index}`,entry:{hlc:hlcOf(index),operation:"link",target:`t✓${index}`}}:{kind:"files",path:`p/${index}`,entry:null};

function frames(count:number,generation:number=4):Readonly<{data:Buffer;ends:number[];records:WalRecordV1[][]}>{
 const pieces=[Buffer.from(walHeaderV1(generation))],ends=[pieces[0]!.length],found:WalRecordV1[][]=[];
 for(let index=0;index<count;index++){
  const batch=[record(index*2),record(index*2+1)],piece=Buffer.from(frameV1([...payloadsV1(batch)][0]!));
  pieces.push(piece);ends.push(ends[ends.length-1]!+piece.length);found.push(batch);
 }
 return {data:Buffer.concat(pieces),ends,records:found};
}
const plain=(records:readonly WalRecordV1[]):unknown=>records.map(item=>item.kind==="memo"?{...item,value:item.value===null?null:[...item.value]}:item);

test("a log cut at any byte keeps exactly the frames before the cut, a damaged frame in the middle is corruption",()=>{
 const {data,ends,records}=frames(4);
 for(let length=0;length<=data.length;length++){
  const scan=scanWalV1(data.subarray(0,length)),whole=ends.filter(end=>end<=length).length-1;
  if(length===0){expect(scan.verdict).toBe("empty");continue;}
  if(length<16){expect(scan.verdict).toBe("torn");expect(scan.good).toBe(0);continue;}
  expect(scan.generation).toBe(4);expect(scan.good).toBe(ends[whole]);
  expect(scan.verdict).toBe(length===ends[whole]?"clean":"torn");
  expect(plain(scan.payloads.flatMap(payload=>decodeRecordsV1(payload)))).toEqual(plain(records.slice(0,whole).flat()));
 }
 for(let position=16;position<data.length;position++){
  const flipped=Buffer.from(data);flipped[position]=flipped[position]!^0x21;
  const scan=scanWalV1(flipped),frame=ends.findIndex(end=>end>position)-1;
  expect(scan.verdict).toBe(frame===3&&position-ends[frame]!>=8?"torn":"corrupt");
  expect(scan.good).toBe(ends[frame]);
 }
 expect(scanWalV1(Buffer.concat([data,Buffer.alloc(40)]))).toMatchObject({verdict:"torn",good:data.length});
 const foreign=Buffer.from(data);foreign[0]=0x58;
 expect(scanWalV1(foreign).verdict).toBe("corrupt");
});

test("a kill at any byte of a log append leaves the earlier frames, and appending after the cut works",async()=>{
 const directory=temp("wal"),file=join(directory,"state.wal"),{records}=frames(3),batches=records.map(batch=>[...payloadsV1(batch)]),clean=join(directory,"clean.wal");
 const total=await appendWalV1(clean,4,true,batches.flat());
 const sizes=batches.map((_,index)=>16+batches.slice(0,index+1).flat().reduce((sum,payload)=>sum+16+payload.length,0));
 for(let killAt=0;killAt<total;killAt++){
  rmSync(file,{force:true});
  await expect(appendWalV1(file,4,true,batches.flat(),{killAt})).rejects.toBeInstanceOf(KilledV1);
  const scan=scanWalV1(readFileSync(file)),whole=sizes.filter(size=>size<=killAt).length;
  if(killAt<16){expect(scan.good).toBe(0);continue;}
  expect(scan.generation).toBe(4);expect(scan.good).toBe(whole===0?16:sizes[whole-1]);
  truncateSync(file,scan.good);
  await appendWalV1(file,4,false,batches.slice(whole).flat());
  const healed=scanWalV1(readFileSync(file));
  expect(healed.verdict).toBe("clean");expect(plain(healed.payloads.flatMap(payload=>decodeRecordsV1(payload)))).toEqual(plain(records.flat()));
 }
 expect(statSync(clean).size).toBe(total);
});

test("records round-trip through frames of at most one mebibyte and unicode names, targets and memos survive",()=>{
 const batch:WalRecordV1[]=Array.from({length:30000},(_,index)=>index%3===0?record(index):{kind:"files",path:`ünï/${"x".repeat(150)}/${index}`,entry:{hlc:hlcOf(index),operation:"link",target:`✓${"y".repeat(100)}`,mtimeMs:index}});
 const payloads=[...payloadsV1(batch)];
 expect(payloads.length).toBeGreaterThan(3);
 for(const payload of payloads)expect(payload.length).toBeLessThan((1<<20)+400);
 expect(plain(payloads.flatMap(payload=>decodeRecordsV1(payload)))).toEqual(plain(batch));
 expect(()=>decodeRecordsV1(payloads[0]!.subarray(0,payloads[0]!.length-3))).toThrow(StateCorruptV1);
});

test("a replica checkpoint of 272 000 entries loads in well under a second without parsing per entry",async()=>{
 const {replica,worktree}=await populated(0),total=272_000;
 for(let index=0;index<total;index++){
  const path=`group${index%97}/area${index%1013}/file-${index}.txt`;
  replica.files.set(path,{hlc:hlcOf(index),operation:"put",sha256:hex(index),bytes:index,executable:false,mtimeMs:1_700_000_000_000+index,seenMs:1_700_000_100_000});
  replica.folds.add(replica.book.find(path)!);
 }
 const started=performance.now();
 await checkpointReplicaV1(replica);
 const written=performance.now()-started,times:number[]=[];
 for(let round=0;round<3;round++){
  const begin=performance.now(),again=await openReplicaV1({worktree,storeRoot:temp("store"),originId:"a1",groupId:"g1",lock:false});
  times.push(performance.now()-begin);
  expect(again.files.size).toBe(total+2);
 }
 process.stdout.write(`[state] write ${written.toFixed(0)} ms, loads ${times.map(time=>time.toFixed(0)).join("/")} ms, file ${(statSync(join(worktree,".cache","f","state.ckp")).size/1048576).toFixed(1)} MiB\n`);
 expect(Math.min(...times)).toBeLessThan(900);
},120000);

function python3Wal(data:Buffer):{generation:number|null;verdict:string;good:number;records:unknown[]}{
 const file=join(temp("walfile"),"x.wal");
 writeFileSync(file,data);
 return python3("wal",file).json as {generation:number|null;verdict:string;good:number;records:unknown[]};
}

if(process.env.SEMIO_WRITE_FIXTURE==="1")test("writes the committed vectors",async()=>{
 const {replica,worktree}=await populated(60,6);
 rememberV1(replica,"view",Buffer.from("{\"digest\":\"vector\"}"));
 await checkpointReplicaV1(replica);
 const file=join(worktree,".cache","f","state.ckp"),decoded=python3("checkpoint",file).json;
 const named=(name:string,...batches:WalRecordV1[][])=>({name,data:Buffer.concat([Buffer.from(walHeaderV1(7)),...batches.map(batch=>Buffer.from(frameV1([...payloadsV1(batch)][0]!)))])});
 const full=named("three frames of puts, links, removals and memos",[record(1),record(2)],[record(0),record(3),record(5)],[record(6),record(9)]);
 const damaged=Buffer.from(full.data);damaged[40]=damaged[40]!^1;
 const cases=[full,{name:"the last frame cut in the middle",data:full.data.subarray(0,full.data.length-9)},{name:"the header only",data:Buffer.from(walHeaderV1(7))},{name:"nothing",data:Buffer.alloc(0)},{name:"a flipped byte in the first frame",data:damaged}];
 const logs=cases.map(item=>{const scan=python3Wal(Buffer.from(item.data));return {name:item.name,base64:Buffer.from(item.data).toString("base64"),generation:scan.generation,verdict:scan.verdict,good:scan.good,records:scan.records};});
 writeFileSync(fixturePath,`${JSON.stringify({version:1,checkpoints:[{name:"sixty files, six queued winners, twenty directories, one memo",base64:readFileSync(file).toString("base64"),decoded}],logs},null,1)}\n`);
 expect(existsSync(fixturePath)).toBe(true);
});
