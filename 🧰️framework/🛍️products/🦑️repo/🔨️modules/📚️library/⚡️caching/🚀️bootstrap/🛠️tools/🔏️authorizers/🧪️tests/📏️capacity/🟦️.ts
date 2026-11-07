import{expect,test}from"bun:test";
import{mkdtemp,mkdir,writeFile,rm}from"node:fs/promises";
import{join,relative,resolve}from"node:path";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{captureNxAuthorizerSourcesV1,censusNxAuthorizerSourcesV1,nxAuthorizerCensusCapacityV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import censusFixture from"./🧫️census.json";
import boundsFixture from"./🧫️bounds.json";
import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";
import{buildSourceCapacityV1}from"../../../../../../🏗️build/📥️source/🔏️closure/📏️capacity/🟦️.ts";
import{buildSourceClosureNodeArgumentsV1}from"../../../../../../🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs";

test("actual Source byte refusal retains bounded origin and independent largest-source census",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");
 await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"cap-")),root=process.cwd(),entry=join(area,"entry.ts"),paths=Array.from({length:fixture.parts},(_,index)=>join(area,String(index).padStart(2,"0")+".ts")),bytes='export default '+JSON.stringify("x".repeat(fixture.partBytes-18))+';',source=paths.map((_,index)=>'import part'+index+' from "./'+String(index).padStart(2,"0")+'.ts";').join("\n"),rows=[{path:relative(root,entry).replaceAll("\\","/"),bytes:Buffer.byteLength(source)},...paths.map(path=>({path:relative(root,path).replaceAll("\\","/"),bytes:Buffer.byteLength(bytes)}))];
 try{
  await writeFile(entry,source);await Promise.all(paths.map(path=>writeFile(path,bytes)));let failure:any;
  try{await captureNxAuthorizerSourcesV1(root,relative(root,entry).replaceAll("\\","/"),AbortSignal.timeout(30000),{installed:true});}catch(error){failure=error;}
  expect(failure?.message).toContain("Authorizer source closure capacity exceeded");expect(failure?.diagnostic).toBeDefined();
  const diagnostic=failure.diagnostic,ajv=new Ajv({strict:true});expect(ajv.validate(schema.$defs.SourceCapacityDiagnostic,diagnostic),JSON.stringify(ajv.errors)).toBe(true);
  const db=new Database(":memory:");try{const expected=db.query("select json_extract(value,'$.path') path, json_extract(value,'$.bytes') bytes from json_each(?) order by bytes desc,path collate binary limit ?").all(JSON.stringify(rows),fixture.maximumLargest);expect(diagnostic.largest).toEqual(expected);expect(diagnostic.bytes).toBe((db.query("select sum(json_extract(value,'$.bytes')) bytes from json_each(?)").get(JSON.stringify(rows))as{bytes:number}).bytes);}finally{db.close();}
  const census=failure.sourceCensus;expect(census).toBeDefined();expect(census.length).toBe(fixture.expectedFiles);expect(census.length).toBeLessThanOrEqual(2049);expect(Object.isFrozen(census)).toBe(true);
  expect(census.map((row:any)=>({path:row.path,bytes:row.bytes})).sort((a:any,b:any)=>a.path<b.path?-1:1)).toEqual([...rows].sort((a,b)=>a.path<b.path?-1:1));
  expect(census.every((row:any)=>Object.isFrozen(row)&&row.origin===(row.path===rows[0].path?"entry":"import")&&row.importer===(row.path===rows[0].path?null:rows[0].path))).toBe(true);
  expect(Object.getOwnPropertyDescriptor(failure,"sourceCensus")).toMatchObject({enumerable:false,writable:false,configurable:false});expect(JSON.stringify(failure)).not.toContain("sourceCensus");
  const measured=await censusNxAuthorizerSourcesV1(root,relative(root,entry).replaceAll("\\","/"),AbortSignal.timeout(90000),{installed:true});expect(measured.files).toBe(fixture.expectedFiles);expect(measured.sourceBytes).toBe(rows.reduce((sum,row)=>sum+row.bytes,0));expect(()=>nxAuthorizerCapturedBytesV1(measured as any)).toThrow();
  expect(diagnostic.files).toBe(fixture.expectedFiles);expect(diagnostic.chain).toEqual([diagnostic.path,rows[0].path]);expect(diagnostic.origin).toBe("import");expect(rows.some(row=>row.path===diagnostic.path)).toBe(true);expect(JSON.stringify(diagnostic).length).toBeLessThan(4096);
 }finally{await rm(area,{recursive:true,force:true});}
},120000);

test("metadata census follows the admitted traversal without issuing an image",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"census-")),root=process.cwd(),entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/");
 try{await writeFile(join(area,"entry.ts"),censusFixture.entry);await writeFile(join(area,"part.ts"),censusFixture.part);for(const row of censusFixture.cases){const controller=new AbortController();if(row.cancel)controller.abort();if(!row.accepted){await expect(censusNxAuthorizerSourcesV1(root,entry,controller.signal)).rejects.toThrow();continue;}const progress:number[]=[];const result=await censusNxAuthorizerSourcesV1(root,entry,controller.signal,{progress:count=>progress.push(count)});const ajv=new Ajv({strict:true,allowUnionTypes:true});expect(ajv.validate(schema.$defs.SourceCensus,result),JSON.stringify(ajv.errors)).toBe(true);expect(result.kind).toBe("source-census");expect(result.files).toBe(2);expect(result.sources).toHaveLength(2);const db=new Database(":memory:");try{expect(result.sourceBytes).toBe((db.query("select sum(json_extract(value,'$.bytes')) bytes from json_each(?)").get(JSON.stringify(result.sources)) as {bytes:number}).bytes);}finally{db.close();}expect(()=>nxAuthorizerCapturedBytesV1(result as any)).toThrow();const admitted=await captureNxAuthorizerSourcesV1(root,entry,controller.signal),bytes=nxAuthorizerCapturedBytesV1(admitted),imports=nxAuthorizerCapturedImportsV1(root,admitted),snapshot={version:1,root:resolve(root),entry,files:admitted.manifest.sources.map(value=>({...value,base64:Buffer.from(bytes.get(value.path)!).toString("base64"),imports:imports.get(value.path)}))};expect(result.snapshotBytes).toBe(Buffer.byteLength(JSON.stringify(snapshot)));expect(progress.at(-1)).toBe(2);}}finally{await rm(area,{recursive:true,force:true});}
},45000);

test("source and snapshot limits accept their boundaries and refuse the next unit",()=>{
 const policy=closureSchema.definitions.Policy.const,oracle=new Ajv({strict:true}).compile({type:"object",required:["files","sourceBytes","snapshotBytes"],additionalProperties:false,properties:{files:{type:"integer",minimum:0,maximum:policy.maximumFiles},sourceBytes:{type:"integer",minimum:0,maximum:policy.maximumSourceBytes},snapshotBytes:{type:"integer",minimum:0,maximum:policy.maximumSnapshotBytes}}});
 expect(schema.properties.sources.maxItems).toBe(policy.maximumFiles);for(const row of boundsFixture.cases){const input={files:row.files,sourceBytes:row.sourceBytes,snapshotBytes:row.snapshotBytes};expect(oracle(input),row.id).toBe(row.accepted);expect(buildSourceCapacityV1(input.files,input.sourceBytes,input.snapshotBytes),row.id).toBe(row.accepted);}
});

test("diagnostic census has independent finite limits",()=>{
 const policy=schema.$defs.SourceCensusPolicy.const,oracle=new Ajv({strict:true}).compile({type:"object",required:["files","sourceBytes"],additionalProperties:false,properties:{files:{type:"integer",minimum:0,maximum:policy.maximumFiles},sourceBytes:{type:"integer",minimum:0,maximum:policy.maximumSourceBytes}}});for(const row of boundsFixture.censusCases){const input={files:row.files,sourceBytes:row.sourceBytes};expect(oracle(input),row.id).toBe(row.accepted);expect(nxAuthorizerCensusCapacityV1(input.files,input.sourceBytes),row.id).toBe(row.accepted);}
});

test("Node channel enforces the same serialized boundary and rejects a forged policy",()=>{
 const policy=closureSchema.definitions.Policy.const,base=Buffer.byteLength(JSON.stringify({payload:""}));for(const row of boundsFixture.cases.filter(row=>row.id.startsWith("snapshot-"))){const payload="x".repeat(row.snapshotBytes-base),prepare=()=>buildSourceClosureNodeArgumentsV1({payload},policy,128,"node");if(row.accepted)expect(Buffer.byteLength(prepare().bootstrap)).toBe(row.snapshotBytes+1);else expect(prepare).toThrow();}expect(()=>buildSourceClosureNodeArgumentsV1({payload:""},{...policy,maximumSnapshotBytes:policy.maximumSnapshotBytes+1},128,"node")).toThrow();
});

test("source census retains the first discovery and attributes workspace manifests",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"origin-")),root=process.cwd(),path=(name:string)=>relative(root,join(area,name)).replaceAll("\\","/");
 try{
  await writeFile(join(area,"entry.ts"),censusFixture.firstEntry);await writeFile(join(area,"other.ts"),censusFixture.other);await writeFile(join(area,"part.ts"),censusFixture.part);await writeFile(join(area,"package.ts"),censusFixture.package);
  const first=await censusNxAuthorizerSourcesV1(root,path("entry.ts"),AbortSignal.timeout(30000));expect(first.sources.find(row=>row.path===path("part.ts"))).toMatchObject({origin:"import",importer:path("entry.ts")});
  const packages=await censusNxAuthorizerSourcesV1(root,path("package.ts"),AbortSignal.timeout(30000),{installed:true});expect(packages.sources.find(row=>row.path==="package.json")).toMatchObject({origin:"workspace-manifest",importer:path("package.ts")});
 }finally{await rm(area,{recursive:true,force:true});}
},45000);

test("census cancellation reaches import projection",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"cancel-")),root=process.cwd(),entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/"),controller=new AbortController();try{await writeFile(join(area,"entry.ts"),censusFixture.entry);await writeFile(join(area,"part.ts"),censusFixture.part);await expect(censusNxAuthorizerSourcesV1(root,entry,controller.signal,{progress:(_files,phase)=>{if(phase==="projection")controller.abort();}})).rejects.toThrow();}finally{await rm(area,{recursive:true,force:true});}
},45000);

test("census projection yields for asynchronously delivered cancellation",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"yield-")),root=process.cwd(),entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/"),controller=new AbortController();try{await writeFile(join(area,"entry.ts"),censusFixture.entry);await writeFile(join(area,"part.ts"),censusFixture.part);let scheduled=false;await expect(censusNxAuthorizerSourcesV1(root,entry,controller.signal,{progress:(_files,phase)=>{if(phase==="projection"&&!scheduled){scheduled=true;setTimeout(()=>controller.abort(),0);}}})).rejects.toThrow();}finally{await rm(area,{recursive:true,force:true});}
},45000);

test("census snapshot size counts Unicode and escaped JSON paths",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"unicode-")),root=resolve(process.cwd()),entry=relative(root,join(area,"ä.ts")).replaceAll("\\","/");try{await writeFile(join(area,"ä.ts"),censusFixture.unicodeEntry);await writeFile(join(area,"ü.ts"),censusFixture.unicodePart);const signal=AbortSignal.timeout(30000),census=await censusNxAuthorizerSourcesV1(root,entry,signal),admitted=await captureNxAuthorizerSourcesV1(root,entry,signal),bytes=nxAuthorizerCapturedBytesV1(admitted),imports=nxAuthorizerCapturedImportsV1(root,admitted),snapshot={version:1,root,entry,files:admitted.manifest.sources.map(row=>({...row,base64:Buffer.from(bytes.get(row.path)!).toString("base64"),imports:imports.get(row.path)}))};expect(snapshot.files.some(row=>Object.hasOwn(row.imports!,"./ü.ts"))).toBe(true);expect(census.snapshotBytes).toBe(Buffer.byteLength(JSON.stringify(snapshot)));if(process.platform==="win32")expect(JSON.stringify(snapshot)).toContain("\\\\");}finally{await rm(area,{recursive:true,force:true});}
},45000);

test("census projects imports across cooperative batches",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"batch-")),root=resolve(process.cwd()),entry=relative(root,join(area,"entry.ts")).replaceAll("\\","/"),names=Array.from({length:20},(_,index)=>"part"+index+".ts");try{await Promise.all(names.map(name=>writeFile(join(area,name),censusFixture.part)));await writeFile(join(area,"entry.ts"),names.map((name,index)=>"import value"+index+" from "+JSON.stringify("./"+name)+";").join("\n"));const signal=AbortSignal.timeout(30000),census=await censusNxAuthorizerSourcesV1(root,entry,signal),admitted=await captureNxAuthorizerSourcesV1(root,entry,signal),bytes=nxAuthorizerCapturedBytesV1(admitted),imports=nxAuthorizerCapturedImportsV1(root,admitted),snapshot={version:1,root,entry,files:admitted.manifest.sources.map(row=>({...row,base64:Buffer.from(bytes.get(row.path)!).toString("base64"),imports:imports.get(row.path)}))};expect(census.files).toBe(21);expect(census.snapshotBytes).toBe(Buffer.byteLength(JSON.stringify(snapshot)));}finally{await rm(area,{recursive:true,force:true});}
},45000);
