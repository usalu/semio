import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{readyManifestDigestV1}from"../🟦️.ts";

/** 📦️ Contract: an edit that cannot change what a dependency sync installs leaves the digest alone, every edit that can changes it. */
const base={name:"@x/a",version:"1.0.0",dependencies:{b:"^1",a:"^2"},scripts:{build:"x",postinstall:"y"},description:"d"};
const digest=(row:unknown)=>readyManifestDigestV1(JSON.stringify(row));

test("edits outside the install-relevant fields and key order do not change the digest",()=>{
 const reference=digest(base);
 expect(digest({...base,description:"other",scripts:{build:"z",postinstall:"y",test:"t"},exports:{".":"./i.ts"}})).toBe(reference);
 expect(digest({scripts:{postinstall:"y"},dependencies:{a:"^2",b:"^1"},version:"1.0.0",name:"@x/a"})).toBe(reference);
});

test("every install-relevant edit changes the digest",()=>{
 const reference=digest(base);
 for(const edit of [{version:"1.0.1"},{dependencies:{b:"^1",a:"^3"}},{devDependencies:{c:"1"}},{scripts:{postinstall:"changed"}},{workspaces:["p/*"]},{overrides:{a:"1"}},{trustedDependencies:["a"]},{packageManager:"bun@1.4.2"}])expect(digest({...base,...edit}),JSON.stringify(edit)).not.toBe(reference);
});

test("a text that is not a JSON object digests as its raw bytes, equal to an independent SHA-256",()=>{
 for(const text of ["{broken","[1,2]","null",""])expect(readyManifestDigestV1(text),text).toBe(createHash("sha256").update(text).digest("hex"));
 expect(readyManifestDigestV1(new TextEncoder().encode("{broken"))).toBe(createHash("sha256").update("{broken").digest("hex"));
});
