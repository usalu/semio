/** 🏷️ Exact released property and quantity domains with explicit applicability and language. */
import data from "./🔣️.json";
import {ifc4x3Catalog,ifcIsSubtype} from "../📚️catalog/🟦️.ts";
export type IfcTemplateKind="PSET_OCCURRENCEDRIVEN"|"PSET_PERFORMANCEDRIVEN"|"PSET_TYPEDRIVENOVERRIDE"|"PSET_MATERIALDRIVEN"|"PSET_PROFILEDRIVEN"|"QTO_TYPEDRIVENOVERRIDE"|"QTO_OCCURRENCEDRIVEN";
export type IfcTemplateSimpleKind="P_SINGLEVALUE"|"P_REFERENCEVALUE"|"P_LISTVALUE"|"P_BOUNDEDVALUE"|"Q_WEIGHT"|"Q_LENGTH"|"Q_AREA"|"Q_VOLUME"|"Q_COUNT"|"Q_TIME";
interface IfcTemplateEntryBase {readonly name:string;readonly definition:string;readonly dataType:string}
export type IfcTemplateEntry=(IfcTemplateEntryBase&{readonly kind:IfcTemplateSimpleKind})|(IfcTemplateEntryBase&{readonly kind:"P_ENUMERATEDVALUE";readonly enumeration:{readonly name:string;readonly literals:readonly string[]}})|(IfcTemplateEntryBase&{readonly kind:"P_TABLEVALUE";readonly secondaryType:string;readonly expression:string});
export interface IfcTemplate {readonly name:string;readonly definition:string;readonly kind:IfcTemplateKind;readonly applicable:string;readonly classes:readonly string[];readonly entries:readonly IfcTemplateEntry[];readonly sourceFile:string;readonly sourceSha256:string}
export interface IfcTemplateCatalog {readonly schema:"IFC4X3_ADD2";readonly source:{readonly url:string;readonly commit:string;readonly sha256:string;readonly language:"en"};readonly templates:readonly IfcTemplate[]}
export type IfcTemplateApplicability="applicable"|"inapplicable"|"requiresPredefinedType"|"unknownEntity";
function freeze(value:unknown):void{if(value&&typeof value==="object"&&!Object.isFrozen(value)){for(const child of Object.values(value))freeze(child);Object.freeze(value);}}
freeze(data);
const catalog=data as unknown as IfcTemplateCatalog,index=new Map(catalog.templates.map(template=>[template.name.toUpperCase(),template]));
/** 📖️ Borrow the complete immutable official ADD2 template catalog. */
export function ifc4x3TemplateCatalog():IfcTemplateCatalog{return catalog;}
/** 🔎️ Resolve an exact official name without selecting a fallback template. */
export function ifc4x3Template(name:string):IfcTemplate|undefined{return index.get(name.toUpperCase());}
/** 🎯️ Apply inherited entity and exact predefined-type selectors independently of MVD conformance. */
export function ifc4x3TemplateApplicability(template:IfcTemplate,entity:string,predefinedType?:string):IfcTemplateApplicability{
 entity=entity.toUpperCase();if(!ifc4x3Catalog.entities[entity])return"unknownEntity";let pending=false;
 for(const selector of template.classes){const [target,predefined]=selector.toUpperCase().split("/");if(!target||!ifcIsSubtype(entity,target))continue;if(!predefined)return"applicable";if(predefinedType===undefined){pending=true;continue;}if(predefinedType.toUpperCase()===predefined)return"applicable";}
 return pending?"requiresPredefinedType":"inapplicable";
}
const labels={P_SINGLEVALUE:["Single Value","Einzelwert"],P_REFERENCEVALUE:["Reference Value","Referenzwert"],P_ENUMERATEDVALUE:["Enumerated Value","Aufzählungswert"],P_LISTVALUE:["List Value","Listenwert"],P_BOUNDEDVALUE:["Bounded Value","Wertebereich"],P_TABLEVALUE:["Table Value","Tabellenwert"],Q_WEIGHT:["Weight","Gewicht"],Q_LENGTH:["Length","Länge"],Q_AREA:["Area","Fläche"],Q_VOLUME:["Volume","Volumen"],Q_COUNT:["Count","Anzahl"],Q_TIME:["Time","Zeit"]} as const;
/** 🌐️ Localize field-form labels only when an explicit supported language is supplied. */
export function ifc4x3TemplateKindLabel(kind:IfcTemplateEntry["kind"],locale:"en"|"de"):string{if(locale!=="en"&&locale!=="de")throw new Error("An explicit supported template locale is required");return labels[kind][locale==="en"?0:1];}
