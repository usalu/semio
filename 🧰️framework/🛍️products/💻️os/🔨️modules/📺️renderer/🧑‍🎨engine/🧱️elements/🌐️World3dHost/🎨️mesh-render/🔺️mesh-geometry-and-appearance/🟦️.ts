/** 🔺️ Mesh geometry and appearance: the buffer geometry of a mesh, the vertex pick data, the world surface material and appearance, the mesh visuals and the edge, face and vertex overlay geometry. */
import { BufferAttribute, BufferGeometry, DoubleSide, EdgesGeometry, MeshStandardMaterial, Texture, ClampToEdgeWrapping, RepeatWrapping, MirroredRepeatWrapping, NearestFilter, LinearFilter, NearestMipmapNearestFilter, LinearMipmapNearestFilter, NearestMipmapLinearFilter, LinearMipmapLinearFilter, TextureLoader, FrontSide, SRGBColorSpace, NoColorSpace } from "@semio-tech/ui-react";
import { type WorldMeshAttribute, type WorldMeshData, type WorldMeshRecord } from "../../🔭️scene-schema/🟦️.ts";
import { surfaceAttributes, surfaceSample, surfaceTuple } from "../🧱️mesh-parsing-and-residency/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** 🎨️ Builds authored draw samples while retaining triangle order and original picking identities. */
export function geometryFromMesh(mesh: WorldMeshData) {
  if (mesh.positions.length % 3 || mesh.indices.length % 3 || mesh.positions.some(value => !Number.isFinite(value)) || mesh.indices.some(value => !Number.isInteger(value) || value < 0 || value >= mesh.positions.length / 3)) throw new Error("surface mesh has invalid positions or triangle indices");
  const normal = surfaceAttributes(mesh,"normal")[0]?.[1];
  const color = surfaceAttributes(mesh,"color")[0]?.[1];
  const tangent=mesh.attributes?.tangent;
  const uv = surfaceAttributes(mesh,"uv");
  const channels = [normal,color,tangent,...uv.map(([,attribute])=>attribute)].filter((value):value is WorldMeshAttribute=>Boolean(value));
  for(const attribute of channels){
    const count=attribute.domain === "vertex" ? mesh.positions.length/3 : attribute.domain === "face" ? mesh.indices.length/3 : mesh.indices.length;
    if(!["vertex","corner","face"].includes(attribute.domain)||(attribute.indices?.length ?? attribute.values.length)!==count||attribute.indices?.some(index=>!Number.isInteger(index)||index<0||index>=attribute.values.length))throw new Error("surface attribute cardinality differs from its domain");
  }
  const geometry = new BufferGeometry();
  const expanded = channels.some(attribute=>attribute.domain !== "vertex");
  const corners = expanded ? mesh.indices.length : mesh.positions.length / 3;
  const positions = expanded ? mesh.indices.flatMap(vertex=>mesh.positions.slice(vertex*3,vertex*3+3)) : mesh.positions;
  geometry.setAttribute("position",new BufferAttribute(new Float32Array(positions),3));
  if (!expanded && mesh.indices.length) geometry.setIndex([...mesh.indices]);
  const bind = (name:string,attribute:WorldMeshAttribute|undefined,width:number,fallback:readonly number[]|undefined)=>{
    if (attribute) {
      const values:number[]=[];
      for(let index=0;index<corners;index++){
        const value=expanded ? surfaceSample(attribute,mesh,index) : attribute.values[attribute.indices?.[index] ?? index];
        const tuple=surfaceTuple(value,width,name);if(name === "normal" && tuple.every(value=>value === 0))throw new Error("surface normal cannot be zero");values.push(...tuple);
      }
      geometry.setAttribute(name,new BufferAttribute(new Float32Array(values),width));
    } else if (fallback?.length) {
      if (fallback.length !== mesh.positions.length/3*width) throw new Error(`${name} buffer cardinality differs from the vertex count`);
      const values=expanded ? mesh.indices.flatMap(vertex=>fallback.slice(vertex*width,vertex*width+width)) : fallback;
      geometry.setAttribute(name,new BufferAttribute(new Float32Array(values),width));
    }
  };
  try {
    bind("normal",normal,3,mesh.normals);
    bind("color",color,4,mesh.colors);
    bind("tangent",tangent,4,undefined);
    if(tangent){const values=geometry.getAttribute("tangent");for(let i=0;i<values.count;i++){if(values.getW(i)!==1 && values.getW(i)!==-1 || values.getX(i)===0 && values.getY(i)===0 && values.getZ(i)===0)throw new Error("surface tangent requires a nonzero direction and signed handedness");}}
    if (uv.length) uv.forEach(([name,attribute],index)=>{const declared=/^uv(\d+)?$/.exec(name);const set=declared ? Number(declared[1] ?? 0) : index;bind(set ? `uv${set}` : "uv",attribute,2,undefined);});
    else bind("uv",undefined,2,mesh.uvs);
    if (geometry.hasAttribute("uv") && !geometry.hasAttribute("uv1")) geometry.setAttribute("uv1",geometry.getAttribute("uv").clone());
    if (!geometry.hasAttribute("normal")) geometry.computeVertexNormals();
    return geometry;
  } catch(error) {geometry.dispose();throw error;}
}

export type VertexPickData = {
  readonly geometry: BufferGeometry;
  readonly vertexIds: readonly number[];
};

export function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
  if (!mesh.vertexIds?.length) return null;
  const positions: number[] = [];
  const vertexIds: number[] = [];
  const emitted = new Set<number>();
  for (let index = 0; index < mesh.vertexIds.length; index += 1) {
    const id = mesh.vertexIds[index]!;
    if (id === 0xffff_ffff || emitted.has(id)) continue;
    emitted.add(id);
    vertexIds.push(id);
    positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return { geometry, vertexIds };
}

export type WorldMeshAppearance = {
  readonly materials: MeshStandardMaterial[];
  readonly textures: Map<string, Texture>;
  readonly urls: Set<string>;
  readonly listeners: Set<()=>void>;
  pending: number;
  fault?: string;
  closed: boolean;
};

/** 🖍️ Applies owned material coefficients using glTF alpha and color-space conventions. */
export function worldSurfaceMaterial(name:string,source:Record<string,unknown>,vertexColors:boolean):MeshStandardMaterial {
  const base=surfaceTuple(source.baseColor ?? [1,1,1,1],4,"baseColor");
  const emissive=surfaceTuple(source.emissive ?? [0,0,0],3,"emissive");
  const alpha=source.alphaMode ?? "OPAQUE";
  if (!["OPAQUE","MASK","BLEND"].includes(String(alpha))) throw new Error("surface material has an invalid alpha mode");
  const coefficient=(name:string,defaultValue:number,maximum=1)=>{const value=source[name]??defaultValue;if(typeof value!=="number"||!Number.isFinite(Math.fround(value))||value<0||value>maximum)throw new Error(`surface material ${name} requires a coefficient in [0,${maximum}]`);return value;};
  if(base.some(value=>value<0||value>1)||emissive.some(value=>value<0))throw new Error("surface color coefficients are outside their domain");
  const normalScale=surfaceTuple(Array.isArray(source.normalScale)?source.normalScale:[source.normalScale ?? 1,source.normalScale ?? 1],2,"normalScale");if(normalScale.some(value=>Math.abs(value)>3.4028234663852886e38))throw new Error("surface material normalScale requires finite coefficients");
  const occlusionStrength=coefficient("occlusionStrength",1);
  const coordinates=source.textureCoordinates;if(coordinates!==undefined && (typeof coordinates!=="object"||coordinates===null||Array.isArray(coordinates)||Object.entries(coordinates).some(([key,value])=>!["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].includes(key)||!Number.isInteger(value)||(value as number)<0||(value as number)>63)))throw new Error("surface material has invalid UV selectors");
  const material=new MeshStandardMaterial({name,vertexColors,metalness:coefficient("metallic",1),roughness:coefficient("roughness",1),opacity:alpha === "OPAQUE" ? 1 : base[3],transparent:alpha === "BLEND",alphaTest:alpha === "MASK" ? coefficient("alphaCutoff",.5,3.4028234663852886e38) : 0,depthWrite:alpha !== "BLEND",side:source.doubleSided === true ? DoubleSide : FrontSide});
  material.color.setRGB(base[0]!,base[1]!,base[2]!);
  material.emissive.setRGB(emissive[0]!,emissive[1]!,emissive[2]!);
  material.normalScale.set(normalScale[0]!,normalScale[1]!);material.aoMapIntensity=occlusionStrength;
  const extra=[...new Set(Object.values((coordinates ?? {}) as Record<string,number>).filter(set=>set>3))].sort((a,b)=>a-b);
  if(extra.length){material.onBeforeCompile=shader=>{shader.vertexShader=extra.map(set=>`attribute vec2 uv${set};`).join("\n")+"\n"+shader.vertexShader;};material.customProgramCacheKey=()=>`surface-uv:${extra.join(",")}`;}
  return material;
}

function buildWorldMeshAppearance(mesh:WorldMeshData,geometry:BufferGeometry):WorldMeshAppearance|null {
  const assignments=surfaceAttributes(mesh,"material")[0]?.[1];
  if (!assignments && !Object.keys(mesh.materials ?? {}).length) return null;
  const appearance:WorldMeshAppearance={materials:[],textures:new Map(),urls:new Set(),listeners:new Set(),pending:0,closed:false};
  const names:string[]=[];
  try {
    for(let corner=0;corner<mesh.indices.length;corner+=3){
      const name=assignments ? surfaceSample(assignments,mesh,corner) : Object.keys(mesh.materials!)[0];
      if (typeof name !== "string" || !mesh.materials?.[name]) throw new Error("face material references an unowned material");
      let index=names.indexOf(name);
      if(index<0){index=names.length;names.push(name);appearance.materials.push(worldSurfaceMaterial(name,mesh.materials[name]!,geometry.hasAttribute("color")));}
      const previous=geometry.groups.at(-1);
      if(previous?.materialIndex===index) previous.count+=3;
      else geometry.addGroup(corner,3,index);
    }
    for(const [index,name] of names.entries()){
      const source=mesh.materials![name]!;const material=appearance.materials[index]!;
      for(const [field,roles,srgb] of [["baseColorTexture",["map"],true],["metallicRoughnessTexture",["metalnessMap","roughnessMap"],false],["normalTexture",["normalMap"],false],["occlusionTexture",["aoMap"],false],["emissiveTexture",["emissiveMap"],true]] as const){
        const id=source[field];if(id===undefined)continue;
        if(typeof id!=="string"||!mesh.textures?.[id])throw new Error(`${field} references an unowned texture`);
        const coordinates=source.textureCoordinates as Record<string,number>|undefined;const set=coordinates?.[field] ?? 0;
        if(!geometry.hasAttribute(set ? `uv${set}` : "uv"))throw new Error(`${field} references an absent UV set`);
        const sampler=(source.textureSamplers as Record<string,Record<string,number>>|undefined)?.[field] ?? {};
        if(Object.entries(sampler).some(([field,value])=>!({wrapS:[33071,33648,10497],wrapT:[33071,33648,10497],magFilter:[9728,9729],minFilter:[9728,9729,9984,9985,9986,9987]} as Record<string,number[]>)[field]?.includes(value)))throw new Error("surface material texture sampler is invalid");
        const owned=mesh.textures[id]!;const key=`${id}:${srgb ? "srgb" : "linear"}:${set}:${sampler.wrapS ?? 10497}:${sampler.wrapT ?? 10497}:${sampler.magFilter ?? 9729}:${sampler.minFilter ?? 9729}`;
        let texture=appearance.textures.get(key);
        if(!texture){
          if(!["image/png","image/jpeg"].includes(owned.mime)||owned.bytes.length>16_000_000||owned.bytes.some(value=>!Number.isInteger(value)||value<0||value>255))throw new Error("surface texture requires bounded owned PNG/JPEG bytes");
          texture=new Texture();texture.colorSpace=srgb ? SRGBColorSpace : NoColorSpace;texture.channel=set;texture.flipY=false;
          const wraps={33071:ClampToEdgeWrapping,33648:MirroredRepeatWrapping,10497:RepeatWrapping};const filters={9728:NearestFilter,9729:LinearFilter,9984:NearestMipmapNearestFilter,9985:LinearMipmapNearestFilter,9986:NearestMipmapLinearFilter,9987:LinearMipmapLinearFilter};
          texture.wrapS=wraps[(sampler.wrapS ?? 10497) as keyof typeof wraps];texture.wrapT=wraps[(sampler.wrapT ?? 10497) as keyof typeof wraps];texture.magFilter=filters[(sampler.magFilter ?? 9729) as 9728|9729];texture.minFilter=filters[(sampler.minFilter ?? 9729) as keyof typeof filters];appearance.textures.set(key,texture);
          if(typeof URL.createObjectURL==="function"){
            const url=URL.createObjectURL(new Blob([Uint8Array.from(owned.bytes)],{type:owned.mime}));appearance.urls.add(url);appearance.pending++;
            const target=texture;
            new TextureLoader().load(url,loaded=>{URL.revokeObjectURL(url);appearance.urls.delete(url);if(!appearance.closed){target.image=loaded.image;target.needsUpdate=true;appearance.pending--;for(const listener of appearance.listeners)listener();}loaded.dispose();},undefined,()=>{URL.revokeObjectURL(url);appearance.urls.delete(url);if(!appearance.closed){appearance.pending--;appearance.fault=`texture ${id} could not be decoded`;for(const listener of appearance.listeners)listener();}});
          }
        }
        for(const role of roles)material[role]=texture;
      }
    }
    return appearance;
  }catch(error){disposeWorldMeshAppearance(appearance);throw error;}
}

export function disposeWorldMeshAppearance(appearance:WorldMeshAppearance):void {
  if(appearance.closed)return;appearance.closed=true;for(const listener of appearance.listeners)listener();appearance.listeners.clear();
  for(const material of appearance.materials)material.dispose();
  for(const texture of appearance.textures.values())texture.dispose();
  for(const url of appearance.urls)URL.revokeObjectURL(url);appearance.urls.clear();
}

/** 🧊️ One existing mesh visual lease owns every draw buffer, material and decoded texture. */
export type MeshVisuals = { readonly record:WorldMeshRecord;readonly geometry:BufferGeometry|null;readonly border:EdgesGeometry|null;readonly vertexPick:VertexPickData|null;readonly edge:BufferGeometry|null;readonly appearance:WorldMeshAppearance|null;readonly fault?:string;closed:boolean; };

/** 🧊️ Builds authored visuals at the existing retained record boundary. */
export function buildMeshVisuals(record:WorldMeshRecord):MeshVisuals {
  if(!record.data)return {record,geometry:null,border:null,vertexPick:null,edge:null,appearance:null,closed:false};
  const geometry=geometryFromMesh(record.data);
  if(record.data.heatmap)geometry.userData.world3dHeatmap=true;
  let border:EdgesGeometry|null=null;let vertexPick:VertexPickData|null=null;let edge:BufferGeometry|null=null;let appearance:WorldMeshAppearance|null=null;
  try {appearance=buildWorldMeshAppearance(record.data,geometry);border=new EdgesGeometry(geometry);vertexPick=buildVertexPickData(record.data);edge=buildEdgeGeometry(record.data);return {record,geometry,border,vertexPick,edge,appearance,closed:false};}
  catch(error){geometry.dispose();border?.dispose();vertexPick?.geometry.dispose();edge?.dispose();if(appearance)disposeWorldMeshAppearance(appearance);throw error;}
}

/** 🧹️ Retires each visual-owned resource exactly once; loader-shared GLB assets remain owned by their loader. */
export function disposeMeshVisuals(visuals:MeshVisuals):void {
  if(visuals.closed)return;visuals.closed=true;
  visuals.geometry?.dispose();visuals.border?.dispose();visuals.vertexPick?.geometry.dispose();visuals.edge?.dispose();
  if(visuals.appearance)disposeWorldMeshAppearance(visuals.appearance);
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
  if (!mesh.edgePositions?.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
  return geometry;
}

/** 🧵️ Curve/centerline meshes have edge samples but no shaded triangles — pick/hover must treat them as whole instances. */
export function isCurveOnlyWorldMesh(mesh: Pick<WorldMeshData, "indices" | "edgePositions">): boolean {
  return Boolean(mesh.edgePositions?.length) && !(mesh.indices?.length > 0);
}

export function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.faceIds?.length || !mesh.indices.length || faceIds.size === 0) return null;
  const positions: number[] = [];
  const normals: number[] = [];
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    const faceId = mesh.faceIds[faceIndex]!;
    if (!faceIds.has(faceId)) continue;
    const i0 = mesh.indices[faceIndex * 3] ?? 0;
    const i1 = mesh.indices[faceIndex * 3 + 1] ?? 0;
    const i2 = mesh.indices[faceIndex * 3 + 2] ?? 0;
    for (const index of [i0, i1, i2]) {
      positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
      normals.push(mesh.normals[index * 3]!, mesh.normals[index * 3 + 1]!, mesh.normals[index * 3 + 2]!);
    }
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(normals), 3));
  return geometry;
}

/** 🖱️➡️ Approximates a picked face's in-plane size from its triangles' local bounding box, dropping the
 * smallest axis (roughly the one aligned with the face normal for axis-aligned primitive faces) — good
 * enough to size a push/pull tool's footprint without needing a true tangent-plane projection. */
export function faceExtentFromMesh(mesh: WorldMeshData, faceId: number): readonly [number, number] | undefined {
  if (!mesh.faceIds?.length || !mesh.indices.length) return undefined;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  let found = false;
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    if (mesh.faceIds[faceIndex] !== faceId) continue;
    found = true;
    for (const corner of [0, 1, 2]) {
      const vertexIndex = mesh.indices[faceIndex * 3 + corner];
      if (vertexIndex == null) continue;
      const x = mesh.positions[vertexIndex * 3] ?? 0;
      const y = mesh.positions[vertexIndex * 3 + 1] ?? 0;
      const z = mesh.positions[vertexIndex * 3 + 2] ?? 0;
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }
  }
  if (!found) return undefined;
  const extents = [maxX - minX, maxY - minY, maxZ - minZ].sort((a, b) => b - a);
  return [extents[0] ?? 0.2, extents[1] ?? 0.2];
}

export function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.edgeIds?.length || !mesh.edgePositions?.length || edgeIds.size === 0) return null;
  const positions: number[] = [];
  for (let edgeIndex = 0; edgeIndex < mesh.edgeIds.length; edgeIndex += 1) {
    if (!edgeIds.has(mesh.edgeIds[edgeIndex]!)) continue;
    const base = edgeIndex * 6;
    positions.push(mesh.edgePositions[base]!, mesh.edgePositions[base + 1]!, mesh.edgePositions[base + 2]!, mesh.edgePositions[base + 3]!, mesh.edgePositions[base + 4]!, mesh.edgePositions[base + 5]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

export function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
  const pick = buildVertexPickData(mesh);
  if (!pick) return null;
  const positions: number[] = [];
  for (let index = 0; index < pick.vertexIds.length; index += 1) {
    if (!vertexIds.has(pick.vertexIds[index]!)) continue;
    positions.push(pick.geometry.attributes.position!.getX(index), pick.geometry.attributes.position!.getY(index), pick.geometry.attributes.position!.getZ(index));
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
