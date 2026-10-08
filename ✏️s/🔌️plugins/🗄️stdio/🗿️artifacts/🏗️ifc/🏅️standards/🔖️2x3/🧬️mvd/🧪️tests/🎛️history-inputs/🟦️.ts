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
