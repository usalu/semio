import{test,expect}from"bun:test";
import{mkdtemp,mkdir,rm}from"node:fs/promises";
import{join,isAbsolute}from"node:path";
import Ajv from"ajv";
import schema from"../../🧬️schema/🎟️writer.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import{acquireResourceLease,requireResourceLeaseCustodyV1,resourceLeasePathV1}from"../../🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";

test("actual shared lease public promotion cannot prove exclusive writer custody",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);
 await receiveScriptProcessInvocation(process.env,async original=>{
  const capabilities=original.capabilities as{command?:unknown;artifacts?:unknown};if(capabilities.command!=="writer"||typeof capabilities.artifacts!=="string"||!isAbsolute(capabilities.artifacts))throw Error("Authored finite writer fixture output required");
  const control=createIssuedResourceControlV1(original,"lease:private-writer-proof",120000),python=Bun.which("python3")??Bun.which("python");if(!python)throw Error("Independent system SQLite oracle required");await mkdir(capabilities.artifacts,{recursive:true});const directory=await mkdtemp(join(capabilities.artifacts,"lease-writer-"));
  const probe="import sqlite3,sys,json\nresult={}\nfor kind,statement in [('reader','BEGIN'),('writer','BEGIN EXCLUSIVE')]:\n c=sqlite3.connect('file:'+sys.argv[1]+'?mode=rw',uri=True,timeout=0)\n try:\n  c.execute(statement);c.execute('SELECT resource FROM semio_resource_lease').fetchall();result[kind]=True\n except sqlite3.OperationalError as e:\n  if str(e)!='database is locked':raise\n  result[kind]=False\n finally:c.close()\nprint(json.dumps(result))";
  try{for(const row of fixture.cases){if(row.mode!=="shared"&&row.mode!=="exclusive")throw Error("Closed fixture transaction mode required");const resource="writer-proof:"+row.name,lease=await acquireResourceLease({directory,resource,mode:row.mode,signal:control.signal,control,maximumDatabaseBytes:fixture.maximumDatabaseBytes});try{if(row.mutate)expect(Reflect.set(lease,"mode",row.publicMode)).toBe(true);const physical=JSON.parse(await captureOwnedCommandV1(python,["-c",probe,resourceLeasePathV1(directory,resource)],directory,{control,env:process.env,output:{maximumBytes:4096,maximumLines:32}}));expect(physical).toEqual({reader:row.reader,writer:false});let accepted=true;try{requireResourceLeaseCustodyV1(lease,directory,resource,control);}catch{accepted=false;}console.log("[DEBUG] Actual issued lease mode and independent SQLite transaction "+JSON.stringify({name:row.name,issuedMode:row.mode,publicMode:lease.mode,physical,accepted,expected:row.accepted}));expect(accepted).toBe(row.accepted);}finally{lease.release();}}}finally{await rm(directory,{recursive:true});}
 });
},120000);
