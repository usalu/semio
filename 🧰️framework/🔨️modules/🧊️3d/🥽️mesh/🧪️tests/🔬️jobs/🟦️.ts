import {expect,test} from "bun:test";
import Ajv from "ajv";
import {BufferGeometry,Float32BufferAttribute} from "three";
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
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);const policy=law.orientationComponents;expect(validate({...law,orientationComponents:{...policy,neighborCopyBytes:0}})).toBe(false);
 for(const name of policy.sources){const adjacency=policy.adjacency[name as keyof typeof policy.adjacency],flags={oriented:adjacency.map(()=>false),flipped:adjacency.map(()=>false)};const visited=new Set<number>();
  for(let root=0;root<adjacency.length;root++){if(visited.has(root))continue;const pending=[root];visited.add(root);applyPatch(flags,[{op:"replace",path:`/oriented/${root}`,value:true}]);while(pending.length){const face=pending.pop()!;for(const [neighbor,relative]of adjacency[face]){if(visited.has(neighbor as number))continue;visited.add(neighbor as number);applyPatch(flags,[{op:"replace",path:`/oriented/${neighbor}`,value:true},{op:"replace",path:`/flipped/${neighbor}`,value:flags.flipped[face]!==relative}]);pending.push(neighbor as number);}}}
  const expected=policy.outputs[name as keyof typeof policy.outputs];expect(flags.oriented).toEqual(expected.oriented);expect(flags.flipped).toEqual(expected.flipped);expect(flags.flipped.filter(Boolean).length).toBe(expected.flipCount);
  if(name==="snapshotMisorientedPreparation"){const source=law.snapshotMisorientedPreparation;source.faces.forEach((original,index)=>{const face=flags.flipped[index]?[...original].reverse():original,geometry=new BufferGeometry().setAttribute("position",new Float32BufferAttribute(face.slice(0,3).flatMap(vertex=>source.positions[vertex]),3));geometry.computeVertexNormals();expect(Array.from(geometry.getAttribute("normal").array.slice(0,3)).map(value=>value===0?0:value)).toEqual(policy.correctedNormal);geometry.dispose();});}}
 expect(new BigUint64Array(1).byteLength+new Uint8Array(1).byteLength).toBe(policy.rootCopyBytes);expect(new BigUint64Array(1).byteLength+new Uint8Array(2).byteLength).toBe(policy.neighborCopyBytes);
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
