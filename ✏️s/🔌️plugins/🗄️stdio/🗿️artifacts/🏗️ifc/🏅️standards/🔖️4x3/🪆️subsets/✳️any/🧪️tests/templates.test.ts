/** 🏷️ Released PSD/QTO metadata matches neutral source and independent template vectors. */
import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🏷️templates/schema.json";
import cases from "../🧬️schema/🏷️templates/cases.json";
import chooser from "../🧬️schema/🏷️templates/✍️apply/chooser.json";
import chooserSchema from "../🧬️schema/🏷️templates/✍️apply/chooser.schema.json";
import {ifc4x3TemplateCatalog,ifc4x3Template,ifc4x3TemplateApplicability,ifc4x3TemplateKindLabel} from "../🧬️schema/🏷️templates/🟦️.ts";
import {ifc4x3Catalog} from "../🧬️schema/📚️catalog/🟦️.ts";

test("the explicit released-template chooser covers every carrier kind without implicit selection",()=>{
 const admit=new Ajv({strict:true,allErrors:true}).compile(chooserSchema),templates=ifc4x3TemplateCatalog().templates;
 expect(admit(chooser)).toBe(true);
 expect(chooser.required).toBe(true);expect(chooser.automaticSelection).toBe(false);
 expect(templates.length).toBe(chooser.choices);
 expect(new Set(templates.map(template=>template.name)).size).toBe(chooser.choices);
 const populations:Record<string,number>={};
 for(const template of templates)populations[template.kind]=(populations[template.kind]??0)+1;
 expect(populations).toEqual(chooser.kinds);
 expect(templates.map(template=>template.name)).toEqual(templates.map(template=>template.name).toSorted((a,b)=>a.toUpperCase().localeCompare(b.toUpperCase(),'en')));
});
test("complete released property and quantity catalog has immutable schema-admitted metadata",async()=>{
 const catalog=ifc4x3TemplateCatalog(),admit=new Ajv({strict:true,allErrors:true}).compile(schema);
 expect(admit(catalog)).toBe(true);
 expect(catalog.templates.length).toBe(cases.counts.templates);
 expect(catalog.templates.filter(value=>value.kind.startsWith("PSET_")).length).toBe(cases.counts.propertySets);
 expect(catalog.templates.filter(value=>value.kind.startsWith("QTO_")).length).toBe(cases.counts.quantitySets);
 expect(catalog.templates.reduce((total,value)=>total+value.entries.length,0)).toBe(cases.counts.entries);
 expect(catalog.templates.flatMap(value=>value.entries).reduce((total,value)=>total+(value.kind==="P_ENUMERATEDVALUE"?value.enumeration.literals.length:0),0)).toBe(cases.counts.enumLiterals);
 expect(Object.isFrozen(catalog.templates[0]!.entries)).toBe(true);
 expect(createHash("sha256").update(await readFile(new URL("../🧬️schema/🏷️templates/annex-a-psd.zip",import.meta.url))).digest("hex")).toBe(catalog.source.sha256);
});
test("every released template value domain and applicability entity resolves in the complete ADD2 schema",()=>{
 const unresolvedDomains:string[]=[],unresolvedTargets:string[]=[];
 for(const template of ifc4x3TemplateCatalog().templates){
  for(const entry of template.entries)for(const name of [entry.dataType,...(entry.kind==="P_TABLEVALUE"?[entry.secondaryType]:[])])if(!ifc4x3Catalog.types[name.toUpperCase()]&&!ifc4x3Catalog.entities[name.toUpperCase()])unresolvedDomains.push(`${template.name}.${entry.name}:${name}`);
  for(const selector of template.classes){const name=selector.split("/")[0]!;if(!ifc4x3Catalog.entities[name.toUpperCase()])unresolvedTargets.push(`${template.name}:${selector}`);}
 }
 expect(unresolvedDomains).toEqual([]);expect(unresolvedTargets).toEqual([]);
});
for(const row of cases.applicability)test(`template applicability ${row.template} ${row.entity} ${row.predefinedType??""}`,()=>{
 expect(row.state).toBe(ifc4x3TemplateApplicability(ifc4x3Template(row.template)!,row.entity,row.predefinedType));
});
test("neutral fields retain exact enum and quantity domains",()=>{
 for(const row of cases.entries){const entry=ifc4x3Template(row.template)!.entries.find(value=>value.name===row.entry)!;expect(row.kind).toBe(entry.kind);expect(entry.dataType).toBe(row.dataType);if(typeof row.enumeration==="string"&&typeof row.literal==="string"&&entry.kind==="P_ENUMERATEDVALUE"){expect(entry.enumeration.name).toBe(row.enumeration);expect(entry.enumeration.literals).toContain(row.literal);}}
 expect(ifc4x3Template("missing")).toBeUndefined();
 expect(ifc4x3TemplateKindLabel("P_SINGLEVALUE","en")).toBe("Single Value");
 expect(ifc4x3TemplateKindLabel("P_SINGLEVALUE","de")).toBe("Einzelwert");
 expect(()=>ifc4x3TemplateKindLabel("P_SINGLEVALUE",undefined as never)).toThrow();
});
