import{expect,test}from"bun:test";
import{AsyncLocalStorage}from"node:async_hooks";
/** 🧭️ Models the rule on a neutral scope pair: the async-local map wins, the live owner's map answers when the runtime dropped the async-local context, and an owner that ended answers nothing. */
function scopes<T>(local:AsyncLocalStorage<Map<string,T>>,ambient:()=>Map<string,T>|undefined):Map<string,T>|undefined{return local.getStore()??ambient();}
test("the owner's scope map answers when the async-local context is lost and stops answering when the owner ended",async()=>{
 const local=new AsyncLocalStorage<Map<string,number>>(),mine=new Map([["dir",1]]);let ambient:Map<string,number>|undefined;
 await local.run(mine,async()=>{ambient=mine;expect(scopes(local,()=>ambient)).toBe(mine);await local.exit(async()=>{expect(local.getStore()).toBeUndefined();expect(scopes(local,()=>ambient)?.get("dir")).toBe(1);});});
 ambient=undefined;await local.exit(async()=>{expect(scopes(local,()=>ambient)).toBeUndefined();});
});
test("the graph owner reads its scopes only through the ambient-aware accessor",async()=>{
 const text=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();
 expect(text.split("nativeScopes.getStore()").length-1).toBe(2);expect(text).toContain("const local=nativeScopes.getStore();if(local)return local;");expect(text).toContain("ambientScopes.splice(at,1)");expect(text).not.toContain("ambientScopes=outer");
});
