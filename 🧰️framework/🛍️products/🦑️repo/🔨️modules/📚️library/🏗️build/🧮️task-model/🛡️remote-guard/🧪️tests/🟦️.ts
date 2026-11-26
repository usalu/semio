import {expect,test} from "bun:test";
import {remoteGuardInputsV1,remoteGuardV1} from "../🟦️.ts";
import {projectBuildTargetForNxV1} from "../../../../🟨️.mjs";

test("wasm and preview-generated are guarded with a bilingual reason, other targets are not",()=>{
 for(const name of["wasm","preview-generated","🔌️wasm"]){const reason=remoteGuardV1(name);expect(reason?.en.length).toBeGreaterThan(10);expect(reason?.de.length).toBeGreaterThan(10);expect(remoteGuardInputsV1(name)).toHaveLength(1);}
 for(const name of["build","test","wasm-opt","check"]){expect(remoteGuardV1(name)).toBeUndefined();expect(remoteGuardInputsV1(name)).toEqual([]);}
});

test("the plugin component target is deliberately not guarded: the two-checkout proof shows its payload byte-identical at every checkout",()=>{
 expect(remoteGuardV1("component")).toBeUndefined();expect(remoteGuardV1("🧩️component")).toBeUndefined();expect(remoteGuardInputsV1("component")).toEqual([]);
 expect(remoteGuardV1("wasm")?.en).toContain("two-checkout proof");
});

test("the Nx projection of a guarded target keeps its local cache and adds the machine identity to its inputs",()=>{
 const target={execution:{kind:"command" as const,command:"x"},cache:true,outputs:["{workspaceRoot}/.cache/b/p/m/x"],inputs:["default"]};
 const guarded=projectBuildTargetForNxV1(target,"wasm"),plain=projectBuildTargetForNxV1(target,"build");
 expect(guarded.cache).toBe(true);
 expect(guarded.inputs?.some((input:unknown)=>typeof input==="object"&&(input as {runtime?:string}).runtime?.includes("hostname"))).toBe(true);
 expect(plain.inputs?.some((input:unknown)=>typeof input==="object"&&(input as {runtime?:string}).runtime?.includes("hostname"))).toBe(false);
});
