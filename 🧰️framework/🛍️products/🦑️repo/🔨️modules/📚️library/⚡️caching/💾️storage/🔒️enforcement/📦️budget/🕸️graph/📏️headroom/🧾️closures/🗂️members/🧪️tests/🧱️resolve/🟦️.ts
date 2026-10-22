import{test,expect}from"bun:test";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{brotliCompressSync}from"node:zlib";
import{fileURLToPath,pathToFileURL}from"node:url";
import{createHash}from"node:crypto";
import{join,relative,sep}from"node:path";
import ts from"typescript";
import{encodeMemberHeaderV1,encodeMemberRowV1,fullMemberRowV1,lightMemberRowV1,memberGroupOfPathV1,memberStampOfRowStampV1,parseMemberIndexV1,proveGroupV1}from"../../🟨️.mjs";
import{indexBuildGraphMembersV1}from"../../🔎️scan/🟦️.ts";
import closures from"../../../🧬️schema/🔣️.json";

const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),P=closures.definitions.Protocol.const;
const text=readFileSync(new URL("../../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("closures.ts",text,ts.ScriptTarget.Latest,true);
const declarations=(names:string[])=>tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&Boolean(node.name&&names.includes(node.name.text))).map(node=>node.getText(tree).replace(/^export /u,"")).join("\n");
const load=(ports:Record<string,unknown>)=>Function(...Object.keys(ports),ts.transpileModule(declarations(["role","acceptReferences","acceptLinks","resolveIncrementalRows"]),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return resolveIncrementalRows")(...Object.values(ports))as(proof:any)=>void;

const root="/img",a="a".repeat(64),b="b".repeat(64),c="c".repeat(64),d="d".repeat(64),source=(character:string)=>"source:"+character.repeat(64);
const stamp7=(ino:number,size:number,mtime:number)=>["1",ino,"33188","1",size,mtime,mtime+1].join(":"),memberStamp=(ino:number,size:number,mtime:number)=>ino+":"+size+":"+mtime;
type Leaf={name:string;kind:string;bytes:number;ino:number;mtime:number};
const rowOf=(group:string,leaf:Leaf)=>({path:join(root,"inference",group,leaf.name),stamp:stamp7(leaf.ino,leaf.bytes,leaf.mtime),kind:leaf.kind,bytes:leaf.bytes,allocatedBytes:4096,modified:0}as any);
function proofOf(){return{image:{root},rows:new Map<string,any>(),references:new Set<string>(),candidates:new Map<string,any>(),retainAll:false,unproven:0,missing:new Set<string>()};}
function build(){
 const missing:string[]=[],ports={P,hash,join,relative,sep,memberGroupOfPathV1,memberStampOfRowStampV1,parseMemberIndexV1,proveGroupV1,smallBytes:(_proof:unknown,row:{path:string})=>Buffer.from(texts.get(row.path)!),missingLeaf:(_proof:unknown,path:string)=>{missing.push(path);}},texts=new Map<string,string>();
 return{missing,texts,run:load(ports)};
}
const indexText=(group:string,rows:readonly object[])=>[encodeMemberHeaderV1(group),...rows.map(row=>JSON.stringify(row))].join("\n")+"\n";

const SESSION_CREDIT_V1=16764800;
const readSessionV1=(worker:string,cwd:string,packets:readonly string[]):string[]=>{
 const input=packets.map(packet=>JSON.stringify({version:1,maximumReplyBytes:SESSION_CREDIT_V1})+"\n"+packet+"\n").join("")+"null\n",run=spawnSync(process.versions.bun?"node":process.execPath,["--max-old-space-size="+P.heapMiB,"--input-type=module","-e","const module=await import(process.argv[1]);await module.runCapturedBuildSourceV1();",pathToFileURL(worker).href],{input,cwd,encoding:"utf8",maxBuffer:P.maximumResultBytes});
 if(run.status!==0)throw Error(run.stderr);
 return run.stdout.split("\n").filter(Boolean);
};

test("a group is proven from its index and stamps alone: hashes come from the index, references and links from full rows, and no leaf is left for the child to read",()=>{
 const group="selections/"+a,leaves:Leaf[]=[{name:"np/"+"1".repeat(24)+".json",kind:"inference",bytes:100,ino:10,mtime:500},{name:"ob/"+d+".br",kind:"observation",bytes:50,ino:11,mtime:501}];
 const rows=[fullMemberRowV1({n:leaves[0]!.name,b:100,h:hash("np"),k:"inference",s:memberStamp(10,100,500),x:300,g:null,r:[source("c")],l:[{p:"ob/"+d+".br",m:"block"}]}),fullMemberRowV1({n:leaves[1]!.name,b:50,h:hash("ob"),k:"observation",s:memberStamp(11,50,501),x:90,g:null,r:[],l:[]})];
 const state=build(),proof=proofOf(),indexRow={path:join(root,"inference",group,"members.jsonl"),stamp:stamp7(1,1,1),kind:"member-index",bytes:900,allocatedBytes:4096,modified:0}as any,text=indexText(group,rows);
 state.texts.set(indexRow.path,text);proof.rows.set(indexRow.path,indexRow);
 const members=leaves.map(leaf=>rowOf(group,leaf));for(const row of members)proof.rows.set(row.path,row);
 proof.rows.get(join(root,"inference",group,"ob",d+".br"))&&0;
 state.run(proof);
 expect(members.map(row=>row.hash)).toEqual([hash("np"),hash("ob")]);expect(members.every(row=>row.trusted)).toBe(true);
 expect(indexRow.hash).toBe(hash(text));expect(indexRow.trusted).toBe(true);
 expect([...proof.references]).toEqual([source("c")]);expect(proof.retainAll).toBe(false);expect(proof.unproven).toBe(0);expect(state.missing).toEqual([]);
});

test("a block link to a missing observation keeps every inventory, a link that may not be missing refuses the proof",()=>{
 const group="contexts/"+b,leaf:Leaf={name:"np/"+"2".repeat(24)+".json",kind:"inference",bytes:10,ino:20,mtime:600};
 const make=(link:{p:string;m?:"block"})=>{const state=build(),proof=proofOf(),index={path:join(root,"inference",group,"members.jsonl"),stamp:stamp7(2,2,2),kind:"member-index",bytes:500,allocatedBytes:4096,modified:0}as any,row=rowOf(group,leaf);state.texts.set(index.path,indexText(group,[fullMemberRowV1({n:leaf.name,b:10,h:hash("x"),k:"inference",s:memberStamp(20,10,600),x:20,g:null,r:[],l:[link]})]));proof.rows.set(index.path,index);proof.rows.set(row.path,row);return{state,proof};};
 const tolerated=make({p:"np/ob/"+d+".br",m:"block"});tolerated.state.run(tolerated.proof);expect(tolerated.proof.retainAll).toBe(true);expect(tolerated.state.missing).toEqual([join(root,"inference",group,"np","ob",d+".br")]);
 const refused=make({p:"mr/pk-1-"+"e".repeat(16)+".idx"});expect(()=>refused.state.run(refused.proof)).toThrow("Complete retained closure publication required");
});

test("a leaf the index cannot vouch for, a stale stamp, a light row, a missing or oversized index each leave the group unproven and every inventory retained",()=>{
 const group="selections/"+c,fresh:Leaf={name:"np/"+"3".repeat(24)+".json",kind:"inference",bytes:10,ino:30,mtime:700},other:Leaf={name:"ip/"+"4".repeat(24)+".json",kind:"precision",bytes:20,ino:31,mtime:701};
 const trial=(text:string|undefined,indexBytes=100)=>{const state=build(),proof=proofOf(),rows=[rowOf(group,fresh),rowOf(group,other)];for(const row of rows)proof.rows.set(row.path,row);if(text!==undefined){const index={path:join(root,"inference",group,"members.jsonl"),stamp:stamp7(3,3,3),kind:"member-index",bytes:indexBytes,allocatedBytes:4096,modified:0}as any;state.texts.set(index.path,text);proof.rows.set(index.path,index);}state.run(proof);return{proof,rows};};
 const light=lightMemberRowV1({n:fresh.name,b:10,h:hash("l"),k:"inference",s:memberStamp(30,10,700)}),full=fullMemberRowV1({n:other.name,b:20,h:hash("f"),k:"precision",s:memberStamp(31,20,701),x:40,g:null,r:[],l:[]}),stale=fullMemberRowV1({n:other.name,b:20,h:hash("f"),k:"precision",s:memberStamp(31,20,999),x:40,g:null,r:[],l:[]});
 const missing=trial(undefined);expect(missing.rows.map(row=>row.hash)).toEqual([undefined,undefined]);expect(missing.proof.retainAll).toBe(true);expect(missing.proof.unproven).toBe(2);
 const lightOnly=trial(indexText(group,[light,full]));expect(lightOnly.rows.map(row=>row.hash)).toEqual([hash("l"),hash("f")]);expect(lightOnly.proof.retainAll).toBe(true);expect(lightOnly.proof.unproven).toBe(0);
 const staleOnly=trial(indexText(group,[light,stale]));expect(staleOnly.rows[1]!.hash).toBeUndefined();expect(staleOnly.rows[0]!.hash).toBe(hash("l"));expect(staleOnly.proof.unproven).toBe(1);expect(staleOnly.proof.retainAll).toBe(true);
 const foreign=trial(indexText("selections/"+a,[light,full]));expect(foreign.rows.map(row=>row.hash)).toEqual([undefined,undefined]);
 const oversized=trial(indexText(group,[light,full]),P.maximumIndexBytes+1);expect(oversized.rows.map(row=>row.hash)).toEqual([undefined,undefined]);expect(oversized.proof.unproven).toBe(2);
});

test("the shared namespace and the generation documents are vouched by their own indexes, the mutable pointer is left to the read child, an unindexed fragment keeps every inventory",()=>{
 const state=build(),proof=proofOf(),fragmentName="o/"+d+".json.br",publicationName="inference/emoji-project-json.publication.json",generationIndex={path:join(root,"generations","members.jsonl"),stamp:stamp7(60,60,60),kind:"member-index",bytes:400,allocatedBytes:4096,modified:0}as any,canonicalIndex={path:join(root,"inference","members.jsonl"),stamp:stamp7(61,61,61),kind:"member-index",bytes:400,allocatedBytes:4096,modified:0}as any;
 const fragment={path:join(root,"generations",...fragmentName.split("/")),stamp:stamp7(62,100,900),kind:"generation-fragment",bytes:100,allocatedBytes:4096,modified:0}as any,stray={path:join(root,"generations","o",c+".json.br"),stamp:stamp7(63,100,901),kind:"generation-fragment",bytes:100,allocatedBytes:4096,modified:0}as any,pointer={path:join(root,"generations","current.json"),stamp:stamp7(64,10,902),kind:"generation-pointer",bytes:10,allocatedBytes:4096,modified:0}as any,publication={path:join(root,"inference",...publicationName.split("/")),stamp:stamp7(65,5000,903),kind:"publication",bytes:5000,allocatedBytes:8192,modified:0}as any;
 state.texts.set(generationIndex.path,indexText("generations",[fullMemberRowV1({n:fragmentName,b:100,h:hash("fragment"),k:"generation-fragment",s:memberStamp(62,100,900),x:900,g:null,r:[source("a")],l:[]})]));
 state.texts.set(canonicalIndex.path,indexText("canonical",[fullMemberRowV1({n:publicationName,b:5000,h:hash("publication"),k:"publication",s:memberStamp(65,5000,903),x:9000,g:null,r:[source("b")],l:[]})]));
 for(const row of[generationIndex,canonicalIndex,fragment,stray,pointer,publication])proof.rows.set(row.path,row);
 state.run(proof);
 expect(fragment.hash).toBe(hash("fragment"));expect(publication.hash).toBe(hash("publication"));expect([...proof.references].sort()).toEqual([source("a"),source("b")]);
 expect(stray.hash).toBeUndefined();expect(proof.unproven).toBe(1);expect(proof.retainAll).toBe(true);
 expect(pointer.hash).toBeUndefined();expect(pointer.trusted).toBeUndefined();expect(generationIndex.trusted).toBe(true);expect(canonicalIndex.trusted).toBe(true);
});

test("only groups whose members are all hashed and of a plannable kind may be retired",()=>{
 expect(text).toContain("const planned=new Set<string>(schema.definitions.RetirementPlan.properties.members.items.properties.kind.enum),groups=[...candidates.values()].filter(group=>group.rows.length&&group.rows.every(row=>row.hash!==undefined&&planned.has(row.kind)))");
 expect(closures.definitions.RetirementPlan.properties.members.items.properties.kind.enum).toContain("member-index");
 expect(closures.definitions.Protocol.const.maximumExpandedBytes).toBeGreaterThanOrEqual(167000000);
});

test("inventories are vouched by their content-addressed name and retirement-pinned members by their plan, so the child opens neither",()=>{
 const state=build(),proof=proofOf(),inventory={path:join(root,"build-input-closures",a+".json"),stamp:stamp7(40,10,800),kind:"source",bytes:10,allocatedBytes:4096,modified:0}as any,lock={path:join(root,"build-lock-closures",b+".json"),stamp:stamp7(41,10,801),kind:"lock",bytes:10,allocatedBytes:4096,modified:0}as any,retired={path:join(root,"retired",c+".d","np","x.json"),stamp:stamp7(42,10,802),kind:"retired",bytes:10,allocatedBytes:4096,modified:0,expectedHash:hash("plan")}as any,snapshot={path:join(root,"snapshots",d+".json"),stamp:stamp7(43,10,803),kind:"snapshot",bytes:10,allocatedBytes:4096,modified:0}as any;
 for(const row of[inventory,lock,retired,snapshot])proof.rows.set(row.path,row);
 state.run(proof);
 expect(inventory.hash).toBe(a);expect(inventory.trusted).toBe(true);expect(lock.hash).toBe(b);expect([...proof.candidates.keys()].sort()).toEqual(["lock:"+b,"source:"+a]);
 expect(retired.hash).toBe(hash("plan"));expect(retired.trusted).toBe(true);
 expect(snapshot.hash).toBeUndefined();expect(snapshot.trusted).toBeUndefined();
});

test("a prepared retirement member keeps its planned hash when the index cannot vouch for it",()=>{
 const group="selections/"+d,leaf:Leaf={name:"np/"+"5".repeat(24)+".json",kind:"inference",bytes:10,ino:50,mtime:900},state=build(),proof=proofOf(),row=rowOf(group,leaf);row.expectedHash=hash("planned");proof.rows.set(row.path,row);
 state.run(proof);expect(row.hash).toBe(hash("planned"));expect(row.trusted).toBe(true);expect(proof.retainAll).toBe(true);
});

test("the proof's cost is its index bytes: ten thousand leaves are resolved without opening one of them",()=>{
 const group="selections/"+a,proof=proofOf(),state=build(),leaves:Leaf[]=Array.from({length:10000},(_,at)=>({name:"np/"+at.toString(16).padStart(24,"0")+".json",kind:"inference",bytes:100+at,ino:1000+at,mtime:5000+at})),index={path:join(root,"inference",group,"members.jsonl"),stamp:stamp7(1,1,1),kind:"member-index",bytes:2000000,allocatedBytes:4096,modified:0}as any;
 state.texts.set(index.path,indexText(group,leaves.map(leaf=>JSON.parse(encodeMemberRowV1(fullMemberRowV1({n:leaf.name,b:leaf.bytes,h:hash(leaf.name),k:"inference",s:memberStamp(leaf.ino,leaf.bytes,leaf.mtime),x:leaf.bytes*2,g:null,r:[source("c")],l:[]}))))));
 proof.rows.set(index.path,index);for(const leaf of leaves){const row=rowOf(group,leaf);proof.rows.set(row.path,row);}
 const started=performance.now();state.run(proof);const elapsed=performance.now()-started;
 expect([...proof.rows.values()].filter(row=>row.kind==="inference").every(row=>row.trusted===true&&row.hash!==undefined)).toBe(true);expect(proof.unproven).toBe(0);expect([...proof.references]).toEqual([source("c")]);
 console.log("[members] resolved "+leaves.length+" leaves in "+elapsed.toFixed(0)+" ms");
 expect(elapsed).toBeLessThan(5000);
},60000);

test("a group indexed through the real read child is proven by the closure resolver without opening one of its leaves again",async()=>{
 const image=mkdtempSync(join(tmpdir(),"mr-")),id="9".repeat(64),group="selections/"+id,directory=join(image,"inference",group),observation=Buffer.from("[]"),key=hash(observation),closure=hash("closure"),body=JSON.stringify({value:"SEMIO_SOURCE_CLOSURE_"+closure}),header=JSON.stringify({authorityObservations:[],blocks:[key],digest:hash("d"),observations:[],resultSha256:hash(body)});
 try{
  mkdirSync(join(directory,"np","ob"),{recursive:true});
  const leaf=join(directory,"np","1".repeat(24)+".json"),block=join(directory,"np","ob",key+".br");
  writeFileSync(leaf,brotliCompressSync(Buffer.from(header+"\n"+body)));writeFileSync(block,brotliCompressSync(observation));
  const worker=fileURLToPath(new URL("../../../👷️read.mjs",import.meta.url)),report=await indexBuildGraphMembersV1(image,{lease:async()=>({release(){}}),read:async(requests,accept)=>{const lines=readSessionV1(worker,image,requests.map(request=>JSON.stringify({files:[request],imageRoot:image,remainingBytes:P.maximumProofBytes,root:process.cwd()})));requests.forEach((request,at)=>accept(request,JSON.parse(lines[at]!)));}},{cancelled:()=>false,progress:()=>{}});
  expect(report).toMatchObject({indexed:2,refused:0,busy:0});
  const stamp=(path:string)=>{const row=lstatSync(path,{bigint:true});return[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");},proof=proofOf(),state=build(),index=join(directory,"members.jsonl");
  const entry=(path:string,kind:string)=>({path,stamp:stamp(path),kind,bytes:lstatSync(path).size,allocatedBytes:4096,modified:0}as any),rows=[entry(leaf,"inference"),entry(block,"observation"),entry(index,"member-index")];
  for(const row of rows)proof.rows.set(row.path,row);
  state.texts.set(index,readFileSync(index,"utf8"));proof.image.root=image;
  state.run(proof);
  expect(rows.map(row=>row.trusted)).toEqual([true,true,true]);expect(rows[0]!.hash).toBe(hash(readFileSync(leaf)));expect(rows[1]!.hash).toBe(hash(readFileSync(block)));
  expect([...proof.references]).toEqual(["source:"+closure]);expect(proof.retainAll).toBe(false);expect(proof.unproven).toBe(0);expect(state.missing).toEqual([]);
 }finally{rmSync(image,{recursive:true,force:true});}
},60000);
