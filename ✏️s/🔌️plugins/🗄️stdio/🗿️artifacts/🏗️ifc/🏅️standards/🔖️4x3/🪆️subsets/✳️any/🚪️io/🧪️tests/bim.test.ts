import {test,expect} from "bun:test";
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from "../bim.ts";
import {validateIfc4x3Snapshot} from "../../🧬️schema/📚️catalog/🟦️.ts";
import {ifcReal} from "../step.ts";
import {readFile} from "node:fs/promises";
test("actual BIM spatial fields map to IFC spatial structure and authored storey data",async()=>{
 const source={schema:"bim.model",project:{name:"Project",description:"Spatial fixture",author:"Author",organization:"Organization",phase_names:["Phase"]},sites:{site:{name:"Site",latitude:47.5,longitude:8.5,elevation:400,true_north:.2,boundary:[]}},buildings:{building:{site:"site",name:"Building",origin:{x:1,y:2},rotation:.3,elevation:4}},storeys:{basement:{building:"building",name:"Basement",level:-1,height:2.5},ground:{building:"building",name:"Ground",level:0,height:3,cut_height:1.2},upper:{building:"building",name:"Upper",level:1,height:3}}};
 const exported=await bimSpatialToIfc4x3(source);expect(exported.artifact).toBeDefined();expect(validateIfc4x3Snapshot(exported.artifact!)).toEqual([]);
 const imported=await ifc4x3ToBimSpatial(exported.artifact!);expect(imported.artifact).toBeDefined();const model=imported.artifact!;
 expect(model.project).toEqual(source.project);expect(Object.values(model.sites!)[0]!.latitude).toBe(47.5);expect(Object.values(model.buildings!)[0]!.origin).toEqual({x:1,y:2});expect(Object.values(model.storeys!).map(row=>row.height)).toEqual([2.5,3,3]);
 expect(Object.values(model.storeys!)[1]!.cut_height).toBe(1.2);
});
test("BIM family omissions and missing references produce explicit outcomes",async()=>{
 const source={schema:"bim.model",project:{name:"Project",description:"",author:"",organization:"",phase_names:[]},buildings:{building:{site:"absent",name:"Building",origin:{x:0,y:0},rotation:0,elevation:0}}};
 const result=await bimSpatialToIfc4x3(source);expect(result.artifact).toBeUndefined();expect(result.fidelity).toBe("unsupported");
});
test.each(["site-translation","context-translation","absolute-building","storey-translation","storey-rotation","storey-authored-level"])("foreign BIM %s is refused atomically",async defect=>{
 const source={schema:"s.bim.model@1",project:{name:"Project",description:"",author:"",organization:"",phase_names:[]},sites:{s:{name:"Site",latitude:0,longitude:0,elevation:0,true_north:0,boundary:[]}},buildings:{b:{site:"s",name:"Building",origin:{x:1,y:2},rotation:0,elevation:0}},storeys:{g:{building:"b",name:"Ground",level:0,height:3},u:{building:"b",name:"Upper",level:1,height:3}}};
 const graph=(await bimSpatialToIfc4x3(source)).artifact!,lookup=(value:unknown)=>graph.entities.find(row=>row.id===(value as {value:bigint}).value)!;
 const target=graph.entities.find(row=>row.name===(defect==="site-translation"?"IFCSITE":defect==="context-translation"?"IFCGEOMETRICREPRESENTATIONCONTEXT":defect==="absolute-building"?"IFCBUILDING":"IFCBUILDINGSTOREY"))!;
 if(defect==="storey-authored-level") {const prop=graph.entities.find(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&row.args[0].value==="Level")!;prop.args[2]={kind:"typedValue",value:{name:"IFCINTEGER",items:[{kind:"integer",value:4n}]}};}
 else if(defect==="context-translation") {const point=lookup(lookup(target.args[4]).args[0]);(point.args[0] as {value:unknown[]}).value[0]=ifcReal(100);}
 else {const placement=lookup(target.args[5]),axis=lookup(placement.args[1]);
  if(defect==="absolute-building")placement.args[0]={kind:"unset"};
  else if(defect==="storey-rotation")axis.args[2]={kind:"reference",value:graph.entities.find(row=>row.name==="IFCDIRECTION")!.id};
  else {const point=lookup(axis.args[0]);(point.args[0] as {value:unknown[]}).value[defect==="site-translation"?0:2]=ifcReal(defect==="site-translation"?100:99);}
 }
 const before=structuredClone(graph),outcome=await ifc4x3ToBimSpatial(graph);expect(outcome.fidelity).toBe("unsupported");expect(outcome.artifact).toBeUndefined();expect(outcome.diagnostics.some(d=>d.severity==="error")).toBe(true);expect(graph).toEqual(before);
});
test.each(["millimetres","wrong-length-dimensions","level-with-length-unit","wrong-length-measure","duplicate-property"])("BIM authored property %s resolves units or refuses atomically",async variant=>{
 const source={schema:"s.bim.model@1",project:{name:"Project",description:"",author:"",organization:"",phase_names:[]},sites:{s:{name:"Site",latitude:0,longitude:0,elevation:0,true_north:0,boundary:[]}},buildings:{b:{site:"s",name:"Building",origin:{x:0,y:0},rotation:0,elevation:0}},storeys:{g:{building:"b",name:"Ground",level:0,height:3,cut_height:1.2}}};
 const graph=(await bimSpatialToIfc4x3(source)).artifact!,id=graph.entities.reduce((max,row)=>row.id>max?row.id:max,0n)+1n;
 graph.entities.push({id,name:"IFCSIUNIT",args:[{kind:"derived"},{kind:"enum",value:variant==="wrong-length-dimensions"?"MASSUNIT":"LENGTHUNIT"},{kind:"enum",value:variant==="wrong-length-dimensions"?"KILO":"MILLI"},{kind:"enum",value:variant==="wrong-length-dimensions"?"GRAM":"METRE"}],complex:[]});
 for(const property of graph.entities.filter(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&["Height","CutHeight"].includes(row.args[0].value))) {
  property.args[3]={kind:"reference",value:id};property.args[2]={kind:"typedValue",value:{name:variant==="wrong-length-measure"?"IFCMASSMEASURE":"IFCLENGTHMEASURE",items:[ifcReal(property.args[0]?.kind==="string"&&property.args[0].value==="Height"?3000:1200)]}};
 }
 if(variant==="level-with-length-unit")graph.entities.find(row=>row.name==="IFCPROPERTYSINGLEVALUE"&&row.args[0]?.kind==="string"&&row.args[0].value==="Level")!.args[3]={kind:"reference",value:id};
 if(variant==="duplicate-property") {const definition=graph.entities.find(row=>row.name==="IFCPROPERTYSET"&&row.args[2]?.kind==="string"&&row.args[2].value==="semio_BimStorey")!,properties=definition.args[4];if(properties?.kind==="aggregate")properties.value.push(structuredClone(properties.value[0]!));}
 const before=structuredClone(graph),outcome=await ifc4x3ToBimSpatial(graph);
 if(variant==="millimetres") {expect(outcome.artifact).toBeDefined();expect(Object.values(outcome.artifact!.storeys!)[0]!.height).toBe(3);expect(Object.values(outcome.artifact!.storeys!)[0]!.cut_height).toBe(1.2);}
 else {expect(outcome.fidelity).toBe("unsupported");expect(outcome.artifact).toBeUndefined();expect(outcome.diagnostics.some(row=>row.severity==="error")).toBe(true);}
 expect(graph).toEqual(before);
});
test.skipIf(!process.env.SEMIO_IFC_ORACLE_OUTPUT)("IfcOpenShell explicit millimetre BIM properties import in metres",async()=>{
 const {parseIfc4x3Step}=await import("../step.ts"),graph=await parseIfc4x3Step(await readFile(`${process.env.SEMIO_IFC_ORACLE_OUTPUT}/bim-mm.ifc`,"utf8")),outcome=await ifc4x3ToBimSpatial(graph);
 expect(outcome.artifact).toBeDefined();const storeys=Object.values(outcome.artifact!.storeys!);expect(storeys.map(row=>row.height)).toEqual([3,3]);expect(storeys[0]!.cut_height).toBe(1.2);
});
