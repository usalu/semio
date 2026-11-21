import {test,expect} from "bun:test";
import {build} from "esbuild";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {join} from "node:path";
import {mkdir,mkdtemp,writeFile,rm} from "node:fs/promises";
import contract from "../🔣️contract.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import snapshotPolicy from "../../📦️snapshot/🔣️policy.json";
import {nxAuthorizerReceiptHoldsV1} from "../../📤️receipt/🟦️.ts";
import {preflightNxAuthorizersV1} from "../🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

test("actual Node preflight rejects unbundled imports and absent runtime hooks before activation",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(contract)).toBe(true);const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifacts required");const database=new Database(":memory:");database.exec("CREATE TABLE outcomes(id TEXT PRIMARY KEY,accepted INTEGER)");for(const row of fixture.cases)database.query("INSERT INTO outcomes VALUES(?1,?2)").run(row.id,Number(row.accepted));
 try{await withBuildTestInvocationV1(async original=>{for(const row of fixture.cases){const directory=join(artifacts,"preflight",row.id),source=row.source==="valid"?contract.functions.map(name=>`export function ${name}(){}`).join("\n"):row.source==="number"?contract.functions.map(name=>`export const ${name}=1;`).join("\n"):row.source==="relative"?'import "./absent-original.mjs";export const value=1;':"export const value=1;",result=await build({stdin:{contents:source,resolveDir:directory,sourcefile:"📜️script.ts"},bundle:false,format:"esm",platform:"node",write:false,logLevel:"silent"});await mkdir(directory,{recursive:true});await writeFile(join(directory,"semio-authorizers.mjs"),result.outputFiles[0]!.contents);let accepted=false;try{await preflightNxAuthorizersV1(directory,scriptCompilerAdmissionControlV1(original,"nx:preflight:"+row.id,10000),scriptInvocationEnvironment(original,process.env));accepted=true;}catch{}const oracle=database.query("SELECT accepted FROM outcomes WHERE id=?1").get(row.id) as {accepted:number};expect(accepted,row.id).toBe(Boolean(oracle.accepted));await rm(directory,{recursive:true,force:true});}});}finally{database.close();}
});

test("authorizer receipt and Node preflight admit the compiler output ceiling",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifacts required");
 const validate=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["bytes","accepted"],properties:{bytes:{type:"integer",minimum:1},accepted:{type:"boolean"}}});
 const source=contract.functions.map(name=>`export function ${name}(){}`).join("\n"),compiled=await build({stdin:{contents:source},bundle:false,format:"esm",platform:"node",write:false,logLevel:"silent"});
 const entry="semio-authorizers.mjs",manifest=JSON.stringify({version:1,mode:"release",entry:"entry.ts",sources:[{path:"entry.ts",sha256:"a".repeat(64)}],resolutions:[]})+"\n",sha256=async(bytes:Uint8Array)=>Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex");
 await withBuildTestInvocationV1(async original=>{
  for(const row of fixture.bundleSizes){
   expect(validate(row)).toBe(true);const directory=await mkdtemp(join(artifacts,"authorizer-"));
   try{
    const body=Buffer.alloc(row.bytes,0x20);body.set(compiled.outputFiles[0]!.contents);
    const accepted=compiled.outputFiles.length===1&&body.byteLength<=snapshotPolicy.maximumOutputBytes;
    expect(accepted).toBe(row.accepted);
    const receipt={version:1,mode:"release",entry,sourceSha256:await sha256(Buffer.from(manifest)),outputs:[{path:entry,sha256:await sha256(body)}]};
    await writeFile(join(directory,".semio-authorizer-source.json"),manifest);await writeFile(join(directory,entry),body);
    expect(nxAuthorizerReceiptHoldsV1(directory,receipt),String(row.bytes)).toBe(accepted);
    const preflight=preflightNxAuthorizersV1(directory,scriptCompilerAdmissionControlV1(original,"nx:preflight:bundle-size",10000),scriptInvocationEnvironment(original,process.env));
    if(accepted)await preflight;else await expect(preflight).rejects.toThrow("Original bounded authorizer module required");
   }finally{await rm(directory,{recursive:true,force:true});}
  }
 });
});
