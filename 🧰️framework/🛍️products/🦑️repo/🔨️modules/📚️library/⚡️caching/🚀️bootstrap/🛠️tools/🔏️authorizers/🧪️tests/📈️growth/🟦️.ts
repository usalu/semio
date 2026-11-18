/** 📈️ Contract: no normal code addition can cross a source-closure bound. Every bound of the closure family is one proportional set (one file per 4096 source bytes, snapshot 1.5 times the sources, diagnostics with room above the admission), closures past the former limit of 2048 files are captured, and a closure past the real bound is refused in both languages with the chain and the way out. */
import{expect,test}from"bun:test";
import{mkdtemp,mkdir,writeFile,rm}from"node:fs/promises";
import{dirname,join,relative}from"node:path";
import{existsSync,readdirSync,readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{build}from"esbuild";
import{captureNxAuthorizerSourcesV1,censusNxAuthorizerSourcesV1,nxAuthorizerCapacityTextV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import snapshotSchema from"../../📦️snapshot/🧬️schema/🔣️.json";
import snapshotPolicy from"../../📦️snapshot/🔣️policy.json";
import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";
import scaleDeclaration from"../../../../../../🏗️build/📏️workspace-scale/🧬️schema/🔣️.json";

const policy=closureSchema.definitions.Policy.const,bytesPerFile=4096;

/** 🌳️ Writes a closure of exactly `count` files: one entry importing hubs of at most 64 leaves each, so no file nears the per-file site bound. */
async function closureTreeV1(area:string,count:number):Promise<void>{
 const hubs=Math.ceil((count-1)/65),leaves=count-1-hubs,writes:Promise<void>[]=[];
 for(let leaf=0;leaf<leaves;leaf++)writes.push(writeFile(join(area,"l"+leaf+".ts"),"export default {ordinal:"+leaf+"};"));
 for(let hub=0;hub<hubs;hub++){const own=Array.from({length:Math.max(0,Math.min(64,leaves-hub*64))},(_,index)=>hub*64+index);writes.push(writeFile(join(area,"h"+hub+".ts"),own.map(leaf=>"import l"+leaf+" from \"./l"+leaf+".ts\";export{l"+leaf+"};").join("\n")+"\nexport const hub"+hub+"=1;"));}
 writes.push(writeFile(join(area,"entry.ts"),Array.from({length:hubs},(_,hub)=>"import*as h"+hub+" from \"./h"+hub+".ts\";export{h"+hub+"};").join("\n")));
 await Promise.all(writes);
}

test("the closure bounds are one proportional family, so one cannot lag behind the others",()=>{
 expect(policy.maximumSourceBytes).toBe(policy.maximumFiles*bytesPerFile);
 expect(policy.maximumSnapshotBytes*2).toBe(policy.maximumSourceBytes*3);
 expect(policy.maximumFileBytes).toBeLessThanOrEqual(policy.maximumSourceBytes);
 expect(schema.properties.sources.maxItems).toBe(policy.maximumFiles);
 expect(snapshotSchema.properties.files.maxItems).toBe(policy.maximumFiles);
 expect(closureSchema.properties.files.maxItems).toBe(policy.maximumFiles);
 expect(snapshotPolicy.maximumSourceBytes).toBe(policy.maximumSourceBytes);
 expect(snapshotPolicy.maximumSnapshotBytes).toBe(policy.maximumSnapshotBytes);
 const diagnostic=schema.$defs.SourceCapacityDiagnostic.properties;
 expect(diagnostic.files.maximum).toBe(policy.maximumFiles+1);
 expect(diagnostic.bytes.maximum).toBe(policy.maximumSourceBytes+policy.maximumFileBytes);
 expect(snapshotPolicy.maximumOutputBytes*2).toBeGreaterThanOrEqual(snapshotPolicy.maximumSourceBytes);
 const census=schema.$defs.SourceCensusPolicy.const,discovery=schema.$defs.RuntimeDataDiscoveryPolicy.const,law=schema.$defs.CompilerClosureLaw.const,memo=schema.$defs.MemoPolicy.const;
 expect(census.maximumFiles).toBeGreaterThanOrEqual(policy.maximumFiles*4);
 expect(census.maximumSourceBytes).toBeGreaterThanOrEqual(policy.maximumSourceBytes*4);
 for(const key of["maximumDirectories","maximumEntries","maximumResults"]as const)expect(discovery[key],key).toBeGreaterThanOrEqual(policy.maximumFiles/2);
 expect(discovery.maximumDeclaredBytes).toBeGreaterThanOrEqual(policy.maximumSourceBytes);
 expect(memo.maximumFileBytes).toBeGreaterThanOrEqual(policy.maximumFiles*2048);
 expect(law.maximumSourceBytes).toBe(law.maximumFiles*bytesPerFile);
 expect(law.maximumFiles*4).toBeLessThanOrEqual(policy.maximumFiles*3);
 expect(law.maximumSnapshotBytes*4).toBeLessThanOrEqual(policy.maximumSnapshotBytes*3);
});

for(const count of[2049,4097])test("a closure of "+count+" files, past the former limit of 2048, is captured and agrees with esbuild",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});
 const area=await mkdtemp(join(artifacts,"growth-")),root=process.cwd();
 try{
  await closureTreeV1(area,count);
  const entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/"),captured=await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(120000));
  expect(captured.manifest.sources.length).toBe(count);
  const oracle=await build({absWorkingDir:root,entryPoints:[entry],bundle:true,platform:"node",format:"esm",write:false,metafile:true,logLevel:"silent"});
  expect(Object.keys(oracle.metafile!.inputs).sort()).toEqual(captured.manifest.sources.map(row=>row.path).sort());
 }finally{await rm(area,{recursive:true,force:true});}
},300000);

test("a closure past the real bound is refused in English and German with the chain and the way out",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});
 const area=await mkdtemp(join(artifacts,"growth-")),root=process.cwd();
 try{
  const count=policy.maximumFiles+1;await closureTreeV1(area,count);
  const entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/");let failure:any;
  try{await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(240000));}catch(error){failure=error;}
  expect(failure?.message).toContain("Authorizer source closure capacity exceeded: files="+count+"/"+policy.maximumFiles);
  expect(failure.message).toContain("Kapazität des Autorisierer-Quellabschlusses überschritten: Dateien="+count+"/"+policy.maximumFiles);
  expect(failure.message).toContain(entry);expect(failure.message).toContain("maximumFiles, maximumSourceBytes and maximumSnapshotBytes together");expect(failure.message).toContain("gemeinsam");
  expect(failure.message).not.toMatch(/\{\w+\}/u);
 }finally{await rm(area,{recursive:true,force:true});}
},300000);

test("every file-count ceiling of the closure family is declared once, equals the family bound and clears the real closure by a quarter",async()=>{
 const family=schema.$defs.SourceClosureFamily.const,bound=policy.maximumFiles,library=fileURLToPath(new URL("../../../../../../",import.meta.url));
 const read=(file:string)=>JSON.parse(readFileSync(library+file,"utf8")),at=(file:string,pointer:string)=>pointer.split("/").reduce((node:any,key)=>node[key],read(file)),declared=new Set<string>();
 for(const row of family.members){declared.add(row.file+"#"+row.pointer+"#"+row.key);expect(at(row.file,row.pointer)[row.key],row.file+" "+row.pointer).toBe(bound+row.offset);}
 for(const row of family.exempt){declared.add(row.file+"#"+row.pointer+"#"+row.key);const value=at(row.file,row.pointer)[row.key];expect(value,row.file+" "+row.pointer).toBeLessThan(bound);expect(row.reason.en.length&&row.reason.de.length,row.pointer).toBeGreaterThan(0);}
 const names=/^(sources|files|images|modules|reads|physical|sourceCount|closure|paths)$/u,undeclared:string[]=[];
 const walk=(node:any,pointer:string[],file:string)=>{if(Array.isArray(node)){node.forEach((value,index)=>walk(value,[...pointer,String(index)],file));return;}if(!node||typeof node!=="object")return;for(const[key,value]of Object.entries<any>(node)){if(value&&typeof value==="object"&&!Array.isArray(value)&&names.test(key))for(const limit of["maxItems","maximum"])if(Number.isInteger(value[limit])&&value[limit]>=64&&value[limit]<bound&&!declared.has(file+"#"+[...pointer,key].join("/")+"#"+limit))undeclared.push(file+" "+[...pointer,key].join("/")+" "+limit+"="+value[limit]);walk(value,[...pointer,key],file);}};
 for(const sub of["🏗️build","⚡️caching"])for(const rel of readdirSync(library+sub,{recursive:true})as string[]){if(!rel.endsWith(".json")||!(rel.includes("🧬️schema")||rel.endsWith("schema.json"))||rel.includes("🧪️tests")||rel.includes("🤖️generated")||rel.includes("node_modules"))continue;const file=sub+"/"+rel;let value:unknown;try{value=read(file);}catch{continue;}walk(value,[],file);}
 expect(undeclared,"ceilings below the closure bound that are neither family members nor exempt with a reason; list them in SourceClosureFamily").toEqual([]);
 let at_=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(at_,"go.work"))){const up=dirname(at_);if(up===at_)throw Error("Workspace root required");at_=up;}
 const census=await censusNxAuthorizerSourcesV1(at_,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts",AbortSignal.timeout(240000),{installed:true});
 expect(bound,"files "+census.files).toBeGreaterThanOrEqual(Math.ceil(census.files*1.25));expect(policy.maximumSourceBytes,"bytes "+census.sourceBytes).toBeGreaterThanOrEqual(Math.ceil(census.sourceBytes*1.25));
},300000);

test("the capacity text fills every field in both languages",()=>{
 const text=nxAuthorizerCapacityTextV1({files:7,bytes:11,path:"a/b.ts",chain:["a/b.ts","a/entry.ts"]}),[english,german]=text.split(" | ");
 expect(english).toContain("files=7/"+policy.maximumFiles+", bytes=11/"+policy.maximumSourceBytes);expect(german).toContain("Dateien=7/"+policy.maximumFiles+", Bytes=11/"+policy.maximumSourceBytes);
 for(const part of[english,german]){expect(part).toContain("a/b.ts <- a/entry.ts");expect(part).not.toMatch(/\{\w+\}/u);}
});

/** 📦️ The channel that carries the snapshot to the loader is a member of the same family: every byte ceiling on the way is the snapshot bound, so the largest closure the policy admits is deliverable; a snapshot above the former 32 MiB crosses the actual owned auxiliary pipe in the auxiliary tests. */
test("every byte ceiling of the snapshot family, among them the owned auxiliary channel, equals the snapshot bound",()=>{
 const family=schema.$defs.SourceClosureFamily.const,bound=policy.maximumSnapshotBytes,library=fileURLToPath(new URL("../../../../../../",import.meta.url));
 expect(family.byteBound).toBe("closure.definitions.Policy.maximumSnapshotBytes");expect(family.byteMembers.length).toBeGreaterThan(0);
 for(const row of family.byteMembers){const value=row.pointer.split("/").reduce((node:any,key:string)=>node[key],JSON.parse(readFileSync(library+row.file,"utf8")))[row.key];expect(value,row.file+" "+row.pointer).toBe(bound+row.offset);}
 expect(snapshotPolicy.maximumSnapshotBytes).toBe(bound);expect(bound).toBeGreaterThan(32*1048576);
});

/** 🩹️ The sealed Nx loader reads the same manifest the schemas bound, so its own refusal limits are family members too. */
test("the Nx loader patch refuses past exactly the bounds of the manifest schema",()=>{
 const library=fileURLToPath(new URL("../../../../../../",import.meta.url)),patch=readFileSync(library+"⚡️caching/🩹️patches/nx@23.2.0.patch","utf8");
 expect(Number(/manifest\.sources\.length > (\d+)/u.exec(patch)![1])).toBe(policy.maximumFiles);expect(Number(/manifest\.resolutions\.length > (\d+)/u.exec(patch)![1])).toBe(schema.properties.resolutions.maxItems);
});

/** 📏️ Contract: a ceiling on a collection that grows with the workspace never falls below the workspace again. Every such ceiling is a declared member of the workspace scale family, holds the declared workspace size times the safety factor, and a workspace-named ceiling that is neither a member nor exempt with a reason fails the scan. */
const scaleFamily=scaleDeclaration.definitions.Family.const;
const scaleLibrary=fileURLToPath(new URL("../../../../../../",import.meta.url));
const scaleRead=(file:string)=>JSON.parse(readFileSync(scaleLibrary+file,"utf8"));
const scalePointer=(file:string,pointer:string)=>pointer.split("/").filter(Boolean).reduce((node:any,key)=>node[key],scaleRead(file));
const scaleRequired=(expression:string)=>Math.ceil(expression.split("*").reduce((product,name)=>product*(scaleFamily.workspace as Record<string,number>)[name]!,1)*scaleFamily.safetyFactor);

test("every declared ceiling of a workspace-scaled collection holds the declared workspace size times the safety factor",()=>{
 const failures:string[]=[];
 for(const row of scaleFamily.members){const value=scalePointer(row.file,row.pointer)[row.key],required=scaleRequired(row.scale);if(!(Number.isInteger(value)&&value>=required))failures.push(row.file+" "+row.pointer+" "+row.key+"="+value+" < "+required+" ("+row.scale+" x "+scaleFamily.safetyFactor+")");}
 expect(failures,"ceilings below the workspace; raise them or derive them from the declared workspace").toEqual([]);
});

test("ceilings of the same documents are one value",()=>{
 for(const group of scaleFamily.equal){const values=group.map(index=>{const row=scaleFamily.members[index]!;return scalePointer(row.file,row.pointer)[row.key];});expect(new Set(values).size,JSON.stringify(values)).toBe(1);}
});

test("a workspace-named ceiling below the owners times the safety factor is a member or exempt with a reason in both languages",()=>{
 const bar=scaleRequired("owners"),declared=new Set<string>(),undeclared:string[]=[];
 for(const row of scaleFamily.members)declared.add(row.file+"#"+row.pointer+"#"+row.key);
 for(const row of scaleFamily.exempt){declared.add(row.file+"#"+row.pointer+"#"+row.key);expect(scalePointer(row.file,row.pointer)[row.key],row.file+" "+row.pointer).toBeLessThan(bar);expect(row.reason.en.length&&row.reason.de.length,row.pointer).toBeGreaterThan(0);}
 const numbers=/^(maximum(Owners|Projects|Documents|Closures|Nodes|Suspects)|entryMaximumDocuments|closureStoreMaximum\w+|outputSignatureMaximumEntries|memoMaximumEntries|digestCacheMaximumFiles|journalMaximumEvents)$/u,arrays=/^(projects|owners|documents|closures|targets|tasks)$/u;
 const walk=(node:any,pointer:string[],file:string)=>{
  if(Array.isArray(node)){node.forEach((value,index)=>walk(value,[...pointer,String(index)],file));return;}
  if(!node||typeof node!=="object")return;
  for(const[key,value]of Object.entries<any>(node)){
   if(Number.isInteger(value)&&numbers.test(key)&&value>=64&&value<bar&&!declared.has(file+"#"+pointer.join("/")+"#"+key))undeclared.push(file+" "+pointer.join("/")+" "+key+"="+value);
   if(value&&typeof value==="object"&&!Array.isArray(value)&&arrays.test(key))for(const limit of["maxItems","maximum"])if(Number.isInteger(value[limit])&&value[limit]>=64&&value[limit]<bar&&!declared.has(file+"#"+[...pointer,key].join("/")+"#"+limit))undeclared.push(file+" "+[...pointer,key].join("/")+" "+limit+"="+value[limit]);
   walk(value,[...pointer,key],file);
  }
 };
 for(const sub of["🏗️build","⚡️caching"])for(const rel of readdirSync(scaleLibrary+sub,{recursive:true})as string[]){if(!rel.endsWith(".json")||rel.includes("🧪️tests")||rel.includes("🤖️generated")||rel.includes("node_modules")||rel.includes("fixtures"))continue;let value:unknown;try{value=scaleRead(sub+"/"+rel);}catch{continue;}walk(value,[],sub+"/"+rel);}
 expect(undeclared,"ceilings named for a workspace collection that sit below the owners times the safety factor; add them to the workspace scale family as members or exempt").toEqual([]);
});
