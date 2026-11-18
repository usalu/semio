import{expect,test}from"bun:test";
import{mergeWalkMembersV1}from"../🟨️.mjs";

/** 🧬️ The walk members of a stored layer are held once per digest as the instance the loaded entries share; a different list only adds what the table lacks. */
test("the first member list of a digest is held as it is, the same list is never merged again and a different list adds only the missing members",()=>{
 const table=new Map<string,string[]>(),first=["w1","w2"],second=["w2","w3"];
 expect(mergeWalkMembersV1(table,"d",first)).toBe(true);
 expect(table.get("d")).toBe(first);
 expect(mergeWalkMembersV1(table,"d",first)).toBe(false);
 expect(mergeWalkMembersV1(table,"d",["w1"])).toBe(false);
 expect(mergeWalkMembersV1(table,"d",second)).toBe(true);
 expect(table.get("d")).toEqual(["w1","w2","w3"]);
 const wide=["w1","w2","w3","w4"];
 expect(mergeWalkMembersV1(table,"d",wide)).toBe(true);
 expect(table.get("d")).toBe(wide);
 expect(mergeWalkMembersV1(table,"e",second)).toBe(true);
 expect(table.get("e")).toBe(second);
});
