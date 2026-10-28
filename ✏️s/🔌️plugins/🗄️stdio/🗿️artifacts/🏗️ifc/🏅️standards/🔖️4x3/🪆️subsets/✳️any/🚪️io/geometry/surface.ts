/** 🪟️ Planar surface groups share source faces while retaining their distinct topology contracts. */
import type {IfcEntity} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {validateSolidMeshSteps,validateSurfaceMeshSteps,type SolidTriangle} from '../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcAggregate,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcMatrixOrientation} from '../orientation.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
import {IfcPlanarFaceWork} from './faces.ts';
function reject(reason:string):never{throw new IfcStepError('geometry.surface',reason);}
interface RingDomain{ring:number[];start:number;direction:number;key:string}
interface FaceDomain{outer:RingDomain;holes:Map<string,RingDomain[]>;count:number}
function* ringDomainSteps(ring:number[]):IfcIoSteps<RingDomain>{let start=0;for(let at=1;at<ring.length;at++){yield;if(ring[at]!<ring[start]!)start=at;}const direction=ring[(start+1)%ring.length]!<ring[(start+ring.length-1)%ring.length]!?1:-1;let a=2166136261,b=0;for(let at=0;at<ring.length;at++){yield;const id=ring[(start+direction*at+ring.length)%ring.length]!;a=Math.imul(a^id,16777619)>>>0;b=(b+Math.imul(id+1,0x9e3779b1))>>>0;}return{ring,start,direction,key:ring.length+':'+a+':'+b};}
function* sameRingSteps(a:RingDomain,b:RingDomain):IfcIoSteps<boolean>{if(a.ring.length!==b.ring.length)return false;const n=a.ring.length;for(let at=0;at<n;at++){yield;if(a.ring[(a.start+a.direction*at+n)%n]!==b.ring[(b.start+b.direction*at+n)%n])return false;}return true;}
function* faceDomainSteps(rings:number[][]):IfcIoSteps<FaceDomain>{const outer=yield* ringDomainSteps(rings[0]!),holes=new Map<string,RingDomain[]>();for(let at=1;at<rings.length;at++){yield;const domain=yield* ringDomainSteps(rings[at]!),bucket=holes.get(domain.key)??[];bucket.push(domain);holes.set(domain.key,bucket);}return{outer,holes,count:rings.length-1};}
function* sameFaceSteps(a:FaceDomain,b:FaceDomain):IfcIoSteps<boolean>{if(a.count!==b.count||!(yield* sameRingSteps(a.outer,b.outer)))return false;const used=new Set<RingDomain>();for(const bucket of a.holes.values())for(const ring of bucket){yield;let found=false;for(const other of b.holes.get(ring.key)??[]){yield;if(!used.has(other)&&(yield* sameRingSteps(ring,other))){used.add(other);found=true;break;}}if(!found)return false;}return true;}
function* rootSteps(parents:number[],id:number):IfcIoSteps<number>{while(parents[id]!==id){yield;id=parents[id]!;}return id;}
function* openShellSteps(triangles:SolidTriangle[]):IfcIoSteps<void>{
 const edges=new Map<string,{count:number;direction:number}>(),links=new Map<number,Map<number,number[]>>();for(const ids of triangles){yield;for(let at=0;at<3;at++){yield;const a=ids[at]!,b=ids[(at+1)%3]!,c=ids[(at+2)%3]!,key=a<b?a+':'+b:b+':'+a,edge=edges.get(key)??{count:0,direction:0};edge.count++;edge.direction+=a<b?1:-1;edges.set(key,edge);const link=links.get(a)??new Map<number,number[]>(),forward=link.get(b)??[],back=link.get(c)??[];forward.push(c);back.push(b);link.set(b,forward);link.set(c,back);links.set(a,link);}}
 let boundary=false;for(const edge of edges.values()){yield;if(edge.count===1)boundary=true;else if(edge.count!==2||edge.direction!==0)reject('OpenShell requires oriented manifold edges');}if(!boundary)reject('OpenShell requires a nonempty boundary');
 for(const link of links.values()){yield;let ends=0;for(const neighbors of link.values()){yield;if(neighbors.length===1)ends++;else if(neighbors.length!==2)reject('OpenShell vertex link must be a path or cycle');}if(ends!==0&&ends!==2)reject('OpenShell boundary vertex link requires two ends');const seen=new Set<number>(),todo=[link.keys().next().value!];while(todo.length){yield;const id=todo.pop()!;if(seen.has(id))continue;seen.add(id);for(const next of link.get(id)!){yield;if(!seen.has(next))todo.push(next);}}if(seen.size!==link.size)reject('OpenShell forbids pinched vertex links');}
}

/** 🧩️ Connected domains, actual shell subtypes and shared-face occurrences retain original progress and grants. */
export function* ifcFaceBasedSurfaceSteps(graph:IfcGeometryGraph,model:IfcEntity,world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 yield;exact(model,['IFCFACEBASEDSURFACEMODEL']);if(!(scale>0)||!Number.isFinite(scale)||!ifcMatrixOrientation(world))reject('Finite positive SI scale and nonsingular world frame required');const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name),members=ifcAggregate(yield* field(model,'IFCFACEBASEDSURFACEMODEL','FbsmFaces'));if(!members.length)reject('A surface model requires connected face sets');
 const work=new IfcPlanarFaceWork(graph,maxValues),triangles:SolidTriangle[]=[],validation:SolidTriangle[]=[],groups:{kind:string;triangles:SolidTriangle[]}[]=[],seenGroups=new Set<bigint>(),domains=new Map<string,FaceDomain[]>();
 for(const value of members){
  yield;const group=graph.ref(value);exact(group,['IFCCONNECTEDFACESET','IFCOPENSHELL','IFCCLOSEDSHELL']);if(seenGroups.has(group.id))reject('FbsmFaces requires UNIQUE source references');seenGroups.add(group.id);const faces=ifcAggregate(yield* field(group,'IFCCONNECTEDFACESET','CfsFaces'));if(!faces.length)reject('A connected face set requires faces');const seenFaces=new Set<bigint>(),parents:number[]=[],ranks:number[]=[],firstVertex=new Map<number,number>(),groupTriangles:SolidTriangle[]=[];
  for(let at=0;at<faces.length;at++){
   yield;const face=graph.ref(faces[at]);if(seenFaces.has(face.id))reject('CfsFaces requires UNIQUE source references');seenFaces.add(face.id);parents.push(at);ranks.push(0);const admitted=yield* work.faceSteps(face);
   for(const ring of admitted.rings)for(const id of ring){yield;const first=firstVertex.get(id);if(first===undefined)firstVertex.set(id,at);else{let a=yield* rootSteps(parents,at),b=yield* rootSteps(parents,first);if(a!==b){if(ranks[a]!<ranks[b]!)[a,b]=[b,a];parents[b]=a;if(ranks[a]===ranks[b])ranks[a]++;}}}
   const domain=yield* faceDomainSteps(admitted.rings),bucket=domains.get(domain.outer.key)??[];let duplicate=false;for(const existing of bucket){yield;if(yield* sameFaceSteps(domain,existing)){duplicate=true;break;}}if(!duplicate){bucket.push(domain);domains.set(domain.outer.key,bucket);for(const triangle of admitted.triangles){yield;validation.push(triangle);}}
   for(const triangle of admitted.triangles){yield;triangles.push(triangle);groupTriangles.push(triangle);}
  }
  const first=yield* rootSteps(parents,0);for(let at=1;at<parents.length;at++){yield;if((yield* rootSteps(parents,at))!==first)reject('Each ConnectedFaceSet must be arcwise connected through common vertices or edges');}groups.push({kind:group.name,triangles:groupTriangles});
 }
 yield* work.normalizeSteps();yield* validateSurfaceMeshSteps({vertices:work.positions,triangles:validation});for(const group of groups){yield;if(group.kind==='IFCCLOSEDSHELL')yield* validateSolidMeshSteps({vertices:work.positions,triangles:group.triangles});else if(group.kind==='IFCOPENSHELL')yield* openShellSteps(group.triangles);}
 yield* work.restoreSteps(world,scale);const indices:number[]=[];for(const triangle of triangles){yield;indices.push(triangle[0],triangle[1],triangle[2]);}yield;return{positions:work.positions,indices};
}
