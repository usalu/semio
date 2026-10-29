/** 🚗 Independent source-bound parking polygons; this test module adds no product dependency. */
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import clipping from 'polygon-clipping';

const EPS=1e-8;
const canonical=value=>Array.isArray(value)?value.map(canonical):value&&typeof value==='object'?Object.fromEntries(Object.keys(value).sort().map(k=>[k,canonical(value[k])])):typeof value==='number'?Math.floor(value*1e9+.5):value;
const pick=(obj,keys)=>Object.fromEntries(keys.filter(k=>k in obj).map(k=>[k,obj[k]]));
const quantity=value=>typeof value==='number'?value:value.value;

/** 📍 Resolve only authored datums and regional slab soffits. */
export function level(source,value){
 if(typeof value==='number')return value;
 const storey=source.storeys.find(s=>s.tag===value);if(storey)return storey.elevation.value;
 if(value.startsWith('soffit:')){const slab=source.elements.find(e=>e.tag===value.slice(7));const base=level(source,slab.geometry.level);return Math.min(...slab.geometry.regions.map(r=>base+r.top_offset.value-r.thickness.value));}
 throw Error('Unknown parking datum '+value);
}

/** 🧱 Independently build every column section intersecting the complete vehicle height. */
export function physicalColumns(source,car){
 const result=[];
 for(const e of source.elements.filter(e=>e.class==='IfcColumn')){
  const type=source.types.find(t=>t.tag===e.type),g=e.geometry;
  for(const section of g.sections??[{profile:type.profile,axis:g.axis,base_level:g.base_level,top_level:g.top_level}]){
   const z0=level(source,section.base_level),z1=level(source,section.top_level);if(Math.min(z1,car.z0+car.height)-Math.max(z0,car.z0)<=1e-9)continue;
   const p=section.profile,axis=section.axis??g.axis,angle=(p.rotation??0)*Math.PI/180,c=Math.cos(angle),s=Math.sin(angle);
   let ring;
   if(p.shape==='circle')ring=Array.from({length:96},(_,i)=>[quantity(p.diameter)/2*Math.cos(i*Math.PI/48),quantity(p.diameter)/2*Math.sin(i*Math.PI/48)]);
   else if(p.shape==='ellipse')ring=Array.from({length:96},(_,i)=>[quantity(p.width)/2*Math.cos(i*Math.PI/48),quantity(p.depth)/2*Math.sin(i*Math.PI/48)]);
   else if(p.shape==='rectangle')ring=[[-1,-1],[1,-1],[1,1],[-1,1]].map(([x,y])=>[x*quantity(p.width)/2,y*quantity(p.depth)/2]);
   else if(p.shape==='polygon')ring=p.outline;else throw Error('Unresolved column '+e.tag);
   result.push({tag:e.tag,outline:ring.map(([x,y])=>[axis[0]+x*c-y*s,axis[1]+x*s+y*c]),z0,z1});
  }
 }
 return result;
}

/** 🔒 Bind the portable geometry oracle to the physical source inputs, excluding labels and relations. */
export function sourceSignature(source,car){
 const active=new Set(physicalColumns(source,car).map(c=>c.tag));
 const bounds=p=>p.mesh?[Math.min(...p.mesh.vertices.map(v=>v[2])),Math.max(...p.mesh.vertices.map(v=>v[2]))]:p.section?[Math.min(...p.section.profile.map(v=>v[1])),Math.max(...p.section.profile.map(v=>v[1]))]:[p.z0,p.z1];
 const zOverlap=g=>{try{const base=level(source,g.base_level??'ground');return g.parts?.some(p=>{const [lo,hi]=bounds(p);return Math.min(base+hi,car.z0+car.height)-Math.max(base+lo,car.z0)>1e-9;})??true;}catch{return true;}};
 const elements=source.elements.filter(e=>e.tag==='SL-1'||active.has(e.tag)||(['ground','cellar'].includes(e.storey)&&zOverlap(e.geometry))).map(e=>pick(e,['tag','class','predefined_type','storey','type','material','geometry','attributes'])).sort((a,b)=>a.tag<b.tag?-1:a.tag>b.tag?1:0);
 const wantedTypes=new Set(elements.map(e=>e.type).filter(Boolean));
 const types=source.types.filter(t=>wantedTypes.has(t.tag)).map(t=>pick(t,['tag','class','material','profile','filling','attributes'])).sort((a,b)=>a.tag<b.tag?-1:a.tag>b.tag?1:0);
 const wantedMaterials=new Set([...elements,...types].flatMap(e=>[e.material,e.filling?.frame_material,e.filling?.panel_material]).filter(Boolean));
 let changed=true;while(changed){changed=false;for(const m of source.materials)if(wantedMaterials.has(m.tag))for(const l of m.layers??[])if(!wantedMaterials.has(l.material)){wantedMaterials.add(l.material);changed=true;}}
 const materials=source.materials.filter(m=>wantedMaterials.has(m.tag)).map(m=>pick(m,['tag','class','layers'])).sort((a,b)=>a.tag<b.tag?-1:a.tag>b.tag?1:0);
 const input={scenario_vehicle:car,elements,types,materials,storeys:source.storeys.map(s=>({tag:s.tag,elevation:s.elevation.value})),car:source.car};
 return createHash('sha256').update(JSON.stringify(canonical(input))).digest('hex');
}

const area=polygons=>polygons.reduce((s,p)=>s+p.reduce((t,r,i)=>t+(i?-1:1)*Math.abs(r.reduce((v,a,j)=>{const b=r[(j+1)%r.length];return v+a[0]*b[1]-b[0]*a[1];},0))/2,0),0);
const intersect=(a,b)=>area(clipping.intersection([a],[b]));
const pointDistance=(p,a,b)=>{const dx=b[0]-a[0],dy=b[1]-a[1],d=dx*dx+dy*dy,t=d?Math.max(0,Math.min(1,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/d)):0;return Math.hypot(p[0]-a[0]-t*dx,p[1]-a[1]-t*dy);};
const cross=(a,b,c)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
const segmentDistance=(a,b,c,d)=>{if(cross(a,b,c)*cross(a,b,d)<=0&&cross(c,d,a)*cross(c,d,b)<=0&&Math.max(Math.min(a[0],b[0]),Math.min(c[0],d[0]))<=Math.min(Math.max(a[0],b[0]),Math.max(c[0],d[0]))+EPS&&Math.max(Math.min(a[1],b[1]),Math.min(c[1],d[1]))<=Math.min(Math.max(a[1],b[1]),Math.max(c[1],d[1]))+EPS)return 0;return Math.min(pointDistance(a,c,d),pointDistance(b,c,d),pointDistance(c,a,b),pointDistance(d,a,b));};
const edgeDistance=(a,b)=>Math.min(...a.flatMap((p,i)=>b.map((q,j)=>segmentDistance(p,a[(i+1)%a.length],q,b[(j+1)%b.length]))));
const polygonDistance=(a,b)=>intersect(a,b)>EPS?0:edgeDistance(a,b);
export const rectangle=(pose,car)=>{const angle=pose.angle_degrees*Math.PI/180,c=Math.cos(angle),s=Math.sin(angle);return [[-1,-1],[1,-1],[1,1],[-1,1]].map(([x,y])=>[pose.center[0]+x*car.length/2*c-y*car.width/2*s,pose.center[1]+x*car.length/2*s+y*car.width/2*c]);};

/** ✂️ Independently project polygon-clipped native sections inside a vehicle height band. */
function convexHull(points){
 const sorted=points.slice().sort((a,b)=>a[0]-b[0]||a[1]-b[1]),cross=(a,b,c)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
 const half=items=>{const out=[];for(const p of items){while(out.length>=2&&cross(out.at(-2),out.at(-1),p)<=0)out.pop();out.push(p);}return out;};return half(sorted).slice(0,-1).concat(half(sorted.slice().reverse()).slice(0,-1));
}

export function bandFootprints(part,low,high){
 const lo=Math.max(low,part.z0),hi=Math.min(high,part.z1);if(hi-lo<=1e-10)return [];
 if(part.section){const s=part.section,cut=clipping.intersection([s.profile],[[[-1e6,lo],[1e6,lo],[1e6,hi],[-1e6,hi]]]);return cut.map(poly=>{const xs=poly[0].map(p=>p[0]),u=Math.min(...xs),v=Math.max(...xs),[a,b]=s.sweep;return s.plane==='XZ'?[[u,a],[v,a],[v,b],[u,b]]:[[a,u],[b,u],[b,v],[a,v]];});}
 if(part.mesh){const v=part.mesh.vertices,points=v.filter(p=>p[2]>=lo&&p[2]<=hi).map(p=>p.slice(0,2));for(const face of part.mesh.faces)for(let i=0;i<face.length;i++){const a=v[face[i]],b=v[face[(i+1)%face.length]];for(const z of [lo,hi])if((a[2]-z)*(b[2]-z)<0)points.push(a.slice(0,2).map((x,j)=>x+(b[j]-x)*(z-a[2])/(b[2]-a[2])));}return [convexHull(points)];}
 if(!part.top_outline)return [part.outline];
 const points=[lo,hi].flatMap(z=>part.outline.map((p,i)=>p.map((v,j)=>v+(part.top_outline[i][j]-v)*(z-part.z0)/(part.z1-part.z0))));
 return [convexHull(points)];
}

/** 🅿️ Evaluate physical static parking without assuming an ingress maneuver. */
export function evaluateParking(fixture,source,poses){
 const cars=poses.map(p=>rectangle(p,fixture.car)),columns=physicalColumns(source,fixture.car),violations=[],measurements=[];
 for(const [i,car]of cars.entries()){
  const outside=area(clipping.difference([car],[fixture.outline])),boundary=edgeDistance(car,fixture.outline);
  if(outside>EPS)violations.push({kind:'outside',car:i,value:outside});
  if(boundary+EPS<fixture.clearance.boundary)violations.push({kind:'boundary',car:i,value:boundary,required:fixture.clearance.boundary});
  let column=Infinity,physical=Infinity;
  for(const c of columns){const d=polygonDistance(car,c.outline);column=Math.min(column,d);if(d+EPS<fixture.clearance.column_surface)violations.push({kind:'column',car:i,tag:c.tag,value:d,required:fixture.clearance.column_surface});}
  for(const o of fixture.physical_obstacles){if(o.kind==='column')continue;const d=polygonDistance(car,o.outline),a=intersect(car,o.outline);physical=Math.min(physical,d);if(a>EPS||d+EPS<fixture.clearance.physical_surface)violations.push({kind:'physical',car:i,tag:o.tag,value:d,area:a});}
  for(const r of fixture.door_access_reserves){const a=intersect(car,r.polygon);if(a>EPS)violations.push({kind:'reserve',car:i,tag:r.opening,value:a});}
  measurements.push({outside_area:outside,boundary_distance:boundary,column_surface_distance:column,physical_surface_distance:physical});
 }
 const pair_gaps=[];for(let i=0;i<cars.length;i++)for(let j=0;j<i;j++){const gap=polygonDistance(cars[i],cars[j]);pair_gaps.push(gap);if(gap+EPS<fixture.clearance.car_pair)violations.push({kind:'pair',cars:[j,i],value:gap,required:fixture.clearance.car_pair});}
 return {valid:!violations.length,violations,cars:measurements,pair_gaps};
}

/** 🧪 Verify the source binding, positive control and explicitly rejected three-car demonstrations. */
export async function validateParking(){
 const fixture=JSON.parse(await readFile(new URL('./🚗️parking.json',import.meta.url),'utf8'));
 const source=JSON.parse(await readFile(new URL('../🔣️.json',import.meta.url),'utf8'));
 assert.equal(sourceSignature(source,fixture.car),fixture.source_signature,'Physical parking inputs changed; independently review the geometry oracle');
 for(const c of fixture.signature_cases){
  const empty={elements:[],types:[],materials:[],storeys:[{tag:'ground',elevation:{value:0}}],car:{}};
  const candidate={...empty,elements:[{tag:'TEST-MESH',class:'IfcRailing',storey:'ground',geometry:{kind:'solid',base_level:c.base_level,parts:[c.part]}}]};
  assert.equal(sourceSignature(candidate,fixture.car)!==sourceSignature(empty,fixture.car),c.included,c.id);
 }
 const space=source.spaces.find(s=>s.tag===fixture.garage);assert.deepEqual(space.outline,fixture.outline);
 assert.deepEqual([...new Set(physicalColumns(source,fixture.car).map(c=>c.tag))].sort(),fixture.active_column_tags);
 for(const c of fixture.band_cases.cases){const actual=bandFootprints(c.part,...fixture.band_cases.band);assert.ok(area(clipping.xor(actual.map(r=>[r]),c.expected.map(r=>[r])))<1e-9,c.id);}
 assert.equal(fixture.target.fleet_size,3);assert.equal(fixture.target.status,'unverified-current-source');assert.equal(fixture.target.global_infeasibility_proven,false);
 const registry=Object.fromEntries([...source.elements,...source.types].map(e=>[e.tag,e]));
 for(const reserve of fixture.door_access_reserves){
  const op=registry[reserve.opening],wall=registry[op.geometry.wall],fill=source.elements.find(e=>e.geometry.opening===op.tag),spec=registry[fill.type].filling;
  const [a,b]=wall.geometry.path,len=Math.hypot(b[0]-a[0],b[1]-a[1]),w=spec.frame_width.value;
  const stations=[op.attributes.start.value+w,op.attributes.end.value-w];
  stations.forEach((v,i)=>{const p=[a[0]+(b[0]-a[0])*v/len,a[1]+(b[1]-a[1])*v/len],q=reserve.polygon[reserve.portal_indices[i]];assert.ok(Math.hypot(p[0]-q[0],p[1]-q[1])<1e-7,reserve.opening);});
 }
 assert.ok(fixture.physical_obstacles.some(o=>o.tag===fixture.garage_portal.filling&&o.role==='panel'));
 assert.ok(fixture.physical_obstacles.some(o=>o.tag==='CW-1'&&o.role==='glass'));
 const verified=evaluateParking(fixture,source,fixture.verified_control.poses);assert.equal(verified.valid,true,JSON.stringify(verified.violations));
 const rejected=fixture.rejected_demonstrations.map(c=>{const result=evaluateParking(fixture,source,c.poses);assert.equal(result.valid,false,c.id);for(const tag of c.required_collision_tags??[])assert.ok(result.violations.some(v=>v.tag===tag),tag);for(const kind of c.required_violation_kinds??[])assert.ok(result.violations.some(v=>v.kind===kind),kind);return {id:c.id,...result};});
 return {target:fixture.target,historical_capacity:fixture.historical_capacity,verified_fleet_size:fixture.verified_control.poses.length,verified,rejected};
}
      