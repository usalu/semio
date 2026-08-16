/** 🔨️ Typed, non-public glTF geometry kernel shared by TypeScript inference leaves. */
export type GltfVector3 = readonly [number, number, number];
export type GltfTriangle = readonly [number, number, number];
export interface GltfTsGeometryContext {
  points: readonly GltfVector3[];
  triangles: readonly GltfTriangle[];
  sampleCount: number;
  valid: boolean;
  diagnostics: readonly string[];
  topology?: { watertight: boolean; manifold: boolean; consistentlyOriented: boolean };
}
export interface GltfTsBounds3 { min: GltfVector3; max: GltfVector3; dimensions: GltfVector3 }
export interface GltfTsQuality { method: 'exact'|'estimate'; coverage: number; sampleCount: number; watertight?: boolean; manifold?: boolean; consistentlyOriented?: boolean }
export interface GltfTsMeasure<T> { value?: T; unit: string; validity: 'available'|'unavailable'; quality: GltfTsQuality; diagnostics: readonly string[]; provenance: { algorithm: string; algorithmVersion: number; policy: string } }
const sub=(a:GltfVector3,b:GltfVector3):[number,number,number]=>[a[0]-b[0],a[1]-b[1],a[2]-b[2]];
const dot=(a:GltfVector3,b:GltfVector3)=>a[0]*b[0]+a[1]*b[1]+a[2]*b[2];
const cross=(a:GltfVector3,b:GltfVector3):[number,number,number]=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const length=(a:GltfVector3)=>Math.sqrt(dot(a,a));
export const bounds=(points:readonly GltfVector3[]):GltfTsBounds3|undefined=>{ if(!points.length)return; const min:[number,number,number]=[...points[0]],max:[number,number,number]=[...points[0]]; for(const p of points)for(let i=0;i<3;i++){min[i]=Math.min(min[i],p[i]);max[i]=Math.max(max[i],p[i]);}return{min,max,dimensions:sub(max,min)}; };
export const surfaceArea=(c:GltfTsGeometryContext)=>c.triangles.reduce((sum,[a,b,d])=>{const p=c.points[a],q=c.points[b],r=c.points[d];return p&&q&&r?sum+length(cross(sub(q,p),sub(r,p)))/2:sum;},0);
export const signedVolume=(c:GltfTsGeometryContext)=>c.triangles.reduce((sum,[a,b,d])=>{const p=c.points[a],q=c.points[b],r=c.points[d];return p&&q&&r?sum+dot(p,cross(q,r))/6:sum;},0);
const quality=(c:GltfTsGeometryContext,method:'exact'|'estimate',coverage:number):GltfTsQuality=>({method,coverage,sampleCount:c.sampleCount,...c.topology});
export const exact=<T>(c:GltfTsGeometryContext,value:T,unit:string):GltfTsMeasure<T>=>({value,unit,validity:'available',quality:quality(c,'exact',c.points.length?1:0),diagnostics:c.diagnostics,provenance:{algorithm:'s.stdio.gltf.geometry',algorithmVersion:2,policy:'gltf-geometry-policy-v2-1e-9-1e-7-4096'}});
export const unavailable=<T>(c:GltfTsGeometryContext,unit:string):GltfTsMeasure<T>=>({unit,validity:'unavailable',quality:quality(c,'estimate',0),diagnostics:c.diagnostics,provenance:{algorithm:'s.stdio.gltf.geometry',algorithmVersion:2,policy:'gltf-geometry-policy-v2-1e-9-1e-7-4096'}});
