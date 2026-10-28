/** 🛤️ Genuine declaration roles isolate direct correspondence without relying on independent list ordinals. */
import type {IfcValue,IfcEntity} from '../../../📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../../../../🚪️io/gltf.ts';
import {ifcReal} from '../../../../🚪️io/step.ts';
export interface IfcDirectFixtureCase {id:string;kind?:string;applicable?:boolean}
/** 🚆️ Construct exact inherited slots from the released first-party catalog; malformed cases remain explicit laws. */
export function ifcAlignmentDirectFixture(row:IfcDirectFixtureCase){
 const graph=createIfc4x3ExchangeGraph(row.id),unset:IfcValue={kind:'unset'},a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),s=(value:string):IfcValue=>({kind:'string',value}),e=(value:string):IfcValue=>({kind:'enum',value}),typed=(name:string,value:IfcValue):IfcValue=>({kind:'typedValue',value:{name,items:[value]}});
 const add=(name:string,values:Record<string,IfcValue>)=>graph.add(name,ifc4x3Catalog.entities[name]!.allAttributes.map(attribute=>values[attribute.name]??unset)),point=(values:number[])=>graph.add('IFCCARTESIANPOINT',[a(values.map(ifcReal))]),direction=(values:number[])=>graph.add('IFCDIRECTION',[a(values.map(ifcReal))]);
 const axis=add('IFCAXIS2PLACEMENT3D',{Location:point([0,0,0])}),placement=add('IFCLOCALPLACEMENT',{RelativePlacement:axis}),vector=add('IFCVECTOR',{Orientation:direction([1,0,0]),Magnitude:ifcReal(1)}),line=add('IFCLINE',{Pnt:point([0,0,0]),Dir:vector});
 const parameters=add('IFCALIGNMENTHORIZONTALSEGMENT',{StartPoint:point([0,0]),StartDirection:ifcReal(0),StartRadiusOfCurvature:ifcReal(0),EndRadiusOfCurvature:ifcReal(0),SegmentLength:ifcReal(0),PredefinedType:e('LINE')});
 const curve=add('IFCCURVESEGMENT',{Transition:e('DISCONTINUOUS'),Placement:axis,SegmentStart:typed('IFCLENGTHMEASURE',ifcReal(0)),SegmentLength:typed('IFCLENGTHMEASURE',ifcReal(0)),ParentCurve:line});
 add('IFCCOMPOSITECURVE',{Segments:a([curve]),SelfIntersect:e('F')});
 const representation=add('IFCSHAPEREPRESENTATION',{ContextOfItems:graph.context,RepresentationIdentifier:s('Axis'),RepresentationType:s('Segment'),Items:a([curve])}),definition=add('IFCPRODUCTDEFINITIONSHAPE',{Representations:a([representation])}),segment=add('IFCALIGNMENTSEGMENT',{GlobalId:s('0000000000000000000002'),ObjectPlacement:placement,Representation:definition,DesignParameters:parameters});
 const entity=(value:IfcValue)=>graph.snapshot.entities.find(entity=>value.kind==='reference'&&entity.id===value.value)!,set=(entity:IfcEntity,name:string,value:IfcValue)=>{entity.args[ifc4x3Catalog.entities[entity.name]!.allAttributes.findIndex(attribute=>attribute.name===name)]=value;};
 const length=(value:IfcValue)=>set(entity(curve),'SegmentLength',value),zero=ifcReal(0);
 if(row.id==='zero-parameter')length(typed('IFCPARAMETERVALUE',zero));
 if(row.id==='integer-zero')length(typed('IFCLENGTHMEASURE',{kind:'integer',value:0n}));
 if(row.id==='negative-zero')length(typed('IFCLENGTHMEASURE',ifcReal(-0)));
 if(row.id==='mixed-case-tag')length(typed('iFcLeNgThMeAsUrE',zero));
 if(row.id==='not-applicable')graph.snapshot.entities.splice(graph.snapshot.entities.indexOf(entity(segment)),1);
 if(row.id==='wrong-segment')entity(segment).name='IFCBUILDINGELEMENTPROXY';
 if(row.id==='business-nonzero'){set(entity(parameters),'SegmentLength',ifcReal(1));set(entity(segment),'Representation',unset);}
 if(row.id==='curve-nonzero-decoy'){length(typed('IFCLENGTHMEASURE',ifcReal(2)));graph.add('IFCCURVESEGMENT',structuredClone(entity(curve).args).map((value,index)=>index===3?typed('IFCLENGTHMEASURE',zero):value));}
 if(row.id==='missing-representation'||row.id==='parent-only')set(entity(segment),'Representation',unset);
 if(row.id==='missing-curve')set(entity(representation),'Items',a([{kind:'reference',value:99999n}]));
 if(row.id==='two-items')set(entity(representation),'Items',a([curve,graph.add('IFCCURVESEGMENT',structuredClone(entity(curve).args))]));
 if(row.id==='repeated-curve')set(entity(representation),'Items',a([curve,curve]));
 if(row.id==='two-representations')set(entity(definition),'Representations',a([representation,graph.add('IFCSHAPEREPRESENTATION',structuredClone(entity(representation).args))]));
 if(row.id==='raw-select')length(zero);
 if(row.id==='unknown-select')length(typed('IFCPOSITIVELENGTHMEASURE',zero));
 if(row.id==='select-arity')length({kind:'typedValue',value:{name:'IFCLENGTHMEASURE',items:[zero,zero]}});
 if(row.id==='nonfinite-select')length(typed('IFCPARAMETERVALUE',{kind:'real',value:{bits:0x7ff0000000000000n}}));
 if(row.id==='unknown-constituent')entity(segment).complex=[{name:'UNKNOWN',args:[]}];
 if(row.id==='ambiguous-owner'){const parametersRow=entity(parameters);parametersRow.complex=[{name:parametersRow.name,args:structuredClone(parametersRow.args)}];}
 if(row.id==='duplicate-id')graph.snapshot.entities.push(structuredClone(entity(curve)));
 return{...graph,segment,curve,parameters,definition,representation};
}
