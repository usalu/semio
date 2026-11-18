import{expect,test}from"bun:test";
import{projectRowsHoldAtEndV1,registerClosureProviderV1}from"../../🟨️.mjs";

const row={kind:"closure",path:JSON.stringify(["/w","k",["some",["/a/"]]]),fact:"true"};

/** 🏁️ The closure rows of a finished inference are verified by the provider one closure at a time, not from a cache that evicted the entry while the owner ran. */
test("closure rows are verified by the provider when its cache lost the entry, and a wrong answer or a provider without verify keeps the old verdict",async()=>{
 const seen:unknown[]=[];
 registerClosureProviderV1({answer:()=>"!stale",settle:async()=>undefined,verify:async(rows:readonly unknown[],value:unknown)=>{seen.push([rows.length,value]);return true;}} as never);
 expect(await projectRowsHoldAtEndV1([row],{closureRecipes:{}},true,undefined)).toBe(true);
 expect(seen).toEqual([[1,{closureRecipes:{}}]]);
 registerClosureProviderV1({answer:()=>"!stale",settle:async()=>undefined,verify:async()=>false} as never);
 expect(await projectRowsHoldAtEndV1([row],{},true,undefined)).toBe(false);
 registerClosureProviderV1({answer:()=>"!stale",settle:async()=>undefined} as never);
 expect(await projectRowsHoldAtEndV1([row],{},true,undefined)).toBe(false);
 registerClosureProviderV1({answer:()=>"true",settle:async()=>undefined} as never);
 expect(await projectRowsHoldAtEndV1([row],{},true,undefined)).toBe(true);
});
