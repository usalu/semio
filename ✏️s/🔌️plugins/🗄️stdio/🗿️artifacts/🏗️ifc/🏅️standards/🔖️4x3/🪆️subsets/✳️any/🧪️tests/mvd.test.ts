import {expect,test} from "bun:test";
import cases from "./mvd-cases.json" with {type:"json"};
import {ifc4x3MvdCatalog,ifc4x3MvdEntityScope,ifc4x3MvdConceptScope,ifc4x3MvdLabel,type Ifc4x3MvdId} from "../🧬️schema/👁️mvd/🟦️.ts";
import {ifc4x3Catalog} from "../🧬️schema/📚️catalog/🟦️.ts";

test("language-neutral entity and concept scope retains official badges",()=>{
 for(const row of cases.cases)expect(String(ifc4x3MvdEntityScope(row.entity,row.view as Ifc4x3MvdId).state)).toBe(row.state);
 for(const row of cases.concepts)expect(String(ifc4x3MvdConceptScope(row.id,row.view as Ifc4x3MvdId).state)).toBe(row.state);
 expect(String(ifc4x3MvdCatalog.schema)).toBe(cases.schema);expect(ifc4x3MvdCatalog.conformanceEvaluated).toBe(false);
 expect(ifc4x3MvdCatalog.entities.map(row=>row.id).toSorted()).toEqual(Object.keys(ifc4x3Catalog.entities).toSorted());
 expect(new Set(ifc4x3MvdCatalog.concepts.map(row=>row.id)).size).toBe(ifc4x3MvdCatalog.concepts.length);
});
test("view labels require an explicit language and unknown input stays unknown",()=>{
 expect(ifc4x3MvdLabel("ReferenceView","en")).toBe("Reference View");
 expect(ifc4x3MvdLabel("ReferenceView","de")).toBe("Referenzansicht");
 expect(ifc4x3MvdConceptScope("missing","ReferenceView")).toEqual({state:"unknown",causes:[]});
 expect(()=>ifc4x3MvdEntityScope("IFCWALL","missing" as Ifc4x3MvdId)).toThrow();
 expect(()=>ifc4x3MvdLabel("ReferenceView","fr" as "en")).toThrow();
 const unlisted=ifc4x3MvdCatalog.concepts.find(row=>!row.universal&&!row.views.ReferenceView)!;
 expect(ifc4x3MvdConceptScope(unlisted.id,"ReferenceView")).toEqual({state:"unlisted",causes:[]});
 expect(()=>{(ifc4x3MvdCatalog.entities as unknown as {id:string}[])[0]!.id="IFCNOTADECLARATION";}).toThrow();
});
