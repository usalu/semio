import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import {summarizeNxSdkMaterialV1} from "../🟦️.ts";

test("SDK material package accounting agrees with independent SQLite",()=>{const db=new Database(":memory:");try{db.exec("create table material(path text,bytes integer,allocated integer)");for(const row of fixture.rows)db.query("insert into material values(?,?,?)").run(row.path,row.bytes,row.allocated);const rows=db.query("with names as(select case when substr(path,14,1)='@' then substr(path,14,instr(substr(path,14),'/')+instr(substr(path,14+instr(substr(path,14),'/')),'/')-1) else substr(path,14,instr(substr(path,14),'/')-1) end name,bytes,allocated from material)select name,count(*) files,sum(bytes) logicalBytes,sum(allocated) allocatedBytes from names group by name order by name").all();expect(rows).toEqual(fixture.packages);expect(rows).toEqual([...summarizeNxSdkMaterialV1(fixture.rows)]);expect(()=>summarizeNxSdkMaterialV1([{path:"../foreign",bytes:1,allocated:1}])).toThrow();}finally{db.close();}});

/** 📥️ Material reports follow the generation-bound base reference across native path dialects. */
test("SDK material resolves the closed referenced base rather than patched generation inputs",async()=>{const{readNxSdkMaterialReferenceV1,nxSdkMaterialSnapshotPathV1}=await import("../🟦️.ts"),{nxSdkSnapshotIdentityV1}=await import("../../📥️snapshot/🟦️.ts"),reference=(await import("./📥️reference.json")).default,schema=(await import("../🧬️schema/🔣️.json")).default,Ajv=(await import("ajv")).default,validate=new Ajv({strict:true}).compile(schema.definitions.MaterialReference),path=await import("node:path"),db=new Database(":memory:");try{for(const row of reference.cases){expect(validate(row.value),row.name).toBe(row.accept);if(row.accept){const actual=readNxSdkMaterialReferenceV1(row.value),oracle=db.query("select json_extract(?1,'$.snapshot') snapshot").get(JSON.stringify(row.value))as{snapshot:string};expect(actual.snapshot,row.name).toBe(oracle.snapshot);}else expect(()=>readNxSdkMaterialReferenceV1(row.value),row.name).toThrow();}for(const row of reference.paths){if(row.platform!=="posix"&&row.platform!=="win32")throw Error("Closed portable path dialect required");const actual=nxSdkMaterialSnapshotPathV1(row.root,reference.reference,row.platform),oracle=path[row.platform].join(row.root,"sdk",reference.reference.snapshot);expect(actual,row.name).toBe(row.expected);expect(actual,row.name).toBe(oracle);}const patched=new Map([["package.json",Buffer.from(JSON.stringify({dependencies:{nx:"23.2.0"},patchedDependencies:{"nx@23.2.0":"patches/changed.patch"}}))],["bun.lock",Buffer.from("{}")]]);expect(nxSdkSnapshotIdentityV1({platform:"darwin",architecture:"arm64",libc:""},patched)).not.toBe(readNxSdkMaterialReferenceV1(reference.reference).snapshot);}finally{db.close();}});

/** 🔏️ Changing the referenced base or another generation input invalidates its authenticated identity. */
test("SDK material reference is sealed into the actual generation input digest",async()=>{const{nxSdkGenerationMaterialReferenceV1}=await import("../🟦️.ts"),fixture=(await import("./📥️reference.json")).default,environment={platform:"darwin",architecture:"arm64",libc:""},files=new Map([["package.json",Buffer.from(JSON.stringify({name:"sdk",dependencies:{nx:"23.2.0"},patchedDependencies:{"nx@23.2.0":"patches/"+"f".repeat(64)+".patch"}}))],["bun.lock",Buffer.from("{}")],["patches/"+"f".repeat(64)+".patch",Buffer.from("original-patch")],[".semio-authorizer-source.json",Buffer.from(JSON.stringify({version:1,mode:"release",entry:"entry.ts",resolutions:[],sources:[{path:"entry.ts",sha256:"a".repeat(64)}]}))],[".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:"bun@1.3.14",resolutions:[],sources:[{path:"entry.ts",sha256:"a".repeat(64)}]}))],[".semio-sdk-material.json",Buffer.from(JSON.stringify(fixture.reference))]]),oracle=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v4\0").update("6:darwin5:arm640:");for(const[path,bytes]of files)oracle.update(Buffer.byteLength(path)+":"+path+bytes.byteLength+":").update(bytes);const digest=oracle.digest("hex");expect(nxSdkGenerationMaterialReferenceV1(environment,files,digest).snapshot).toBe(fixture.reference.snapshot);const changed=new Map(files);changed.set(".semio-sdk-material.json",Buffer.from(JSON.stringify({...fixture.reference,snapshot:"b".repeat(64)})));expect(()=>nxSdkGenerationMaterialReferenceV1(environment,changed,digest)).toThrow();changed.set(".semio-sdk-material.json",files.get(".semio-sdk-material.json")!);changed.set("package.json",Buffer.from("changed dependency"));expect(()=>nxSdkGenerationMaterialReferenceV1(environment,changed,digest)).toThrow();expect(()=>nxSdkGenerationMaterialReferenceV1({...environment,architecture:"x64"},files,digest)).toThrow();});

test("SDK material reads full valid authorizer digest manifests within their own byte and aggregate bounds",async()=>{
 const fs=await import("node:fs/promises"),{tmpdir}=await import("node:os"),{join,basename}=await import("node:path"),{createHash}=await import("node:crypto"),ts=await import("typescript"),Ajv=(await import("ajv")).default;
 const routing=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const leases=await import("../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts");
 const physical=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts");
 const authorizer=(await import("../../../🔏️authorizers/🧬️schema/🔣️.json")).default,material=(await import("../🧬️schema/🔣️.json")).default;
 const reader=await fs.readFile(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("material.ts",reader,ts.ScriptTarget.Latest,true);
 const actual=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="boundedInput");
 if(!actual)throw Error("Actual SDK material input reader required");
 const code=ts.transpileModule(actual.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const ports={basename,protocol:material.definitions.Protocol.const,authorizerDigestManifestPolicy:authorizer.$defs.GenerationDigestManifestPolicy?.const,requireIssuedResourceControlRefinementV1:routing.requireIssuedResourceControlRefinementV1,checkResourceLeaseControl:leases.checkResourceLeaseControl,readPhysicalCargoSourceV1:physical.readPhysicalCargoSourceV1};
 const read=new Function(...Object.keys(ports),code+";return boundedInput;")(...Object.values(ports));
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=await fs.mkdtemp(join(parent,"sdk-metadata-"));
 try{
  const names=Array.from({length:fixture.authorizerManifests.compilerRows},(_,i)=>fixture.authorizerManifests.pathStem+String(i).padStart(5,"0")+".ts");
  const rows=names.map(path=>({path,sha256:createHash("sha256").update(path).digest("hex")}));
  const source={version:1,mode:"release",entry:names[0],resolutions:[],sources:rows.slice(0,fixture.authorizerManifests.sourceRows)};
  const compiler={version:1,mode:"release",bun:"bun@1.4.2",physicalSourceRoot:root,entryRole:"program",runtimeEntryRole:"module",resolutions:[],sources:rows};
  const ajv=new Ajv({strict:true});ajv.addSchema(authorizer);
  expect(ajv.validate(authorizer.$id,source)).toBe(true);expect(ajv.validate(authorizer.$id+"#/$defs/CompilerPublication",compiler)).toBe(true);
  const cases=[[".semio-authorizer-source.json",source],[".semio-authorizer-compiler.json",compiler]]as const;
  await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"test:sdk-material",maximumElapsedMilliseconds:15000},{},Date.now()),async original=>{
   const control=routing.createIssuedResourceControlV1(original,"test:sdk-material",0),store={root,control},budget=material.definitions.Protocol.const.maximumIdentityBytes;
   for(const [name,value]of cases){
    const file=join(root,name),bytes=Buffer.from(JSON.stringify(value)+"\n");
    expect(bytes.byteLength).toBeGreaterThan(262144);await fs.writeFile(file,bytes);
    expect(await read(file,store,control,budget)).toEqual(bytes);
    await expect(read(file,store,control,bytes.byteLength-1)).rejects.toMatchObject({key:"process.source.read-bytes-exceeded"});
   }
   const ordinary=join(root,"package.json");await fs.writeFile(ordinary,JSON.stringify({name:"sdk",description:"x".repeat(262144)}));
   await expect(read(ordinary,store,control,budget)).rejects.toMatchObject({key:"process.source.read-bytes-exceeded"});
  });
 }finally{await fs.rm(root,{recursive:true,force:true});}
});
