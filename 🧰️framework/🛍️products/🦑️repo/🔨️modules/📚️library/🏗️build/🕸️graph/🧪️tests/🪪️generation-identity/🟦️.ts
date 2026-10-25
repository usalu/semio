/** 🪪️ Contract: the identity of a graph generation names the store format and the catalog providers, never the owners of the registry, so a project added or removed by a peer is absorbed incrementally instead of rebuilding every project. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import fixture from"./🔣️.json";
import schema from"../../🕰️generations/🧬️schema/🔣️.json";

const graph=await import("../../🟦️.ts");
const registries=fixture.registries as unknown as Record<string,Parameters<typeof graph.buildGraphGenerationIdentityV1>[0]>;

test("registries that differ only in their owners and consumer roots have one identity, a different provider has another",()=>{
 for(const[left,right]of fixture.same)expect(graph.buildGraphGenerationIdentityV1(registries[left]!)).toBe(graph.buildGraphGenerationIdentityV1(registries[right]!));
 for(const[left,right]of fixture.different)expect(graph.buildGraphGenerationIdentityV1(registries[left]!)).not.toBe(graph.buildGraphGenerationIdentityV1(registries[right]!));
});

test("the independent Python derivation names the same identities",()=>{
 const result=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("../../🕰️generations/🧬️schema/🔣️.json",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 const expected=JSON.parse(result.stdout.toString()) as Record<string,string>;
 expect(schema.definitions.Identity.const.length).toBeGreaterThan(0);
 for(const name of Object.keys(registries))expect(graph.buildGraphGenerationIdentityV1(registries[name]!)).toBe(expected[name]!);
});

test("the head of a project record keeps the whole registry only for owner admission",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../📥️inference/📦️project-nodes/🟨️.mjs",import.meta.url)),"utf8");
 expect(text).toContain("options?.ownerAdmission?context.registry??null:context.registry?.catalogProviders?.map(");
});
