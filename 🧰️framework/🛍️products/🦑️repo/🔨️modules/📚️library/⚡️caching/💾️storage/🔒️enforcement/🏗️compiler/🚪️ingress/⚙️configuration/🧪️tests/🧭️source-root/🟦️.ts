/** 📸️ Contract: a request reads the checkout snapshot only for manifests inside it; host tools on the live workspace (the scheduler) never resolve against the snapshot root. */
import {expect,test} from "bun:test";
import {join} from "node:path";
import {sourceRootOfV1} from "../../🟦️.ts";

const request=(manifest:string,sourceRoot?:string)=>({workspace:"/live",manifest,...sourceRoot===undefined?{}:{sourceRoot}});

test("the source root is the snapshot only for manifests inside it",()=>{
 expect(sourceRootOfV1(request("/live/a/Cargo.toml"))).toBe("/live");
 expect(sourceRootOfV1(request("/snap/a/Cargo.toml","/snap"))).toBe("/snap");
 expect(sourceRootOfV1(request("/snap/Cargo.toml","/snap"))).toBe("/snap");
 expect(sourceRootOfV1(request("/live/🧰️framework/x/Cargo.toml","/snap"))).toBe("/live");
 expect(sourceRootOfV1(request("/snapshot-other/Cargo.toml","/snap"))).toBe("/live");
});

import {cargoConfigurationArgumentsV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import {repositoryCargoConfigurationPathV1,repositoryCargoManifestArgumentsV1} from "../../../../../../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {repositoryCargoStorageArgumentsV1} from "../../../📥️arguments/📋️policy/🟦️.ts";

const build="/live/.cache/b/r/m/u/l/p5c2c93",target=build+"/t";
const passArguments=(manifest:string)=>["build","--locked","--manifest-path",manifest,"-p","semio-hub-playbook-component","--lib","--target","wasm32-wasip2","--profile","release","--keep-going","--message-format=json"];

test("the storage admission accepts the --config path the argument builder names, inside a snapshot and for a live manifest, for a lean-profile root",()=>{
 const options={workspace:"/live",sourceRoot:"/live/.cache/s/0123456789ab"};
 for(const manifest of ["/live/.cache/s/0123456789ab/🌎️hub/Cargo.toml","/live/🧰️framework/scheduler/Cargo.toml"]){
  const request={workspace:options.workspace,sourceRoot:options.sourceRoot,manifest},arguments_=cargoConfigurationArgumentsV1(passArguments(manifest),repositoryCargoConfigurationPathV1(options,manifest),build);
  const context={version:1 as const,compilerConfiguration:join(sourceRootOfV1(request),".cargo/config.toml"),buildDirectory:build,targetDirectory:target};
  expect(()=>repositoryCargoStorageArgumentsV1(arguments_,context)).not.toThrow();
  expect(arguments_.includes(context.compilerConfiguration)).toBe(true);
 }
});

test("a --config path of the other root, and a profile selector among the storage arguments, are refused",()=>{
 const options={workspace:"/live",sourceRoot:"/live/.cache/s/0123456789ab"},manifest="/live/.cache/s/0123456789ab/🌎️hub/Cargo.toml";
 const context={version:1 as const,compilerConfiguration:join(options.sourceRoot,".cargo/config.toml"),buildDirectory:build,targetDirectory:target};
 const live=cargoConfigurationArgumentsV1(passArguments(manifest),"/live/.cargo/config.toml",build);
 expect(()=>repositoryCargoStorageArgumentsV1(live,context)).toThrow("Cargo configuration belongs to original repository admission");
 const lean=[...cargoConfigurationArgumentsV1(passArguments(manifest),repositoryCargoConfigurationPathV1(options,manifest),build),"--config",'profile.release.opt-level="s"'];
 expect(()=>repositoryCargoStorageArgumentsV1(lean,context)).toThrow("Cargo configuration belongs to original repository admission");
});

test("every spelling of a manifest's --config selectors goes through one builder: a further step under a snapshot view never names the live config, the old live-root spelling is refused",()=>{
 const options={workspace:"/live",sourceRoot:"/live/.cache/s/0123456789ab"},manifest="/live/.cache/s/0123456789ab/🌎️hub/Cargo.toml";
 const context={version:1 as const,compilerConfiguration:join(options.sourceRoot,".cargo/config.toml"),buildDirectory:build,targetDirectory:target};
 const first=repositoryCargoManifestArgumentsV1(options,manifest,passArguments(manifest),build);
 const step=repositoryCargoManifestArgumentsV1(options,manifest,[...first,"-Z","unstable-options","--unit-graph"],build);
 expect(()=>repositoryCargoStorageArgumentsV1(step,context)).not.toThrow();
 expect(step.filter(argument=>argument.endsWith(".cargo/config.toml"))).toEqual([context.compilerConfiguration]);
 const old=cargoConfigurationArgumentsV1([...first,"-Z","unstable-options","--unit-graph"],join(options.workspace,".cargo/config.toml"),build);
 expect(old.filter(argument=>argument.endsWith(".cargo/config.toml")).length).toBe(2);
 expect(()=>repositoryCargoStorageArgumentsV1(old,context)).toThrow("Cargo configuration belongs to original repository admission");
});
