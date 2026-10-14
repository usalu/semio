import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {resolve,sep} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import {javascriptDependencyBindingsV1} from "../🟦️.ts";

test("closed dependency scopes agree with independent SQLite identities and schema",()=>{const validate=new Ajv({strict:true}).compile(schema),oracle=new Database(":memory:");try{for(const row of corpus.cases){if(row.accepted){expect(validate({version:1,scope:row.scope,members:row.members,generation:"a".repeat(64)})).toBe(true);const actual=javascriptDependencyBindingsV1("/r","/d",row.scope,row.members,"a".repeat(64));expect(actual.length).toBe(row.count!);for(const [index,binding]of actual.entries()){const identity="a".repeat(20),expected=oracle.query("SELECT ?1 || ?3 || 'j' || ?3 || ?2 || ?3 || ?4 AS path").get(resolve("/d"),identity,sep,index===0?"node_modules":"w"+sep+(index-1)+sep+"node_modules") as {path:string};expect(binding.targetDirectory).toBe(expected.path);}}else expect(()=>javascriptDependencyBindingsV1("/r","/d",row.scope,row.members,"a".repeat(64))).toThrow();}}finally{oracle.close();}});

