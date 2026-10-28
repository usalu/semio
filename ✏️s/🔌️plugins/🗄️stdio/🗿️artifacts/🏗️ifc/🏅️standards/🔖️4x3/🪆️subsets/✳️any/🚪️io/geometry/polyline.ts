/** 🪢️ Closed planar polyline area sweeps share checked ring topology and original continuations. */
import type {IfcEntity} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {ifcNumber} from '../step.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcIdentity,ifcTransform,ifcMultiply,ifcAggregate,ifcNormalizeDirection,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {triangulateRingsSteps,type RingPoint} from '../../../../../../📐️geometry/🕳️rings/🟦️.ts';

/** 🕳️ Profile holes are explicit curve rings; relational openings remain a separate solid operation. */
export function* ifcPolylineExtrusionSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 yield;const profile=graph.ref(solid.args[0]),voids=profile.name==='IFCARBITRARYPROFILEDEFWITHVOIDS';
 if(solid.name!=='IFCEXTRUDEDAREASOLID'||solid.args.length!==4||solid.complex?.length||(!voids&&profile.name!=='IFCARBITRARYCLOSEDPROFILEDEF')||profile.args.length!==(voids?4:3)||profile.complex?.length||profile.args[0]?.kind!=='enum'||profile.args[0].value!=='AREA')throw new IfcStepError('geometry.profile','Declared closed polyline AREA profile required');
 const inner=voids?ifcAggregate(profile.args[3]):[];if(voids&&!inner.length)throw new IfcStepError('geometry.profile','A voided profile requires inner curves');
 const points:RingPoint[]=[],rings:number[][]=[];
 for(let ordinal=0;ordinal<=inner.length;ordinal++){
  yield;const curve=graph.ref(ordinal?inner[ordinal-1]:profile.args[2]);if(curve.name!=='IFCPOLYLINE'||curve.args.length!==1||curve.complex?.length)throw new IfcStepError('geometry.profile-curve','An explicit closed two-dimensional polyline is required');
  const references=ifcAggregate(curve.args[0]),ring:number[]=[];if(references.length<4)throw new IfcStepError('geometry.profile-curve','A closed profile requires at least three distinct vertices');
  for(const value of references){yield;const point=graph.ref(value),coordinates=ifcAggregate(point.args[0]);if(point.name!=='IFCCARTESIANPOINT'||point.args.length!==1||point.complex?.length||coordinates.length!==2)throw new IfcStepError('geometry.profile-dimension','Profile polyline points must have exactly two coordinates');ring.push(points.length);points.push([ifcNumber(coordinates[0]),ifcNumber(coordinates[1]),0]);}
  const first=points[ring[0]!]!,last=points[ring[ring.length-1]!]!;if(first[0]!==last[0]||first[1]!==last[1])throw new IfcStepError('geometry.profile-closure','Profile polyline must be explicitly closed');points.pop();ring.pop();
  let extent=0;for(const id of ring){yield;const point=points[id]!;extent=Math.max(extent,Math.abs(point[0]-first[0]),Math.abs(point[1]-first[1]));}
  if(!Number.isFinite(extent)||extent<=0)throw new IfcStepError('geometry.profile-range','A finite nondegenerate profile is required');
  let area=0;for(let at=0;at<ring.length;at++){yield;const a=points[ring[at]!]!,b=points[ring[(at+1)%ring.length]!]!;area+=(a[0]-first[0])/extent*((b[1]-first[1])/extent)-(b[0]-first[0])/extent*((a[1]-first[1])/extent);}
  if(!Number.isFinite(area)||area===0)throw new IfcStepError('geometry.profile-area','A nonzero finite profile area is required');
  if((area>0)!==(ordinal===0))for(let at=0;at<Math.floor(ring.length/2);at++){yield;const other=ring.length-at-1,value=ring[at]!;ring[at]=ring[other]!;ring[other]=value;}
  rings.push(ring);
 }
 const position=solid.args[1];let solidFrame=ifcIdentity();if(position?.kind!=='unset'){if(graph.ref(position).name!=='IFCAXIS2PLACEMENT3D')throw new IfcStepError('geometry.placement','Solid placement must be three-dimensional');solidFrame=graph.axis(position);}
 const direction=graph.ref(solid.args[2]);if(direction.name!=='IFCDIRECTION'||direction.args.length!==1||ifcAggregate(direction.args[0]).length!==3)throw new IfcStepError('geometry.direction','An explicit three-dimensional extrusion direction is required');
 const vector=ifcNormalizeDirection(graph.vector(solid.args[2])),depth=ifcNumber(solid.args[3]);if(depth<=0||vector[2]===0||!Number.isFinite(scale)||scale<=0)throw new IfcStepError('geometry.extrusion','Positive depth, transverse direction and finite SI scale required');
 const caps=yield* triangulateRingsSteps(points,rings),frame=ifcMultiply(world,solidFrame),positions:IfcTriangleMesh['positions']=[],indices:number[]=[],count=points.length;
 for(let layer=0;layer<2;layer++)for(const source of points){yield;const local=source.map((n,k)=>n+layer*depth*vector[k]!) as RingPoint,point=ifcTransform(frame,local).map(n=>n*scale) as RingPoint;if(point.some(n=>!Number.isFinite(n)))throw new IfcStepError('geometry.range','Sweep coordinates exceed finite SI range');positions.push(point);}
 const positive=vector[2]>0;
 for(const[a,b,c]of caps){yield;indices.push(a,positive?c:b,positive?b:c,a+count,(positive?b:c)+count,(positive?c:b)+count);}
 for(const ring of rings)for(let at=0;at<ring.length;at++){yield;const a=ring[at]!,b=ring[(at+1)%ring.length]!,faces=positive?[[a,b,b+count],[a,b+count,a+count]]:[[b,a,a+count],[b,a+count,b+count]];for(const face of faces)indices.push(face[0]!,face[1]!,face[2]!);}
 return{positions,indices};
}
