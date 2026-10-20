import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {join,resolve} from "node:path";
import {mkdtempSync,mkdirSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {spawnSync} from "node:child_process";
import {parse} from "@iarna/toml";
import componentSchema from "../../🧬️schema/🔣️.json";const schema=componentSchema.definitions.StorageLayout;
import fixture from "./🧫️fixtures/🔣️.json";
import {bazelCacheLayoutV1,bazelCacheFlagsV1,bazelDiskCacheMaximumBytesV1,bazelServerBaseV1,requireBazelHostPathsV1} from "../🟦️.ts";
import {canonicalArchitectureEnvironment} from "../../../../../⚡️caching/🦀️cargo/🌿️environment/🟦️.ts";

test("every Bazel generated writer selects an authored finite partition against an independent SQL oracle",()=>{
 const roots={tools:resolve(fixture.roots.tools),dependencies:resolve(fixture.roots.dependencies),products:resolve(fixture.roots.products)},layout=bazelCacheLayoutV1(roots),validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");
 expect(validate(layout),JSON.stringify(validate.errors)).toBe(true);
 try{database.exec("CREATE TABLE roots(partition TEXT PRIMARY KEY,path TEXT)");for(const[key,value]of Object.entries(roots))database.query("INSERT INTO roots VALUES(?1,?2)").run(key,value.replaceAll("\\","/"));for(const row of fixture.paths){const oracle=database.query("SELECT path||'/'||?2 AS path FROM roots WHERE partition=?1").get(row.partition,row.suffix) as {path:string};expect(layout[row.name as keyof typeof layout].toString().replaceAll("\\","/")).toBe(oracle.path);}}
 finally{database.close();}
 expect(()=>bazelCacheLayoutV1({...roots,tools:"relative"})).toThrow("Absolute");expect(()=>bazelCacheLayoutV1({...roots,products:"/"+"x".repeat(256)})).toThrow("256");
});

test("native grants select the authored Cargo target against an independent TOML oracle",()=>{
 const root=process.cwd(),configuration=parse(readFileSync(join(root,".cargo/config.toml"),"utf8")) as {build:{"target-dir":string;"build-dir":string}};
 for(const row of fixture.cargoAuthority){const expected=resolve(root,row.target==="configuration"?configuration.build["target-dir"]:row.target),actual=canonicalArchitectureEnvironment(root,row.environment);expect(actual.CARGO_TARGET_DIR).toBe(expected);expect(actual.CARGO_BUILD_BUILD_DIR).toBe(resolve(root,configuration.build["build-dir"]));}
});

const cachePolicy=componentSchema.definitions.CachePolicy.const,cache=fixture.cache,layoutOf=()=>bazelCacheLayoutV1({tools:resolve(fixture.roots.tools),dependencies:resolve(fixture.roots.dependencies),products:resolve(fixture.roots.products)});

test("the disk cache bound is the authored share of the products budget against an independent SQL oracle",()=>{
 const budget=JSON.parse(readFileSync(join(import.meta.dir,"../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"),"utf8")),database=new Database(":memory:");
 try{
  const row=database.query("SELECT ?1*?2/10000/1048576*1048576 AS bytes").get(budget.partitions.products.maximumLogicalBytes,cachePolicy.diskCache.productsShareBasisPoints) as {bytes:number};
  expect(bazelDiskCacheMaximumBytesV1()).toBe(row.bytes);expect(row.bytes).toBe(cache.diskCacheMaximumBytes);expect(bazelDiskCacheMaximumBytesV1(cache.productsMaximumLogicalBytes)).toBe(cache.diskCacheMaximumBytes);
  expect(row.bytes).toBeLessThan(budget.partitions.products.maximumLogicalBytes);expect(row.bytes%1048576).toBe(0);
 }finally{database.close();}
 expect(()=>bazelDiskCacheMaximumBytesV1(0)).toThrow("Finite");
});

test("generated Bazel cache flags agree with the fixture, the closed schema and an independent hash oracle",()=>{
 const layout=layoutOf(),validate=new Ajv({strict:true}).compile({definitions:{CacheFlags:componentSchema.definitions.CacheFlags},$ref:"#/definitions/CacheFlags"}),workspace=cache.workspaces[0]!.workspace,helperPath=join(layout.temporary,"remote-cache-credentials."+(process.platform==="win32"?"cmd":"sh"));
 for(const row of cache.workspaces)expect(bazelServerBaseV1(layout,row.workspace)).toBe(join(layout.server,createHash("sha256").update(row.workspace).digest("hex").slice(0,16))),expect(bazelServerBaseV1(layout,row.workspace)).toBe(join(layout.server,row.sha256Prefix));
 expect(bazelServerBaseV1(layout,cache.workspaces[0]!.workspace)).not.toBe(bazelServerBaseV1(layout,cache.workspaces[1]!.workspace));
 expect(()=>bazelServerBaseV1(layout,"relative")).toThrow("Absolute");
 for(const row of cache.accepted){
  const options={environment:row.environment,skipCache:(row as {skipCache?:boolean}).skipCache,skipRemoteCache:(row as {skipRemoteCache?:boolean}).skipRemoteCache},flags=bazelCacheFlagsV1(layout,workspace,options);
  expect(validate(flags),row.name+JSON.stringify(validate.errors)).toBe(true);
  expect(flags.command,row.name).toEqual(row.command.map(flag=>flag.replace("{actions}",layout.actions).replace("{helper}",helperPath)));expect(flags.remote,row.name).toBe(row.remote);expect(flags.startup).toEqual(["--output_base="+bazelServerBaseV1(layout,workspace)]);
  expect(flags.environment,row.name).toEqual(row.tokenEnvironment);expect(flags.helper?.path,row.name).toBe(row.helper?helperPath:undefined);
  expect(flags.command.some(flag=>flag.startsWith("--remote_cache=")),row.name).toBe(row.remote);
  expect(JSON.stringify(flags.command)+JSON.stringify(flags.helper??{}),row.name+" keeps the token out of flags and helper").not.toContain("tok-123");
 }
 for(const row of cache.refused)expect(()=>bazelCacheFlagsV1(layout,workspace,{environment:row.environment}),row.name).toThrow(row.message);
 expect(bazelCacheFlagsV1(layout,workspace,{environment:{PATH:"/usr/bin",HOME:"/home/x"}}).remote).toBe(false);
});

test("the repository .bazelrc carries exactly the authored static cache policy and no host paths",()=>{
 const text=readFileSync(join(process.cwd(),".bazelrc"),"utf8"),lines=text.split(/\r?\n/u).filter(Boolean),database=new Database(":memory:");
 try{
  const idle=cachePolicy.diskCache.garbageCollectionIdleDelaySeconds,age=cachePolicy.diskCache.garbageCollectionMaximumAgeDays,repositoryAge=cachePolicy.repositoryContents.garbageCollectionMaximumAgeDays,
   rows=database.query("SELECT printf('build --experimental_disk_cache_gc_idle_delay=%ds',?1) AS line UNION ALL SELECT printf('build --experimental_disk_cache_gc_max_age=%dd',?2) UNION ALL SELECT printf('build --repo_contents_cache_gc_max_age=%dd',?3)").all(idle,age,repositoryAge) as {line:string}[];
  expect(rows.map(row=>row.line)).toEqual(cache.bazelrc);for(const line of cache.bazelrc)expect(lines).toContain(line);expect(text,"remote cache flags come only from the neutral contract").not.toMatch(/remote_/u);
 }finally{database.close();}
 for(const line of lines){expect(line).toMatch(/^(common|build) --[a-z_]+(=\S+)?$/u);expect(line).not.toMatch(/--(disk_cache|output_base|output_user_root|repository_cache|remote_cache)=/u);expect(line).not.toMatch(/=(\/|[A-Za-z]:[\\/]|~)/u);expect(line).not.toContain("symlink_prefix");expect(line).not.toContain("/Users/");}
 expect(cachePolicy.diskCache.garbageCollectionIdleDelaySeconds+cachePolicy.diskCache.garbageCollectionWindowSeconds).toBeLessThanOrEqual(componentSchema.definitions.Policy.const.serverIdleSeconds);
 expect(cache.serverIdleSeconds).toBe(componentSchema.definitions.Policy.const.serverIdleSeconds);
});

test.skipIf(!process.env.SEMIO_TEST_BAZEL_BINARY)("the provisioned Bazel binary accepts every generated and static cache flag",()=>{
 const binary=process.env.SEMIO_TEST_BAZEL_BINARY!,directory=mkdtempSync(join(tmpdir(),"bazel-flags-")),workspace=join(directory,"workspace"),layout=bazelCacheLayoutV1({tools:join(directory,"tools"),dependencies:join(directory,"dependencies"),products:join(directory,"products")});
 try{
  mkdirSync(workspace);writeFileSync(join(workspace,"MODULE.bazel"),'module(name="flags")\n');writeFileSync(join(workspace,"BUILD.bazel"),'genrule(name="x",outs=["x.txt"],cmd="echo x > $@")\n');
  for(const row of cache.accepted){
   const flags=bazelCacheFlagsV1(layout,workspace,{environment:row.environment,skipCache:(row as {skipCache?:boolean}).skipCache});if(flags.helper){if(process.platform==="win32")continue;mkdirSync(layout.temporary,{recursive:true});writeFileSync(flags.helper.path,flags.helper.content,{mode:flags.helper.mode});}
   const result=spawnSync(binary,["--batch",`--output_user_root=${join(directory,"root")}`,`--install_base=${join(directory,"install")}`,...flags.startup,"--nosystem_rc","--nohome_rc","--noworkspace_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"build","--nobuild","--lockfile_mode=off","--incompatible_autoload_externally=",...flags.command,"//:x"],{cwd:workspace,encoding:"utf8",timeout:240000});
   expect(result.status,row.name+"\n"+result.stderr).toBe(0);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
},600000);

test("Bazel host paths must be ASCII on macOS (install, outputs, server) and on Linux (install), against an independent Python regex",()=>{
 const policy=componentSchema.definitions.HostPaths.const,python=Bun.which("python3");
 expect(new Ajv({strict:true}).compile(componentSchema.definitions.HostPathPolicy)(policy)).toBe(true);expect(Object.keys(policy.platforms)).toEqual(["darwin","linux"]);
 for(const row of fixture.hostPaths){
  const check=()=>requireBazelHostPathsV1(row.platform,{...row.layout,modules:"/m"});
  if(row.accepted)expect(check,row.name).not.toThrow();else expect(check,row.name).toThrow(`ASCII Bazel host path required on ${row.platform}`);
 }
 if(python){
  const rules=policy.platforms as Record<string,{pattern:string;guarded?:string[]}>,rows=fixture.hostPaths.filter(row=>row.platform in rules),result=Bun.spawnSync([python,"-I","-c","import json,re,sys\nrules,rows=json.loads(sys.stdin.read())\nprint(json.dumps([all(re.fullmatch(rules[name],path) is not None for path in paths) for name,paths in rows]))"],{stdin:new TextEncoder().encode(JSON.stringify([Object.fromEntries(Object.entries(rules).map(([name,rule])=>[name,rule.pattern])),rows.map(row=>[row.platform,(rules[row.platform]!.guarded??policy.guarded).map(key=>(row.layout as Record<string,string>)[key])])])),stdout:"pipe",stderr:"pipe"});
  expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual(rows.map(row=>row.accepted));
 }
 const roots={tools:resolve(fixture.roots.tools),dependencies:resolve(fixture.roots.dependencies),products:resolve(fixture.roots.products)},emoji={...roots,tools:join(roots.tools,"\u{1F9F0}\uFE0F")};
 if(process.platform==="darwin"||process.platform==="linux")expect(()=>bazelCacheLayoutV1(emoji)).toThrow("ASCII Bazel host path required");else expect(bazelCacheLayoutV1(emoji).install).toContain("\u{1F9F0}");
 const emojiProducts={...roots,products:join(roots.products,"\u{1F9F0}\uFE0F")};
 expect(policy.guarded).toEqual(["install","outputs","server"]);
 if(process.platform==="darwin")expect(()=>bazelCacheLayoutV1(emojiProducts)).toThrow("ASCII Bazel host path required on darwin: outputs");else expect(bazelServerBaseV1(bazelCacheLayoutV1(emojiProducts),"/w")).toContain("\u{1F9F0}");
});

test("the production Bazel layout names an ASCII output base and the rc never overrides the sandbox strategy",()=>{
 const layout=bazelCacheLayoutV1(fixture.sandbox.roots),flags=bazelCacheFlagsV1(layout,fixture.sandbox.workspace,{environment:{}}),base=flags.startup.find(flag=>flag.startsWith("--output_base="))!.slice("--output_base=".length),rc=readFileSync(join(import.meta.dir,"../../../../../../../../../../.bazelrc"),"utf8"),pattern=new RegExp(componentSchema.definitions.HostPaths.const.platforms.darwin.pattern);
 expect(base.startsWith(layout.server+"/")).toBe(true);expect(pattern.test(base)).toBe(true);
 expect(flags.command.some(flag=>flag.startsWith("--sandbox_")||flag.startsWith("--spawn_strategy")||flag.startsWith("--strategy"))).toBe(false);
 for(const name of fixture.sandbox.forbiddenRcFlags)expect(rc,name).not.toMatch(new RegExp(`(^|\\s)--(no)?${name}\\b`,"m"));
});

test.skipIf(!process.env.SEMIO_TEST_BAZEL_BINARY||process.platform!=="darwin")("the real macOS Bazel crashes on a non-ASCII install base and its sandbox profile double-encodes a non-ASCII output base, so only an ASCII output base runs sandboxed",()=>{
 const binary=process.env.SEMIO_TEST_BAZEL_BINARY!,base=mkdtempSync(join(process.cwd(),".bazel-sandbox-")),directory=join(base,"d"),workspace=join(directory,"workspace"),environment={...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"},common=["--nosystem_rc","--nohome_rc","--max_idle_secs=30"],emoji=(name:string)=>join(directory,"\u{1F9F0}\uFE0F"+name),asciiInstall=join(directory,"install"),run=(startup:string[],command:string[])=>spawnSync(binary,[...common,...startup,...command],{cwd:workspace,encoding:"utf8",timeout:300000,env:environment}),sandboxed=["build","--lockfile_mode=off","--spawn_strategy=sandboxed","--sandbox_debug","--execution_log_json_file="+join(directory,"log.json"),"//:x"];
 try{
  expect(process.cwd()).toMatch(new RegExp(componentSchema.definitions.HostPaths.const.platforms.darwin.pattern));
  mkdirSync(workspace,{recursive:true});writeFileSync(join(workspace,"MODULE.bazel"),'module(name="ascii")\n');writeFileSync(join(workspace,"BUILD.bazel"),'genrule(name="x",outs=["\u{1F9F0}\uFE0F.txt"],cmd="echo x > $@")\n');
  const install=run([`--output_base=${join(directory,"base-a")}`,`--install_base=${emoji("install")}`],["info"]);expect(install.status).not.toBe(0);expect(install.stderr).toContain("Could not find or load main class");
  const ascii=run([`--output_base=${join(directory,"base-ascii")}`,`--install_base=${asciiInstall}`],sandboxed);expect(ascii.status,ascii.stderr).toBe(0);expect(readFileSync(join(directory,"log.json"),"utf8")).toContain(fixture.sandbox.expectedRunner);
  const emojiBase=emoji("base"),failed=run([`--output_base=${emojiBase}`,`--install_base=${asciiInstall}`],sandboxed);expect(failed.status).not.toBe(0);expect(failed.stderr).toContain("Operation not permitted");
  const sandboxes=join(emojiBase,"sandbox/darwin-sandbox"),profile=readFileSync(join(sandboxes,readdirSync(sandboxes)[0]!,"sandbox.sb"),"utf8"),encoded=Buffer.from(Buffer.from(emojiBase,"utf8").toString("latin1"),"utf8").toString("utf8");expect(profile).toContain(encoded);expect(profile).not.toContain(emojiBase);
 }finally{for(const name of ["base-ascii","\u{1F9F0}\uFE0Fbase"])spawnSync(binary,[...common,`--output_base=${join(directory,name)}`,`--install_base=${asciiInstall}`,"shutdown"],{cwd:workspace,timeout:120000});rmSync(base,{recursive:true,force:true});}
},900000);
