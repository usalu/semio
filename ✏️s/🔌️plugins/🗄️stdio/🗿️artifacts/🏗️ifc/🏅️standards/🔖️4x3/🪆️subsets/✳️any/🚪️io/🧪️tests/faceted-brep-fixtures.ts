/** 🧱️ Authored geometry, unit, pose and orientation data produce real declared faceted topology. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
interface Prism {outer:number[][];holes:number[][][];base:number;height:number}
export interface IfcFacetedCase {
 id:string;expected:string;volume?:number;bounds?:number[][];
 geometry:Prism&{voids:(Prism&{reverse:boolean})[]};
 unit:{prefix:string|null;coordinateScale:number;siScale:number};placement:{origin:number[];rotation:number};orientation:{reversePolygon:boolean;bound:boolean};
 modifier:{kind:string;ordinal?:number;coordinates?:number[];dimensions?:number;order?:number[]};
}
/** 🏗️ Preserve source point sharing and exact bound flags without fixture-ID geometry branches. */
export function ifcFacetedFixture(row:IfcFacetedCase){
 const graph=createIfc4x3ExchangeGraph(row.id),{snapshot,add,context,worldCoordinateSystem}=graph;
 const unset:IfcValue={kind:'unset'},a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),s=(value:string):IfcValue=>({kind:'string',value}),e=(value:string):IfcValue=>({kind:'enum',value}),ref=(entity:IfcEntity):IfcValue=>({kind:'reference',value:entity.id});
 const put=(name:string,args:IfcValue[])=>{add(name,args);return snapshot.entities.at(-1)!;};
 const points:IfcEntity[]=[],loops:IfcEntity[]=[],faces:IfcEntity[]=[],cache=new Map<string,IfcEntity>();
 const point=(coordinates:number[])=>{const values=coordinates.map(n=>n*row.unit.coordinateScale),key=values.join(',');let entity=cache.get(key);if(!entity){entity=put('IFCCARTESIANPOINT',[a(values.map(ifcReal))]);cache.set(key,entity);points.push(entity);}return entity;};
 const face=(rings:number[][][],reverse:boolean)=>{const bounds=rings.map((ring,index)=>{const ordered=row.orientation.reversePolygon!==reverse?ring.toReversed():ring,loop=put('IFCPOLYLOOP',[a(ordered.map(p=>ref(point(p))))]);loops.push(loop);return ref(put(index===0&&row.modifier.kind!=='no-outer'?'IFCFACEOUTERBOUND':'IFCFACEBOUND',[ref(loop),e(row.orientation.bound?'T':'F')]));});if(row.modifier.kind==='outer-last'&&bounds.length>1)bounds.push(bounds.shift()!);const entity=put('IFCFACE',[a(bounds)]);faces.push(entity);return ref(entity);};
 const prism=(shape:Prism,reverse=false)=>{const rings=[shape.outer,...shape.holes],bottom=(r:number[][])=>r.map(p=>[p[0]!,p[1]!,shape.base]),top=(r:number[][])=>r.map(p=>[p[0]!,p[1]!,shape.base+shape.height]),members=[face(rings.map(r=>bottom(r).toReversed()),reverse),face(rings.map(top),reverse)];for(const ring of rings)for(let i=0;i<ring.length;i++){const p=ring[i]!,q=ring[(i+1)%ring.length]!;members.push(face([[[p[0]!,p[1]!,shape.base],[q[0]!,q[1]!,shape.base],[q[0]!,q[1]!,shape.base+shape.height],[p[0]!,p[1]!,shape.base+shape.height]]],reverse));}return ref(put('IFCCLOSEDSHELL',[a(members)]));};
 if(row.unit.prefix!==null)snapshot.entities.find(entity=>entity.name==='IFCSIUNIT')!.args[2]=e(row.unit.prefix);
 const outer=prism(row.geometry),voids=row.geometry.voids.map(shape=>prism(shape,shape.reverse)),solid=put(voids.length?'IFCFACETEDBREPWITHVOIDS':'IFCFACETEDBREP',[outer,...(voids.length?[a(voids)]:[])]),modifier=row.modifier;
 if(modifier.kind==='point-coordinates'){const entity=points[modifier.ordinal!]!,old=entity.args[0] as Extract<IfcValue,{kind:'aggregate'}>;entity.args[0]=a(modifier.coordinates?modifier.coordinates.map(ifcReal):old.value.slice(0,modifier.dimensions));}
 if(modifier.kind==='polygon-order'){const entity=loops[modifier.ordinal!]!,old=entity.args[0] as Extract<IfcValue,{kind:'aggregate'}>;entity.args[0]=a(modifier.order!.map(i=>old.value[i]!));}
 if(modifier.kind==='two-outer'){const values=faces[0]!.args[0] as Extract<IfcValue,{kind:'aggregate'}>;values.value.push(ref(put('IFCFACEOUTERBOUND',[ref(loops[0]!),e('T')])));}
 if(modifier.kind==='empty-bounds')faces[0]!.args[0]=a([]);
 if(modifier.kind==='wrong-bound')faces[0]!.args[0]=a([ref(put('IFCPLANE',[worldCoordinateSystem]))]);
 if(modifier.kind==='wrong-loop'){const value=(faces[0]!.args[0] as Extract<IfcValue,{kind:'aggregate'}>).value[0];if(value?.kind!=='reference')throw Error('Authored bound reference required');const bound=graph.snapshot.entities.find(entity=>entity.id===value.value)!;bound.args[0]={kind:'reference',value:1n};}
 if(modifier.kind==='wrong-point'){const values=loops[0]!.args[0] as Extract<IfcValue,{kind:'aggregate'}>;values.value[0]=ref(put('IFCDIRECTION',[a([ifcReal(0),ifcReal(0),ifcReal(1)])]));}
 const representation=put('IFCSHAPEREPRESENTATION',[context,s('Body'),s('Brep'),a([ref(solid)])]),shape=put('IFCPRODUCTDEFINITIONSHAPE',[unset,unset,a([ref(representation)])]);
 const axis=put('IFCAXIS2PLACEMENT3D',[ref(put('IFCCARTESIANPOINT',[a(row.placement.origin.map(ifcReal))])),ref(put('IFCDIRECTION',[a([0,0,1].map(ifcReal))])),ref(put('IFCDIRECTION',[a([Math.cos(row.placement.rotation),Math.sin(row.placement.rotation),0].map(ifcReal))]))]),placement=put('IFCLOCALPLACEMENT',[unset,ref(axis)]),product=put('IFCBUILDINGELEMENTPROXY',[s('0000000000000000000002'),unset,s(row.id),unset,unset,ref(placement),ref(shape),unset,unset]);
 return{...graph,solid,product,points,loops,faces,representation};
}
