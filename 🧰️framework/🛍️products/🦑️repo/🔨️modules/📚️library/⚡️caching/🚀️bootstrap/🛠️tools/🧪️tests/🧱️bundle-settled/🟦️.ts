import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{authorizerBundleEvidenceV1}from"../../🟦️.ts";

/** 🧱️ Contract: an authorizer bundle is published only after both captured source closures still hold, so a bundle is never built from a snapshot a peer edited meanwhile; the refusal is a concurrent edit that the readiness supervisor classes as source-transient.
 * @see ../../🟦️.ts */
test("the bundle is published only after both source captures were verified unchanged",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),from=text.indexOf("async function ensureNxAuthorizerBundleV1"),body=text.slice(from,text.indexOf("\n}\n",from));
 const guard=body.indexOf("for(const[sources,name]of"),publish=body.indexOf("renameSync(staging, destination)");
 expect(guard).toBeGreaterThan(0);expect(publish).toBeGreaterThan(guard);
 expect(body.slice(guard,publish)).toContain("concurrentEditV1(\"bytes\"");
 expect(body.slice(guard,publish)).toContain("[authorizer,\"authorizer\"],[compiler,\"compiler\"]");
});
test("a failed bundle names the captured and the on-disk digest of every module its message points at",()=>{
 const root="/w",message='Internal error: Bundle failed: No matching export in "semio-captured-source:/w/a/idx.ts" for import "X" (reference 1) and "semio-captured-source:/w/b/run.ts" and again "semio-captured-source:/w/a/idx.ts"';
 const evidence=authorizerBundleEvidenceV1(message,root,new Map([["a/idx.ts","aaa"]]),path=>path==="a/idx.ts"?"bbb":null);
 expect(evidence).toBe("a/idx.ts captured sha256 aaa, on disk bbb; b/run.ts captured sha256 absent, on disk absent");
 expect(authorizerBundleEvidenceV1("stage failed",root,new Map(),()=>null)).toBe("");
});
test("the publication catch routes a bundler failure through the evidence and recheck",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8");
 expect(text).toContain("throw await authorizerBundleFailureV1(root,[[authorizer,\"authorizer\"],[compiler,\"compiler\"]],error,signal)");
});
test("a bundle refused for a peer edit is built again inside the command from a fresh capture and the refusal names the moved source",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),ready=text.slice(text.indexOf("const ready=async"),text.indexOf("if (present) return ready"));
 expect(ready).toContain("retryOnConcurrentEditV1(\"build\"");expect(ready).toContain("attempt===1?{authorizerFiles,authorizer,compiler}:await captureNxAuthorizerPairV1(");
 const body=text.slice(text.indexOf("async function ensureNxAuthorizerBundleV1"));
 expect(body).toContain("nxAuthorizerSourcesMovedV1(root,sources,signal)");expect(body).toContain("concurrentEditV1(\"bytes\",moved,");
 expect(text).not.toContain("concurrentEditV1(\"bytes\",sources.manifest.entry");
});
