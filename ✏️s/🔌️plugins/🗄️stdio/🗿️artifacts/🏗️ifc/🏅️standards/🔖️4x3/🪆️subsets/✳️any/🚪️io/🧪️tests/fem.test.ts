import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {fem3dToIfc4x3,ifc4x3ToFem3d} from "../fem.ts";
import {ifcReal,parseIfc4x3Step,ifcNumber} from "../step.ts";
import {validateIfc4x3Snapshot} from "../../🧬️schema/📚️catalog/🟦️.ts";
import type {Fem3dSnapshot} from "../../../../../../../../../🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";
const w=(n:number)=>{const v=ifcReal(n);if(v.kind!=="real")throw Error("real");return v.value;};
const fixture=():Fem3dSnapshot=>({nodes:[{id:"n1",x:w(0),y:w(0),z:w(0)},{id:"n2",x:w(2),y:w(0),z:w(0)}],elements:[{kind:"frame",id:"f1",start:"n1",end:"n2",materialId:"steel",sectionId:"s1",roll:w(0)}],materials:[{id:"steel",name:"Steel",e:w(2e11),g:w(8e10),nu:w(.25),rho:w(7850)}],sections:[{id:"s1",name:"Section",area:w(.01),iy:w(1e-5),iz:w(2e-5),j:w(3e-5)}],solids:[],supports:[{id:"fixed",nodeId:"n1",fixed:["Tx","Ty","Tz","Rx","Ry","Rz"]}],loadCases:[{id:"case",name:"Load",selfWeight:false,loads:[{kind:"nodal",id:"force",nodeId:"n2",dof:"Tz",value:w(-1000)},{kind:"memberUdl",id:"udl",elementId:"f1",wx:w(0),wy:w(0),wz:w(-500)}]}],combinations:[],analysis:{modalCount:0n,bucklingCount:0n,deformationScale:w(1)}});
test("real FEM3d topology, engineering values, supports and actions roundtrip",async()=>{
 const exported=await fem3dToIfc4x3(fixture());expect(exported.artifact).toBeDefined();expect(validateIfc4x3Snapshot(exported.artifact!)).toEqual([]);
 const imported=await ifc4x3ToFem3d(exported.artifact!);expect(imported.artifact).toBeDefined();const model=imported.artifact!;
 expect(model.nodes.length).toBe(2);expect(model.elements.length).toBe(1);expect(model.materials[0]!.e).toEqual(w(2e11));expect(model.sections[0]!.j).toEqual(w(3e-5));
 expect(model.supports[0]!.fixed).toEqual(["Tx","Ty","Tz","Rx","Ry","Rz"]);expect(model.loadCases[0]!.loads.length).toBe(2);
 expect(model.loadCases[0]!.loads.find(l=>l.kind==="nodal")).toHaveProperty("value",w(-1000));
});
test("official structural model is refused when missing representable engineering semantics",async()=>{
 const source=await parseIfc4x3Step(await readFile(new URL("structural.ifc",import.meta.url),"utf8")),outcome=await ifc4x3ToFem3d(source);
 expect(outcome.artifact).toBeUndefined();expect(outcome.fidelity).toBe("unsupported");expect(outcome.diagnostics[0]!.code).toBe("fem.shared-placement");
});
test("FEM unsupported semantics are refused before publication",async()=>{
 const model=fixture();model.loadCases[0]!.loads.push({kind:"area",id:"area",solidId:"absent",pressure:w(1)});
 expect((await fem3dToIfc4x3(model)).artifact).toBeUndefined();
 const abort=new AbortController();abort.abort();await expect(fem3dToIfc4x3(fixture(),{signal:abort.signal})).rejects.toHaveProperty("code","io.cancelled");
});
test("FEM frame roll follows the native reference basis",async()=>{
 const model=fixture();model.elements[0]={...model.elements[0]!,kind:"frame",roll:w(.25)};model.nodes[1]!.x=w(0);model.nodes[1]!.z=w(2);
 const exported=await fem3dToIfc4x3(model),imported=await ifc4x3ToFem3d(exported.artifact!),member=imported.artifact!.elements[0]!;
 expect(member.kind).toBe("frame");if(member.kind==="frame")expect(ifcNumber({kind:"real",value:member.roll})).toBeCloseTo(.25,12);
});
test.each(["inertia-dimension","density-dimension","pressure-name","wrong-measure","missing-analysis-membership","factored-loads","member-release","member-additional-condition","member-supported-length","member-condition-axes","missing-endpoint","action-placement","unassigned-action","duplicate-action"])("foreign FEM %s is refused atomically",async defect=>{
 const graph=(await fem3dToIfc4x3(fixture())).artifact!,entity=(id:bigint)=>graph.entities.find(row=>row.id===id)!,reference=(value:unknown)=>entity((value as {value:bigint}).value);
 if(defect==="inertia-dimension"||defect==="density-dimension") {
  const unit=graph.entities.find(row=>row.name==="IFCDERIVEDUNIT"&&row.args[1]?.kind==="enum"&&row.args[1].value===(defect==="inertia-dimension"?"MOMENTOFINERTIAUNIT":"MASSDENSITYUNIT"))!;
  const part=reference((unit.args[0] as {value:unknown[]}).value[defect==="density-dimension"?1:0]);part.args[1]={kind:"integer",value:1n};
 }
 if(defect==="pressure-name")graph.entities.find(row=>row.name==="IFCSIUNIT"&&row.args[3]?.kind==="enum"&&row.args[3].value==="PASCAL")!.args[3]={kind:"enum",value:"NEWTON"};
 if(defect==="wrong-measure")graph.entities.find(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&row.args[0].value==="TorsionalConstant")!.args[2]={kind:"typedValue",value:{name:"IFCLENGTHMEASURE",items:[ifcReal(3e-5)]}};
 if(defect==="missing-analysis-membership")graph.entities=graph.entities.filter(row=>!(row.name==="IFCRELASSIGNSTOGROUP"&&reference(row.args[6]).name==="IFCSTRUCTURALANALYSISMODEL"));
 if(defect==="factored-loads") {const assignment=graph.entities.find(row=>row.name==="IFCRELASSIGNSTOGROUP"&&reference(row.args[6]).name==="IFCSTRUCTURALLOADCASE")!;assignment.name="IFCRELASSIGNSTOGROUPBYFACTOR";assignment.args.push(ifcReal(2));}
 if(defect==="member-release")graph.entities.find(row=>row.name==="IFCRELCONNECTSSTRUCTURALMEMBER")!.args[6]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCBOUNDARYNODECONDITION")!.id};
 if(defect==="member-additional-condition")graph.entities.find(row=>row.name==="IFCRELCONNECTSSTRUCTURALMEMBER")!.args[7]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCBOUNDARYNODECONDITION")!.id};
 if(defect==="member-supported-length")graph.entities.find(row=>row.name==="IFCRELCONNECTSSTRUCTURALMEMBER")!.args[8]=ifcReal(1);
 if(defect==="member-condition-axes")graph.entities.find(row=>row.name==="IFCRELCONNECTSSTRUCTURALMEMBER")!.args[9]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCAXIS2PLACEMENT3D")!.id};
 if(defect==="missing-endpoint") {const relation=graph.entities.find(row=>row.name==="IFCRELCONNECTSSTRUCTURALMEMBER")!;graph.entities=graph.entities.filter(row=>row.id!==relation.id);}
 if(defect==="action-placement")graph.entities.find(row=>row.name==="IFCSTRUCTURALPOINTACTION")!.args[5]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCAXIS2PLACEMENT3D")!.id};
 if(defect==="unassigned-action")graph.entities=graph.entities.filter(row=>!(row.name==="IFCRELASSIGNSTOGROUP"&&reference(row.args[6]).name==="IFCSTRUCTURALLOADCASE"));
 if(defect==="duplicate-action") {const assignment=graph.entities.find(row=>row.name==="IFCRELASSIGNSTOGROUP"&&reference(row.args[6]).name==="IFCSTRUCTURALLOADCASE")!,values=assignment.args[4];if(values?.kind==="aggregate")values.value.push(structuredClone(values.value[0]!));}
 const before=structuredClone(graph),outcome=await ifc4x3ToFem3d(graph);
 expect(outcome.fidelity).toBe("unsupported");expect(outcome.artifact).toBeUndefined();expect(outcome.diagnostics.some(d=>d.severity==="error")).toBe(true);expect(graph).toEqual(before);
});
test("shared FEM section, material and boundary definitions preserve canonical destination identities",async()=>{
 const source=fixture();source.elements.push({...source.elements[0]!,id:"f2"});source.supports.push({id:"fixed2",nodeId:"n2",fixed:["Tx","Ty","Tz","Rx","Ry","Rz"]});
 const graph=(await fem3dToIfc4x3(source)).artifact!,definitions=graph.entities.filter(row=>row.name==="IFCRELDEFINESBYPROPERTIES"),connections=graph.entities.filter(row=>row.name==="IFCSTRUCTURALPOINTCONNECTION");
 for(const definition of definitions)definition.args[5]=structuredClone(definitions[0]!.args[5]!);
 connections[1]!.args[7]=structuredClone(connections[0]!.args[7]!);
 const before=structuredClone(graph),outcome=await ifc4x3ToFem3d(graph);expect(outcome.artifact).toBeDefined();const model=outcome.artifact!;
 expect(model.sections.length).toBe(1);expect(model.materials.length).toBe(1);expect(model.elements.length).toBe(2);expect(new Set(model.elements.map(row=>row.sectionId)).size).toBe(1);expect(new Set(model.elements.map(row=>row.materialId)).size).toBe(1);
 expect(model.supports.length).toBe(2);expect(new Set(model.supports.map(row=>row.id)).size).toBe(2);expect(new Set(model.supports.map(row=>row.nodeId)).size).toBe(2);expect(graph).toEqual(before);
 const exported=await fem3dToIfc4x3(model);expect(exported.artifact).toBeDefined();expect(validateIfc4x3Snapshot(exported.artifact!)).toEqual([]);
 const again=await ifc4x3ToFem3d(exported.artifact!);expect(again.artifact!.sections.length).toBe(1);expect(new Set(again.artifact!.elements.map(row=>row.sectionId)).size).toBe(1);expect(new Set(again.artifact!.supports.map(row=>row.id)).size).toBe(2);
});
test.each(["shared-material-properties","conflicting-material-properties","ambiguous-material","ratio-with-unit"])("FEM %s retains unambiguous definitions or refuses",async variant=>{
 const graph=(await fem3dToIfc4x3(fixture())).artifact!;let next=graph.entities.reduce((max,row)=>row.id>max?row.id:max,0n)+1n;
 const definitions=graph.entities.find(row=>row.name==="IFCMATERIALPROPERTIES")!,copy=structuredClone(definitions);copy.id=next++;copy.args[0]={kind:"string",value:"Shared engineering properties"};
 if(variant==="shared-material-properties")graph.entities.push(copy);
 if(variant==="conflicting-material-properties") {const property=structuredClone(graph.entities.find(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&row.args[0].value==="YoungModulus")!);property.id=next++;property.args[2]={kind:"typedValue",value:{name:"IFCMODULUSOFELASTICITYMEASURE",items:[ifcReal(1e11)]}};graph.entities.push(property,copy);copy.args[2]={kind:"aggregate",value:[{kind:"reference",value:property.id}]};}
 if(variant==="ambiguous-material") {const material=structuredClone(graph.entities.find(row=>row.name==="IFCMATERIAL")!);material.id=next++;const association=structuredClone(graph.entities.find(row=>row.name==="IFCRELASSOCIATESMATERIAL")!);association.id=next++;association.args[0]={kind:"string",value:String(association.id).padStart(22,"0")};association.args[5]={kind:"reference",value:material.id};copy.args[3]={kind:"reference",value:material.id};graph.entities.push(material,copy,association);}
 if(variant==="ratio-with-unit")graph.entities.find(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&row.args[0].value==="PoissonRatio")!.args[3]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCSIUNIT"&&row.args[3]?.kind==="enum"&&row.args[3].value==="METRE")!.id};
 const before=structuredClone(graph),outcome=await ifc4x3ToFem3d(graph);
 if(variant==="shared-material-properties") {expect(outcome.artifact).toBeDefined();expect(outcome.artifact!.materials.length).toBe(1);expect(outcome.artifact!.materials[0]!.e).toEqual(w(2e11));}
 else {expect(outcome.fidelity).toBe("unsupported");expect(outcome.artifact).toBeUndefined();expect(outcome.diagnostics.some(row=>row.severity==="error")).toBe(true);}
 expect(graph).toEqual(before);
});
test.skipIf(!process.env.SEMIO_IFC_ORACLE_OUTPUT)("IfcOpenShell shared-definition graphs reimport with canonical FEM identities",async()=>{
 const {parseIfc4x3Step}=await import("../step.ts");
 for(const name of ["fem-shared","fem-shared-roundtrip"]) {const graph=await parseIfc4x3Step(await readFile(`${process.env.SEMIO_IFC_ORACLE_OUTPUT}/${name}.ifc`,"utf8")),outcome=await ifc4x3ToFem3d(graph);expect(outcome.artifact).toBeDefined();expect(outcome.artifact!.sections.length).toBe(1);expect(outcome.artifact!.materials.length).toBe(1);expect(outcome.artifact!.elements.length).toBe(2);expect(new Set(outcome.artifact!.supports.map(row=>row.id)).size).toBe(2);}
});
test("FEM canonical definition projection remains cancellable before publication",async()=>{
 const graph=(await fem3dToIfc4x3(fixture())).artifact!,before=structuredClone(graph),abort=new AbortController();
 await expect(ifc4x3ToFem3d(graph,{signal:abort.signal,progress:progress=>{if(progress.phase==="fem-members")abort.abort();}})).rejects.toHaveProperty("code","io.cancelled");expect(graph).toEqual(before);
});
