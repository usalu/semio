import data from "./🔣️.json" with {type:"json"};

export type Ifc4x3MvdId = "ReferenceView" | "Alignment-basedReferenceView";
export type Ifc4x3MvdLocale = "en" | "de";
export interface Ifc4x3MvdEntry {
 readonly id:string;
 readonly name:string;
 readonly href:string;
 readonly universal:boolean;
 readonly views:Readonly<Partial<Record<Ifc4x3MvdId,readonly string[]>>>;
}
export interface Ifc4x3MvdScope {
 readonly state:"universal"|"included"|"unlisted"|"unknown";
 readonly causes:readonly string[];
}
export interface Ifc4x3MvdCatalog {
 readonly schema:"IFC4X3_ADD2";
 readonly sourceCommit:string;
 readonly sources:readonly Readonly<{url:string;sha256:string}>[];
 readonly conformanceEvaluated:false;
 readonly views:readonly Readonly<{id:Ifc4x3MvdId;label:Readonly<Record<Ifc4x3MvdLocale,string>>}>[];
 readonly entities:readonly Ifc4x3MvdEntry[];
 readonly concepts:readonly Ifc4x3MvdEntry[];
}

function freeze<T>(value:T):T {
 if(value!==null&&typeof value==="object")for(const child of Object.values(value))freeze(child);
 return Object.freeze(value);
}

export const ifc4x3MvdCatalog:Ifc4x3MvdCatalog=freeze(data as Ifc4x3MvdCatalog);
const entities=new Map(ifc4x3MvdCatalog.entities.map(row=>[row.id,row]));
const concepts=new Map(ifc4x3MvdCatalog.concepts.map(row=>[row.id,row]));

function scope(entry:Ifc4x3MvdEntry|undefined,view:Ifc4x3MvdId):Ifc4x3MvdScope {
 if(!ifc4x3MvdCatalog.views.some(row=>row.id===view))throw new Error("Unknown IFC4x3 view identifier");
 return {state:entry?.universal?"universal":entry?.views[view]?"included":entry?"unlisted":"unknown",causes:entry?.views[view]?[...entry.views[view]]:[]};
}

/** 👁️ Report published informative scope without asserting exchange conformance or restricting editing. */
export function ifc4x3MvdEntityScope(entity:string,view:Ifc4x3MvdId):Ifc4x3MvdScope {
 return scope(entities.get(entity.toUpperCase()),view);
}

/** 🧩 Resolve a concept's exact release path and declared view badges. */
export function ifc4x3MvdConceptScope(id:string,view:Ifc4x3MvdId):Ifc4x3MvdScope {
 return scope(concepts.get(id),view);
}

/** 🗣️ Select an explicit language for a published view label. */
export function ifc4x3MvdLabel(view:Ifc4x3MvdId,locale:Ifc4x3MvdLocale):string {
 const row=ifc4x3MvdCatalog.views.find(row=>row.id===view);
 if(!row||(locale!=="en"&&locale!=="de"))throw new Error("Unknown IFC4x3 view or language");
 return row.label[locale];
}
