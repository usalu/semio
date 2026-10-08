import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
const root=resolve(import.meta.dir,"../../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const before={schema:"stdio.ifc.2x3",document:{header:{fileDescription:[{kind:"list",values:[{kind:"str",value:"ViewDefinition [CoordinationView_V2.0]"}]},{kind:"str",value:"2;1"}],fileName:[],fileSchema:[{kind:"list",values:[{kind:"str",value:"IFC2X3"}]}]},instances:[]}};
const mutation={SetViewDefinition:{view:"StructuralAnalysisView"}};
test("IFC model view committed history intent agrees with independent RFC6902",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(read("🪆️subsets/🧱️base/🧬️schema/🔣️.json"));const validate=ajv.compile(read("🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"));
 expect(validate(before)).toBe(true);
 const after=applyPatch(structuredClone(before),[{op:"replace",path:"/document/header/fileDescription/0/values/0/value",value:"ViewDefinition [StructuralAnalysisView]"}],true,true).newDocument;
 expect(validate(after)).toBe(true);expect(after).not.toEqual(before);
 for(const subset of ["🤝️cv20","🏢️cobie","🧮️sav"]){
  const leaf=ajv.compile(read(`🪆️subsets/${subset}/🧬️schema/🧬️mutations/👁️set-view-definition/🧬️schema/🔣️.json`));expect(leaf(mutation.SetViewDefinition)).toBe(true);
  const path=resolve(root,`🪆️subsets/${subset}/🧫️fixtures/🧬️history-edits/view/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation,before,after});
  console.log("[DEBUG] IFC native model view history intent agrees with RFC6902",subset);
 }
});

const producerBefore={...before,edmPreamble:{producer:"Exporter One",module:"",creationDate:"",host:"",database:"",databaseVersion:"",databaseCreationDate:"",schema:"IFC2X3",model:"",modelCreationDate:"",headerModel:"",headerModelCreationDate:"",user:"",group:"",license:"",options:""}};
const producerMutation={mutation:"patchSnapshot",patch:{operation:"set",path:"/edmPreamble/producer",value:"Exporter Two"}};
test("IFC subset editor native vocabulary retains a committed EDM producer intent",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(read("🪆️subsets/🧱️base/🧬️schema/🔣️.json"));ajv.addSchema(read("../../../../📇️registry/🧬️schema/🔣️.json"));const validate=ajv.compile(read("🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"));const patch=ajv.compile({$ref:"https://json.schemas.assets.semio-tech.com/s/stdio/registry/schema.json#/$defs/SnapshotPatch"});
 expect(validate(producerBefore)).toBe(true);expect(patch(producerMutation.patch)).toBe(true);
 const after=applyPatch(structuredClone(producerBefore),[{op:"replace",path:"/edmPreamble/producer",value:"Exporter Two"}],true,true).newDocument;
 expect(validate(after)).toBe(true);expect(after.document).toEqual(producerBefore.document);expect(after.edmPreamble.producer).toBe("Exporter Two");
 for(const subset of ["🤝️cv20","🏢️cobie","🧮️sav"]){
  const editor=readFileSync(resolve(root,`🪆️subsets/${subset}/✏️editor/🦀️.rs`),"utf8");expect(editor).toContain("type Mutation = Ifc2x3Mutation;");
  const path=resolve(root,`🪆️subsets/${subset}/🧫️fixtures/🧬️history-edits/producer/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:producerMutation,before:producerBefore,after});
  console.log("[DEBUG] IFC subset editor EDM producer intent agrees with RFC6902",subset);
 }
});
