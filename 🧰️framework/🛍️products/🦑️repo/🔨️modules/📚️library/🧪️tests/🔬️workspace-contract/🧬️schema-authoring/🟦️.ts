import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{parseTree,getNodeValue,type Node,type ParseError}from"jsonc-parser";
import{semioSchemaAjvV1}from"./../../../../../../../🔨️modules/🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
import{validateJsonSchemaSubset}from"./../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import fixture from"../../🧫️fixtures/🧬️schema-authoring/🔣️.json";
const root=resolve(import.meta.dirname,"../../../../../../../.."),resources=fixture.resources.map(path=>JSON.parse(readFileSync(resolve(root,path),"utf8")));
const schemas=Object.fromEntries(Object.entries(fixture.contracts).map(([name,path])=>[name,JSON.parse(readFileSync(resolve(root,path),"utf8"))]));
function duplicateMembers(node:Node,path="$"):string[]{
 const names=new Set<string>(),duplicates:string[]=[];
 if(node.type==="object")for(const property of node.children??[]){const name=String(property.children![0]!.value);if(names.has(name))duplicates.push(path+"."+name);names.add(name);duplicates.push(...duplicateMembers(property.children![1]!,path+"."+name));}
 else for(const child of node.children??[])duplicates.push(...duplicateMembers(child,path));
 return duplicates;
}
for(const [name,path]of Object.entries(fixture.contracts))test(name+" has unique object members and valid Draft07 structure",()=>{const errors:ParseError[]=[],tree=parseTree(readFileSync(resolve(root,path),"utf8"),errors,{disallowComments:true,allowTrailingComma:false});expect(errors).toEqual([]);expect(tree).toBeDefined();expect(duplicateMembers(tree!)).toEqual([]);expect(getNodeValue(tree!)).toEqual(schemas[name]);expect(semioSchemaAjvV1().validateSchema(schemas[name])).toBe(true);
});
for(const row of fixture.cases)test(row.name,()=>{
 const schema=schemas[row.contract]!,ajv=semioSchemaAjvV1({allErrors:true});for(const document of resources)ajv.addSchema(document);
 expect(ajv.compile(schema)(row.value)).toBe(row.valid);expect(validateJsonSchemaSubset(schema,row.value,schema,resources).length===0).toBe(row.valid);
});
test("mesh creation uses the public nullable managed mesh-state contract",()=>{expect(schemas.mesh.properties.meshState).toEqual({$ref:"https://json.schemas.assets.semio-tech.com/s/lowpoly/lowpoly/artifact.json#/$defs/LowpolyObject/properties/meshState"});});
test("driver schema preserves localized public field metadata",()=>{expect(schemas.ui.$defs.UiDriver.properties.driverId["x-semio-ui"].label).toEqual({en:"Driver ID",de:"Treiber-ID"});expect(schemas.ui.$defs.UiDriver.properties.config["x-semio-ui"].label).toEqual({en:"Configuration",de:"Konfiguration"});});

for(const row of fixture.boardCases)test("board selection highlights "+row.name,()=>{
 const document=JSON.parse(readFileSync(resolve(root,fixture.boardContract),"utf8")),property=document.$defs.SceneDescriptor.properties.selectionExitHighlightIds,schema={...property,$defs:document.$defs};
 expect(semioSchemaAjvV1().compile(schema)(row.value)).toBe(row.valid);expect(validateJsonSchemaSubset(schema,row.value,schema).length===0).toBe(row.valid);
});
