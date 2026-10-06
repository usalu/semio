import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import * as api from "../../../🟦️.ts";
import * as transport from "../🚪️io/🔣️json/🟦️.ts";
import {binary64Value} from "../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";

import schema from "../🔣️.json";
const plain=(value:unknown):unknown=>{if(value&&typeof value==="object"&&!Array.isArray(value)&&typeof (value as {bits?:unknown}).bits==="bigint")return binary64Value(value as {bits:bigint});if(Array.isArray(value))return value.map(plain);if(value&&typeof value==="object")return Object.fromEntries(Object.entries(value).map(([key,value])=>[key,plain(value)]));return value;};
test("plugin-owned Block records and explicit exports agree with schema and SQLite JSON",()=>{
const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["runtimeExports"]).toEqual(["parseBlockAttribute","parseBlockAuthor","parseBlockCamera2d","parseBlockCamera3d","parseBlockCompatibilityRule","parseBlockKindIdentity","parseBlockMeta","parseBlockRepresentation"]);expect(Object.keys(api).sort()).toEqual(fixture.runtimeExports);const parsers:Record<string,(v:unknown)=>unknown>={BlockKindIdentity:transport.kind,BlockAttribute:transport.attribute,BlockAuthor:transport.author,BlockCompatibilityRule:transport.compatible,BlockRepresentation:transport.representation,BlockCamera2d:transport.camera2d,BlockCamera3d:transport.camera3d,BlockMeta:api.parseBlockMeta},database=new Database(":memory:");try{for(const row of fixture.cases){const input=JSON.stringify(row.input),owned=plain(parsers[row.type]!(row.input)),oracle=JSON.parse((database.query("SELECT json(?1) AS body").get(input) as {body:string}).body);expect(owned,row.type).toEqual(oracle);const count=(database.query("SELECT COUNT(*) AS count FROM json_each(?1)").get(input) as {count:number}).count;expect(Object.keys(owned as object).length,row.type).toBe(count);console.log("[DEBUG] Block plugin shared schema "+JSON.stringify({type:row.type,fields:count}));}}finally{database.close();}});
