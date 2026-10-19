/** 🧪️ Receipt: 1 test covering neutral Rust artifact receipt schema limits remain explicit. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{fileURLToPath}from"node:url";
import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{withIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandResultV1}from"@semio-tech/process/execution";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
test("neutral Rust artifact receipt schema limits remain explicit",()=>{expect(new Ajv({strict:true}).validate({type:"array",items:schema.definitions.Case},corpus.cases)).toBe(true);});
for(const row of corpus.cases)registerBuildTestLawV1("default Rust bounded ordinary artifact capture "+row.name,"sdk-fence",original=>withIssuedResourceControlV1(original,"rust:receipt-data-law",0,async control=>{const result=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./📦️child/📜️script.ts",import.meta.url)),row.name],process.cwd(),{control,env:{...scriptInvocationEnvironment(original,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"},output:{maximumBytes:8192,maximumLines:64}});if(!result.stdout.trim())throw Error("Actual Receipt child has no terminal DATA: "+result.stderr);const data=JSON.parse(result.stdout.trim().split("\n").at(-1)??"") as{failed:boolean;message:string;receipt:unknown;oracle:boolean};expect(result.status,row.name+":"+result.stderr).toBe(row.accepted?0:1);expect(data.failed,row.name+":"+data.message).toBe(!row.accepted);if(row.message)expect(data.message).toBe(row.message);if(row.accepted){expect(new Ajv({strict:true}).validate(schema,data.receipt)).toBe(true);expect(data.oracle).toBe(true);}expect(original.control.signal.aborted).toBe(false);}));
