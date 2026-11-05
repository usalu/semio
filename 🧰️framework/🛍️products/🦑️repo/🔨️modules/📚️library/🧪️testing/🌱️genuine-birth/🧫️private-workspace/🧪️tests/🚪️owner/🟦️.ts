import assert from "node:assert/strict";
import {existsSync} from "node:fs";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {type ScriptInvocation,readIssuedScriptProcessEnvelopeV1} from "@semio-tech/process/routing";
import {type ResourceLeaseControl} from "@semio-tech/process/leases";
import {readNativeCallerAuthorityV1} from "../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {runPrivateNativeTestProcessV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🧪️ Observes a real private Native child, unchanged parent limits and actual store retirement. */
export async function testPrivateNativeBirthV1(codeRoot:string,original:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 const before=readIssuedScriptProcessEnvelopeV1(original),parent=readNativeCallerAuthorityV1(original.capabilities),result=await runPrivateNativeTestProcessV1({original,control:phaseControl,codeRoot,purpose:"native-birth-witness"}),prefix=schema.definitions.Protocol.const.receiptPrefix,lines=result.stdout.split("\n").filter(line=>line.startsWith(prefix));assert.equal(lines.length,1,result.stderr);
 const receipt=JSON.parse(lines[0]!.slice(prefix.length)) as {codeRoot:string;workspace:string;parentProcessId:number;processId:number;deadlineEpochMilliseconds:number;transport:{maximumBytes:number;maximumLines:number};child:{maximumElapsedMilliseconds:number};network:{offline:boolean};retired:boolean};
 assert.deepEqual(validateJsonSchemaSubset(schema.definitions.Receipt,receipt,schema),[]);assert.equal(new Ajv({strict:true}).validate({...schema.definitions.Receipt,definitions:schema.definitions},receipt),true);
 assert.equal(receipt.codeRoot,codeRoot);assert.notEqual(receipt.workspace,codeRoot);assert.equal(receipt.parentProcessId,process.pid);assert.notEqual(receipt.processId,process.pid);assert.equal(receipt.retired,true);assert.equal(existsSync(receipt.workspace),false);assert.equal(receipt.network.offline,parent.network.offline);assert.ok(receipt.transport.maximumBytes<=parent.transport.maximumBytes&&receipt.transport.maximumLines<=parent.transport.maximumLines&&receipt.child.maximumElapsedMilliseconds<=parent.child.maximumElapsedMilliseconds);if(before.deadlineEpochMilliseconds!==null)assert.ok(receipt.deadlineEpochMilliseconds<=before.deadlineEpochMilliseconds);assert.deepEqual(readIssuedScriptProcessEnvelopeV1(original),before);
}
