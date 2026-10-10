import {expect,test} from "bun:test";
import Ajv from "ajv";
import {BufferGeometry,Float32BufferAttribute} from "three";
import law from "../../🧫️fixtures/🛠️modeling/🎟️owners.json";
import schema from "../../🧬️schema/🛠️modeling/🎟️owners.json";
import cases from "../../🧫️fixtures/🛠️modeling/🔣️.json";
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
