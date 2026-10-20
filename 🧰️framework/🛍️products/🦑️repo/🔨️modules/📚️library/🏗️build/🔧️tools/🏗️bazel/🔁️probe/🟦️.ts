import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import type {CacheProbeInputV1,CacheProbeSpecV1,CacheProbeTargetV1,CacheProbeToolV1} from "../../../⚙️operations/🟦️.ts";

/** 🔬️ What the Bazel side of the cache probe needs from the provisioned tool: the executable, its install base and the fixed module catalogue. */
export type BazelProbeOptionsV1=Readonly<{version:string;executable:string;installBase:string;module:string;lock:string;repository:string}>;

/** 🎲️ The per-run input of every uncached target: Bazel's own action cache must not skip an action the neutral contract declares uncached. */
export const BAZEL_PROBE_VOLATILE_V1="inputs/volatile.txt";

/** 🧾️ One leaf input of a probe target, with named groups expanded in declaration order and repeats dropped. */
export type BazelProbeLeafV1=Readonly<{file:string}>|Readonly<{env:string}>;

const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");

/** 🌿️ Expands the named input groups of one target into its ordered, distinct file and environment inputs. */
export function bazelProbeLeavesV1(specification:CacheProbeSpecV1,target:CacheProbeTargetV1):readonly BazelProbeLeafV1[]{
 const seen=new Set<string>(),leaves:BazelProbeLeafV1[]=[],expand=(input:CacheProbeInputV1,trail:readonly string[])=>{
  if("named" in input){if(trail.includes(input.named))throw Error("Acyclic named probe inputs required");const group=specification.namedInputs[input.named];if(!group)throw Error("Declared named probe input required: "+input.named);for(const member of group)expand(member,[...trail,input.named]);return;}
  const key="file" in input?"file:"+input.file:"env:"+input.env;if(!seen.has(key)){seen.add(key);leaves.push(input);}
 };
 for(const input of target.inputs??[])expand(input,[]);
 return leaves;
}

/** 🏷️ The ASCII name of an input digest file, so no label ever carries a fixture file name. */
export const bazelProbeInputNameV1=(leaf:BazelProbeLeafV1):string=>"inputs/i-"+sha("file" in leaf?"file:"+leaf.file:"env:"+leaf.env).slice(0,12)+".txt";

/** 🆔️ The ASCII scratch workspace of one fixture instance: Bazel never sees the fixture path as a label or a workspace root, and a recreated fixture starts from a cold action cache. */
export const bazelProbeWorkspaceV1=(fixture:string,instance:string,base:string=tmpdir()):string=>join(base,bazelProbeWorkspacePrefixV1(fixture)+sha(instance).slice(0,8));

/** 🏷️ The name prefix that groups every instance of one fixture path, so stale scratch workspaces of that path can be swept. */
export const bazelProbeWorkspacePrefixV1=(fixture:string):string=>"semio-bzl-probe-"+sha(fixture).slice(0,8)+"-";

const quote=(value:string)=>"'"+value.replaceAll("'","'\\''")+"'";
const starlark=(values:readonly string[])=>"["+values.map(value=>JSON.stringify(value)).join(", ")+"]";

/** 📜️ The deterministic ASCII Bazel workspace of the neutral probe: one rule, one target per probe target and one shell script per command; digests and the fixture pointer are written per run. */
export function renderBazelProbeV1(specification:CacheProbeSpecV1,workspace:string,module:string):Readonly<Record<string,string>>{
 const files:Record<string,string>={"MODULE.bazel":module,"probe.bzl":[
  "def _impl(ctx):",
  "    out = ctx.actions.declare_directory(ctx.label.name + \"_out\")",
  "    ctx.actions.run_shell(",
  "        mnemonic = \"CacheProbe\",",
  "        inputs = ctx.files.srcs + ctx.files.deps + [ctx.file.script],",
  "        outputs = [out],",
  "        command = \"sh \" + ctx.file.script.path + \" \" + out.path,",
  "        use_default_shell_env = True,",
  "        execution_requirements = ctx.attr.requirements,",
  "    )",
  "    return [DefaultInfo(files = depset([out]))]",
  "",
  "probe_task = rule(",
  "    implementation = _impl,",
  "    attrs = {",
  "        \"srcs\": attr.label_list(allow_files = True),",
  "        \"deps\": attr.label_list(),",
  "        \"script\": attr.label(allow_single_file = True, mandatory = True),",
  "        \"requirements\": attr.string_dict(),",
  "    },",
  ")",""].join("\n")};
 const build=["load(\":probe.bzl\", \"probe_task\")",""];
 for(const target of specification.targets){
  const sources=[...bazelProbeLeavesV1(specification,target).map(bazelProbeInputNameV1),...target.cache?[]:[BAZEL_PROBE_VOLATILE_V1]],requirements=target.cache?{local:"1","no-sandbox":"1"}:{local:"1","no-sandbox":"1","no-cache":"1","no-remote":"1"};
  build.push("probe_task(",`    name = ${JSON.stringify(target.name)},`,`    srcs = ${starlark(sources)},`,`    deps = ${starlark((target.dependsOn??[]).map(name=>":"+name))},`,`    script = ${JSON.stringify("run/"+target.name+".sh")},`,`    requirements = {${Object.entries(requirements).map(([key,value])=>JSON.stringify(key)+": "+JSON.stringify(value)).join(", ")}},`,")","");
  files["run/"+target.name+".sh"]=["set -e","out=\"$(cd \"$(dirname \"$1\")\" && pwd)/$(basename \"$1\")\"",`cd "$(cat ${quote(join(workspace,"fixture.path"))})"`,target.command,"mkdir -p \"$out\"",...target.outputs.map(output=>`if [ -e ${quote(output)} ]; then mkdir -p "$out"/${quote(output)} && cp -pR ${quote(output)}/. "$out"/${quote(output)}/; fi`),""].join("\n");
 }
 files["BUILD.bazel"]=build.join("\n");
 return Object.freeze(files);
}

/** 🔢️ The digest file contents of one run (nonce for uncached targets, SHA-256 of each file input (or `missing`) and the value of each environment input. */
export function bazelProbeDigestsV1(specification:CacheProbeSpecV1,read:(file:string)=>Uint8Array|undefined,environment:Readonly<Record<string,string|undefined>>,nonce:string):Readonly<Record<string,string>>{
 const digests:Record<string,string>={[BAZEL_PROBE_VOLATILE_V1]:nonce+"\n"};
 for(const target of specification.targets)for(const leaf of bazelProbeLeavesV1(specification,target)){const bytes="file" in leaf?read(leaf.file):undefined;digests[bazelProbeInputNameV1(leaf)]="file" in leaf?(bytes?sha(bytes):"missing")+"\n":"env="+(environment[leaf.env]??"")+"\n";}
 return Object.freeze(digests);
}

/** 🏁️ The Bazel startup and build flags of one probe run; the lock and repository cache are the production module catalogue, so nothing is fetched. */
export function bazelProbeArgumentsV1(options:BazelProbeOptionsV1,specification:CacheProbeSpecV1,workspace:string,task:string):readonly string[]{
 const size=/^(\d+)(KB|MB|GB)$/u.exec(specification.maxCacheSize);if(!size)throw Error("Sized probe cache bound required");
 return ["--batch","--host_jvm_args=-Xmx1g",`--output_base=${join(workspace,"base")}`,`--install_base=${options.installBase}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc","build","//:"+task,"--lockfile_mode=error","--incompatible_autoload_externally=",`--repository_cache=${options.repository}`,"--repository_disable_download","--incompatible_strict_action_env",`--disk_cache=${join(workspace,"disk")}`,`--experimental_disk_cache_gc_max_size=${size[1]}${size[2][0]}`,"--spawn_strategy=local","--action_env=PATH","--action_env=HOME","--noshow_progress","--color=no","--curses=no","--verbose_failures","--jobs=1"];
}

/** 🔌️ The Bazel implementation of the tool-neutral cache probe: a task is a Bazel action keyed by input digests, its outputs are restored from `bazel-bin` into the fixture. */
export function bazelCacheProbeV1(options:BazelProbeOptionsV1):CacheProbeToolV1{
 const runner=fileURLToPath(new URL("./🏭️runner/🟦️.ts",import.meta.url));
 return Object.freeze<CacheProbeToolV1>({
  tool:"bazel",
  label:"Bazel",
  version:()=>options.version,
  files:specification=>({"bazel-probe.json":{tool:"bazel",version:options.version,workspace:"scratch",targets:specification.targets.map(target=>target.name)}}),
  invocation:(specification,task,fixture,environment)=>({command:process.execPath,arguments:[runner],environment:{...environment,SEMIO_BAZEL_PROBE:JSON.stringify({options,specification,task,fixture,environment:{CACHE_PROBE_MODE:environment.CACHE_PROBE_MODE}})}})
 });
}
