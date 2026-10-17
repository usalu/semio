import{types}from"node:util";
import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema.json" with {type:"json"};
/** 📋️ Narrows parsed fixture DATA by its observed object shape. */
export function isTestRecordV1(value:unknown):value is Record<string,unknown>{return value!==null&&typeof value==="object"&&!Array.isArray(value);}
/** 🔎️ Requires each actual parsed DATA ancestor before its fields are read. */
export function testRecordV1(value:unknown,...path:readonly string[]):Record<string,unknown>{for(const key of path){if(!isTestRecordV1(value))throw Error("Fixture object required");value=value[key];}if(!isTestRecordV1(value))throw Error("Fixture object required");return value;}
function strings(value:unknown):value is string[]{return Array.isArray(value)&&value.every(item=>typeof item==="string");}
/** 🧷️ Requires the actual selected fixture entry before its fields are consumed. */
export function testValueV1<T>(value:T|undefined):T{if(value===undefined)throw Error("Actual fixture value required");return value;}
/** 🔤️ Keeps string-array fixture outputs typed without coercion, casts or dropped entries. */
export function testStringsV1(value:unknown):readonly string[]{if(!strings(value))throw Error("Fixture string array required");return value;}
/** 🧪️ Parsed DATA predicates agree with a separate JSON/SQLite type oracle. */
export function testFixtureValuesV1():void{assert(new Ajv({strict:true}).validate(schema,corpus));const database=new Database(":memory:");try{for(const row of corpus.records){const oracle=database.query("select json_type(?)='object' admitted").get(JSON.stringify(row.value));assert.deepEqual(oracle,{admitted:Number(row.accepted)});if(row.accepted)assert.equal(testRecordV1(row.value),row.value);else assert.throws(()=>testRecordV1(row.value));}for(const row of corpus.strings){const oracle=database.query("select json_type(?)='array' and not exists(select 1 from json_each(?) where type!='text') admitted").get(JSON.stringify(row.value),JSON.stringify(row.value));assert.deepEqual(oracle,{admitted:Number(row.accepted)});if(row.accepted)assert.equal(testStringsV1(row.value),row.value);else assert.throws(()=>testStringsV1(row.value));}}finally{database.close();}}
/** 🔎️ Calls an actual own JavaScript method while keeping its dynamic result untrusted. */
export function testMethodV1(owner:unknown,name:string,...args:readonly unknown[]):unknown{if(!isTestRecordV1(owner)||types.isProxy(owner))throw Error("Actual fixture method owner required");const field=Object.getOwnPropertyDescriptor(owner,name);if(!field||!("value"in field)||typeof field.value!=="function")throw Error("Actual own fixture method required");return Reflect.apply(field.value,owner,args);}
/** 📋️ Requires each actual Nx input DATA entry without coercing or dropping unknown values. */
function inputsV1(value:unknown):value is(string|Record<string,unknown>)[]{if(!Array.isArray(value)||types.isProxy(value))return false;for(let index=0;index<value.length;index++){const row=Object.getOwnPropertyDescriptor(value,String(index));if(!row||!("value"in row)||typeof row.value!=="string"&&!isTestRecordV1(row.value))return false;}return true;}
export function testInputsV1(value:unknown):readonly(string|Record<string,unknown>)[]{if(!inputsV1(value))throw Error("Fixture input array required");return value;}
