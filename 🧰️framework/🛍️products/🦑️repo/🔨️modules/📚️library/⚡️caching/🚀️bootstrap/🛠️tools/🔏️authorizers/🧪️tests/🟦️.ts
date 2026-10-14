import {expect,test} from "bun:test";
import {mkdtemp,mkdir,writeFile,readFile,rm} from "node:fs/promises";
import {join,dirname} from "node:path";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {build} from "esbuild";
import {fileURLToPath,pathToFileURL} from "node:url";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesHoldV1} from "../🟦️.ts";
import {nxAuthorizerReceiptHoldsV1} from "../📤️receipt/🟦️.ts";

test("immutable authorizer sources agree with independent esbuild closure and Ajv",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"ab-")),signal=new AbortController().signal;
 try{
  for(const[path,bytes]of Object.entries(fixture.files)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const before=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal),validate=new Ajv({strict:true}).compile(schema);
  expect(validate(before.manifest),JSON.stringify(validate.errors)).toBe(true);expect(before.manifest.sources.map(row=>row.path)).toEqual(fixture.expectedFiles);
  const compiled=await build({absWorkingDir:root,entryPoints:["entry.ts"],bundle:true,platform:"node",format:"esm",write:false,metafile:true,logLevel:"silent"});
  expect(Object.keys(compiled.metafile!.inputs).sort()).toEqual(fixture.expectedFiles);expect(before.digest).toBe(createHash("sha256").update(JSON.stringify(before.manifest)+"\n").digest("hex"));
  expect(await nxAuthorizerSourcesHoldV1(root,before,signal)).toBe(true);await writeFile(join(root,fixture.mutation.path),fixture.mutation.bytes);expect(await nxAuthorizerSourcesHoldV1(root,before,signal)).toBe(false);
  const after=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal);expect(after.digest).not.toBe(before.digest);console.log("[DEBUG] immutable authorizer source closure="+before.manifest.sources.length+" editRefused=true");
 }finally{await rm(root,{recursive:true,force:true});}
});

test("release authorizer compiler executes captured bytes in actual Node and refuses stale input",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"ar-"));
 try{for(const[path,bytes]of Object.entries(fixture.files))await writeFile(join(root,path),bytes);
  await receiveScriptProcessInvocation(process.env,async original=>{
   const control=scriptCompilerAdmissionControlV1(original,"authorizers:oracle",120000),before=await captureNxAuthorizerSourcesV1(root,"entry.ts",control.signal),manifest=join(root,"source.json"),output=join(root,"out"),worker=fileURLToPath(new URL("../📜️script.ts",import.meta.url)),env=scriptInvocationEnvironment(original,process.env);
   await writeFile(manifest,JSON.stringify(before.manifest)+"\n");
   await captureOwnedCommandV1(process.execPath,[worker,"compile",root,manifest,output],root,{control,env});
   const bundle=pathToFileURL(join(output,"semio-authorizers.mjs")).href,node=await captureOwnedCommandV1("node",["--input-type=module","--eval","const {value,latin,encoded}=await import("+JSON.stringify(bundle)+");console.log(JSON.stringify({value,latin,encoded}));"],root,{control,env});
   expect(JSON.parse(node.trim())).toEqual({value:fixture.expectedValues.original,latin:fixture.expectedValues.latin,encoded:fixture.expectedValues.encoded});
   await writeFile(join(output,".semio-authorizer-source.json"),JSON.stringify(before.manifest)+"\n");
   const body=await readFile(join(output,"semio-authorizers.mjs")),receipt={version:1,mode:"release",entry:"semio-authorizers.mjs",sourceSha256:before.digest,outputs:[{path:"semio-authorizers.mjs",sha256:createHash("sha256").update(body).digest("hex")}]};
   const ajv=new Ajv({strict:true});ajv.addSchema(schema);const validate=ajv.compile({$ref:schema.$id+"#/$defs/Receipt"});expect(validate(receipt),JSON.stringify(validate.errors)).toBe(true);expect(nxAuthorizerReceiptHoldsV1(output,receipt)).toBe(true);
   for(const row of fixture.receiptMutations){const file=join(output,row.path),originalBytes=await readFile(file);await writeFile(file,row.bytes);expect(nxAuthorizerReceiptHoldsV1(output,receipt),row.id).toBe(row.accept);await writeFile(file,originalBytes);}
   await writeFile(join(root,fixture.mutation.path),fixture.mutation.bytes);
   await expect(captureOwnedCommandV1(process.execPath,[worker,"compile",root,manifest,output],root,{control,env})).rejects.toThrow("Original authorizer source changed");
   console.log("[DEBUG] immutable release authorizer Node value="+fixture.expectedValues.original+" staleCompilerInputRefused=true");
  });
 }finally{await rm(root,{recursive:true,force:true});}
},180000);
