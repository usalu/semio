import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{readFile}from"node:fs/promises";
import{readRustSourceClosureV1}from"../../../🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import{join,resolve,dirname}from"node:path";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import TOML from"@iarna/toml";
import ts from"typescript";
import{validateJsonSchemaSubset}from"../../../🧬️schema/✅️validator/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
const owner=join(import.meta.dir,".."),replication=dirname(owner);
test("protocol errors have a closed language-neutral contract with independent SQLite text",()=>{
 expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");try{for(const row of fixture.cases){const statement=db.query(`select case ?1 when 'chain' then 'chain mismatch at commit '||?2 when 'tail' then 'torn tail at offset '||?2 when 'critical' then 'unknown critical record kind 0x'||printf('%x',?2) when 'dictionary' then 'dictionary index out of range: '||?2 when 'order' then 'dictionary out of order: expected base_count '||?2||', got '||?3 when 'verifier' then 'signature verification required but no verifier supplied' when 'signature' then 'signature invalid for commit '||?2 when 'frame' then 'frame back_len mismatch at offset '||?2 when 'limit' then 'limit exceeded: '||?4 when 'malformed' then 'malformed wire at offset '||?2||': '||?4 when 'io' then 'io error: '||?4 end as message`);try{expect((statement.get(row.kind,row.first,row.second,row.text)as{message:string}).message).toBe(row.message);}finally{statement.finalize();}}}finally{db.close();}
 const bytes=Buffer.from(fixture.crypto.message,"hex");expect(bytes.byteLength).toBe(32);expect(bytes.toString("hex")).toBe(fixture.crypto.signature);expect(fixture.crypto.scheme).toBe("fixture/identity");expect(fixture.crypto.key).toBe("caller-owned-key");
 console.log("[DEBUG] Protocol error neutral text matches independent SQLite, including Unicode and NUL; closed schema matches Ajv");
});
test("error retirement and crypto contracts have real independent bounded Cargo owners",()=>{
 for(const [directory,name,boundary,dependencies]of[[owner,fixture.errorOwner,fixture.errorBoundary,["semio-framework-pack-error","semio-framework-diagnostic","semio-framework-value"]],[join(replication,"🔐️crypto"),fixture.cryptoOwner,fixture.cryptoBoundary,[fixture.errorOwner]]]as const){
  const path=join(directory,"📦️packages/🦀️rust/Cargo.toml"),manifest=TOML.parse(readFileSync(path,"utf8"))as any;
  expect(manifest.package.name).toBe(name);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(boundary);expect(Object.keys(manifest.dependencies).sort()).toEqual([...dependencies].sort());expect(resolve(dirname(path),manifest.lib.path)).toBe(join(directory,"🦀️.rs"));
  const body=readFileSync(join(directory,"🦀️.rs"),"utf8");expect(Buffer.byteLength(body)).toBeLessThan(boundary.maxSourceBytes);expect(body).not.toContain("crate::wire::");expect(body).not.toContain("semio_framework_replication::");
 }
 const path=join(replication,"📦️packages/🦀️rust/Cargo.toml"),manifest=TOML.parse(readFileSync(path,"utf8"))as any;
 for(const [name,directory]of[[fixture.errorOwner,owner],[fixture.cryptoOwner,join(replication,"🔐️crypto")]])expect(resolve(dirname(path),manifest.dependencies[name!].path)).toBe(join(directory!,"📦️packages/🦀️rust"));
 const wire=readFileSync(join(replication,"🧾️wire/🦀️.rs"),"utf8");expect(wire).not.toContain("pub enum ProtocolError");expect(wire).not.toContain('mod cause_retirement;');
 const glue=readFileSync(join(replication,"📦️packages/🦀️rust/🦀️.rs"),"utf8");expect(glue).toContain("pub use semio_framework_replication_error::{");expect(glue).toContain("pub use semio_framework_replication_crypto as crypto;");expect(glue).not.toContain('"../../🔐️crypto/🦀️.rs"');
 console.log("[DEBUG] Actual nominal error/retirement and crypto Cargo owners have explicit bounded sources and no upward wire dependency");
});
test("both lower owners have actual workspace membership, registry and editor commands",()=>{
 const repo=resolve(import.meta.dir,"../../../../.."),framework=TOML.parse(readFileSync(join(repo,"🧰️framework/Cargo.toml"),"utf8"))as any,registry=JSON.parse(readFileSync(join(repo,"📋️build-owners.json"),"utf8"));
 for(const [directory,slug]of[["⚠️error","replication-error"],["🔐️crypto","replication-crypto"]]){
  const root="🧰️framework/🔨️modules/📡️replication/"+directory+"/📦️packages/🦀️rust",name="@semio-tech/"+slug+"-rs";expect(framework.workspace.members).toContain(root.slice("🧰️framework/".length));expect(registry.owners.filter((row:any)=>row.name===name)).toEqual([{name,root,manifest:root+"/📋️project.json",packages:[]}]);
  const metadata=JSON.parse(readFileSync(join(repo,root,"📋️project.json"),"utf8"));expect(metadata.name).toBe(name);expect(metadata.targets["test-native"].options.command).toBe("bun ./📜️script.ts test-native");expect(metadata.targets["test-contract"].options.command).toBe("bun ./📜️script.ts test-contract");
  for(const file of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const parsed=ts.parseConfigFileTextToJson(file,readFileSync(join(repo,file),"utf8"));expect(parsed.error).toBeUndefined();for(const command of["test-contract","test-native"])for(const mode of["prototype","release"])expect(parsed.config.configurations.filter((row:any)=>row.command==="bun repo run "+name+":"+command+" --configuration="+mode)).toHaveLength(1);}
 }
 console.log("[DEBUG] Both real lower Cargo members, unique Nx owners and all prototype/release editor commands agree with independent TOML/TypeScript parsers");
});
test("all host cfg production and unit closures fit the real lower compiler Source caps",async()=>{
 const workspace=resolve(import.meta.dir,"../../../../.."),db=new Database(":memory:");db.exec("create table inputs(path text primary key,bytes integer,lines integer)");
 try{for(const [directory,boundary]of[["⚠️error",fixture.errorBoundary],["🔐️crypto",fixture.cryptoBoundary]]as const)for(const host of[{os:"macos",arch:"aarch64",family:"unix"},{os:"linux",arch:"x86_64",family:"unix"},{os:"windows",arch:"x86_64",family:"windows"}])for(const tests of[false,true]){
  const root="🧰️framework/🔨️modules/📡️replication/"+directory,files=await readRustSourceClosureV1([root+"/🦀️.rs"],workspace,["target_arch="+host.arch,"target_os="+host.os,"target_family="+host.family,"target_pointer_width=64",host.family,...(tests?["test"]:[])],async path=>{try{return await readFile(path)}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:join(workspace,root,"📦️packages/🦀️rust"),generated:()=>{throw Error("Actual generated ownership required");}});
  db.exec("delete from inputs");for(const file of files){const bytes=readFileSync(join(workspace,file.path));expect(bytes.byteLength).toBe(file.bytes);expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(file.digest);db.query("insert into inputs values(?,?,?)").run(file.path,file.bytes,file.activeLines);}
  const total=db.query("select count(*) as files,sum(bytes) as bytes,sum(lines) as lines from inputs").get()as{files:number;bytes:number;lines:number};expect(total.files).toBeLessThanOrEqual(boundary.maxSourceFiles);expect(total.bytes).toBeLessThanOrEqual(boundary.maxSourceBytes);expect(total.lines).toBeLessThanOrEqual(boundary.maxActiveLines);console.log("[DEBUG] Actual lower Rust cfg Source closure "+JSON.stringify({directory,host:host.os,tests,...total}));
 }}finally{db.close();}
});
