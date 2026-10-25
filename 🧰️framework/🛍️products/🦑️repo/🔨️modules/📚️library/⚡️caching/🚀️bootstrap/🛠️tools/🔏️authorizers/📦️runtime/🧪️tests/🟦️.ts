/** 🛡️ Runtime: the Node-reachable closure of the authorizer bundle names no `bun:` module statically, so the Node preflight can load it. */
import{test,expect}from"bun:test";
import{fileURLToPath}from"node:url";
const entry=fileURLToPath(new URL("../🟦️.ts",import.meta.url)),staticBun=/(?:\bfrom|\bimport)\s*\(?\s*["']bun:|\brequire\(\s*["']bun:/u;

test("the authorizer bundle closure holds no static bun: specifier",async()=>{
 const built=await Bun.build({entrypoints:[entry],target:"node",format:"esm"});
 expect(built.success,String(built.logs)).toBe(true);
 const text=await built.outputs[0]!.text(),hits=[...text.matchAll(new RegExp(staticBun.source,"gu"))].map(match=>text.slice(Math.max(0,match.index-60),match.index+60).replace(/\s+/gu," "));
 expect(hits).toEqual([]);
 expect(staticBun.test('import{Database}from"bun:sqlite";')).toBe(true);expect(staticBun.test('const m=await import("bun:sqlite");')).toBe(true);expect(staticBun.test('createRequire(x)("bun:sqlite")')).toBe(false);expect(staticBun.test('process.getBuiltinModule("bun:sqlite")')).toBe(false);
},120000);
