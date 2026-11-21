import {afterAll,expect,test} from "bun:test";
import {runToolSyncV1,startToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync,statSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {renderCargoProfileConfigurationV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import {unitCacheEnvironmentV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {cargoScratchBaseV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🏖️scratch/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {sharedStageCacheV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/☁️stage-fleet/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {encodeStoreReceiptV1,observeStoreContentV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs";
import {publishStoreReceiptContentV1,restoreStoreReceiptContentV1,fleetContentFromEnvironmentV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/☁️fleet-content/🟦️.ts";
import {stageCacheV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import {optimizeStageRequestV1,transpileStageRequestV1} from "../../🚀️commands/🟦️.ts";
import {transpilePluginComponentModulesAsync} from "../../🟦️.ts";
import {optimizePluginCoreModuleAsync,pluginCoreModuleFilesV1,pluginWasmOptToolV1} from "../../🪶️wasm-opt/🟦️.ts";
import {stripWasmDebugSectionsV1} from "../../🪶️strip-debug/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🪞️ The two-checkout proof for the plugin payload pipeline, repeatable and independent of the graph and of the live tree: one hand-written wasm32-wasip2 component crate is built at different temporary roots (each with its own Cargo home) through the repository's real rustc wrapper and the real wasm profile of the build, and the bytes are compared; the real jco transpile and wasm-opt steps run on each result through the stage cache's fleet tier against a cache server this test starts on a free port with a temporary store and test tokens; the second root computes nothing and every output is byte-identical to the first root's. The receipts of the outputs are equal and the fleet content store restores them byte-for-byte. Needs the pinned toolchain with the `wasm32-wasip2` target and the repository's tools (jco; a Binaryen for the optimize step): when one is missing the affected part is skipped and says why, it never passes silently. */
const here=dirname(fileURLToPath(import.meta.url));
const repository=process.env.SEMIO_TEST_REPOSITORY_ROOT??(()=>{for(let at=here;;at=dirname(at)){if(existsSync(join(at,"rust-toolchain.toml"))&&existsSync(join(at,"Cargo.toml")))return at;if(dirname(at)===at)throw Error("repository root not found");}})();
const toolchainText=readFileSync(join(repository,"rust-toolchain.toml"),"utf8"),channel=/channel\s*=\s*"([^"]+)"/u.exec(toolchainText)![1]!;
const base=cargoScratchBaseV1(),scratch:string[]=[],servers:RemoteCacheServerV1[]=[],token="test-write-token-not-a-secret";
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"tc-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const sysroot=(()=>{try{return runToolSyncV1("cargo",["--print","sysroot"],{command:"rustc",encoding:"utf8",env:{...process.env,RUSTUP_TOOLCHAIN:channel}}).stdout.trim();}catch{return "";}})();
const toolchainReady=sysroot!==""&&existsSync(join(sysroot,"lib","rustlib","wasm32-wasip2","lib"));
const jcoReady=existsSync(join(repository,"node_modules","@bytecodealliance","jco","package.json"));
const binaryen=(()=>{const found=join(repository,"node_modules","binaryen","bin","wasm-opt");return process.env.SEMIO_WASM_OPT_BIN??(existsSync(found)?found:undefined);})();
const section=(text:string,header:string)=>{const lines=text.split("\n"),start=lines.indexOf(header);if(start<0)throw Error("section absent: "+header);const rows=[header];for(let at=start+1;at<lines.length&&!lines[at]!.startsWith("[");at++)rows.push(lines[at]!);return rows.join("\n");};
const liveConfig=readFileSync(join(repository,".cargo","config.toml"),"utf8"),cargoConfig=`${section(liveConfig,"[unstable]")}\n${section(liveConfig,"[target.wasm32-wasip2]")}\n`;
const selectors=renderCargoProfileConfigurationV1({version:1,mode:"release",target:"wasm32-wasip2",producer:"artifact",operation:"build"},"isolated","isolated").selectors;

type Checkout=Readonly<{root:string;workspace:string;cargoHome:string;cache:string;lane:string;wasm:string}>;
function checkout(name:string,depth:string,control=false):Checkout{
 const root=directory(name),workspace=join(root,depth,"ws"),cargoHome=join(root,"cargo-home"),cache=join(root,"cache"),lane=join(cache,"l","l1");
 for(const [path,text] of Object.entries(fixture.files as Record<string,string>)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}
 if(control)for(const [path,text] of Object.entries(fixture.control.files as Record<string,string>)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}
 mkdirSync(join(workspace,".cargo"),{recursive:true});writeFileSync(join(workspace,".cargo","config.toml"),cargoConfig+(control?fixture.control.config.replace("{vendor}",join(workspace,"vendor")):""));writeFileSync(join(workspace,"rust-toolchain.toml"),toolchainText);
 mkdirSync(cargoHome,{recursive:true});mkdirSync(lane,{recursive:true});
 return {root,workspace,cargoHome,cache,lane,wasm:join(lane,"t","wasm32-wasip2","release",`${fixture.package.replaceAll("-","_")}.wasm`)};
}
async function build(c:Checkout,remote?:Record<string,string>,extra:readonly string[]=[]):Promise<{code:number;events:string[]}>{
 const wrapper=unitCacheEnvironmentV1({cacheRoot:c.cache,lane:"l1",laneRoot:c.lane,compiler:"rustc",toolchain:"test-toolchain",focus:[],slots:2,workspaceRoot:c.workspace,environment:{}});
 const env:Record<string,string|undefined>={...Object.fromEntries(Object.entries(process.env).filter(([key])=>!key.startsWith("SEMIO_")&&!key.startsWith("CARGO")&&!key.startsWith("RUSTC"))),...wrapper,RUSTUP_TOOLCHAIN:channel,CARGO_HOME:c.cargoHome,CARGO_TARGET_DIR:join(c.lane,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",...remote};
 const child=startToolV1("cargo",["build","--manifest-path",join(c.workspace,"Cargo.toml"),"-p",fixture.package,"--target","wasm32-wasip2","--profile","release",...selectors.flatMap(selector=>["--config",selector]),...extra],{cwd:c.workspace,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"],caller:import.meta.url});
 let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});
 const code=await new Promise<number>(done=>child.on("close",status=>done(status??1)));
 if(code!==0)throw Error(output);
 const logs=join(c.cache,"x");
 return {code,events:existsSync(logs)?readdirSync(logs).flatMap(name=>readFileSync(join(logs,name),"utf8").split("\n").filter(Boolean).map(line=>(JSON.parse(line) as {kind:string}).kind)):[]};
}
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const tree=(root:string):Record<string,string>=>{const rows:Record<string,string>={};const walk=(prefix:string)=>{for(const entry of readdirSync(join(root,prefix),{withFileTypes:true})){const path=prefix?`${prefix}/${entry.name}`:entry.name;if(entry.isDirectory())walk(path);else rows[path]=sha(join(root,path));}};walk("");return rows;};
const leaks=(root:string,needles:readonly string[]):string[]=>{const found:string[]=[];const walk=(prefix:string)=>{for(const entry of readdirSync(join(root,prefix),{withFileTypes:true})){const path=prefix?`${prefix}/${entry.name}`:entry.name;if(entry.isDirectory())walk(path);else{const bytes=readFileSync(join(root,path));for(const needle of needles)if(bytes.includes(needle))found.push(`${path}: ${needle}`);}}};walk("");return found;};
issueRepositoryScriptProvidersV1(process.env);

const wasmPhase=toolchainReady?test:test.skip;
let server:RemoteCacheServerV1|undefined;
const a=toolchainReady?checkout("a","x"):undefined,b=toolchainReady?checkout("b","y/z/deeper/still"):undefined,c=toolchainReady?checkout("c","w/v"):undefined;

wasmPhase("the wasm component is byte-identical when built at two checkout paths with two Cargo homes, and a fleet hit installs the same bytes",async()=>{
 server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:token}});servers.push(server);
 const remote={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:token};
 const first=await build(a!,remote),second=await build(b!);
 expect(first.events.filter(kind=>kind==="miss").length).toBeGreaterThanOrEqual(2);expect(second.events.filter(kind=>kind==="miss").length).toBeGreaterThanOrEqual(2);
 expect(sha(b!.wasm),"two independent compiles at different paths").toBe(sha(a!.wasm));
 const needles=[a!.workspace,b!.workspace,a!.cargoHome,b!.cargoHome,a!.root,b!.root,homedir(),sysroot,base].filter(needle=>needle.length>3);
 for(const path of [a!.wasm,b!.wasm]){const bytes=readFileSync(path);for(const needle of needles)expect(bytes.includes(Buffer.from(needle)),`${path} holds ${needle}`).toBe(false);}
 const counters=createFleetClientV1(readFleetSettingsV1(join(a!.root,"state"),remote)!);
 for(let waited=0;waited<60&&(await (await fetch(`${server.url}/status`,{headers:{authorization:`Bearer ${token}`}})).json() as {counters:{ac:{puts:number}}}).counters.ac.puts<first.events.filter(kind=>kind==="miss").length;waited++)await new Promise(done=>setTimeout(done,500));
 void counters;
 const third=await build(c!,remote);
 expect(third.events.filter(kind=>kind==="miss").length,"a unit was compiled instead of fetched").toBe(0);expect(third.events.filter(kind=>kind==="remote-hit").length).toBeGreaterThanOrEqual(2);
 expect(sha(c!.wasm)).toBe(sha(a!.wasm));
},300000);

wasmPhase("control: a dependency from an absolute directory source makes the bytes depend on the checkout path, so the comparison above can fail",async()=>{
 const one=checkout("control-a","x",true),two=checkout("control-b","y/z/deeper",true);
 await build(one,undefined,["--offline"]);await build(two,undefined,["--offline"]);
 expect(sha(two.wasm)).not.toBe(sha(one.wasm));
},300000);

const stagePhase=toolchainReady&&jcoReady?test:test.skip;
type Stages=Readonly<{transpiled:string;optimized:string|null}>;
async function stages(c:Checkout,cache:ReturnType<typeof sharedStageCacheV1>):Promise<Stages>{
 const base=fixture.package.replaceAll("-","_"),context={repoRoot:repository,preview2VendorDir:"",environment:process.env,...(binaryen?{wasmOptBin:binaryen}:{})};
 const component=createHash("sha256").update(stripWasmDebugSectionsV1(readFileSync(c.wasm))).digest("hex"),transpiled=join(c.root,"transpiled");
 await cache.run(transpileStageRequestV1(component,"jco-test","materialization-test",base,"native-transpile-test"),transpiled,directory=>transpilePluginComponentModulesAsync(c.wasm,directory,base,context),undefined,base);
 if(!binaryen)return {transpiled,optimized:null};
 const tool=pluginWasmOptToolV1(context),version=runToolSyncV1("wasm-opt",["--version"],{encoding:"utf8",env:{...process.env,SEMIO_TOOL_WASM_OPT:binaryen}}).stdout.trim(),optimized=join(c.root,"optimized");mkdirSync(optimized,{recursive:true});
 for(const file of pluginCoreModuleFilesV1(transpiled,base)){
  const core=join(transpiled,file),stage=join(c.root,`optimize-${file}`);
  await cache.run(optimizeStageRequestV1(sha(core),version,tool.args),stage,directory=>optimizePluginCoreModuleAsync(core,join(directory,"core.wasm"),{...context,cores:1}),undefined,`${base}/${file}`);
  writeFileSync(join(optimized,file),readFileSync(join(stage,"core.wasm")));
 }
 return {transpiled,optimized};
}
const outputs:Stages[]=[];
stagePhase("jco transpile and wasm-opt give the same files at every checkout, and the third checkout computes neither through the fleet tier",async()=>{
 const remote={SEMIO_REMOTE_CACHE_ENDPOINT:server!.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:token};
 const make=(m:Checkout,fleet:boolean)=>sharedStageCacheV1(stageCacheV1(join(m.root,"stages")),fleet?{client:createFleetClientV1(readFleetSettingsV1(join(m.root,"state"),remote)!)}:{});
 const caches=[make(a!,true),make(b!,false),make(c!,true)];
 for(const [index,m] of [a!,b!,c!].entries())outputs.push(await stages(m,caches[index]!));
 expect(tree(outputs[1]!.transpiled)).toEqual(tree(outputs[0]!.transpiled));expect(tree(outputs[2]!.transpiled)).toEqual(tree(outputs[0]!.transpiled));
 if(binaryen){expect(tree(outputs[1]!.optimized!)).toEqual(tree(outputs[0]!.optimized!));expect(tree(outputs[2]!.optimized!)).toEqual(tree(outputs[0]!.optimized!));}
 expect(Object.keys(tree(outputs[0]!.transpiled)).length).toBeGreaterThan(3);
 const cores=binaryen?pluginCoreModuleFilesV1(outputs[0]!.transpiled,fixture.package.replaceAll("-","_")).length:0,count=1+cores;
 expect(caches[0]!.fleet()).toMatchObject({served:0,shared:count});expect(caches[2]!.fleet()).toMatchObject({served:count,shared:0});expect(caches[2]!.stats()).toMatchObject({hits:count,misses:0});
 const needles=[a!,b!,c!].flatMap(m=>[m.workspace,m.cargoHome,m.root]).concat([homedir(),sysroot,base]).filter(needle=>needle.length>3);
 for(const output of outputs)for(const dir of [output.transpiled,output.optimized])if(dir)expect(leaks(dir,needles)).toEqual([]);
 if(!binaryen)console.log("[two-checkout] no Binaryen found (SEMIO_WASM_OPT_BIN or node_modules/binaryen): the optimize step was not run");
},300000);

stagePhase("the store receipts of the published outputs are equal at every checkout and the fleet content store restores them byte-for-byte",async()=>{
 const files=tree(outputs[0]!.transpiled);
 const receipt=async(m:Checkout)=>{const module=join(m.workspace,"dist","components","release","fixture");rmSync(module,{recursive:true,force:true});mkdirSync(module,{recursive:true});for(const name of Object.keys(files)){mkdirSync(dirname(join(module,name)),{recursive:true});writeFileSync(join(module,name),readFileSync(join(outputs[0]!.transpiled,name)));}const entry=await observeStoreContentV1(m.workspace,module);return encodeStoreReceiptV1({project:"fixture",target:"component",entries:[entry],links:[]});};
 const [one,two,three]=[await receipt(a!),await receipt(b!),await receipt(c!)];
 expect(two).toBe(one);expect(three).toBe(one);expect(one).not.toContain(a!.workspace);expect(one).not.toContain(b!.workspace);
 const store=fleetContentFromEnvironmentV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server!.url}/main`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:token})!;
 const receiptFile=join(a!.root,"receipt.json");writeFileSync(receiptFile,one);
 expect((await publishStoreReceiptContentV1({workspaceRoot:a!.workspace,receipt:receiptFile,store})).state).toBe("published");
 const fresh=checkout("d","q/r");
 const restored=await restoreStoreReceiptContentV1({workspaceRoot:fresh.workspace,receipt:JSON.parse(one),store});
 expect(restored.state).toBe("restored");
 expect(tree(join(fresh.workspace,"dist","components","release","fixture"))).toEqual(files);
},300000);

test("the profile selectors the proof builds with are the build system's own wasm component profile",()=>{
 expect(selectors).toContain('profile.release.trim-paths="object"');
 expect(selectors.some(selector=>selector.startsWith("profile.release.opt-level="))).toBe(true);
 expect(cargoConfig).toContain("link-arg=-zstack-size=8388608");
 expect(statSync(repository).isDirectory()).toBe(true);expect(resolve(tmpdir()).length).toBeGreaterThan(0);
});
