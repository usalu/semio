import {expect,test} from "bun:test";
import Ajv from "ajv";
import loadManifold from "manifold-3d";
import authoredIntersection from "../🧫️fixtures/🛠️modeling/⚗️authored-intersection.json";
import union from "../🧫️fixtures/🛠️modeling/➕️convex-union.json";
import difference from "../🧫️fixtures/🛠️modeling/➖️convex-difference.json";
import prismaticDifference from "../🧫️fixtures/🛠️modeling/🔺️prismatic-difference.json";
import intersection from "../🧫️fixtures/🛠️modeling/⚗️convex-intersection.json";
import intersectionSchema from "../../🧬️schema/🛠️modeling/⚗️convex-intersection.json";
import {BufferGeometry,ExtrudeGeometry,Float32BufferAttribute,Line3,Plane,Shape,Triangle,Vector2,Vector3} from "three";
import polygonCapture from "../🧫️fixtures/🛠️modeling/📇️polygon-capture.json";
import bevelSurface from "../🧫️fixtures/🛠️modeling/🪚️bevel.json";
import selectedWeld from "../🧫️fixtures/🛠️modeling/🧲️selected-weld.json";
import dissolveSeam from "../🧫️fixtures/🛠️modeling/🧵️dissolve-seam.json";
import authoredUnwrap from "../🧫️fixtures/🛠️modeling/🗺️authored-unwrap.json";
import boundaryExtrude from "../🧫️fixtures/🛠️modeling/📏️boundary-extrude.json";
import boundaryExtrudeSchema from "../../🧬️schema/🛠️modeling/📏️boundary-extrude.json";
import bridgeFixture from "../🧫️fixtures/🛠️modeling/🌉️bridge.json";
import bridgeSchema from "../../🧬️schema/🛠️modeling/🌉️bridge.json";
import {mergeVertices} from "three/addons/utils/BufferGeometryUtils.js";
import law from "../🧫️fixtures/🛠️modeling/🎟️owners.json";
import schema from "../../🧬️schema/🛠️modeling/🎟️owners.json";
import cases from "../🧫️fixtures/🛠️modeling/🔣️.json";
import {applyPatch} from "fast-json-patch";

test("original orientation initializes each source face with independently patched empty adjacency and flags",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,orientationInitialization:{...law.orientationInitialization,parentDepth:0}})).toBe(false);
  const policy=law.orientationInitialization;expect(validate({...law,orientationInitialization:{...policy,normalDemandFields:[]}})).toBe(false);
  for(const name of policy.sources){
    const source=law[name as "snapshotPreparation"|"snapshotClosedPreparation"],output={adjacency:[] as number[][],oriented:[] as boolean[],flipped:[] as boolean[]};
    for(const face of source.faces){applyPatch(output,[{op:"add",path:"/adjacency/-",value:[]},{op:"add",path:"/oriented/-",value:false},{op:"add",path:"/flipped/-",value:false}]);expect(face.length).toBe(4);}
    expect(output.adjacency).toEqual(source.faces.map(()=>[]));expect(output.oriented).toEqual(source.faces.map(()=>false));expect(output.flipped).toEqual(source.faces.map(()=>false));expect(new Uint8Array(source.faces.length).byteLength).toBe(source.faces.length*policy.flagBytes);expect(new BigUint64Array(policy.adjacencySlotPointers).byteLength+policy.flagBytes*2).toBe(26);
  }
});

test("original modeling snapshot agrees with independent source and reversed normal authority",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const policy=law.snapshotPreparation,geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(policy.positions.flat(),3));
  const original=geometry.getAttribute("position").array;
  geometry.setIndex([0,1,2,0,2,3]);geometry.computeVertexNormals();
  expect(Array.from(geometry.getAttribute("normal").array.slice(0,3))).toEqual(policy.normal);
  geometry.setIndex([3,2,0,2,1,0]);geometry.computeVertexNormals();
  expect(Array.from(geometry.getAttribute("normal").array.slice(0,3))).toEqual(policy.flippedNormal);
  expect(geometry.getAttribute("position").array).toBe(original);
  expect(Array.from(original)).toEqual(policy.positions.flat());
  expect(Float32Array.BYTES_PER_ELEMENT*3).toBe(policy.sourceBytesCopiedPerPosition);
  expect(Uint32Array.BYTES_PER_ELEMENT).toBe(policy.sourceBytesCopiedPerBoundary);
  expect(Uint32Array.BYTES_PER_ELEMENT*2).toBe(policy.copiedBytesPerSwap);
  const sumBytes=new Float64Array(3).byteLength,pointBytes=new Float32Array(3).byteLength;
  expect(sumBytes+pointBytes).toBe(policy.normalFirstCopyBytes);
  expect(sumBytes).toBe(policy.normalMiddleCopyBytes);
  expect(sumBytes+pointBytes+sumBytes+pointBytes).toBe(policy.normalLastCopyBytes);
  const corners=policy.faces[0].length,forwardBytes=original.byteLength+corners*Uint32Array.BYTES_PER_ELEMENT+corners*sumBytes+pointBytes+pointBytes+sumBytes+pointBytes;
  expect(forwardBytes).toBe(policy.forwardCopyBytes);
  expect(forwardBytes+Math.floor(corners/2)*policy.copiedBytesPerSwap).toBe(policy.flippedCopyBytes);
  expect(policy.forwardAtoms.reduce((sum,atom)=>sum+atom,0)).toBe(forwardBytes);
  geometry.dispose();
});

test("every original modeling caller projects the same independent source",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  expect(law.snapshotCallers.operations).toEqual(cases.ownedCapture.operations.filter(operation=>!["translate","rotate","scale"].includes(operation)));
  for(const operation of law.snapshotCallers.operations){
    const policy=law.snapshotCallers.closedSourceOperations.includes(operation)?law.snapshotClosedPreparation:law.snapshotPreparation;
    const geometry=new BufferGeometry();geometry.setAttribute("position",new Float32BufferAttribute(policy.positions.flat(),3));const original=geometry.getAttribute("position").array;
    const normals:number[][]=[],directed=new Set<string>();let volume=0;
    for(const face of policy.faces){
      const indices=face.slice(1,-1).flatMap((_,index)=>[face[0],face[index+1],face[index+2]]);geometry.setIndex(indices);geometry.computeVertexNormals();normals.push(Array.from(geometry.getAttribute("normal").array.slice(face[0]*3,face[0]*3+3)));
      for(let index=0;index<face.length;index++)directed.add(`${face[index]}:${face[(index+1)%face.length]}`);
      for(let index=0;index<indices.length;index+=3){const [a,b,c]=indices.slice(index,index+3).map(vertex=>policy.positions[vertex]);volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;}
    }
    const atoms=[0,0,0,...policy.positions.map(()=>new Float32Array(3).byteLength),...policy.faces.flatMap(face=>[0,...face.map(()=>new Uint32Array(1).byteLength),...face.map((_,index)=>new Float64Array(3).byteLength+(index===0?new Float32Array(3).byteLength:0)+(index+1===face.length?48:0))])];
    expect(Array.from(original)).toEqual(policy.positions.flat());expect(atoms).toEqual(policy.forwardAtoms);expect(atoms.reduce((sum,atom)=>sum+atom,0)).toBe(policy.forwardCopyBytes);
    if("closed" in policy){expect(normals).toEqual(policy.normals);for(const edge of directed){expect(directed.has(edge.split(":").reverse().join(":"))).toBe(true);}expect(volume).toBeCloseTo(policy.volume,12);}
    else expect(normals).toEqual([policy.normal]);
    expect(geometry.getAttribute("position").array).toBe(original);geometry.dispose();
  }
});

test("original translation agrees with independent in-place geometry transformation",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const geometry=new BufferGeometry();geometry.setAttribute("position",new Float32BufferAttribute(law.snapshotPreparation.positions.flat(),3));const original=geometry.getAttribute("position").array;geometry.translate(...law.translation.offset as [number,number,number]);expect(Array.from(original)).toEqual(law.translation.positions.flat());expect(geometry.getAttribute("position").array).toBe(original);expect(Float32Array.BYTES_PER_ELEMENT*3).toBe(law.translation.positionCopyBytes);geometry.dispose();
});

test("original cold translation preserves independent source until the transform starts",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const geometry=new BufferGeometry();geometry.setAttribute("position",new Float32BufferAttribute(law.snapshotPreparation.positions.flat(),3));const original=geometry.getAttribute("position").array;expect(Array.from(original)).toEqual(law.snapshotPreparation.positions.flat());expect(geometry.getAttribute("position").array).toBe(original);geometry.translate(...law.translation.offset as [number,number,number]);expect(Array.from(original)).toEqual(law.translation.positions.flat());expect(geometry.getAttribute("position").array).toBe(original);geometry.dispose();
});


test("original orientation incidence preserves every independently patched edge owner and source direction",()=>{
  const validate=new Ajv({strict:true}).compile(schema),policy=law.orientationIncidence;expect(validate(law)).toBe(true);expect(validate({...law,orientationIncidence:{...policy,parentFrames:0}})).toBe(false);
  for(const name of policy.sources){const source=law[name as "snapshotPreparation"|"snapshotClosedPreparation"],rows=new Map<string,[number,boolean][]>(),keys:number[][]=[];
    source.faces.forEach((face,index)=>face.forEach((a,corner)=>{const b=face[(corner+1)%face.length],key=[Math.min(a,b),Math.max(a,b)],text=JSON.stringify(key);if(!rows.has(text)){rows.set(text,[]);keys.push(key);}const values={owners:rows.get(text)!};applyPatch(values,[{op:"add",path:"/owners/-",value:[index,a<b]}]);}));
    const output=policy.outputs[name as keyof typeof policy.outputs];expect(keys).toEqual(output.keys);expect([...rows].map(([key,owners])=>[JSON.parse(key),owners]).sort((a,b)=>a[0][0]-b[0][0]||a[0][1]-b[0][1])).toEqual(output.owners);expect(new Uint32Array(2).byteLength).toBe(policy.keyCopyBytes);expect(new BigUint64Array(2).byteLength).toBe(policy.ownerCopyBytes);
    if(name==="snapshotClosedPreparation"){for(const owners of rows.values()){expect(owners.length).toBe(2);expect(owners[0][1]).not.toBe(owners[1][1]);}}
  }
});


/** 📐️ Face owner rows of the committed law are `[face, flag]` pairs; the fixture type is a union of row shapes. */
function ownerPairs(value:unknown):readonly(readonly[number,boolean])[]{
  if(!Array.isArray(value))throw Error("Face owners must be an array");
  return value.map(row=>{if(!Array.isArray(row)||row.length!==2||typeof row[0]!=="number"||typeof row[1]!=="boolean")throw Error("Face owner must be [face, flag]");return [row[0],row[1]] as const;});
}
test("original orientation adjacency retains independently patched reciprocal face neighbors in source encounter order",()=>{
  const validate=new Ajv({strict:true}).compile(schema),policy=law.orientationAdjacency;expect(validate(law)).toBe(true);expect(validate({...law,orientationAdjacency:{...policy,neighborCopyBytes:0}})).toBe(false);
  for(const name of policy.sources){const source=law[name as "snapshotPreparation"|"snapshotClosedPreparation"],incidence=law.orientationIncidence.outputs[name as keyof typeof law.orientationIncidence.outputs],rows=new Map(incidence.owners.map(([key,owners])=>[JSON.stringify(key),owners])),output={adjacency:source.faces.map(()=>[] as [number,boolean][])};
    for(const key of incidence.keys){const owners=ownerPairs(rows.get(JSON.stringify(key)));if(owners.length!==2)continue;const[[a,af],[b,bf]]=[owners[0]!,owners[1]!];applyPatch(output,[{op:"add",path:`/adjacency/${a}/-`,value:[b,af===bf]},{op:"add",path:`/adjacency/${b}/-`,value:[a,af===bf]}]);}
    const expectedAt93x38 = policy.outputs[name as keyof typeof policy.outputs]; expect<typeof expectedAt93x38>(output.adjacency).toEqual(expectedAt93x38);expect(new BigUint64Array(2).byteLength).toBe(policy.neighborCopyBytes);for(const[face,neighbors]of output.adjacency.entries()){for(const[neighbor,flip]of neighbors){expect(output.adjacency[neighbor].some(([source,relative])=>source===face&&relative===flip)).toBe(true);}}
  }
});


test("original orientation component traversal preserves independently patched visits and winding parity",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(law)).toBe(true);
 const policy=law.orientationComponents;
 expect(validate({...law,orientationComponents:{...policy,neighborCopyBytes:0}})).toBe(false);
 const stackWrite=new BigUint64Array(1).byteLength;
 const componentWrite=new BigUint64Array(1).byteLength;
 const rootBytes=stackWrite+componentWrite;
 const neighborBytes=rootBytes+new Uint8Array(1).byteLength;
 expect(stackWrite).toBe(policy.stackElementBytes);
 expect(rootBytes).toBe(policy.rootCopyBytes);
 expect(neighborBytes).toBe(policy.neighborCopyBytes);
 for(const name of policy.sources){
  const adjacency=policy.adjacency[name as keyof typeof policy.adjacency];
  const flags={oriented:adjacency.map(()=>false),flipped:adjacency.map(()=>false)};
  const visited=new Set<number>();
  let copied=0;
  for(let root=0;root<adjacency.length;root++){
   if(visited.has(root))continue;
   const pending=[root];
   visited.add(root);
   copied+=rootBytes;
   applyPatch(flags,[{op:"replace",path:`/oriented/${root}`,value:true}]);
   while(pending.length){
    const face=pending.pop()!;
    for(const [neighbor,relative]of adjacency[face]){
     if(visited.has(neighbor as number))continue;
     visited.add(neighbor as number);
     copied+=neighborBytes;
     applyPatch(flags,[{op:"replace",path:`/oriented/${neighbor}`,value:true},{op:"replace",path:`/flipped/${neighbor}`,value:flags.flipped[face]!==relative}]);
     pending.push(neighbor as number);
    }
   }
  }
  const expected=policy.outputs[name as keyof typeof policy.outputs];
  expect(flags.oriented).toEqual(expected.oriented);
  expect(flags.flipped).toEqual(expected.flipped);
  expect(flags.flipped.filter(Boolean).length).toBe(expected.flipCount);
  expect(copied).toBe(expected.copyBytes);
  if(name==="snapshotMisorientedPreparation"){const source=law.snapshotMisorientedPreparation;source.faces.forEach((original,index)=>{const face=flags.flipped[index]?[...original].reverse():original,geometry=new BufferGeometry().setAttribute("position",new Float32BufferAttribute(face.slice(0,3).flatMap(vertex=>source.positions[vertex]),3));geometry.computeVertexNormals();expect(Array.from(geometry.getAttribute("normal").array.slice(0,3)).map(value=>value===0?0:value)).toEqual(policy.correctedNormal);geometry.dispose();});}
 }
});


test("original orientation corners agree with independently patched winding and geometric normals",()=>{
 const validate=new Ajv({strict:true}).compile(schema),policy=law.orientationCorners;expect(validate(law)).toBe(true);expect(validate({...law,orientationCorners:{...policy,swapCopyBytes:0}})).toBe(false);expect(new Uint32Array(2).byteLength).toBe(policy.swapCopyBytes);
 for(const name of policy.sources){const source=law[name as keyof typeof law] as typeof law.snapshotPreparation,flags=law.orientationComponents.outputs[name as keyof typeof law.orientationComponents.outputs].flipped,output={faces:source.faces.map(face=>[...face])};let copied=0;
  output.faces.forEach((face,index)=>{if(!flags[index])return;for(let corner=0;corner<Math.floor(face.length/2);corner++){const opposite=face.length-1-corner,a=face[corner],b=face[opposite];applyPatch(output,[{op:"replace",path:`/faces/${index}/${corner}`,value:b},{op:"replace",path:`/faces/${index}/${opposite}`,value:a}]);copied+=new Uint32Array(2).byteLength;}});
  expect(output).toEqual({faces:policy.outputs[name as keyof typeof policy.outputs].faces});expect(copied).toBe(policy.outputs[name as keyof typeof policy.outputs].copyBytes);
  if(name==="snapshotMisorientedPreparation"){for(const face of output.faces){const geometry=new BufferGeometry().setAttribute("position",new Float32BufferAttribute(face.slice(0,3).flatMap(vertex=>source.positions[vertex]),3));geometry.computeVertexNormals();expect(Array.from(geometry.getAttribute("normal").array.slice(0,3)).map(value=>value===0?0:value)).toEqual(law.orientationComponents.correctedNormal);geometry.dispose();}}
 }
});


test("original reconstruction membership agrees with independent ordered source sets",()=>{
 const validate=new Ajv({strict:true}).compile(schema),policy=law.reconstructionMembership;expect(validate(law)).toBe(true);expect(validate({...law,reconstructionMembership:{...policy,parentFrames:0}})).toBe(false);
 for(const name of policy.sources){const source=law[name as keyof typeof law] as typeof law.snapshotPreparation,original=new BufferGeometry().setAttribute("position",new Float32BufferAttribute(source.faces.flat().flatMap(vertex=>source.positions[vertex]),3)),merged=mergeVertices(original),positions=merged.getAttribute("position"),members=Array.from({length:positions.count},(_,index)=>source.positions.findIndex(point=>point[0]===positions.getX(index)&&point[1]===positions.getY(index)&&point[2]===positions.getZ(index))).sort((a,b)=>a-b);expect(members).toEqual(policy.members[name as keyof typeof policy.members]);expect([...new Set(source.faces.flat())].sort((a,b)=>a-b)).toEqual(members);merged.dispose();original.dispose();}
});

test("independent bevel plane preserves and interpolates authored source samples",()=>{
  const {positions,faces,edge,amount,smoothFace,uvs,vertexNormals,retained,intersection,volume}=bevelSurface;
  const [a,b]=edge;
  const face=faces[smoothFace];
  const point=(id:number)=>new Vector3().fromArray(positions[id]);
  const normal=(id:number)=>new Vector3().fromArray(vertexNormals[id]);
  const uv=(id:number)=>new Vector2().fromArray(uvs[face.indexOf(id)]);
  const cutPlane=new Plane(new Vector3(0,1,1).normalize(),-amount/Math.SQRT2);
  const inside=face[(face.indexOf(a)+1)%face.length];
  const edgeLine=new Line3(point(a),point(inside));
  const crossing=cutPlane.intersectLine(edgeLine,new Vector3())!;
  const t=edgeLine.closestPointToPointParameter(crossing,true);
  crossing.toArray().forEach((value,index)=>expect(value).toBeCloseTo(intersection.position[index],12));
  uv(a).lerp(uv(inside),t).toArray().forEach((value,index)=>expect(value).toBeCloseTo(intersection.uv[index],12));
  normal(a).lerp(normal(inside),t).normalize().toArray().forEach((value,index)=>expect(value).toBeCloseTo(intersection.normal[index],12));
  expect(point(inside).toArray()).toEqual(retained.position);
  expect(uv(inside).toArray()).toEqual(retained.uv);
  expect(normal(inside).toArray()).toEqual(retained.normal);
  expect(1-point(a).distanceTo(point(b))*amount*amount/2).toBeCloseTo(volume,12);
});

test("independent selected weld retains the ordered positional representatives and surface samples",()=>{
 const {positions,faces,selection,threshold,normals,uvs,pointValues,expected}=selectedWeld;
 const order=[...selection,...positions.map((_,id)=>id).filter(id=>!selection.includes(id))];
 const geometry=new BufferGeometry().setAttribute("position",new Float32BufferAttribute(order.flatMap(id=>positions[id]),3));
 geometry.setIndex([...faces].reverse().flatMap(face=>[0,1,2,0,2,3].map(index=>order.indexOf(face[index]))));
 const welded=mergeVertices(geometry,threshold);
 const points=welded.getAttribute("position");
 expect(points.count).toBe(expected.vertexCount);
 const index=Array.from(welded.index!.array);
 const position=(id:number)=>new Vector3().fromBufferAttribute(points,id);
 let area=0;
 for(let i=0;i<index.length;i+=3){
  const [a,b,c]=index.slice(i,i+3).map(position);
  area+=b.sub(a).cross(c.sub(a)).length()/2;
 }
 expect(area).toBeCloseTo(expected.area,12);
 const corners=faces.flat();
 const vector=(id:number)=>new Vector3().fromArray(positions[id]);
 for(const sample of expected.points){
  const point=new Vector3().fromArray(sample.position);
  expect(Array.from({length:points.count},(_,id)=>position(id)).some(value=>value.equals(point))).toBe(true);
  const cluster=selection.filter(id=>vector(id).distanceTo(point)<=threshold);
  expect(normals[cluster[0]]).toEqual(sample.normal);
  expect(cluster.reduce((total,id)=>total+pointValues[id],0)/cluster.length).toBe(sample.linear);
  expect(corners.flatMap((id,corner)=>cluster.includes(id)?[uvs[corner]]:[])).toEqual(sample.uvs);
 }
 geometry.dispose();
 welded.dispose();
});

test("independent dissolve geometry preserves area but would extend a one-segment seam",()=>{
 const {positions,face,cornerUvs,incomingSeam,outgoingSeam,replacement,selection,expected}=dissolveSeam;
 const position=(id:number)=>new Vector3().fromArray(positions[id]);
 const length=([a,b]:number[])=>position(a).distanceTo(position(b));
 expect(length(incomingSeam)).toBe(expected.protectedLength);
 expect(length(outgoingSeam)).toBe(expected.protectedLength);
 expect(length(replacement)).toBe(expected.replacementLength);
 const area=(ring:number[])=>ring.slice(1,-1).reduce((sum,b,index)=>{
  const a=position(ring[0]),c=position(ring[index+2]);
  return sum+position(b).sub(a).cross(c.sub(a)).length()/2;
 },0);
 expect(area(face)).toBe(expected.area);
 expect(area(expected.face)).toBe(expected.area);
 const [a,b]=replacement.map(id=>new Vector2().fromArray(cornerUvs[id]));
 const t=expected.protectedLength/expected.replacementLength;
 expect(a.lerp(b,t).toArray()).toEqual(cornerUvs[selection]);
});

test("independent UV islands retain planar shape and require distinct samples for the seam",()=>{
 const {positions,faces,seam,active,secondary,expected}=authoredUnwrap;
 const point=(id:number)=>new Vector3().fromArray(positions[id]);
 const shape=(face:number[])=>{
  const [a,b,c]=face.map(point);
  const lengths=[a.distanceTo(b),b.distanceTo(c),c.distanceTo(a)].sort((first,second)=>first-second);
  return {area:b.sub(a).cross(c.sub(a)).length()/2,lengths};
 };
 for(const face of faces){
  const {area,lengths}=shape(face);
  expect(area).toBe(expected.areaPerIsland);
  expect(lengths).toEqual([expected.shortSide,expected.shortSide,expected.longSide]);
  expect(seam.every(id=>face.includes(id))).toBe(true);
 }
 expect(faces.flat().length).toBe(expected.corners);
 expect([secondary.name,active.name].sort()[0]).toBe(active.name);
 expect(active.values[active.indices[seam[0]]]).toEqual(active.values[active.indices[seam[1]]]);
});

test("independent Bridge oracle checks anchored walls and a closed, outward shell",()=>{
 const {rims,selection,sameLoop,expected}=bridgeFixture;
 const selectionValid=new Ajv({strict:true}).compile(bridgeSchema);
 expect(selectionValid(selection)).toBe(true);
 expect(selectionValid([0])).toBe(false);
 expect(selectionValid([0,0])).toBe(false);
 expect(selectionValid(sameLoop)).toBe(true);
 const {unequal}=bridgeFixture;
 expect(selectionValid(unequal.selection)).toBe(true);
 for(const fixture of [bridgeFixture,unequal]){
  const {positions,rims,expected}=fixture;
  const polygons=[...rims,...expected.walls];
  const vertex=(point:number)=>new Vector3().fromArray(positions[point]);
  const incidence=new Map<string,number>();
  let area=0,volume=0;
  for(const face of polygons){
   for(let i=0;i<face.length;i++){
    const a=face[i],b=face[(i+1)%face.length];
    const key=[a,b].sort((a,b)=>a-b).join(",");
    incidence.set(key,(incidence.get(key)??0)+(a<b?1:-1));
   }
   const origin=vertex(face[0]);
   for(let i=1;i+1<face.length;i++){
    const b=vertex(face[i]),c=vertex(face[i+1]);
    area+=b.clone().sub(origin).cross(c.clone().sub(origin)).length()/2;
    volume+=origin.dot(b.cross(c))/6;
   }
  }
  expect([...incidence.values()].every(value=>value===0)).toBe(true);
  expect(incidence.size).toBe(polygons.reduce((total,face)=>total+face.length,0)/2);
  expect(area).toBeCloseTo(expected.area,12);
  expect(volume).toBeCloseTo(expected.volume,12);
 }
 expect(unequal.expected.walls.map(face=>face.length)).toEqual([3,3,3,3,3,4]);
 expect(expected.walls[0]).toEqual([rims[0][1],rims[0][0],rims[1][0],rims[1][3]]);
});

test("independent extrusion of the trimmed L profile matches the isolated Bevel shell",()=>{
  const {positions,edge,resultOutline,expected}=bevelSurface.concave;
  const outline=new Shape(resultOutline.map(([x,y])=>new Vector2(x,y)));
  const depth=positions[edge[1]][2]-positions[edge[0]][2];
  const geometry=new ExtrudeGeometry(outline,{depth,steps:1,bevelEnabled:false});
  const corners=geometry.getAttribute("position");
  const point=(i:number)=>new Vector3().fromBufferAttribute(corners,i);
  let area=0,volume=0;
  for(let i=0;i<corners.count;i+=3){
    const a=point(i),b=point(i+1),c=point(i+2);
    area+=b.clone().sub(a).cross(c.clone().sub(a)).length()/2;
    volume+=a.dot(b.cross(c))/6;
  }
  expect(area).toBeCloseTo(expected.area,8);
  expect(volume).toBeCloseTo(expected.volume,8);
  expect(mergeVertices(geometry.deleteAttribute("uv").deleteAttribute("normal")).getAttribute("position").count).toBe(expected.vertices);
});


test("independent polygon source projection preserves reversed corner and predecessor edge identities",()=>{
  const {faces,flipped,positions,cornerIndex,cornerUv,seam,expected}=polygonCapture;
  const vertices=positions.map(position=>new Vector3().fromArray(position));
  const corners:number[]=[],edges:number[]=[];
  let offset=0;
  const logical=faces.map((face,id)=>{
    const ordered=flipped[id]?[...face].reverse():face;
    const rawByVertex=new Map(face.map((vertex,slot)=>[vertex,slot+offset]));
    const rawEdge=new Map(face.map((vertex,slot)=>[
      [vertex,face[(slot+1)%face.length]].sort((a,b)=>a-b).join(","),slot+offset,
    ]));
    ordered.forEach((vertex,i)=>{
      corners.push(rawByVertex.get(vertex)!);
      edges.push(rawEdge.get([vertex,ordered[(i+1)%ordered.length]].sort((a,b)=>a-b).join(","))!);
    });
    offset+=face.length;
    const normal=vertices[ordered[1]].clone().sub(vertices[ordered[0]])
      .cross(vertices[ordered[2]].clone().sub(vertices[ordered[0]])).normalize();
    expect(normal.z).toBe(flipped[id]?-1:1);
    return ordered;
  });
  expect(logical).toEqual(expected.faces);
  expect(corners).toEqual(expected.corners);
  expect(edges).toEqual(expected.edges);
  expect(corners.map(source=>cornerIndex[source])).toEqual(expected.cornerIndex);
  expect(corners.indexOf(cornerUv.index)).toBe(expected.surface[2].index);
  expect(edges.indexOf(seam)).toBe(expected.surface[3].index);
});


test("independent boundary extrusion measures the swept connected wall strip",()=>{
 const law=boundaryExtrude;
 const check=new Ajv({strict:true}).compile(boundaryExtrudeSchema);
 expect(check(law.invalid[0])).toBe(false);
 for(const row of law.cases){
   expect(check(row.request)).toBe(true);
   const order=[...new Set([...row.request.edges].sort((a,b)=>a-b).flatMap(edge=>[law.faces[0][edge],law.faces[0][(edge+1)%4]]))];
   const points=law.positions.map(point=>new Vector3().fromArray(point));
   const offset=new Vector3().fromArray(row.request.offset);
   const shifted=order.map(i=>points[i].clone().add(offset));
   expect(shifted.map(point=>point.toArray())).toEqual(row.addedPoints);
   const vertices=[...points,...shifted];
   let area=0;
   for(const face of [...law.faces,...row.walls]){
     const origin=vertices[face[0]];
     for(let j=1;j+1<face.length;j++){
       area+=vertices[face[j]].clone().sub(origin).cross(vertices[face[j+1]].clone().sub(origin)).length()/2;
     }
   }
   expect(area).toBeCloseTo(row.area,12);
   expect(row.walls.length).toBe(row.request.edges.length);
   expect(row.walls.every(wall=>law.faces.some(face=>face.some((id,i)=>id===wall[1]&&face[(i+1)%face.length]===wall[0])))).toBe(true);
 }
});


test("native convex intersection law matches Manifold's independent solid Boolean",async()=>{
 const module=await loadManifold();
 module.setup();
 const cube=([lo,hi]:number[][],rotate=0)=>{
   const size=hi.map((value,i)=>value-lo[i]) as [number,number,number];
   const center=lo.map((value,i)=>value+size[i]/2) as [number,number,number];
   return module.Manifold.cube(size,true).rotate([0,0,rotate]).translate(center);
 };
 const validate=new Ajv({strict:true}).compile(intersectionSchema);
 const [a,b]=intersection.left;
 const vertices=[[a[0],a[1],a[2]],[b[0],a[1],a[2]],[b[0],b[1],a[2]],[a[0],b[1],a[2]],
   [a[0],a[1],b[2]],[b[0],a[1],b[2]],[b[0],b[1],b[2]],[a[0],b[1],b[2]]];
 const mesh={vertices,faces:intersection.faces};
 expect(validate({left:mesh,right:mesh})).toBe(true);
 for(const row of intersection.cases){
   const left=cube(row.left??intersection.left);
   const right=cube(row.right,row.rotateZ);
   const output=left.intersect(right);
   if(row.empty){expect(output.volume()).toBeCloseTo(0,10);}
   else {
     expect(output.status()).toBe("NoError");
     expect(output.volume()).toBeCloseTo(row.volume!,6);
     expect(output.surfaceArea()).toBeCloseTo(row.area!,6);
   }
   output.delete();
   right.delete();
   left.delete();
 }
});


test("authored intersection fixture retains both independent supporting surface domains",async()=>{
 const module=await loadManifold();
 module.setup();
 const cube=(bounds:number[][])=>{
   return module.Manifold.cube(bounds[1].map((x,i)=>x-bounds[0][i]) as [number,number,number]).translate(bounds[0] as [number,number,number]);
 };
 const law=authoredIntersection;
 const left=cube(law.left.bounds),right=cube(law.right.bounds),result=left.intersect(right);
 expect(result.volume()).toBeCloseTo(law.expected.volume,8);
 expect(result.surfaceArea()).toBeCloseTo(law.expected.area,8);
 const data=result.getMesh();
 const point=(vertex:number)=>new Vector3().fromArray(data.vertProperties,vertex*data.numProp);
 const surface=[0,0];
 for(let i=0;i<data.triVerts.length;i+=3){
   const [a,b,c]=Array.from(data.triVerts.slice(i,i+3),point);
   const area=b.clone().sub(a).cross(c.clone().sub(a)).length()/2;
   const onLeft=[a,b,c].every(p=>Math.abs(p.x-law.left.bounds[1][0])<1e-6);
   surface[onLeft?0:1]+=area;
 }
 expect(surface[0]).toBeCloseTo(1.5,8);
 expect(surface[1]).toBeCloseTo(6.5,8);
 expect(law.left.uvBias).not.toBe(law.right.uvBias);
 expect(law.left.material).not.toBe(law.right.material);
 left.delete();right.delete();result.delete();
});

test("bounded convex difference agrees with Manifold on slab cuts, contact and containment",async()=>{
 const module=await loadManifold();
 module.setup();
 const cube=([lo,hi]:number[][])=>module.Manifold.cube(hi.map((n,i)=>n-lo[i]) as [number,number,number]).translate(lo as [number,number,number]);
 const stockWithOpeningDiagonal = (face: number) => {
   const [[x,y,z],[a,b,c]] = difference.left;
   const positions = [[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]];
   const stockFaces = difference.faces.slice();
   const [p,q,r,s] = stockFaces[face];
   stockFaces.splice(face, 1, [p,q,r], [p,r,s]);
   return new module.Manifold(new module.Mesh({
     numProp: 3,
     vertProperties: new Float32Array(positions.flat()),
     triVerts: new Uint32Array(stockFaces.flatMap(ring => ring.slice(2).flatMap((id, i) => [ring[0], ring[i + 1], id]))),
   }));
 };
 for(const row of difference.cases){
   const left = row.splitStockOpening === undefined ? cube(difference.left) : stockWithOpeningDiagonal(row.splitStockOpening);
   const right=cube(row.right),output=left.subtract(right);
   expect(output.status()).toBe("NoError");
   expect(output.volume()).toBeCloseTo(row.volume,8);
   expect(output.surfaceArea()).toBeCloseTo(row.area,8);
   if(row.cutPlane!==undefined){
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const x=row.cutPlane===4?row.right[0][0]:row.right[1][0],expectedNormal=row.cutPlane===4?1:-1;
     let area=0;
     for(let i=0;i<data.triVerts.length;i+=3){
       const [a,b,c]=Array.from(data.triVerts.slice(i,i+3),point);
       if([a,b,c].every(p=>Math.abs(p.x-x)<1e-6)){
         const cross=b.sub(a).cross(c.sub(a));
         expect(cross.clone().normalize().x).toBe(expectedNormal);
         area+=cross.length()/2;
       }
     }
     expect(area).toBeCloseTo(4,8);
   }
   if(row.cutPlanes&&!row.slab&&!row.tunnel&&!row.pocket&&!row.cavity&&!row.recess){
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const normals=new Set<string>();
     for(let i=0;i<data.triVerts.length;i+=3){
       const [a,b,c]=Array.from(data.triVerts.slice(i,i+3),point);
       const normal=b.clone().sub(a).cross(c.clone().sub(a)).normalize();
       if(row.cutPlanes.includes(0)&&[a,b,c].every(p=>p.z===row.right[0][2]))normals.add("z:"+normal.z);
       if(row.cutPlanes.includes(1)&&[a,b,c].every(p=>p.z===row.right[1][2]))normals.add("z:"+normal.z);
       if(row.cutPlanes.includes(2)&&[a,b,c].every(p=>p.y===row.right[0][1]))normals.add("y:"+normal.y);
       if(row.cutPlanes.includes(4)&&[a,b,c].every(p=>p.x===row.right[0][0]))normals.add("x:"+normal.x);
       if(row.cutPlanes.includes(3)&&[a,b,c].every(p=>p.y===row.right[1][1]))normals.add("y:"+normal.y);
       if(row.cutPlanes.includes(5)&&[a,b,c].every(p=>p.x===row.right[1][0]))normals.add("x:"+normal.x);
     }
     const sign=row.cutPlanes.includes(2)?1:-1;
     const axes=row.octant?["x","y","z"]:["x","y"];
     expect([...normals].sort()).toEqual(axes.map(axis=>axis+":"+sign));
   }
   if(row.slab){
     const {axis,intervals,volumes}=row.slab;
     const shells=output.decompose().sort((a,b)=>a.boundingBox().min[axis]-b.boundingBox().min[axis]);
     expect(shells.length).toBe(2);
     for(const [i,shell] of shells.entries()){
       expect(shell.volume()).toBeCloseTo(volumes[i],8);
       expect([shell.boundingBox().min[axis],shell.boundingBox().max[axis]]).toEqual(intervals[i]);
       shell.delete();
     }
   }
   if(row.tunnel||row.pocket){
     const depthAxis=(row.tunnel??row.pocket)!.axis;
     expect(output.genus()).toBe(row.tunnel?1:0);
     const shells=output.decompose();
     expect(shells.length).toBe(1);
     shells.forEach(shell=>shell.delete());
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const planes=[[2,-1],[2,1],[1,-1],[1,1],[0,-1],[0,1]];
     for(const face of row.cutPlanes!){
       const [axis,side]=planes[face];
       const coordinate=row.right[Number(side>0)][axis];
       const opening=new Set<number>();
       for(let i=0;i<data.triVerts.length;i+=3){
         const [a,b,c]=Array.from(data.triVerts.slice(i,i+3),point);
         if([a,b,c].every(p=>p.getComponent(axis)===coordinate)){
           expect(b.clone().sub(a).cross(c.clone().sub(a)).normalize().getComponent(axis)).toBe(-side);
           for(const p of [a,b,c])opening.add(p.getComponent(depthAxis));
         }
       }
       const depths=row.tunnel?.opening??(row.pocket!.bottomPlane===face?[row.pocket!.bottom]:[row.pocket!.opening,row.pocket!.bottom].sort((a,b)=>a-b));
       expect([...opening].sort((a,b)=>a-b)).toEqual(depths);
     }
   }
   if(row.recess){
     expect(output.genus()).toBe(0);
     const shells=output.decompose();
     expect(shells).toHaveLength(1);
     for(const shell of shells)shell.delete();
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const [low,high]=difference.left;
     const [cutLow,cutHigh]=row.right;
     const opening=[2,2,1,1,0,0].flatMap((axis,face)=>{
       const middle=low.map((start,k)=>(Math.max(start,cutLow[k])+Math.min(high[k],cutHigh[k]))/2);
       middle[axis]=(face%2===0?low:high)[axis];
       const target=new Vector3().fromArray(middle);
       const nearest=new Vector3();
       for(let i=0;i<data.triVerts.length;i+=3){
         const triangle=new Triangle(...Array.from(data.triVerts.slice(i,i+3),point) as [Vector3,Vector3,Vector3]);
         if(triangle.closestPointToPoint(target,nearest).distanceToSquared(target)<1e-16)return [];
       }
       return [face];
     });
     expect(opening).toEqual(row.openStockFaces!);
   }
   if(row.cavity){
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const [lo,hi]=difference.left;
     const volumes=[0,0];
     for(let i=0;i<data.triVerts.length;i+=3){
       const triangle=Array.from(data.triVerts.slice(i,i+3),point);
       const inside=triangle.every(p=>[0,1,2].every(axis=>p.getComponent(axis)>lo[axis]&&p.getComponent(axis)<hi[axis]));
       const outer=[0,1,2].some(axis=>[lo[axis],hi[axis]].some(bound=>triangle.every(p=>p.getComponent(axis)===bound)));
       expect(inside!==outer).toBe(true);
       const [a,b,c]=triangle;
       volumes[Number(inside)]+=a.dot(b.cross(c))/6;
     }
     volumes.forEach((volume,i)=>expect(volume).toBeCloseTo(row.cavity!.volumes[i],8));
   }
   output.delete();right.delete();left.delete();
 }
});

test("prismatic Difference retains the Manifold tunnel and blind pocket topology", async () => {
  const module = await loadManifold();
  module.setup();
  const stock = new module.Manifold(new module.Mesh({
    numProp: 3,
    vertProperties: new Float32Array(prismaticDifference.stock.positions.flat()),
    triVerts: new Uint32Array(prismaticDifference.stock.faces.flatMap(ring =>
      ring.slice(2).flatMap((vertex, i) => [ring[0], ring[i + 1], vertex]))),
  }));
  try {
    for (const row of prismaticDifference.cases) {
      const spec = prismaticDifference.profiles[row.profile as keyof typeof prismaticDifference.profiles];
      const profile = new module.CrossSection(spec.footprint as [number, number][]);
      const [start, end] = row.height;
      const prism = profile.extrude(end - start);
      const cutter = prism.translate([0, 0, start]);
      const residual = stock.subtract(cutter);
      const shells = residual.decompose();
      try {
        expect(shells).toHaveLength(1);
        expect(residual.volume()).toBeCloseTo(row.volume, 8);
        expect(residual.surfaceArea()).toBeCloseTo(row.area, 8);
        expect(2 - 2 * residual.genus()).toBe(row.euler);
      } finally {
        for (const shape of [...shells, residual, cutter, prism, profile]) shape.delete();
      }
    }
  } finally {
    stock.delete();
  }
});

test("authored convex Union geometry agrees with Manifold on matching sections and separate shells",async()=>{
 const module=await loadManifold();
 module.setup();
 const cube=([lo,hi]:number[][])=>module.Manifold.cube(hi.map((n,i)=>n-lo[i]) as [number,number,number]).translate(lo as [number,number,number]);
 for(const row of union.cases){
   const stock=row.left??union.left;
   const left=cube(stock),right=cube(row.right),output=left.add(right);
   expect(output.status()).toBe("NoError");
   expect(output.volume()).toBeCloseTo(row.volume,8);
   expect(output.surfaceArea()).toBeCloseTo(row.area,8);
   if(row.faces===10){
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const seam=row.name==="opposite direction overlap"?1:row.right[0][0];
     for(let i=0;i<data.triVerts.length;i+=3){
       const points=Array.from(data.triVerts.slice(i,i+3),point);
       expect(points.every(p=>Math.abs(p.x-seam)<1e-6)).toBe(false);
     }
   }
   if(row.partial){
     const components=output.decompose();
     expect(components.length).toBe(1);
     components.forEach(component=>component.delete());
     expect(output.genus()).toBe(0);
     const data=output.getMesh();
     const point=(id:number)=>new Vector3().fromArray(data.vertProperties,id*data.numProp);
     const supported=(points:Vector3[],[[x,y,z],[a,b,c]]:number[][])=>[[x,a],[y,b],[z,c]].some((bounds,axis)=>bounds.some(value=>points.every(p=>p.getComponent(axis)===value)));
     const contained=(p:Vector3[],[lo,hi]:number[][])=>p.every(point=>lo.every((bound,axis)=>bound<point.getComponent(axis)&&point.getComponent(axis)<hi[axis]));
     for(let i=0;i<data.triVerts.length;i+=3){
       const points=Array.from(data.triVerts.slice(i,i+3),point);
       expect(supported(points,stock)||supported(points,row.right)).toBe(true);
       expect(contained(points,stock)||contained(points,row.right)).toBe(false);
     }
   }
   output.delete();right.delete();left.delete();
 }
});
