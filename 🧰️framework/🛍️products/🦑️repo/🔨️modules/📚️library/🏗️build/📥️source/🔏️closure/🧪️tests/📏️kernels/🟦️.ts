import{test,expect}from"bun:test";
import{existsSync}from"node:fs";
import{dirname,join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{censusNxAuthorizerSourcesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import selection from"../../../../⌨️command/🧭️selection/🧬️schema/🔣️.json";
import schema from"../../🧬️schema/🔣️.json";

const headroomFiles=32,headroomBytes=1_048_576;
const root=(()=>{let at=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(at,"go.work"))){const up=dirname(at);if(up===at)throw Error("Workspace root required");at=up;}return at;})();
const commandDirectory=fileURLToPath(new URL("../../../../⌨️command/",import.meta.url));

/** 📏️ Every command kernel is admitted as one closure whose size is bounded by the policy; this test fails while a kernel still fits the policy but is close to it, so the landing that grows it names the kernel instead of breaking every `repo` command of that class. */
for(const[kind,file]of Object.entries(selection.properties.kernels.const))test("kernel closure "+kind+" keeps headroom below the admission policy",async()=>{
 const policy=schema.definitions.Policy.const,entry=relative(root,join(commandDirectory,file)).replaceAll("\\","/"),census=await censusNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(120000),{installed:true});
 expect({kind,files:census.files,limit:policy.maximumFiles,fits:census.files<=policy.maximumFiles-headroomFiles}).toEqual({kind,files:census.files,limit:policy.maximumFiles,fits:true});
 expect({kind,bytes:census.sourceBytes,limit:policy.maximumSourceBytes,fits:census.sourceBytes<=policy.maximumSourceBytes-headroomBytes}).toEqual({kind,bytes:census.sourceBytes,limit:policy.maximumSourceBytes,fits:true});
},180000);
