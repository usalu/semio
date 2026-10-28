/** 🧱️ Declared planar faces share original source topology and cooperative solid admission. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {validateSolidMeshSteps,type SolidTriangle} from '../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcAggregate,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcMatrixOrientation} from '../orientation.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
import {IfcPlanarFaceWork} from './faces.ts';
function reject(reason:string):never{throw new IfcStepError('geometry.facets',reason);}
const edgeKey=(a:number,b:number)=>a<b?a+':'+b:b+':'+a;

/** 📐️ Complete exact faceted shells retain bound direction, inner surfaces and declared source owners. */
export function* ifcFacetedBrepSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 yield;const orientation=ifcMatrixOrientation(world);if(!orientation||!(scale>0)||!Number.isFinite(scale)||!Number.isSafeInteger(maxValues)||maxValues<0)reject('Finite nonsingular placement, SI scale and original value grant required');
 const work=new IfcPlanarFaceWork(graph,maxValues),positions=work.positions,triangles:SolidTriangle[]=[],ranges:{start:number;end:number;inner:boolean}[]=[];
 const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name);
 const withVoids=solid.name==='IFCFACETEDBREPWITHVOIDS';exact(solid,['IFCFACETEDBREP','IFCFACETEDBREPWITHVOIDS']);
 const outer=yield* field(solid,'IFCMANIFOLDSOLIDBREP','Outer'),voids=withVoids?ifcAggregate(yield* field(solid,'IFCFACETEDBREPWITHVOIDS','Voids')):[];if(withVoids&&!voids.length)reject('A voided BRep requires at least one inner shell');
 const shellIds=new Set<bigint>();
 for(let ordinal=0;ordinal<=voids.length;ordinal++){
  yield;const shell=graph.ref(ordinal?voids[ordinal-1]:outer);exact(shell,['IFCCLOSEDSHELL']);if(shellIds.has(shell.id))reject('Outer and inner shells must be distinct');shellIds.add(shell.id);
  const faces=ifcAggregate(yield* field(shell,'IFCCONNECTEDFACESET','CfsFaces')),faceIds=new Set<bigint>(),edges=new Map<string,{count:number;direction:number;faces:number[]}>(),vertices=new Map<number,Set<number>>(),adjacency:number[][]=[],start=triangles.length;if(!faces.length)reject('A closed shell requires faces');
  for(let faceOrdinal=0;faceOrdinal<faces.length;faceOrdinal++){
   yield;const face=graph.ref(faces[faceOrdinal]);exact(face,['IFCFACE']);if(faceIds.has(face.id))reject('A shell face may occur only once');faceIds.add(face.id);adjacency.push([]);
   const admitted=yield* work.faceSteps(face);for(const ring of admitted.rings){yield;for(const id of ring){yield;const incident=vertices.get(id)??new Set<number>();incident.add(faceOrdinal);vertices.set(id,incident);}for(let at=0;at<ring.length;at++){yield;const a=ring[at]!,b=ring[(at+1)%ring.length]!,key=edgeKey(a,b),edge=edges.get(key)??{count:0,direction:0,faces:[]};edge.count++;edge.direction+=a<b?1:-1;edge.faces.push(faceOrdinal);edges.set(key,edge);}}
   for(const triangle of admitted.triangles){yield;triangles.push(triangle);}
  }
  for(const edge of edges.values()){yield;if(edge.count!==2||edge.direction!==0||edge.faces[0]===edge.faces[1])reject('Every shell edge requires two opposing distinct faces');adjacency[edge.faces[0]!]!.push(edge.faces[1]!);adjacency[edge.faces[1]!]!.push(edge.faces[0]!);}
  for(const incident of vertices.values()){yield;if(incident.size<3)reject('A shell vertex must meet at least three distinct faces');}
  const seen=new Set<number>(),todo=[0];while(todo.length){yield;const id=todo.pop()!;if(seen.has(id))continue;seen.add(id);for(const next of adjacency[id]!){yield;if(!seen.has(next))todo.push(next);}}if(seen.size!==faces.length)reject('Every declared ClosedShell must be connected');ranges.push({start,end:triangles.length,inner:ordinal>0});
 }
 yield* work.normalizeSteps();
 for(const range of ranges){yield;let volume=0;for(let at=range.start;at<range.end;at++){yield;const ids=triangles[at]!,a=positions[ids[0]]!,b=positions[ids[1]]!,c=positions[ids[2]]!;volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;}if(!Number.isFinite(volume)||(range.inner?volume>=0:volume<=0))reject('Outer shell must face outward and void shells inward');}
 yield* validateSolidMeshSteps({vertices:positions,triangles});
 yield* work.restoreSteps(world,scale);
 const indices:number[]=[];for(const[a,b,c]of triangles){yield;indices.push(a,b,c);}yield;return{positions,indices};
}
