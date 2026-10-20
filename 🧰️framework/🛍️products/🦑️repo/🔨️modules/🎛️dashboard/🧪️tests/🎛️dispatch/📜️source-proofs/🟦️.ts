import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

test("dashboard static feature proofs match actual behavioral test names and independent SQLite",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const root=join(process.cwd(),fixture.sourceRoot),source=readModuleTreeSource(join(root,fixture.testRoot)),db=new Database(":memory:");
 try{for(const feature of fixture.features){const text=readFileSync(join(root,feature.path),"utf8"),scenarios=[...text.matchAll(/^\s*Scenario:\s*(.+?)\s*$/gm)].map(match=>match[1]);expect(scenarios).toEqual(feature.cases.map(row=>row.scenario));
 const names=(db.query("SELECT json_extract(value,'$.scenario') scenario FROM json_each(?1)").all(JSON.stringify(feature.cases))as{scenario:string}[]).map(row=>row.scenario);expect(names).toEqual(scenarios);
 for(const row of feature.cases)for(const name of row.tests){expect(source,new Error(name).message).toMatch(new RegExp("#\\[test\\]\\s*fn\\s+"+name+"\\s*\\("));expect((db.query("SELECT instr(?1,?2)>0 present").get(source,"fn "+name+"(")as{present:number}).present).toBe(1);}
 }}finally{db.close();}
});

test("dashboard source silence checks use the shared forbidden tokens without executing a terminal",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");try{for(const file of fixture.silentSources){const source=readFileSync(join(process.cwd(),fixture.sourceRoot,file),"utf8");for(const token of fixture.forbidden){expect(source,file+":"+token).not.toContain(token);expect((db.query("SELECT instr(?1,?2) found").get(source,token)as{found:number}).found).toBe(0);}}}finally{db.close();}
});
