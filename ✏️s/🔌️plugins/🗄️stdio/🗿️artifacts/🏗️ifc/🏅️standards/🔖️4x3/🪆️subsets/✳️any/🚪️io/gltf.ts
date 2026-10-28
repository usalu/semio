import type {IfcSnapshot, IfcEntity, IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import type {GltfSnapshot, GltfDocument} from "../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import {extractIfc4x3Triangles,ifcIdentity,ifcMultiply,ifcTransform,type IfcMatrix,type IfcExchangeOutcome,type IfcExchangeDiagnostic} from "./geometry.ts";
import {ifcCheckpoint,ifcReal,ifcNumber,IfcStepError,type IfcIoControl} from "./step.ts";
import {ifcMatrixOrientation,IfcNormalTransform} from "./orientation.ts";
import {captureIfcIoControl,captureIfcIoControlScope,runIfcIoSteps} from "./control.ts";
import {ifcGltfPositionSteps,ifcGltfMeshSteps} from './gltf-mesh.ts';
import {captureIfcGeometryControl,type IfcGeometryControl} from "./geometry/control.ts";
import {captureIfcGltfSource,captureIfcGlbBytes} from "./gltf-source.ts";
import {encodeIfcGltfJson} from "./gltf-json.ts";
import {decodeIfcGltfJson} from './gltf-json-reader.ts';

const word=(number:number)=>{const value=ifcReal(number);if(value.kind!=="real")throw Error("real");return value.value;};
const numeric=(value:{readonly bits:bigint})=>ifcNumber({kind:"real",value});
const unsupported=(message:string,entityIds:bigint[]=[]):IfcExchangeDiagnostic=>({code:"exchange.loss",severity:"warning",message,entityIds});
const supportedVersion=(asset:GltfDocument["asset"]):boolean=>asset.version==="2.0"&&(asset.minVersion===undefined||asset.minVersion==="2.0");

/** 🧊 Produce the real glTF artifact with explicit SI metre and Z-up to Y-up conversion. */
export async function ifc4x3ToGltf(snapshot:IfcSnapshot,control:IfcGeometryControl={}):Promise<IfcExchangeOutcome<GltfSnapshot>> {
  control=captureIfcGeometryControl(control);
  const extracted=await extractIfc4x3Triangles(snapshot,control);
  if(!extracted.artifact?.length)return {fidelity:"unsupported",diagnostics:[...extracted.diagnostics,{code:"geometry.empty",severity:"error",message:"No supported product triangulations",entityIds:[]}]};
  const document:GltfDocument={asset:{version:"2.0",generator:"semio IFC4x3"},scene:0n,scenes:[{nodes:[]}],nodes:[],meshes:[],accessors:[],bufferViews:[],buffers:[],materials:[],textures:[],images:[],samplers:[],skins:[],animations:[],cameras:[],extensionsUsed:[],extensionsRequired:[]};
  const chunks:Uint8Array[]=[],batch=control.batchSize!,diagnostics=[...extracted.diagnostics];let byteOffset=0,normalOmitted=false;
  for(let meshIndex=0;meshIndex<extracted.artifact.length;meshIndex++) {
    await ifcCheckpoint(control,"gltf",meshIndex,extracted.artifact.length);
    const mesh=await runIfcIoSteps(ifcGltfMeshSteps(extracted.artifact[meshIndex]!,Math.min(0xffffffff,control.maxValues!)-byteOffset),control,'gltf-normal-domain'),positionBytes=mesh.positions.length*12,indexBytes=mesh.indices.length*4,normalBytes=(mesh.normals?.length??0)*12,totalBytes=byteOffset+positionBytes+indexBytes+normalBytes;
    if(!Number.isSafeInteger(totalBytes)||totalBytes>0xffffffff||totalBytes>control.maxValues!)throw new IfcStepError("gltf.limit","Encoded buffer bytes exceed admitted artifact limits");
    const positions=new Uint8Array(positionBytes),p=new DataView(positions.buffer),indices=new Uint8Array(indexBytes),ix=new DataView(indices.buffer);
    const normals=mesh.normals?new Uint8Array(normalBytes):undefined,n=normals?new DataView(normals.buffer):undefined;if(mesh.normalOmitted){normalOmitted=true;diagnostics.push({code:'gltf.normal-omission',severity:'warning',message:'Referenced zero normals cannot define glTF unit directions; authored normals are omitted for this mesh',entityIds:[mesh.entityId]});}
    const min=[Infinity,Infinity,Infinity],max=[-Infinity,-Infinity,-Infinity];
    for(let i=0;i<mesh.positions.length;i++) {
      if(i%batch===0)await ifcCheckpoint(control,"gltf-vertices",i,mesh.positions.length);
      const [x,y,z]=mesh.positions[i]!,vector=[x,z,-y];
      for(let k=0;k<3;k++) {const rounded=Math.fround(vector[k]!);if(!Number.isFinite(rounded))throw new IfcStepError("gltf.range","Position exceeds float32 range");p.setFloat32(i*12+k*4,rounded,true);min[k]=Math.min(min[k]!,rounded);max[k]=Math.max(max[k]!,rounded);}
    }
    await ifcCheckpoint(control,"gltf-vertices",mesh.positions.length,mesh.positions.length);
    await runIfcIoSteps(ifcGltfPositionSteps(positions,mesh.indices),control,'gltf-position-validation');
    if(mesh.normals){for(let at=0;at<mesh.normals.length;at++){if(at%batch===0)await ifcCheckpoint(control,'gltf-normals',at,mesh.normals.length);const[x,y,z]=mesh.normals[at]!,vector=[x,z,-y];for(let axis=0;axis<3;axis++)n!.setFloat32(at*12+axis*4,Math.fround(vector[axis]!),true);}await ifcCheckpoint(control,'gltf-normals',mesh.normals.length,mesh.normals.length);}
    for(let i=0;i<mesh.indices.length;i++){
      if(i%batch===0)await ifcCheckpoint(control,"gltf-indices",i,mesh.indices.length);
      ix.setUint32(i*4,mesh.indices[i]!,true);
    }
    await ifcCheckpoint(control,"gltf-indices",mesh.indices.length,mesh.indices.length);
    const view=BigInt(document.bufferViews.length),accessor=BigInt(document.accessors.length);
    document.bufferViews.push({buffer:0n,byteOffset:BigInt(byteOffset),byteLength:BigInt(positions.length),target:34962n},{buffer:0n,byteOffset:BigInt(byteOffset+positions.length),byteLength:BigInt(indices.length),target:34963n});
    document.accessors.push({bufferView:view,byteOffset:0n,componentType:5126,normalized:false,count:BigInt(mesh.positions.length),type:"VEC3",min:min.map(word),max:max.map(word)},{bufferView:view+1n,byteOffset:0n,componentType:5125,normalized:false,count:BigInt(mesh.indices.length),type:"SCALAR"});
    const attributes:[string,bigint][]=[["POSITION",accessor]];if(normals){document.bufferViews.push({buffer:0n,byteOffset:BigInt(byteOffset+positions.length+indices.length),byteLength:BigInt(normals.length),target:34962n});document.accessors.push({bufferView:view+2n,byteOffset:0n,componentType:5126,normalized:false,count:BigInt(mesh.normals!.length),type:'VEC3'});attributes.push(['NORMAL',accessor+2n]);}
    document.meshes.push({name:mesh.name,weights:[],primitives:[{attributes,indices:accessor+1n,mode:4n,targets:[]}]});
    document.nodes.push({children:[],weights:[],mesh:BigInt(meshIndex),name:mesh.name,extras:{kind:"object",members:[["ifcProductId",{kind:"string",value:String(mesh.productId)}],["ifcRepresentationId",{kind:"string",value:String(mesh.entityId)}]]}});
    document.scenes[0]!.nodes.push(BigInt(meshIndex));chunks.push(positions,indices);if(normals)chunks.push(normals);byteOffset=totalBytes;
  }
  document.buffers.push({byteLength:BigInt(byteOffset)});
  const bytes=new Array<number>(byteOffset);let offset=0;
  for(const chunk of chunks)for(let index=0;index<chunk.length;index++){
    if(offset%batch===0)await ifcCheckpoint(control,"gltf-buffer-bytes",offset,byteOffset);
    bytes[offset++]=chunk[index]!;
  }
  await ifcCheckpoint(control,"gltf-buffer-bytes",byteOffset,byteOffset);
  await ifcCheckpoint(control,"gltf",extracted.artifact.length,extracted.artifact.length);
  return {artifact:{schema:"stdio.gltf",document,buffers:[bytes],sourceForm:"glb"},fidelity:normalOmitted?'partial':extracted.fidelity,diagnostics:[...diagnostics,unsupported("glTF retains represented triangles, projected authored unit normals and product identities; IFC relationships, authored parameters, materials, properties and analysis semantics are omitted; positions use float32")]};
}

async function copyGlbBytes(source:ArrayLike<number>,target:Uint8Array|number[],count:number,control:IfcIoControl):Promise<void> {
  for(let index=0;index<count;index++) {
    if(index%(control.batchSize!)===0)await ifcCheckpoint(control,"glb-bytes",index,count);
    const value=source[index]!;if(!Number.isInteger(value)||value<0||value>255)throw new IfcStepError("glb.bytes","Buffer bytes must be unsigned integers");target[index]=value;
  }
  await ifcCheckpoint(control,"glb-bytes",count,count);
}

/** 💾 Frame an embedded glTF artifact as GLB 2.0 without external URI dependencies. */
export async function writeIfcGltfGlb(snapshot:GltfSnapshot,control:IfcIoControl={}):Promise<Uint8Array> {
  control=captureIfcIoControl(control);
  const captured=await captureIfcGltfSource(snapshot,control);
  if(!captured.artifact){const diagnostic=captured.diagnostics[0];throw new IfcStepError(diagnostic?.code==="gltf.bytes"?"glb.bytes":"exchange.source",diagnostic?.message??"glTF source admission refused");}
  snapshot=captured.artifact;
  await ifcCheckpoint(control,"glb",0,1);
  if(!supportedVersion(snapshot.document.asset))throw new IfcStepError("gltf.version","Supported glTF 2.0 asset version required");
  if(snapshot.document.buffers.length!==1||snapshot.buffers.length!==1||snapshot.document.buffers[0]!.uri)throw new IfcStepError("glb.buffers","Exactly one embedded buffer is required");
  const binary=snapshot.buffers[0]!;
  if(binary.length>control.maxValues!)throw new IfcStepError("glb.limit","Buffer values exceed caller limit");
  if(binary.length<1||snapshot.document.buffers[0]!.byteLength!==BigInt(binary.length))throw new IfcStepError("glb.length","Positive matching buffer byteLength required");
  const binLength=Math.ceil(binary.length/4)*4;
  if(0xffffffff-28-binLength<4)throw new IfcStepError("glb.length","BIN leaves no framed JSON capacity");
  const jsonCeiling=Math.floor((Math.min(control.maxValues!,0xffffffff)-28-binLength)/4)*4;
  if(jsonCeiling<4)throw new IfcStepError("glb.limit","Framed JSON bytes exceed caller limit");
  const json=await encodeIfcGltfJson(snapshot.document,control,jsonCeiling),jsonLength=Math.ceil(json.byteLength/4)*4,length=12+8+jsonLength+8+binLength;
  if(!Number.isSafeInteger(length)||length>0xffffffff)throw new IfcStepError("glb.length","GLB exceeds unsigned 32-bit length");
  if(length>control.maxValues!)throw new IfcStepError("glb.limit","Framed GLB bytes exceed caller limit");
  const bytes=new Uint8Array(length),view=new DataView(bytes.buffer);
  view.setUint32(0,0x46546c67,true);view.setUint32(4,2,true);view.setUint32(8,length,true);
  view.setUint32(12,jsonLength,true);view.setUint32(16,0x4e4f534a,true);bytes.fill(32,20+json.byteLength,20+jsonLength);
  let offset=0;for(const chunk of json.chunks)for(const byte of chunk){
    if(offset%(control.batchSize!)===0)await ifcCheckpoint(control,"glb-json-bytes",offset,json.byteLength);
    bytes[20+offset++]=byte;
  }
  await ifcCheckpoint(control,"glb-json-bytes",json.byteLength,json.byteLength);
  const binStart=20+jsonLength;view.setUint32(binStart,binLength,true);view.setUint32(binStart+4,0x004e4942,true);await copyGlbBytes(binary,bytes.subarray(binStart+8),binary.length,control);
  await ifcCheckpoint(control,"glb",1,1);return bytes;
}

/** 📂 Read private embedded GLB bytes through the shared lossless syntax and glTF wire roles. */
export async function readIfcGltfGlb(bytes:Uint8Array,control:IfcIoControl={}):Promise<GltfSnapshot> {
  control=captureIfcIoControl(control);
  bytes=await captureIfcGlbBytes(bytes,control);
  await ifcCheckpoint(control,"glb",0,1);
  if(bytes.length<20)throw new IfcStepError("glb.header","Truncated GLB");
  const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
  if(view.getUint32(0,true)!==0x46546c67||view.getUint32(4,true)!==2||view.getUint32(8,true)!==bytes.length)throw new IfcStepError("glb.header","Invalid GLB 2.0 header");
  let at=12,chunks=0,document:GltfDocument|undefined,binary:Uint8Array|undefined;
  while(at<bytes.length) {
    if(at+8>bytes.length)throw new IfcStepError("glb.chunk","Truncated chunk");
    const length=view.getUint32(at,true),type=view.getUint32(at+4,true);at+=8;
    if(length%4||at+length>bytes.length)throw new IfcStepError("glb.chunk","Invalid aligned chunk extent");
    if(!document) {if(type!==0x4e4f534a)throw new IfcStepError("glb.chunk","JSON must be first");document=await decodeIfcGltfJson(bytes.subarray(at,at+length),control);}
    else if(type===0x004e4942&&!binary&&chunks===1)binary=bytes.subarray(at,at+length);
    else if(type===0x4e4f534a||type===0x004e4942||!binary)throw new IfcStepError("glb.chunk","Duplicate or misplaced GLB chunk");
    at+=length;chunks++;
  }
  if(!document||!binary||document.buffers.length!==1||document.buffers[0]!.uri)throw new IfcStepError("glb.buffers","One embedded buffer required");
  if(!supportedVersion(document.asset))throw new IfcStepError("gltf.version","Supported glTF 2.0 asset version required");
  const size=Number(document.buffers[0]!.byteLength);
  if(!Number.isSafeInteger(size)||size<1||size>binary.length||binary.length-size>3)throw new IfcStepError("glb.length","Invalid declared buffer size");
  if(size>control.maxValues!)throw new IfcStepError("glb.limit","Buffer values exceed caller limit");
  if(binary.subarray(size).some(byte=>byte!==0))throw new IfcStepError("glb.padding","BIN padding must contain zero bytes");
  const buffer=new Array<number>(size);await copyGlbBytes(binary,buffer,size,control);
  await ifcCheckpoint(control,"glb",1,1);
  return {schema:"stdio.gltf",document,buffers:[buffer],sourceForm:"glb"};
}

const aggregate=(value:IfcValue[]):IfcValue=>({kind:"aggregate",value});
const text=(value:string):IfcValue=>({kind:"string",value});
const enumValue=(value:string):IfcValue=>({kind:"enum",value});
const unset:IfcValue={kind:"unset"};
const ref=(value:bigint):IfcValue=>({kind:"reference",value});

/** 🌱 A valid minimal IFC project/context in metres for real geometric projections. */
export function createIfc4x3ExchangeGraph(name:string):{snapshot:IfcSnapshot;add:(name:string,args:IfcValue[])=>IfcValue;context:IfcValue;worldCoordinateSystem:IfcValue} {
  const snapshot:IfcSnapshot={schema:"stdio.ifc",header:{fileDescription:[aggregate([text("ViewDefinition [notYetAssigned]")]),text("2;1")],fileName:[text(name),text(new Date().toISOString()),aggregate([text("")]),aggregate([text("")]),text("semio"),text("semio"),text("")],fileSchema:[aggregate([text("IFC4X3_ADD2")])]},entities:[]};
  let lastId=0n;
  const add=(name:string,args:IfcValue[]):IfcValue=>{const id=++lastId;snapshot.entities.push({id,name,args,complex:[]});return ref(id);};
  const origin=add("IFCCARTESIANPOINT",[aggregate([ifcReal(0),ifcReal(0),ifcReal(0)])]),axis=add("IFCAXIS2PLACEMENT3D",[origin,unset,unset]);
  const context=add("IFCGEOMETRICREPRESENTATIONCONTEXT",[unset,text("Model"),{kind:"integer",value:3n},ifcReal(1e-6),axis,unset]);
  const unit=add("IFCSIUNIT",[{kind:"derived"},enumValue("LENGTHUNIT"),unset,enumValue("METRE")]),units=add("IFCUNITASSIGNMENT",[aggregate([unit])]);
  add("IFCPROJECT",[text("0000000000000000000001"),unset,text(name),unset,unset,unset,unset,aggregate([context]),units]);
  return {snapshot,add,context,worldCoordinateSystem:axis};
}

function gltfNodeMatrix(node:GltfDocument["nodes"][number]):IfcMatrix {
  if(node.matrix) {if(node.matrix.length!==16||node.translation||node.rotation||node.scale)throw new IfcStepError("gltf.transform","An exclusive 16-entry matrix is required");const m=node.matrix.map(numeric) as IfcMatrix;if(m[3]||m[7]||m[11]||m[15]!==1)throw new IfcStepError("gltf.transform","Affine node matrix required");return m;}
  const [x,y,z,w]=node.rotation?.map(numeric)??[0,0,0,1],s=node.scale?.map(numeric)??[1,1,1],t=node.translation?.map(numeric)??[0,0,0];
  if(Math.abs(Math.hypot(x!,y!,z!,w!)-1)>1e-5)throw new IfcStepError("gltf.transform","Unit quaternion required");
  return [(1-2*(y!*y!+z!*z!))*s[0]!,2*(x!*y!+z!*w!)*s[0]!,2*(x!*z!-y!*w!)*s[0]!,0,2*(x!*y!-z!*w!)*s[1]!, (1-2*(x!*x!+z!*z!))*s[1]!,2*(y!*z!+x!*w!)*s[1]!,0,2*(x!*z!+y!*w!)*s[2]!,2*(y!*z!-x!*w!)*s[2]!, (1-2*(x!*x!+y!*y!))*s[2]!,0,...t,1] as IfcMatrix;
}

async function accessor(snapshot:GltfSnapshot,index:bigint,type:"VEC3"|"SCALAR",control:IfcIoControl):Promise<number[][]> {
  const doc=snapshot.document,a=doc.accessors[Number(index)];
  if(!a||a.type!==type||a.bufferView===undefined||a.sparse||a.normalized)throw new IfcStepError("gltf.accessor","Dense non-normalized accessor required");
  const v=doc.bufferViews[Number(a.bufferView)],buffer=v?snapshot.buffers[Number(v.buffer)]:undefined;
  const declared=v?doc.buffers[Number(v.buffer)]:undefined;
  if(!v||!buffer||!declared)throw new IfcStepError("gltf.accessor","Missing buffer view");
  const width=type==="VEC3"?3:1,size=a.componentType===5126||a.componentType===5125?4:a.componentType===5123?2:a.componentType===5121?1:0;
  if(!size||(type==="VEC3"&&a.componentType!==5126)||(type==="SCALAR"&&a.componentType===5126))throw new IfcStepError("gltf.accessor","Unsupported component type");
  const stride=Number(v.byteStride??BigInt(width*size)),offset=Number(a.byteOffset),viewOffset=Number(v.byteOffset),start=viewOffset+offset,count=Number(a.count),length=Number(v.byteLength),declaredLength=Number(declared.byteLength),extent=offset+(count-1)*stride+width*size;
  if(![stride,start,count,length,offset,viewOffset,declaredLength,extent,viewOffset+length].every(Number.isSafeInteger)||offset<0||viewOffset<0||length<0||count<1||count*width>control.maxValues!||stride<width*size||offset%size||viewOffset%size||(v.target!==undefined&&v.target!==(type==="VEC3"?34962n:34963n))||(type==="VEC3"&&v.byteStride!==undefined&&(stride%4!==0||stride>252))||(type==="SCALAR"&&v.byteStride!==undefined)||extent>length||viewOffset+length>buffer.length||viewOffset+length>declaredLength)throw new IfcStepError("gltf.accessor","Accessor alignment, role or extent is invalid");
  const data=new DataView(new ArrayBuffer(4)),rows:number[][]=[],batch=control.batchSize!;
  for(let i=0;i<count;i++) {
    if(i%batch===0)await ifcCheckpoint(control,"gltf-accessor",i,count);
    const row:number[]=[];
    for(let k=0;k<width;k++) {
      const at=start+i*stride+k*size;
      for(let byte=0;byte<size;byte++){const value=buffer[at+byte]!;if(!Number.isInteger(value)||value<0||value>255)throw new IfcStepError("gltf.accessor","Buffer bytes must be unsigned integers");data.setUint8(byte,value);}
      const n=a.componentType===5126?data.getFloat32(0,true):size===4?data.getUint32(0,true):size===2?data.getUint16(0,true):data.getUint8(0);if(!Number.isFinite(n))throw new IfcStepError("gltf.accessor","Non-finite coordinate");row.push(n);
    }
    rows.push(row);
  }
  await ifcCheckpoint(control,"gltf-accessor",count,count);
  return rows;
}

/** 📥 Import static triangle geometry as actual IFC face sets/proxies, never authored BIM parameters. */
export async function gltfToIfc4x3(snapshot:GltfSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<IfcSnapshot>> {
  const scope=captureIfcIoControlScope(control);control=scope.control;
  const captured=await captureIfcGltfSource(snapshot,control);if(!captured.artifact)return{fidelity:captured.fidelity,diagnostics:captured.diagnostics};snapshot=captured.artifact;
  const doc=snapshot.document,diagnostics:IfcExchangeDiagnostic[]=[];
  if(!supportedVersion(doc.asset)||doc.extensionsRequired.length||doc.animations.length||doc.skins.length)return {fidelity:"unsupported",diagnostics:[{code:"gltf.unsupported-semantics",severity:"error",message:"Only static glTF 2.0 without required extensions or skinning is supported",entityIds:[]}]};
  const graph=createIfc4x3ExchangeGraph("glTF geometry"),placement=graph.add("IFCLOCALPLACEMENT",[unset,graph.worldCoordinateSystem]),active=new Set<bigint>();let primitiveCount=0;
  const visit=async(index:bigint,parent:IfcMatrix):Promise<void>=>{
    if(active.has(index)||active.size>256)throw new IfcStepError("gltf.cycle","Cyclic or too deep node hierarchy");
    const node=doc.nodes[Number(index)];if(!node)throw new IfcStepError("gltf.node","Missing node");
    await ifcCheckpoint(control,"gltf-import",primitiveCount,doc.nodes.length);active.add(index);
    const transform=ifcMultiply(parent,gltfNodeMatrix(node));
    if(node.skin!==undefined||node.weights.length)throw new IfcStepError("gltf.deformation","Skinned and morphed nodes require evaluation");
    if(node.mesh!==undefined) {
      const mesh=doc.meshes[Number(node.mesh)];if(!mesh)throw new IfcStepError("gltf.mesh","Missing mesh");
      for(const primitive of mesh.primitives) {
        if((primitive.mode??4n)!==4n||primitive.targets.length)throw new IfcStepError("gltf.topology","Only static triangle primitives supported");
        const position=primitive.attributes.find(([key])=>key==="POSITION")?.[1];if(position===undefined)throw new IfcStepError("gltf.position","POSITION accessor required");
        const rows=await accessor(snapshot,position,"VEC3",control),pointsValues:IfcValue[]=[],batch=control.batchSize!;
        for(let index=0;index<rows.length;index++){
          if(index%batch===0)await ifcCheckpoint(control,"gltf-materialize-positions",index,rows.length);
          const [x,y,z]=ifcTransform(transform,rows[index] as [number,number,number]);pointsValues.push(aggregate([ifcReal(x),ifcReal(-z),ifcReal(y)]));
        }
        await ifcCheckpoint(control,"gltf-materialize-positions",rows.length,rows.length);
        const normalAccessor=primitive.attributes.find(([key])=>key==='NORMAL')?.[1],normalRows=normalAccessor===undefined?undefined:await accessor(snapshot,normalAccessor,'VEC3',control),normalValues:IfcValue[]=[];if(normalRows){if(normalRows.length!==rows.length)throw new IfcStepError('gltf.normals','NORMAL and POSITION accessor counts must agree');const normalTransform=IfcNormalTransform.capture(transform);for(let at=0;at<normalRows.length;at++){if(at%batch===0)await ifcCheckpoint(control,'gltf-materialize-normals',at,normalRows.length);const row=normalRows[at]! as [number,number,number],length=Math.hypot(...row);if(!Number.isFinite(length)||Math.abs(length-1)>1e-5)throw new IfcStepError('gltf.normals','Normalized nonzero NORMAL vectors required');const[x,y,z]=normalTransform.direction(row);normalValues.push(aggregate([ifcReal(x),ifcReal(-z),ifcReal(y)]));}await ifcCheckpoint(control,'gltf-materialize-normals',normalRows.length,normalRows.length);}
        const indices=primitive.indices===undefined?undefined:await accessor(snapshot,primitive.indices,"SCALAR",control),count=indices?.length??rows.length,indexAt=(index:number):number=>indices?indices[index]![0]!:index;
        if(count%3)throw new IfcStepError("gltf.indices","Invalid triangle indices");
        for(let index=0;index<count;index++){
          if(index%batch===0)await ifcCheckpoint(control,"gltf-materialize-indices",index,count);
          const value=indexAt(index);if(!Number.isSafeInteger(value)||value<0||value>=rows.length)throw new IfcStepError("gltf.indices","Invalid triangle indices");
        }
        await ifcCheckpoint(control,"gltf-materialize-indices",count,count);
        const determinant=ifcMatrixOrientation(transform);
        if(determinant===0)throw new IfcStepError("gltf.transform","Finite nonsingular transform required");
        const points=graph.add("IFCCARTESIANPOINTLIST3D",[aggregate(pointsValues),unset]),triangles:IfcValue[]=[],triangleCount=count/3,offsets=determinant<0?[0,2,1]:[0,1,2];
        for(let index=0;index<triangleCount;index++){
          if(index%batch===0)await ifcCheckpoint(control,"gltf-materialize-triangles",index,triangleCount);
          triangles.push(aggregate(offsets.map(offset=>({kind:"integer",value:BigInt(indexAt(index*3+offset)+1)}))));
        }
        await ifcCheckpoint(control,"gltf-materialize-triangles",triangleCount,triangleCount);
        const faces=graph.add("IFCTRIANGULATEDFACESET",[points,normalRows?aggregate(normalValues):unset,unset,aggregate(triangles),unset]);
        const rep=graph.add("IFCSHAPEREPRESENTATION",[graph.context,text("Body"),text("Tessellation"),aggregate([faces])]);
        const shape=graph.add("IFCPRODUCTDEFINITIONSHAPE",[unset,unset,aggregate([rep])]);
        graph.add("IFCBUILDINGELEMENTPROXY",[text((primitiveCount+2).toString().padStart(22,"0")),unset,text(node.name??mesh.name??""),unset,unset,placement,shape,unset,enumValue("NOTDEFINED")]);
        primitiveCount++;
        if(primitive.material!==undefined||primitive.attributes.some(([name])=>name!=="POSITION"&&name!=='NORMAL'))diagnostics.push(unsupported("Primitive materials and attributes other than POSITION/NORMAL are omitted"));
      }
    }
    for(const child of node.children)await visit(child,transform);active.delete(index);
  };
  try {
    const scene=doc.scene===undefined?doc.scenes[0]:doc.scenes[Number(doc.scene)];if(!scene)throw new IfcStepError("gltf.scene","An explicit scene is required");
    for(const root of scene.nodes)await visit(root,ifcIdentity());
  } catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return {fidelity:"unsupported",diagnostics:[...diagnostics,{code:"gltf.unsupported",severity:"error",message:String(error),entityIds:[]}]};}
  if(!primitiveCount)return {fidelity:"unsupported",diagnostics:[{code:"gltf.empty",severity:"error",message:"Scene has no triangle geometry",entityIds:[]}]};
  await ifcCheckpoint(control,"gltf-import",doc.nodes.length,doc.nodes.length);
  return {artifact:graph.snapshot,fidelity:"geometry",diagnostics:[...diagnostics,unsupported("Triangle proxies cannot reconstruct authored BIM parameters, structural analysis, spatial containment or original IFC relationships")]};
}
