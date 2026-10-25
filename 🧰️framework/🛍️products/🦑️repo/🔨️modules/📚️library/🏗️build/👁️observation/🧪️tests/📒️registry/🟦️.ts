import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

/** 🧪️ A workspace whose committed owner registry lists the owners of one fixture registry. */
function workspace(registry:keyof typeof fixture.owners|null){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"registry-"));roots.push(root);
 if(registry)writeFileSync(join(root,"📋️build-owners.json"),JSON.stringify({version:1,owners:fixture.owners[registry],catalogProviders:[]}));
 return root;
}
const rowOf=(root:string,name:string,from:keyof typeof fixture.owners)=>{
 const owner=fixture.owners[from].find(row=>row.name===name);
 return{kind:"registry",path:JSON.stringify([root,name]),fact:owner?JSON.stringify({root:owner.root,manifest:owner.manifest}):"null"};
};

test("the fixture is closed under its schema",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
});

test("the independent Python oracle agrees on every step",()=>{
 const result=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});
});

test("a recorded registry row holds exactly while that one owner row is unchanged, whatever else the registry lists",async()=>{
 for(const step of fixture.steps){
  const root=workspace(step.registry as keyof typeof fixture.owners|null);
  for(const entry of fixture.recorded){
   const row=rowOf(root,entry.name,entry.from as keyof typeof fixture.owners);
   expect(await api.observationsHold([row],1,new Set(),new Set()),step.label+" / "+entry.name).toBe(step.holds[entry.name as keyof typeof step.holds]);
  }
 }
});

test("an inference records the row it read, in the same spelling the check recomputes",async()=>{
 const root=workspace("base"),captured=await api.captureObservedSourceInputsV1(async()=>{api.recordRegistryOwnerV1(root,"a",fixture.owners.base[0]);api.recordRegistryOwnerV1(root,"n",undefined);return 1;},undefined,false);
 const rows=[...captured.observations].filter(row=>row.kind==="registry");
 expect(rows.map(row=>row.fact).sort()).toEqual(["null",JSON.stringify({root:"pkg/a",manifest:"pkg/a/project.json"})].sort());
 expect(await api.observationsHold(rows,1,new Set(),new Set())).toBe(true);
});
