import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {BoxGeometry,Vector3,Triangle,Group,Mesh} from "three";
/** 🧱️ Independent Three preserves every original child identity while its source grows. */
test("original arena growth identity agrees with Three",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️reachability/🔣️.json",import.meta.url),"utf8")).arenaGrowth;
  const source=new Group(),original:Group[]=[];
  for(let slot=0;slot<law.slots;slot++){const incoming=new Group();source.add(incoming);original.push(incoming);expect(source.children[slot]).toBe(incoming);expect(incoming.parent).toBe(source);}
  expect(source.children).toEqual(original);
  for(const incoming of original)expect(incoming.parent).toBe(source);
  console.log("[DEBUG] Original arena independent Three growth slots=1337 sameOriginalSlots=true");
});
/** 🎟️ Independent Three topology and strict neutral ownership authority. */
test("original ReachSet neutral topology agrees with Three",()=>{
  const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));const law=read("../../🧫️fixtures/🎟️reachability/🔣️.json");const schema=read("../../🧬️schema/🎟️reachability/🔣️.json");const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);const missing={...law};delete missing.physical;expect(validate(missing)).toBe(false);
  const geometry=new BoxGeometry(1,1,1);const position=geometry.getAttribute("position");const index=geometry.getIndex()!;const vertices=new Set<string>();const edges=new Set<string>();let area=0,volume=0;
  for(let face=0;face<6;face++){const corners=[0,1,3,2].map(offset=>new Vector3().fromBufferAttribute(position,face*4+offset));for(let i=0;i<4;i++){const key=(point:Vector3)=>point.toArray().join(",");vertices.add(key(corners[i]));edges.add([key(corners[i]),key(corners[(i+1)%4])].sort().join("|"));}}
  for(let cursor=0;cursor<index.count;cursor+=3){const [a,b,c]=[0,1,2].map(offset=>new Vector3().fromBufferAttribute(position,index.getX(cursor+offset)));area+=new Triangle(a,b,c).getArea();volume+=a.dot(b.clone().cross(c))/6;}
  expect(vertices.size).toBe(law.box.vertices);expect(edges.size).toBe(law.box.edges);expect(position.count).toBe(law.box.coedges);expect(geometry.groups.length).toBe(law.box.faces);expect(area).toBe(law.box.area);expect(Math.abs(volume)).toBeCloseTo(law.box.volume,12);geometry.dispose();console.log("[DEBUG] Original ReachSet independent Three vertices=8 edges=12 coedges=24 faces=6 area=6 volume=1");
});

/** 🗄️ Independent Three keeps exact original object identities through the neutral remove/reuse order. */
test("original arena neutral reuse identity agrees with Three",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️reachability/🔣️.json",import.meta.url),"utf8")).arenaRemoval;const source=new Group();const objects=Array.from({length:law.slots},()=>new Group());source.add(...objects);for(let cycle=0;cycle<law.cycles;cycle++){for(const slot of law.removeOrder)source.remove(objects[slot]);expect(source.children.length).toBe(law.slots-law.removeOrder.length);const reused=law.reuseOrder.map((slot:number)=>objects[slot]);source.add(...reused);expect(source.children.slice(-reused.length)).toEqual(reused);expect(source.children.length).toBe(law.slots);for(const object of objects)expect(object.parent).toBe(source);}console.log("[DEBUG] Original arena independent Three cycles=32 slots=64 reusedOriginalIdentity=true");
});

/** 🧹️ Independent Three keeps original claimed objects after source growth during retention. */
test("original retention source changes agree with Three",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️reachability/🔣️.json",import.meta.url),"utf8"));for(const changed of [false,true]){const source=new Group();const kept=new Mesh(new BoxGeometry(1,1,1));const removed=new Mesh(new BoxGeometry(2,2,2));const incoming=new Mesh(new BoxGeometry(3,3,3));source.add(kept,removed);if(changed)source.add(incoming);const claims=new Set([kept,...changed?[incoming]:[]]);for(const original of [...source.children])if(!claims.has(original as Mesh))source.remove(original);expect(source.children.length).toBe(law.retention.keptBoxes+Number(changed));expect(source.children[0]).toBe(kept);expect(removed.parent).toBe(null);if(changed)expect(source.children[1]).toBe(incoming);for(const mesh of [kept,removed,incoming])mesh.geometry.dispose();}console.log("[DEBUG] Original retention independent Three sourceChangesRestart=true originalClaimedObjects=true");
});

/** 🧹️ Independent Three removes the unreachable original box and preserves the exact kept object. */
test("original compaction neutral ownership agrees with Three",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️reachability/🔣️.json",import.meta.url),"utf8"));const root=new Group();const kept=new Mesh(new BoxGeometry(1,1,1));const removed=new Mesh(new BoxGeometry(2,2,2));root.add(kept,removed);expect(root.children.length).toBe(1+law.compaction.unreachableBoxes);root.remove(removed);expect(root.children).toEqual([kept]);expect(root.children[0]===kept).toBe(law.compaction.keptIdsUnchanged);expect(kept.geometry.groups.length).toBe(law.box.faces);expect(removed.parent).toBe(null);kept.geometry.dispose();removed.geometry.dispose();console.log("[DEBUG] Original compaction independent Three unreachableBoxes=1 keptIdentity=true faces=6");
});
