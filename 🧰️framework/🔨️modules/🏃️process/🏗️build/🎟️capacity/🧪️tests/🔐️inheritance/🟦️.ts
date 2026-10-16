import {expect,test} from "bun:test";
import Ajv from "ajv";
import {availableParallelism} from "node:os";
import {existsSync,mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../../🟦️.ts";
import {resourceLeasePathV1} from "../../../../🔒️leases/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/🟦️.ts";
import capacitySchema from "../../🧬️schema/🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("Unpaid inherited markers cannot enter while the actual parent owns every CPU",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:unpaid-capacity-parent",30000),output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
 mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"f-")),limits={maximumBytes:65536,maximumLines:128},oracle=new Ajv({strict:true}),validate=oracle.compile(schema.definitions.ChildResult);
 for(const value of fixture.invalidResults){expect(validate(value)).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.ChildResult,value).length).toBeGreaterThan(0);}
 const expected=fixture.expected;expect(oracle.compile(schema.definitions.Observation)(expected)).toBe(true);expect(validateJsonSchemaSubset(schema.definitions.Observation,expected)).toEqual([]);
 const parent=await acquireCompilerCapacityLeaseV1({directory,control,environment:{}}),gate=resourceLeasePathV1(directory,capacitySchema.properties.resource.const),python=process.platform==="win32"?"python":"python3";
 const probe=async()=>{const result=await captureOwnedCommandResultV1(python,["-c","import sqlite3,sys\nd=sqlite3.connect(sys.argv[1],timeout=0)\ntry:\n d.execute('begin');d.execute('select resource from semio_resource_lease').fetchone();d.rollback()\n try:\n  d.execute('begin exclusive');print('free');d.rollback()\n except sqlite3.OperationalError as error:\n  if 'locked' not in str(error):raise\n  print('shared-busy')\nexcept sqlite3.OperationalError as error:\n if 'locked' not in str(error):raise\n print('exclusive-busy')\nfinally:d.close()",gate],process.cwd(),{control,output:limits});expect(result.status,result.stderr).toBe(0);expect(result.signal).toBeNull();return result.stdout.trim()==="exclusive-busy";};
 try{
  expect(parent.parallelism).toBe(availableParallelism());expect(parent.mode).toBe("exclusive");const parentExclusiveBefore=await probe();
  const result=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),directory],process.cwd(),{control,output:limits,env:scriptInvocationEnvironment(original,{...process.env,...fixture.environment})});
  expect(result.status,result.stderr).toBe(0);expect(result.signal).toBeNull();const child=JSON.parse(result.stdout);expect(validate(child),JSON.stringify(validate.errors)).toBe(true);expect(validateJsonSchemaSubset(schema.definitions.ChildResult,child)).toEqual([]);
  const observation={version:1,parentExclusiveBefore,parentExclusiveAfter:await probe(),childEntered:child.entered,childCoreDatabases:Array.from({length:availableParallelism()},(_,index)=>resourceLeasePathV1(directory,capacitySchema.definitions.Protocol.const.coreResourcePrefix+index)).filter(existsSync).length};
  console.log("[DEBUG] Unpaid compiler capacity reservation observation "+JSON.stringify(observation));expect(observation).toEqual(expected);
 }finally{parent.release();try{expect(await probe()).toBe(false);}finally{rmSync(directory,{recursive:true,force:true});}}
}),30000);
