/** 🎛️ Committed conformance-class edits agree with independent RFC6902 and schema readers. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";

const subsets=["1️⃣cc1","2️⃣cc2","3️⃣cc3","4️⃣cc4","5️⃣cc5","6️⃣cc6"];
const root=fileURLToPath(new URL("../../🪆️subsets/",import.meta.url));
for(const [index,subset] of subsets.entries())test(`STEP ${subset} commits every native conformance intent with independent expected output`,()=>{
 const owner=join(root,subset),cases=join(owner,"🧫️fixtures/🎛️history-inputs"),kinds=index===0?["restore-entities","set-file-schema","set-product-identity","remove-shape-representation"]:index===5?["restore-entities","set-file-schema","set-product-identity","set-shape-representation"]:["restore-entities","set-file-schema","set-product-identity","set-shape-representation","demote-shape-representation"];
 const directories=readdirSync(cases).sort();expect(directories.length).toBe(kinds.length);
 const actual=[];
 for(const directory of directories){
  const fixture=JSON.parse(readFileSync(join(cases,directory,"🦠️mutation/🔣️.json"),"utf8"));actual.push(fixture.oracle.kind);
  const variant=fixture.oracle.kind.split("-").map((word:string)=>word[0]!.toUpperCase()+word.slice(1)).join("");expect(fixture.mutation).toEqual({[variant]:fixture.oracle.params});
  const leaf=readdirSync(join(owner,"🧬️schema/🧬️mutations")).find(name=>name.endsWith(fixture.oracle.kind));expect(leaf).toBeDefined();const payload=JSON.parse(readFileSync(join(owner,"🧬️schema/🧬️mutations",leaf!,"🧬️schema/🔣️.json"),"utf8"));const admits=new Ajv({strict:false}).compile(payload);expect(admits(fixture.oracle.params),JSON.stringify(admits.errors)).toBe(true);
  expect(jsonPatch.applyPatch(structuredClone(fixture.before),fixture.oracle.patch,true).newDocument).toEqual(fixture.after);expect(fixture.after).not.toEqual(fixture.before);
  expect(jsonPatch.applyPatch(structuredClone(fixture.after),jsonPatch.compare(fixture.after,fixture.before),true).newDocument).toEqual(fixture.before);
 }
 expect(actual.sort()).toEqual(kinds.sort());console.log(`[DEBUG] STEP ${subset} committed ${actual.length} class-native intent fixtures with independent schema/RFC6902 output`);
});

for(const [index,subset] of subsets.entries())test(`STEP ${subset} native editor and viewer retain the declared class mutation owner`,()=>{
 const owner=join(root,subset),name=`StepCc${index+1}Mutation`;
 for(const leaf of ["✏️editor/🦀️.rs","👁️viewer/🦀️.rs"]){const native=readFileSync(join(owner,leaf),"utf8");expect(native).toContain(`type Mutation = ${name};`);expect(native).not.toContain("type Mutation = StepMutation;");}
 const codec=readFileSync(join(owner,"🚪️io/🧬️mutations/🦀️.rs"),"utf8");expect(codec).toContain(name);expect(readFileSync(join(owner,"🚪️io/🧬️mutations/💾️binary/🦀️.rs"),"utf8")).toContain("VariantTag::Key");
 console.log(`[DEBUG] STEP ${subset} editor/viewer retain ${name} and its direct externally tagged native codec`);
});


for(const [index,subset] of subsets.entries())test(`STEP ${subset} construction retains its independently declared class-native intent`,()=>{
 const owner=join(root,subset),name=`StepCc${index+1}Mutation`,fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🎛️history-inputs/restore-entities/🦠️mutation/🔣️.json"),"utf8"));expect(jsonPatch.applyPatch(structuredClone(fixture.before),fixture.oracle.patch,true).newDocument).toEqual(fixture.after);
 const native=readFileSync(join(owner,"🚪️io/🦀️.rs"),"utf8"),construction=native.slice(native.indexOf("pub mod derived_construction"));expect(construction).toContain(`type Mutation = ${name};`);expect(construction).toContain(`apply_step_cc${index+1}_mutation`);expect(construction).not.toContain("type Mutation = StepMutation;");
 console.log(`[DEBUG] STEP ${subset} construction retains ${name} and the independently authored native RestoreEntities intent`);
});

for(const subset of subsets)test(`STEP ${subset} instance words stay references while human names remain primitive controls`,async()=>{
 const owner=join(root,subset),identity=JSON.parse(readFileSync(join(owner,"🧬️schema/🧬️mutations/🪪set-product-identity/🧬️schema/🔣️.json"),"utf8"));
 for(const field of ["product","formation","definition"]){const descriptor=identity.$defs.ProductIdentity.properties[field];expect(descriptor.type).toBe("integer");expect(descriptor["x-semio-ui"].widget).toBe("reference");expect(descriptor["x-semio-ui"].role).toBe("target");expect(descriptor["x-semio-ui"].ref).toEqual({kind:"entity"});}
 for(const field of ["productName","formationId","definitionId"])expect(identity.$defs.ProductIdentity.properties[field].type).toBe("string");
 const {mutationInputDefs}=await import(join(process.cwd(),"🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"));const identityControl=mutationInputDefs(identity,()=>undefined).find((input:any)=>input.id==="/identity")!;expect(identityControl.schema.kind).toBe("object");for(const field of ["product","formation","definition"]){const input=(identityControl.schema as any).fields.find((input:any)=>input.id===`/${field}`)!;expect(input.schema.kind).toBe("reference");expect(input.schema.idType).toBe("integer");expect(input.schema.many??false).toBe(false);}

 if(subset!=="1️⃣cc1"){const representation=JSON.parse(readFileSync(join(owner,"🧬️schema/🧬️mutations/🪜set-shape-representation/🧬️schema/🔣️.json"),"utf8"));for(const field of ["items","context"]){const descriptor=representation.$defs.ShapeRepresentationRow.properties[field];expect(descriptor["x-semio-ui"].widget).toBe("reference");expect(descriptor["x-semio-ui"].role).toBe("target");expect(descriptor["x-semio-ui"].ref).toEqual({kind:"entity"});}const control=mutationInputDefs(representation,()=>undefined).find((input:any)=>input.id==="/representation")!;expect(control.schema.kind).toBe("object");for(const field of ["items","context"]){const input=(control.schema as any).fields.find((input:any)=>input.id===`/${field}`)!;expect(input.schema.kind).toBe("reference");expect(input.schema.idType).toBe("integer");expect(input.schema.many??false).toBe(field==="items");}}
 const fixtures=join(owner,"🧫️fixtures/🎛️history-inputs");for(const directory of readdirSync(fixtures)){const fixture=JSON.parse(readFileSync(join(fixtures,directory,"🦠️mutation/🔣️.json"),"utf8"));expect(jsonPatch.applyPatch(structuredClone(fixture.before),fixture.oracle.patch,true).newDocument).toEqual(fixture.after);}
 console.log(`[DEBUG] STEP ${subset} owned instance identities/edges expose reference controls and unchanged independent intent semantics`);
});

test("STEP snapshot instance and reference words remain addresses with independently valid unchanged fixtures",()=>{
 const snapshot=JSON.parse(readFileSync(join(root,"🧱️base/🧬️schema/📸️snapshot/🔣️.json"),"utf8")),admits=new Ajv({strict:false}).compile(snapshot);
 for(const subset of subsets)for(const directory of readdirSync(join(root,subset,"🧫️fixtures/🎛️history-inputs"))){const fixture=JSON.parse(readFileSync(join(root,subset,"🧫️fixtures/🎛️history-inputs",directory,"🦠️mutation/🔣️.json"),"utf8"));expect(admits(fixture.before),JSON.stringify(admits.errors)).toBe(true);expect(admits(fixture.after),JSON.stringify(admits.errors)).toBe(true);}
 const reference=snapshot.$defs.StepValue.oneOf.find((variant:any)=>variant.properties?.reference)?.properties.reference;
 for(const descriptor of [snapshot.$defs.StepEntity.properties.id,reference]){expect(descriptor.type).toBe("integer");expect(descriptor["x-semio-ui"].widget).toBe("reference");expect(descriptor["x-semio-ui"].role).toBe("target");expect(descriptor["x-semio-ui"].ref).toEqual({kind:"entity"});}
 console.log("[DEBUG] STEP snapshot native identity/reference words keep exact integer wire and all28 independently valid graphs");
});
