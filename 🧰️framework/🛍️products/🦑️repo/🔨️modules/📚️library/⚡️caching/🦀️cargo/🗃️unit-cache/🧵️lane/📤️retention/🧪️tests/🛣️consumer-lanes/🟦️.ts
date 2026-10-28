/** 🛣️ Contract: the lane an artifact consumer takes is the same lane for the same product, and the directories a real pooled lane hands out are recognised as owned by their store (and nobody else's). Real lane paths from the real pool; the crate selection of `buildRepositoryCargoArtifacts` is checked on real manifests.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {crateKeyV1} from "../../../🟦️.ts";
import {acquirePooledLaneV1} from "../../../🏊️pool/🟦️.ts";
import {cargoLaneCratesV1} from "../../../../../../📦️artifacts/🏗️native-build/🟦️.ts";
import {compilerOriginOwnedV1,laneBuildDirectoryOwnedV1} from "../../🟦️.ts";

const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});

test("the crates of an artifact build are its packages, else its manifest's package, else its manifest path, so one product keeps one warm lane",()=>{
 const root=mkdtempSync(join(base,"cl-")),manifest=join(root,"owner","Cargo.toml");
 try{
  mkdirSync(join(root,"owner"));writeFileSync(manifest,'[package]\nname = "alpha"\nversion = "0.1.0"\n');
  expect(cargoLaneCratesV1(["--package","beta@1.0","--bin","x"],manifest,root)).toEqual(["beta"]);
  expect(cargoLaneCratesV1(["-pgamma"],manifest,root)).toEqual(["gamma"]);
  expect(cargoLaneCratesV1(["--bin","x","--profile","prototype"],manifest,root)).toEqual(["alpha"]);
  expect(crateKeyV1(cargoLaneCratesV1(["--lib"],manifest,root))).toBe(crateKeyV1(["alpha"]));
  writeFileSync(manifest,"[[[ not toml");expect(cargoLaneCratesV1([],manifest,root)).toEqual(["owner/Cargo.toml"]);
  expect(cargoLaneCratesV1([],join(root,"gone","Cargo.toml"),root)).toEqual(["gone/Cargo.toml"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the build and target directories of a real pooled lane, and the recipe slot inside it, are owned by their cache root and by no other",async()=>{
 const root=mkdtempSync(join(base,"cl-")),cache=join(root,"u"),other=join(root,"v"),shared={build:join(root,"b","p"),target:join(root,"t","p")};
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["a"],quotaBytes:Number.MAX_SAFE_INTEGER}),slot=join(lane.paths.build,"t","0123456789abcdef");
  try{
   expect(laneBuildDirectoryOwnedV1(cache,lane.paths.build)).toBe(true);
   expect(compilerOriginOwnedV1({cacheRoot:cache,shared,origin:{buildDirectory:lane.paths.build,targetDirectory:lane.paths.target}})).toBe(true);
   expect(compilerOriginOwnedV1({cacheRoot:cache,shared,origin:{buildDirectory:lane.paths.build,targetDirectory:slot}})).toBe(true);
   expect(compilerOriginOwnedV1({cacheRoot:other,shared,origin:{buildDirectory:lane.paths.build,targetDirectory:lane.paths.target}})).toBe(false);
   expect(laneBuildDirectoryOwnedV1(other,lane.paths.build)).toBe(false);
   expect(laneBuildDirectoryOwnedV1(cache,lane.paths.target)).toBe(false);
   expect(compilerOriginOwnedV1({cacheRoot:cache,shared,origin:{buildDirectory:shared.build,targetDirectory:shared.target}})).toBe(true);
   expect(compilerOriginOwnedV1({cacheRoot:cache,shared,origin:{buildDirectory:shared.build,targetDirectory:lane.paths.target}})).toBe(false);
  }finally{lane.release();}
 }finally{rmSync(root,{recursive:true,force:true});}
});
