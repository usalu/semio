import {test as register} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {ifc2x3Catalog,ifc2x3AttributeDeclarations,ifc2x3Attribute,ifc2x3IsSubtype,ifc2x3EnumMembers,ifc2x3SelectAccepts,ifc2x3ResolveType} from "../🟦️.ts";
import {extractExpressCatalog} from "../../../../../../../📚️catalog/🛠️extract/🟦️.ts";
import {renderRustCatalog} from "../../../../../../../📚️catalog/🛠️extract/🦀️render.ts";
export const records:{name:string;passed:boolean;error?:string}[]=[];
export const checks:Promise<void>[]=[];
const test=(name:string,run:()=>void):void=>{checks.push(register(name,()=>{try{run();records.push({name,passed:true});}catch(error){records.push({name,passed:false,error:String(error)});throw error;}}));};
const read=(file:string):string=>readFileSync(new URL(file,import.meta.url),"utf8"),vectors=JSON.parse(read("./🔣️.json"));
test("IFC2x3 complete catalog reproduces unchanged normative input and native output",()=>{
 const source=read("../IFC2X3_TC1.exp"),manifest=JSON.parse(read("../source.json"));assert.equal(createHash("sha256").update(source).digest("hex"),manifest.source.sha256);
 const extracted=extractExpressCatalog(source,manifest);assert.deepEqual(extracted,ifc2x3Catalog);assert.equal(renderRustCatalog(extracted),read("../data.rs"));
 assert.deepEqual({entities:Object.keys(extracted.entities).length,types:Object.keys(extracted.types).length,enumerations:Object.values(extracted.types).filter(t=>t.kind==="enum").length,selects:Object.values(extracted.types).filter(t=>t.kind==="select").length,definedTypes:Object.values(extracted.types).filter(t=>t.kind==="defined").length,functions:Object.keys(extracted.functions).length,rules:Object.keys(extracted.rules).length},vectors.counts);
 assert.equal(ifc2x3Catalog.schema,"IFC2X3");assert.ok(Object.values(ifc2x3Catalog.validation).every(v=>v===false));
});
test("IFC2x3 declarations and queries satisfy independent schema and neutral domains",()=>{
 const schema=JSON.parse(read("../../../../../../../📚️catalog/🧬️schema/🔣️.json")),validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.equal(validate(ifc2x3Catalog),true,JSON.stringify(validate.errors));
 for(const [a,b,result]of vectors.subtypes)assert.equal(ifc2x3IsSubtype(a,b),result);
 for(const row of vectors.attributes){const attr=ifc2x3Attribute(row.entity,row.index)!;assert.deepEqual({name:attr.name,optional:attr.optional,derived:attr.derived},{name:row.name,optional:row.optional,derived:row.derived});assert.equal(ifc2x3AttributeDeclarations(row.entity)?.[row.index],attr);}
 for(const row of vectors.enums)assert.deepEqual(ifc2x3EnumMembers(row.name),row.members);
 for(const row of vectors.selects){for(const name of row.accepts)assert.equal(ifc2x3SelectAccepts(row.name,name),true);for(const name of row.rejects)assert.equal(ifc2x3SelectAccepts(row.name,name),false);}
 assert.equal(ifc2x3AttributeDeclarations("IfcWall")?.length,8);const label=ifc2x3ResolveType({kind:"named",name:"IFCLABEL"})!;assert.equal(label.name,"STRING");assert.equal(label.width,null);
});
test("existing independent IfcOpenShell IFC2x3 metadata agrees on every declaration",()=>{
 const reference=JSON.parse(read("./ifcopenshell-reference.json")) as {implementation:string;entities:Record<string,{attributes:string[];predefinedTypes?:string[]}>;types:string[]};assert.equal(reference.implementation,"IfcOpenShell");
 assert.deepEqual(Object.keys(ifc2x3Catalog.entities).sort(),Object.keys(reference.entities).map(v=>v.toUpperCase()).sort());assert.deepEqual(Object.keys(ifc2x3Catalog.types).sort(),reference.types.map(v=>v.toUpperCase()).sort());let attributes=0,enums=0;
 for(const [name,row]of Object.entries(reference.entities)){const entity=ifc2x3Catalog.entities[name.toUpperCase()],names=new Set([...entity.allAttributes.map(a=>a.name),...entity.derived.map(a=>a.name),...entity.inverse.map(a=>/^\w+/.exec(a)?.[0])]);for(const attr of row.attributes){assert.ok(names.has(attr),name+"."+attr);attributes++;}if(row.predefinedTypes){const attr=entity.allAttributes.find(a=>a.name==="PredefinedType"),members=attr?.typeExpr.name?ifc2x3EnumMembers(attr.typeExpr.name):undefined;if(members){assert.deepEqual([...members].sort(),row.predefinedTypes,name);enums++;}}}
 assert.equal(attributes,1402);assert.equal(enums,88);
});
