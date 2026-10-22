import "./🎟️phase/🟦️.ts";
import {test,expect} from "bun:test";
import {join,resolve} from "node:path";
import {mkdtemp,mkdir,rm,readFile} from "node:fs/promises";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import schema from "./📋️schema.json";
import {watchBuildChildArgumentsV1} from "../🟦️.ts";
import {validateBuildTaskArgumentsV1} from "../../🟦️.ts";
import {createBuildBootstrapCapabilitiesV1} from "../../../📥️caller/📤️child/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../📥️caller/🟦️.ts";
import {registerBuildTestLawV1} from "@semio-tech/process/testing/invocation";
import {readIssuedScriptProcessEnvelopeV1,scriptProcessEnvironment,createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandResultV1} from "@semio-tech/process/execution";

const root=process.cwd();
test("portable watch rejects missing callbacks before graph admission",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");try{
  for(const row of corpus.watchGrammar){const result=db.query("select case when boundary is null then 0 else coalesce(length(json_extract(?1,'$['||(boundary+1)||']'))>0,0) end valid from (select min(cast(key as integer)) boundary from json_each(?1) where value='--')").get(JSON.stringify(row.args)) as {valid:number};expect(Boolean(result.valid)).toBe(row.valid);const validate=()=>validateBuildTaskArgumentsV1(row.args);if(row.valid)expect(validate).not.toThrow();else expect(validate).toThrow("Expected repo watch --projects=<projects> -- <command>");}
 }finally{db.close();}
});
test("portable watch declares exact literal and convenience descendants before birth",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");try{
 for(const tool of ['nx','bazel'])for(const row of corpus.cases){
  const expected=(row.name==='direct'?db.query("select json_group_array(value) value from json_each(?1) where cast(key as integer)>(select cast(key as integer)+2 from json_each(?1) where value='--')").get(JSON.stringify(row.parent)):db.query("select json_array('run',replace(json_extract(?1,'$[1]'),':watch',':build'),'--output=stream') value").get(JSON.stringify(row.parent))) as {value:string};expect(JSON.parse(expected.value)).toEqual(row.child);
  const grant=await createBuildBootstrapCapabilitiesV1(root,row.parent,{...process.env,SEMIO_BUILD_TOOL:tool,SEMIO_SCRIPT_CAPABILITIES:undefined});expect(grant.launches).toContainEqual(['repo',...row.child]);for(const child of corpus.refused)expect(grant.launches).not.toContainEqual(['repo',...child]);
 }
 expect(watchBuildChildArgumentsV1(root,['bun','repo',...corpus.cases[0]!.child])).toEqual(corpus.cases[0]!.child);expect(watchBuildChildArgumentsV1(root,['node','-e','repo run foreign:build'])).toBeUndefined();expect(watchBuildChildArgumentsV1(root,['/foreign/bun','repo','run','foreign:build'])).toBeUndefined();
 await expect(createBuildBootstrapCapabilitiesV1(root,['watch','--projects=app','--','bun','repo','--tool=nx','run','app:build'],{...process.env,SEMIO_BUILD_TOOL:'bazel',SEMIO_SCRIPT_CAPABILITIES:undefined})).rejects.toThrow('cannot change');
 }finally{db.close();}
});

registerBuildTestLawV1("actual watch child retains Original bounds, coalesces events and drains cancellation","sdk-runtime",async original=>{
 const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),'watch-authority-')),envelope=readIssuedScriptProcessEnvelopeV1(original),base=createNativeBuildCallerCapabilitiesV1(root,corpus.cases[0]!.parent,'bazel',process.env),native=(envelope.capabilities as any).native??base.native,capabilities=await createBuildBootstrapCapabilitiesV1(root,corpus.cases[0]!.parent,{...process.env,SEMIO_BUILD_TOOL:'bazel',SEMIO_SCRIPT_CAPABILITIES:JSON.stringify({...base,native})}),environment=scriptProcessEnvironment({...envelope,capabilities},{...process.env,SEMIO_BUILD_TOOL:'bazel',SEMIO_SCRIPT_CAPABILITIES:undefined});
 let failed=false;try{
  const childFunding=25000,parts:object[]=[];for(const mode of ['authority','fault-error','fault-undefined','fault-overflow']){const own=join(directory,mode);await mkdir(own);const result=await captureOwnedCommandResultV1(process.execPath,[join(import.meta.dir,'📜️script.ts'),mode,own],root,{control:createIssuedResourceControlV1(original,"watch:actual-"+mode,childFunding),env:environment,output:{maximumBytes:262144,maximumLines:2048}});expect(result.status,mode).toBe(mode==='authority'?1:0);expect(result.signal).toBeNull();parts.push(JSON.parse(await readFile(join(own,'law-'+(mode==='authority'?mode:mode.slice(6))+'.json'),'utf8')));}
  const receipt=Object.assign({},...parts);expect(receipt).toMatchObject({version:1,strictReceiver:true,coalesced:true,childrenDrained:true,parentActive:true,errorDrain:true,undefinedPrimary:true,overflowRefused:true,cancelledChildRejected:true,policy:{...envelope.policy,maximumElapsedMilliseconds:childFunding},deadline:expect.any(Number),native});
 }catch(error){failed=true;throw error;}finally{try{await rm(directory,{recursive:true,force:true});}catch(error){if(!failed)throw error;}}
});
