import {expect,test} from "bun:test";
import Ajv from "ajv";
import {BufferGeometry,Float32BufferAttribute,Mesh,EdgesGeometry,ShapeUtils,Vector2,DataTexture,MeshStandardMaterial} from "three";
import law from "../../🧫️fixtures/🧩️tessellation/🎟️owners.json";
import schema from "../../🧬️schema/🧩️tessellation/🎟️owners.json";

test("original tessellation cancellation preserves retained buffers until closure",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  const source=new Mesh(geometry);
  const owner={source,output:geometry.toNonIndexed(),cancelled:false};
  const sourceArray=owner.source.geometry.getAttribute("position").array;
  const outputArray=owner.output.getAttribute("position").array;
  const sourceBytes=sourceArray.byteLength,outputBytes=outputArray.byteLength;
  for(const copy of law.copyGrants){
    owner.cancelled=true;
    expect(owner.source).toBe(source);
    expect(owner.source.geometry.getAttribute("position").array).toBe(sourceArray);
    expect(owner.output.getAttribute("position").array).toBe(outputArray);
    expect(sourceArray.byteLength+outputArray.byteLength).toBe(sourceBytes+outputBytes);
    expect(copy).toBeGreaterThan(0);
  }
  expect(Array.from(outputArray)).toEqual(law.positions.flat());
  owner.output.dispose();
  owner.source.geometry.dispose();
});

test("original tessellation buffer extents bound independent triangle and edge output",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  const original=geometry.getAttribute("position").array;
  const output=geometry.toNonIndexed(),edges=new EdgesGeometry(geometry);
  const halfedges=law.faces.reduce((count,face)=>count+face.length,0);
  const reservation=new Map(law.normalReservation.buffers.map(buffer=>[buffer.name,halfedges*buffer.elementsPerHalfedge]));
  expect(output.getAttribute("position").array.length).toBeLessThanOrEqual(reservation.get("positions")!);
  expect(edges.getAttribute("position").array.length).toBeLessThanOrEqual(reservation.get("edgePositions")!);
  expect(output.getAttribute("position").count).toBeLessThanOrEqual(reservation.get("vertexIds")!);
  expect(geometry.index!.array.length).toBeLessThanOrEqual(reservation.get("indices")!);
  expect(geometry.getAttribute("position").array).toBe(original);
  expect(reservation.size).toBe(law.normalReservation.buffers.length);
  expect(law.normalReservation.copyBytes).toBe(0);
  expect(law.normalReservation.releaseBytes).toBe(0);
  edges.dispose();output.dispose();geometry.dispose();
});

test("original tessellation metadata capture borrows independent authored attribute buffers",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  for(const attribute of law.metadataCapture.attributes)geometry.setAttribute(attribute.name,new Float32BufferAttribute(attribute.values.flat(),attribute.values[0].length));
  const originals=new Map(law.metadataCapture.attributes.map(attribute=>[attribute.name,geometry.getAttribute(attribute.name).array]));
  for(const attribute of law.metadataCapture.attributes){expect(geometry.getAttribute(attribute.name).array).toBe(originals.get(attribute.name));expect(Array.from(geometry.getAttribute(attribute.name).array)).toEqual(attribute.values.flat());}
  expect(originals.size).toBe(law.metadataCapture.attributes.length);
  geometry.dispose();
});

test("original tessellation charges the exact independent authored corner extent",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  for(const attribute of law.metadataCapture.attributes)geometry.setAttribute(attribute.name,new Float32BufferAttribute(attribute.values.flat(),attribute.values[0].length));
  const output=geometry.toNonIndexed();
  for(const name of["position","normal","uv","color"]as const){const attribute=output.getAttribute(name);expect(Array.from(attribute.array.slice(0,attribute.itemSize))).toEqual(law.cornerEmission[name]);}
  const writtenBytes=["position","normal","uv","color"].reduce((bytes,name)=>bytes+output.getAttribute(name).itemSize*Float32Array.BYTES_PER_ELEMENT,0)+2*Uint32Array.BYTES_PER_ELEMENT;
  expect(writtenBytes).toBe(law.cornerEmission.copyBytes);
  output.dispose();geometry.dispose();
});

test("original tessellation prices the exact independent authored edge extent",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  const edges=new EdgesGeometry(geometry);
  const pairs=Array.from({length:edges.getAttribute("position").count/2},(_,index)=>Array.from(edges.getAttribute("position").array.slice(index*6,index*6+6)));
  expect(pairs.some(pair=>JSON.stringify(pair)===JSON.stringify(law.edgeEmission.positions)||JSON.stringify([...pair.slice(3),...pair.slice(0,3)])===JSON.stringify(law.edgeEmission.positions))).toBe(true);
  const uv=law.metadataCapture.attributes.find(attribute=>attribute.semantic==="uv")!;
  expect(law.faces[0].slice(0,2).flatMap(vertex=>uv.values[vertex])).toEqual(law.edgeEmission.uvs);
  expect(law.edgeEmission.copyBytes).toBe((law.edgeEmission.positions.length+law.edgeEmission.uvs.length)*Float32Array.BYTES_PER_ELEMENT+Uint32Array.BYTES_PER_ELEMENT+Uint8Array.BYTES_PER_ELEMENT);
  edges.dispose();geometry.dispose();
});

test("original tessellation prices the exact independent triangle index extent",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  expect(Array.from(geometry.index!.array)).toEqual(law.triangleEmission.indices);
  expect(geometry.index!.count/3).toBe(law.triangleEmission.maximumTriangles);
  expect(law.triangleEmission.copyBytes).toBe((law.triangleEmission.indices.length+1)*Uint32Array.BYTES_PER_ELEMENT);
  geometry.dispose();
});

test("original tessellation projects independent authored samples through original indices",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  for(const attribute of law.metadataCapture.attributes){const values=law.attributeProjection.sourceIndices.flatMap(index=>attribute.values[index]);geometry.setAttribute(attribute.name,new Float32BufferAttribute(values,attribute.values[0].length));}
  const originals=new Map(law.metadataCapture.attributes.map(attribute=>[attribute.name,geometry.getAttribute(attribute.name).array]));
  const output=geometry.toNonIndexed();
  for(const attribute of law.metadataCapture.attributes){expect(Array.from(output.getAttribute(attribute.name).array)).toEqual(law.attributeProjection.outputIndices.flatMap(index=>attribute.values[index]));expect(geometry.getAttribute(attribute.name).array).toBe(originals.get(attribute.name));}
  expect(law.attributeProjection.atomicIndexBytes).toBe(Uint32Array.BYTES_PER_ELEMENT);
  output.dispose();geometry.dispose();
});

test("original face preparation agrees with the independent triangle normal",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  geometry.computeVertexNormals();
  const normals=geometry.getAttribute("normal");
  for(let corner=0;corner<normals.count;corner++)expect(Array.from(normals.array.slice(corner*3,corner*3+3))).toEqual(law.facePreparation.normal);
  expect(Array.from(geometry.getAttribute("position").array)).toEqual(law.facePreparation.sourcePositions.flat());
  expect(law.facePreparation.normalCompletionCopyBytes).toBe(law.facePreparation.normalSumCopyBytes+3*Float32Array.BYTES_PER_ELEMENT+6*Float64Array.BYTES_PER_ELEMENT);
  geometry.dispose();
});

test("original normal tessellation agrees with independent authored triangle and edge output",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));
  geometry.setIndex(law.faces.flat());
  for(const attribute of law.metadataCapture.attributes)geometry.setAttribute(attribute.name,new Float32BufferAttribute(attribute.values.flat(),attribute.values[0].length));
  const output=geometry.toNonIndexed(),edges=new EdgesGeometry(geometry);
  for(const[name,expected]of[["position",law.normalTessellation.positions],["normal",law.normalTessellation.normals],["color",law.normalTessellation.colors],["uv",law.normalTessellation.uvs]]as const)expect(Array.from(output.getAttribute(name).array)).toEqual(expected);
  const actual=Array.from({length:edges.getAttribute("position").count/2},(_,index)=>Array.from(edges.getAttribute("position").array.slice(index*6,index*6+6)));
  for(let index=0;index<law.normalTessellation.edgeIds.length;index++){const expected=law.normalTessellation.edgePositions.slice(index*6,index*6+6);expect(actual.some(pair=>JSON.stringify(pair)===JSON.stringify(expected)||JSON.stringify([...pair.slice(3),...pair.slice(0,3)])===JSON.stringify(expected))).toBe(true);}
  const scalarBytes=[law.normalTessellation.positions,law.normalTessellation.normals,law.normalTessellation.colors,law.normalTessellation.uvs,law.normalTessellation.indices,law.normalTessellation.vertexIds,law.normalTessellation.faceIds,law.normalTessellation.edgePositions,law.normalTessellation.edgeUvs,law.normalTessellation.edgeIds].reduce((sum,values)=>sum+values.length*Float32Array.BYTES_PER_ELEMENT,0);
  const projectionBytes=law.metadataCapture.attributes.length*law.normalTessellation.vertexIds.length*Uint32Array.BYTES_PER_ELEMENT;
  expect(scalarBytes+law.normalTessellation.edgeIsSeam.length+projectionBytes).toBe(law.normalTessellation.publishedBufferBytes);
  edges.dispose();output.dispose();geometry.dispose();
});

test("original normal polygon corpus agrees with independent triangulation and asset ownership",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  for(const polygon of law.polygons){
    let triangles=0,area=0;
    for(const face of polygon.faces){const points=face.map(vertex=>new Vector2(polygon.positions[vertex][0],polygon.positions[vertex][1]));const indices=ShapeUtils.triangulateShape(points,[]);triangles+=indices.length;for(const[a,b,c]of indices){area+=Math.abs((points[b].x-points[a].x)*(points[c].y-points[a].y)-(points[b].y-points[a].y)*(points[c].x-points[a].x))/2;}}
    expect(triangles).toBe(polygon.triangles);expect(area).toBe(polygon.area);
  }
  const bytes=Uint8Array.from(law.surfaceAssetCustody.texture.bytes),texture=new DataTexture(bytes,1,1);
  const material=new MeshStandardMaterial({name:law.surfaceAssetCustody.materialKey,map:texture});material.userData.label=law.surfaceAssetCustody.material.label;
  expect(texture.image.data).toBe(bytes);expect(Array.from(texture.image.data)).toEqual(law.surfaceAssetCustody.texture.bytes);expect(material.map).toBe(texture);expect(material.userData.label).toBe(law.surfaceAssetCustody.material.label);
  material.dispose();texture.dispose();
});

test("original indexed channels agree with independent corner buffers and UV seam",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const fixture=law.indexedChannels;
  for(const faceUv of[false,true]){
    const geometry=new BufferGeometry();
    geometry.setAttribute("position",new Float32BufferAttribute(fixture.vertexIds.flatMap(id=>fixture.positions[id]),3));
    for(const name of["normal","uv","color"]as const){
      const attribute=fixture.attributes.find(attribute=>attribute.name===name)!;
      const values=faceUv&&name==="uv"?fixture.faceUv.values:attribute.values;
      const ids=attribute.domain==="face"||faceUv&&name==="uv"?fixture.cornerIds.map(id=>Math.floor(id/3)):fixture.cornerIds;
      const samples=ids.flatMap(id=>values[attribute.indices&&!(faceUv&&name==="uv")?attribute.indices[id]:id]as number[]);
      geometry.setAttribute(name,new Float32BufferAttribute(samples,(values[0]as number[]).length));
    }
    expect(Array.from(geometry.getAttribute("normal").array)).toEqual(fixture.normals);
    expect(Array.from(geometry.getAttribute("color").array)).toEqual(fixture.colors);
    expect(Array.from(geometry.getAttribute("uv").array)).toEqual(faceUv?fixture.faceUv.uvs:fixture.uvs);
    const uv=geometry.getAttribute("uv"),pair=(id:number)=>new Vector2(uv.getX(id),uv.getY(id));
    expect(pair(2).equals(pair(4))&&pair(0).equals(pair(3))).toBe(false);
    const edgeUvs=fixture.edgeIds.flatMap(id=>{const next=Math.floor(id/3)*3+(id+1)%3;return[pair(id).x,pair(id).y,pair(next).x,pair(next).y];});
    expect(edgeUvs).toEqual(faceUv?fixture.faceUv.edgeUvs:fixture.edgeUvs);
    geometry.dispose();
  }
  for(const attribute of fixture.attributes){const ids=attribute.domain==="face"?fixture.faceIds:attribute.domain==="edge"?fixture.edgeIds:attribute.domain==="vertex"?fixture.vertexIds:fixture.cornerIds;expect(ids.map(id=>attribute.indices?attribute.indices[id]:id)).toEqual(attribute.outputIndices);}
});

test("original modeling tessellation handback keeps the independent source allocation",()=>{
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const polygon=law.polygons.find(polygon=>polygon.id===law.modelingTessellation.polygon)!;
  const geometry=new BufferGeometry();geometry.setAttribute("position",new Float32BufferAttribute(polygon.positions.flat(),3));
  const source=geometry.getAttribute("position").array;
  const contour=polygon.faces[0].map(id=>new Vector2(polygon.positions[id][0],polygon.positions[id][1]));
  geometry.setIndex(ShapeUtils.triangulateShape(contour,[]).flat());
  const output=geometry.toNonIndexed();
  expect(output.getAttribute("position").count).toBe(polygon.triangles*3);
  expect(geometry.getAttribute("position").array).toBe(source);
  expect(law.modelingTessellation.successfulHandbackReleaseBytes).toBe(0);
  expect(law.modelingTessellation.handoffCuts).toEqual([null,0,3,17]);
  output.dispose();geometry.dispose();
});
