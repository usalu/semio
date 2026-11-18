import{expect,test}from"bun:test";
import{createTextArenaV1}from"../../🔤️text-arena/🟨️.mjs";

const lcg=(seed:number)=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const text=(at:number)=>"t"+at+(at%9===0?"é𝄞日本":"")+"/".repeat(at%4)+"y".repeat(at%25);

test("the arena numbers each distinct text once and sorts them in the byte order a Buffer comparison gives",()=>{
 const random=lcg(3),arena=createTextArenaV1(),model=new Map<string,number>(),list:string[]=[];
 for(let step=0;step<8000;step++){
  const value=text(Math.floor(random()*3000)),number=arena.add(value);
  if(!model.has(value)){model.set(value,number);list.push(value);}
  expect(number).toBe(model.get(value)!);
 }
 expect(arena.count()).toBe(model.size);
 for(const[value,number]of model){expect(arena.id(value)).toBe(number);expect(arena.bytes(number).toString("utf8")).toBe(value);}
 expect(arena.id("never added")).toBeUndefined();
 const expected=[...model.entries()].sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0]))).map(entry=>entry[1]);
 expect([...arena.sorted()]).toEqual(expected);
 expect(arena.size()/arena.count()).toBeLessThan(120);
});

test("a text the validator refuses is not added and the refusal reaches the caller",()=>{
 const arena=createTextArenaV1({validate:value=>{if(value.includes("bad"))throw new Error("refused");}});
 expect(arena.add("good")).toBe(0);
 expect(()=>arena.add("bad one")).toThrow("refused");
 expect(arena.count()).toBe(1);expect(arena.add("next")).toBe(1);
});
