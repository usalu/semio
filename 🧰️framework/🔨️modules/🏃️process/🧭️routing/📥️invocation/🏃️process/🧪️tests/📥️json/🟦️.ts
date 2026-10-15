import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/📥️json.json";
import limits from "../../🧬️schema/📥️json.json";
import {readScriptProcessEnvelope} from "../../🟦️.ts";
import {readScriptProcessJsonWireV1} from "../../📥️json/🟦️.ts";

/** 🧪️ Compares bounded capability admission with neutral JSON and independent schema/serialization observations. */
export function proveScriptProcessJsonAdmissionV1():void{
 const policy=Object.fromEntries(Object.entries(limits.properties).map(([name,row])=>[name,row.const])) as Record<string,number>,result={...fixture.observations},violations:string[]=[],oracle=new Ajv({strict:true}).compile({type:"object"}),admit=(value:unknown):object|undefined=>{try{return readScriptProcessEnvelope({version:1,policy:{version:1,owner:"bounded-json-proof",maximumElapsedMilliseconds:1000},deadlineEpochMilliseconds:2000,capabilities:value},1250).capabilities;}catch{return undefined;}};
 let accessorCalls=0,toJsonCalls=0,proxyCalls=0;
 for(const row of fixture.json){const admitted=admit(row.value),accepted=admitted!==undefined;assert.equal(oracle(row.value),row.accepted);if(accepted!==row.accepted)violations.push(row.id);if(admitted)assert.deepEqual(JSON.parse(JSON.stringify(admitted)),JSON.parse(JSON.stringify(row.value)));}
 const constructed=(id:string):unknown=>{
  switch(id){
   case"cycle":{const value:{self?:unknown}={};value.self=value;return value;}
   case"object-proxy":case"array-proxy":return{value:new Proxy(id==="array-proxy"?[1]:{value:1},{getPrototypeOf(target){proxyCalls++;return Reflect.getPrototypeOf(target);},ownKeys(target){proxyCalls++;return Reflect.ownKeys(target);},getOwnPropertyDescriptor(target,key){proxyCalls++;return Reflect.getOwnPropertyDescriptor(target,key);},get(target,key,receiver){proxyCalls++;return Reflect.get(target,key,receiver);}})};
   case"revoked-proxy":{const value=Proxy.revocable({},{});value.revoke();return{value:value.proxy};}
   case"shared-json-object":{const child={name:"same-data"};return{left:child,right:child};}
   case"accessor":return Object.defineProperty({},"value",{enumerable:true,get(){accessorCalls++;return"forbidden";}});
   case"custom-prototype":return Object.assign(Object.create({inherited:true}),{value:1});
   case"date":return{value:new Date(0)};
   case"typed-array":return{value:new Uint8Array([1])};
   case"symbol-key":return{[Symbol("foreign")]:1};
   case"undefined":return{value:undefined};
   case"function":return{toJSON(){toJsonCalls++;return{};}};
   case"bigint":return{value:1n};
   case"nan":return{value:NaN};
   case"infinity":return{value:Infinity};
   case"sparse-array":return{value:Array(1)};
   case"array-extra-property":return{value:Object.assign([1],{extra:true})};
   case"array-accessor":return{value:Object.defineProperty([0],"0",{enumerable:true,get(){accessorCalls++;return"forbidden";}})};
   case"unpaired-surrogate":return{value:"\ud800"};
   case"depth-overflow":{let value:object={};for(let n=0;n<policy.maximumDepth!+2;n++)value={value};return value;}
   case"node-overflow":return{value:Array.from({length:policy.maximumArrayItems!},()=>({value:1}))};
   case"object-overflow":return Object.fromEntries(Array.from({length:policy.maximumObjectMembers!+1},(_,index)=>["v"+index,0]));
   case"array-overflow":return{value:Array(policy.maximumArrayItems!+1).fill(0)};
   case"key-utf8-overflow":return{["汉".repeat(Math.floor(policy.maximumKeyUtf8Bytes!/3)+1)]:0};
   case"string-utf8-overflow":return{value:"汉".repeat(Math.floor(policy.maximumStringUtf8Bytes!/3)+1)};
   case"aggregate-utf8-overflow":return{value:Array(5).fill("x".repeat(policy.maximumStringUtf8Bytes!))};
   case"escaped-wire-overflow":return{value:"\0".repeat(policy.maximumStringUtf8Bytes!)};
   default:throw Error("Closed constructed JSON specimen required");
  }
 };
 for(const row of fixture.constructed){const value=constructed(row.id),admitted=admit(value);if((admitted!==undefined)!==row.accepted)violations.push(row.id);if(row.id==="shared-json-object"&&admitted){assert.deepEqual(JSON.parse(JSON.stringify(admitted)),JSON.parse(JSON.stringify(value)));result.sharedObjectDoesNotBecomeCycle=true;}}
 const original={nested:{name:"original"},values:[1]},captured=admit(original) as typeof original|undefined;original.nested.name="changed";original.values.push(2);result.admittedGraphIsImmutableSnapshot=!!captured&&captured.nested.name==="original"&&captured.values.length===1&&Object.isFrozen(captured)&&Object.isFrozen(captured.nested)&&Object.isFrozen(captured.values);result.noAccessorExecuted=accessorCalls===0;result.noProxyTrapExecuted=proxyCalls===0;result.noToJsonExecuted=toJsonCalls===0;result.independentJsonSchemaAgreement=true;result.rejectedBeforeRecursiveEnvelopeValidation=admit(constructed("cycle"))===undefined;
 assert.throws(()=>readScriptProcessJsonWireV1(" ".repeat(policy.maximumWireUtf8Bytes!+1)+"{}"),/Bounded original Script process wire/);assert.throws(()=>readScriptProcessJsonWireV1('{"value":"'+"汉".repeat(policy.maximumWireUtf8Bytes!/3)+'"}'),/Bounded original Script process wire/);assert.deepEqual(JSON.parse(JSON.stringify(readScriptProcessJsonWireV1('{"command":"build"}'))),{command:"build"});
 assert.deepEqual(violations,[],"closed capability JSON must refuse hostile or over-budget values before recursive envelope validation");assert.deepEqual(result,fixture.observations);console.log("[DEBUG] Actual canonical bounded capability JSON admission "+JSON.stringify({neutral:fixture.json.length,constructed:fixture.constructed.length,boundedWireCases:3,...result}));
}
