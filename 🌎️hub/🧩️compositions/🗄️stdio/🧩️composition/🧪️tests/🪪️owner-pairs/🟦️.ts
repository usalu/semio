import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import {dirname,resolve} from "node:path";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";
import {admitCompositionContributionV1} from "../../📋️contract/🟦️.ts";
const root=resolve(import.meta.dir,"../../../../../..");
const hub=resolve(root,"🌎️hub/🧩️compositions/🗄️stdio");
const read=(path:string)=>Bun.file(path).text();
test("exact role pairs satisfy the neutral schema and independent SQLite app identity",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of fixture.rows)for(const role of row.roles){const identity=db.query("SELECT ?1 || char(64) || ?2 || char(47) || ?3 || char(35) || ?4 AS app").get(row.kind,row.standard,row.subset,role.role)as{app:string};expect(identity.app).toBe(role.app);}}finally{db.close();}
});
for(const row of fixture.rows){
 test(row.artifact+" has real typed runtime editor/viewer registrations and physical Cargo entrypoints",async()=>{
  const contribution=admitCompositionContributionV1(JSON.parse(await read(resolve(root,row.contribution))));
  const owners=JSON.parse(await read(resolve(hub,"🧩️composition/🔣️.json"))).owners;
  const owner=owners.find((owner:any)=>owner.id===row.owner);expect(owner).toBeDefined();expect(owner.enum).toBe(row.runtimeEnum);
  const runtime=await read(resolve(hub,owner.root));
  const directory=dirname(resolve(hub,owner.root));
  const cargoPath=resolve(directory,"📦️packages/🦀️rust/Cargo.toml");
  const cargo=toml.parse(await read(cargoPath))as any;
  expect(cargo.package.name).toBe(row.library);expect(resolve(dirname(cargoPath),cargo.lib.path)).toBe(resolve(hub,owner.root));
  for(const role of row.roles){
   const declared=contribution.apps.filter(app=>app.owner===row.owner&&app.variant===role.variant);expect(declared.length).toBe(1);
   expect(declared[0]!.role).toBe(role.role);expect(declared[0]!.type).toBe(role.type);expect(declared[0]!.factory).toBe(role.factory);
   const wrapper=role.role==="editor"?"EditorApp":"ViewerApp";
   expect(runtime).toContain(role.variant+"(VcsArtifactApp<"+wrapper+"<"+role.type+">>)");
   expect(runtime).toContain("builder."+role.role+"::<"+role.type+">("+role.factory+"())");
   expect(cargo.dependencies[role.type.split("::")[0]!.replaceAll("_","-")]).toBeDefined();
   const source=await read(resolve(root,role.source));expect(source).toContain("impl Artifact"+(role.role==="editor"?"Editor":"Viewer")+" for "+role.type.split("::").at(-1));
   expect(source).toContain(JSON.stringify(row.kind));expect(source).toContain("StandardId("+JSON.stringify(row.standard)+")");
  }
  const componentPath=resolve(directory,"🧩️component/📦️packages/🦀️rust/Cargo.toml");
  const component=toml.parse(await read(componentPath))as any;
  expect(component.package.metadata.component.package).toBe("semio:"+row.owner);
  const entry=await read(resolve(dirname(componentPath),component.lib.path));
  expect(entry).toContain(row.library.replaceAll("-","_")+"::plugin, "+row.library.replaceAll("-","_")+"::"+row.runtimeEnum);
  expect(component.dependencies[row.library]).toBeDefined();
  const sourceRoots=component.package.metadata.semio.sources.artifacts.map((path:string)=>resolve(dirname(componentPath),path));
  expect(sourceRoots).toContain(resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"));
 });
 test(row.artifact+" component source metadata preserves its declared exact playground",async()=>{
  const contribution=admitCompositionContributionV1(JSON.parse(await read(resolve(root,row.contribution))));
  expect(contribution.playgrounds).toContainEqual(row.playground);
  const component=toml.parse(await read(resolve(hub,"🧩️extensions",row.family,"🧩️component/📦️packages/🦀️rust/Cargo.toml")))as any;
  const {owner,...playground}=row.playground;
  expect(component.package.metadata.semio.playground.filter((entry:any)=>entry.app===playground.app)).toEqual([playground]);
 });
}
