import{expect}from"bun:test";
import ts from"typescript";
import{readFile}from"node:fs/promises";

/** 🧰️ Extracts the real `prepareNxGraphCacheStorageV1` declaration from its source and binds it to the given ports, so the lifetime tests drive the actual code with simulated storage. */
export async function actualNxRecoveryV1(ports:Record<string,unknown>):Promise<any>{const source=await readFile(new URL("../../../../../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("nx.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find((value):value is ts.FunctionDeclaration=>ts.isFunctionDeclaration(value)&&value.name?.text==="prepareNxGraphCacheStorageV1");expect(node).toBeDefined();return Function(...Object.keys(ports),ts.transpileModule(node!.getText(tree).replace(/^export\s+/,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return prepareNxGraphCacheStorageV1")(...Object.values(ports));}
