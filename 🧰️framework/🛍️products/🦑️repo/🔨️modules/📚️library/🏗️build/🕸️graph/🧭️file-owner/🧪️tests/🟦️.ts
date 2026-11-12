/** 🧭️ Contract: the withheld root lookup answers the length of the longest withheld root containing a file exactly as the original scan of every withheld root does, and its cost is the depth of the file, not the number of withheld roots; the indexed owner lookup returns exactly what the original sorted scan returns for seeded random workspaces (reference model generated in Python), including nested roots, the root project and equal-length roots. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {createProjectFileIndexV1,createWithheldRootIndexV1} from "../🟨️.mjs";

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

for(const row of fixture.withheldWorlds)test("the withheld root lookup agrees with the scan of every withheld root: "+row.name,()=>{
 const deepest=createWithheldRootIndexV1(row.roots);
 expect(row.files.map(file=>deepest(file))).toEqual(row.deepest);
 const scan=(file:string)=>Math.max(-1,...row.roots.filter(root=>file.startsWith(root+"/")).map(root=>root.length));
 expect(row.files.map(scan)).toEqual(row.deepest);
});
test("a withheld subtree is detected below its owner and never above it",()=>{
 const deepest=createWithheldRootIndexV1(["a/b","c"]);
 expect(deepest("a/b/x.ts")).toBe(3);expect(deepest("a/x.ts")).toBe(-1);expect(deepest("c/d/e.ts")).toBe(1);expect(deepest("a/bb/x.ts")).toBe(-1);expect(deepest("z.ts")).toBe(-1);
 expect(createWithheldRootIndexV1([])("a/b/x.ts")).toBe(-1);
});
test("thousands of withheld roots cost one walk per file",()=>{
 const roots=Array.from({length:5000},(_,at)=>"w"+at+"/inner"),deepest=createWithheldRootIndexV1(roots),files=Array.from({length:20000},(_,at)=>"w"+(at%6000)+"/inner/deep/file"+at+".ts");
 const answers=files.map(file=>deepest(file));
 for(const at of [0,1,4999,5000,5999,12345,19999]){const head=files[at]!.split("/")[0]!;expect(answers[at]).toBe(Number(head.slice(1))<5000?(head+"/inner").length:-1);}
});
