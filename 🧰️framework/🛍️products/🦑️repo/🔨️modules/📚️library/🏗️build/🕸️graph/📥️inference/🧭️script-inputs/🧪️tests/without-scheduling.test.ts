/** 🗓️ Contract: a closure read through one view of scheduling prefixes is filtered once, exactly as an independent filter says, and every later read of the same closure through the same prefixes returns the identical array; a different prefix set, the standard set and an empty set never share an answer. */
import {expect,test} from "bun:test";
import {withoutSchedulingV1} from "../🟨️.mjs";

const closure=["{workspaceRoot}/a/x.ts","{workspaceRoot}/b/y.ts","b/z.ts","c/w.ts",{externalDependencies:["p"]}] as const;
const reference=(entries:readonly unknown[],prefixes:readonly string[])=>entries.filter(entry=>typeof entry!=="string"||!prefixes.some(prefix=>(entry.startsWith("{workspaceRoot}/")?entry.slice("{workspaceRoot}/".length):entry).startsWith(prefix)));

test("a view filters its closure as the independent filter does and answers the same array again",()=>{
 for(const prefixes of [["a/"],["b/"],["a/","c/"],[]] as const){
  const first=withoutSchedulingV1(closure as never,[...prefixes]);
  expect(first,prefixes.join()).toEqual(reference(closure,prefixes) as never);
  expect(withoutSchedulingV1(closure as never,[...prefixes])).toBe(first);
 }
});

test("prefix sets of one closure never share an answer and an empty set keeps everything",()=>{
 const a=withoutSchedulingV1(closure as never,["a/"]),b=withoutSchedulingV1(closure as never,["b/"]),none=withoutSchedulingV1(closure as never,[]);
 expect(a).not.toBe(b);
 expect(none).toEqual([...closure] as never);
 expect(a.length).toBe(4);
 expect(b.length).toBe(3);
});
