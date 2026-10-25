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

/** 🧪️ A workspace whose dependency store holds one generation with one installed package. */
function store(generation:string){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"resolution-"));roots.push(root);
 const modules=join(root,".cache","b","d","m","j",generation,"node_modules",fixture.generations.package);mkdirSync(modules,{recursive:true});
 writeFileSync(join(modules,fixture.generations.file),"module.exports=1;\n");writeFileSync(join(modules,"package.json"),JSON.stringify({name:fixture.generations.package,main:fixture.generations.file}));
 const from=join(root,".cache","b","d","m","j",generation,"from.mjs");writeFileSync(from,"");
 return{root,from};
}

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile(fixtureSchema);expect(validate(fixture)).toBe(true);
});

test("a module resolution is recorded without the dependency generation id",()=>{
 for(const row of fixture.cases)expect(api.canonicalResolutionV1(row.input)).toBe(row.expected);
});

test("the independent Python oracle agrees on every case",()=>{
 const result=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});
});

test("a recorded resolution survives a new dependency generation of the same package and is stale when the package vanishes",async()=>{
 const{recorded,current,package:name}=fixture.generations,next=store(current);
 const key=JSON.stringify([next.from,name]);
 const row={kind:"resolve",path:key,fact:join(next.root,".cache","b","d","m","j",recorded,"node_modules",name,fixture.generations.file)};
 expect(await api.observationsHold([row],1,new Set(),new Set())).toBe(true);
 const gone={kind:"resolve",path:JSON.stringify([next.from,"missing-package"]),fact:row.fact};
 expect(await api.observationsHold([gone],1,new Set(),new Set())).toBe(false);
});

test("a resolution recorded now carries the placeholder, never the generation id",()=>{
 const{current,package:name}=fixture.generations,next=store(current);
 const resolved=api.observedRequireResolve(next.from,name);
 expect(resolved).toContain(current);expect(api.canonicalResolutionV1(resolved)).not.toContain(current);
});
