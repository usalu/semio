/** 🚧️ Contract: an untagged withheld owner is valid only while the bytes of the source that withholds it are among its facts; a generation that carries no fact about that source cannot notice the repair, so the plan infers that owner again. Tagged owners and sources outside the workspace are not judged, a path matches by whole components, and a class of owners that share their rows is judged owner by owner. The independent oracle is a Python model over the same fixture. */
import{expect,test}from"bun:test";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import{createFactIndexV1,unfoundedWithheldOwnersV1}from"../../🟨️.mjs";
import{encodeFactIndexV1,openFactIndexV1}from"../../🧮️fact-codec/🟨️.mjs";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const python=()=>JSON.parse(Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]).stdout.toString()) as string[];
const owners=Object.fromEntries(Object.entries(fixture.owners).map(([name,row])=>[name,{kind:row.kind,config:row.config,...("tag"in row?{tag:row.tag}:{})}]));
const index=()=>createFactIndexV1(new Map(Object.entries(fixture.owners).map(([name,row])=>[name,row.rows as never[]])));

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("withheld owners without a fact about their source are named, as the independent model names them",()=>{
 const named=unfoundedWithheldOwnersV1(index() as never,owners).sort();
 expect(named).toEqual(fixture.unfounded);expect(python()).toEqual(fixture.unfounded);
});

test("a lazily opened fact index answers the same",()=>{
 const view=openFactIndexV1(encodeFactIndexV1(index() as never,{maximumBytes:1<<26}) as never);
 expect(unfoundedWithheldOwnersV1(view as never,owners).sort()).toEqual(fixture.unfounded);
});
