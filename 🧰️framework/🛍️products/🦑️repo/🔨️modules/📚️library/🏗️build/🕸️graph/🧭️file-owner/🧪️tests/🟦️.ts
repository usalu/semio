/** 🧭️ Contract: the indexed owner lookup returns exactly what the original sorted scan returns for seeded random workspaces (reference model generated in Python), including nested roots, the root project and equal-length roots. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {createProjectFileIndexV1} from "../🟨️.mjs";

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.worlds)test("owners agree with the sorted scan: "+row.name,()=>{
 const index=createProjectFileIndexV1(Object.fromEntries(row.projects.map(project=>[project.name,{root:project.root}])));
 expect(row.files.map(file=>index(file)??null)).toEqual(row.owners);
 const original=(file:string)=>{const ordered=[...row.projects].sort((a,b)=>b.root.length-a.root.length);return ordered.find(project=>project.root==="."||file.startsWith(project.root+"/"))?.name??null;};
 expect(row.files.map(original)).toEqual(row.owners);
});
test("a lookup touches only the ancestors of the file",()=>{
 const index=createProjectFileIndexV1({a:{root:"a"},"a/b":{root:"a/b"}});
 expect(index("a/b/c/d.ts")).toBe("a/b");expect(index("a/x.ts")).toBe("a");expect(index("z.ts")).toBeUndefined();
});
