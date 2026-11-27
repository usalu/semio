/** 🪶️ Contract: an offline lockfile-only Cargo command (`update`, `generate-lockfile`) selects no native owner preparation, so it never spawns the `synchronize` child that queues on the exclusive `cargo-preparation:` lease; online lock commands and every compiling command keep their preparation, and an independent Python reading of the arguments agrees. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {cargoLockOnlyInvocationV1,cargoWorkspacePreparationInvocationV1} from "../../🟦️.ts";

const python=testPythonV1()??"python3",script=`
import json,sys
for args in json.load(sys.stdin):
    print(json.dumps(bool(args) and args[0] in ("update","generate-lockfile") and ("--offline" in args or "--frozen" in args)))
`;
const vectors=[
 ["update","--offline","--workspace","--manifest-path","Cargo.toml"],
 ["generate-lockfile","--frozen","--manifest-path","Cargo.toml"],
 ["update","--workspace","--manifest-path","Cargo.toml"],
 ["generate-lockfile","--manifest-path","Cargo.toml"],
 ["build","--offline","--manifest-path","Cargo.toml"],
 ["metadata","--offline","--manifest-path","Cargo.toml"],
 ["fetch","--frozen","--manifest-path","Cargo.toml"],
 [],
];

test("an offline lockfile-only invocation is recognised, nothing else is, and Python agrees",()=>{
 const run=spawnSync(python,["-I","-c",script],{input:JSON.stringify(vectors),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=run.stdout.trim().split("\n").map(line=>JSON.parse(line) as boolean);
 expect(vectors.map(args=>cargoLockOnlyInvocationV1(args))).toEqual(expected);
 expect(expected).toEqual([true,true,false,false,false,false,false,false]);
});

test("a repository workspace selects its owner preparation for every command except an offline lockfile-only one",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"lock-only-")));
 try{
  writeFileSync(join(root,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["kernel"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=["kernel/Cargo.toml"]\n[workspace.package]\nedition="2021"\n');
  mkdirSync(join(root,"kernel"));writeFileSync(join(root,"kernel/Cargo.toml"),'[package]\nname="owned-kernel"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="lib.rs"\n');writeFileSync(join(root,"kernel/lib.rs"),"");
  const storage=repositoryCargoPreparationStorageV1(root,root),selected=(args:readonly string[])=>cargoWorkspacePreparationInvocationV1(storage,root,args,root,{})!==undefined;
  expect(vectors.slice(0,7).map(args=>selected(args))).toEqual([false,false,true,true,true,true,true]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a standalone package workspace the repository inventory does not admit runs from its own directory without preparation",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"standalone-")));
 try{
  writeFileSync(join(root,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["kernel"]\nexclude=["bridge"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=["bridge/**"]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=["kernel/Cargo.toml"]\n[workspace.package]\nedition="2021"\n');
  mkdirSync(join(root,"kernel"));writeFileSync(join(root,"kernel/Cargo.toml"),'[package]\nname="owned-kernel"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="lib.rs"\n');writeFileSync(join(root,"kernel/lib.rs"),"");
  mkdirSync(join(root,"bridge"));writeFileSync(join(root,"bridge/Cargo.toml"),'[workspace]\n[package]\nname="standalone-bridge"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n');writeFileSync(join(root,"bridge/lib.rs"),"");
  const storage=repositoryCargoPreparationStorageV1(root,root),selected=(args:readonly string[],cwd:string)=>cargoWorkspacePreparationInvocationV1(storage,root,args,cwd,{});
  expect(selected(["check"],join(root,"bridge"))).toBeUndefined();
  expect(()=>selected(["check","-p","standalone-bridge"],root)).toThrow("Unknown current repository Cargo package: standalone-bridge");
  expect(selected(["check"],join(root,"kernel"))).toBeDefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});
