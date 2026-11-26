/** 📏️ The sealed door entry is captured whole under the declared admission bound, with margin: command families and backend adapters that only a kernel runs stay outside it, and every command still resolves to a kernel whose own closure is bounded by the kernel law.
 * @see ../../../📥️source/🔏️closure/🧪️tests/📏️kernels/🟦️.ts the per-kernel bound */
import{existsSync}from"node:fs";
import{dirname,join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{expect,test}from"bun:test";
import{censusNxAuthorizerSourcesV1}from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import catalog from"../../../⚙️operations/🔣️catalog.json";
import selection from"../../../⌨️command/🧭️selection/🧬️schema/🔣️.json";
import fixture from"../../../⌨️command/🧭️selection/🔣️.json";
import{repositoryCommandKernelV1}from"../../../⌨️command/🧭️selection/🟦️.ts";
import schema from"../../../📥️source/🔏️closure/🧬️schema/🔣️.json";

const headroomShare=2;
const root=(()=>{let at=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(at,"go.work"))){const up=dirname(at);if(up===at)throw Error("Workspace root required");at=up;}return at;})();
const entry=relative(root,fileURLToPath(new URL("../../🚪️entry/🟦️.ts",import.meta.url))).replaceAll("\\","/");
const census=await censusNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(240000),{installed:true});

test("the sealed door entry keeps headroom below the admission policy",()=>{
 const policy=schema.definitions.Policy.const;
 expect({files:census.files,limit:policy.maximumFiles,fits:census.files<=Math.floor(policy.maximumFiles/headroomShare)}).toEqual({files:census.files,limit:policy.maximumFiles,fits:true});
 expect({bytes:census.sourceBytes,limit:policy.maximumSourceBytes,fits:census.sourceBytes<=Math.floor(policy.maximumSourceBytes/headroomShare)}).toEqual({bytes:census.sourceBytes,limit:policy.maximumSourceBytes,fits:true});
},300000);

test("the sealed door entry captures no backend adapter, no verb kernel and no warm-only storage image",()=>{
 const kernels=Object.values(selection.properties.kernels.const).map(file=>relative(root,join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command",file)).replaceAll("\\","/")),captured=new Set(census.sources.map(row=>row.path));
 expect(census.sources.filter(row=>/\/🔌️backend\/🟦️\.ts$/u.test(row.path)||/\/🔌️backends\/🚚️loading\/🟦️\.ts$/u.test(row.path)||/\/🔧️tools\/💾️storage\/🟦️\.ts$/u.test(row.path)).map(row=>row.path)).toEqual([]);
 expect(kernels.filter(path=>captured.has(path))).toEqual([]);
});

test("every command still resolves to an existing verb kernel",()=>{
 const directory=fileURLToPath(new URL("../../../⌨️command/",import.meta.url)),kinds=new Set(Object.keys(selection.properties.kernels.const));
 for(const row of fixture.cases){const kernel=repositoryCommandKernelV1(row.args,{});expect<unknown>({args:row.args,kind:kernel.kind,known:kinds.has(kernel.kind),exists:existsSync(kernel.entry)}).toEqual({args:row.args,kind:row.kind,known:true,exists:true});}
 for(const file of Object.values(selection.properties.kernels.const))expect(existsSync(join(directory,file)),file).toBe(true);
 for(const row of catalog.operations){const kernel=repositoryCommandKernelV1([row.name],{});expect({name:row.name,known:kinds.has(kernel.kind),exists:existsSync(kernel.entry)}).toEqual({name:row.name,known:true,exists:true});}
});
