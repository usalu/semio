import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {inspectCommandIngressConsumer,type ConsumerFixture} from "../../../../📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️testing/🧩️consumer/🟦️.ts";
const root=resolve(import.meta.dir,"../../../../../.."),fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")) as ConsumerFixture,schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
test("the general owner retains only its source-owned canonical command ingress consumer authority",async()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);expect(await inspectCommandIngressConsumer(root,fixture)).toEqual({owner:fixture.owner,cases:fixture.cases.length});});
test("the general closed consumer corpus rejects omitted, unknown and extra authority",()=>{const validate=new Ajv({strict:true}).compile(schema);for(const change of [(value:any)=>value.cases.pop(),(value:any)=>value.cases[0].kind="unknown",(value:any)=>value.extra=true]){const hostile=structuredClone(fixture);change(hostile);expect(validate(hostile)).toBe(false);}});
