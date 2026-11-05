import {test,expect} from "bun:test";
import Ajv from "ajv";
import {posix,win32,resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "./🧫️fixtures/🧬️.json" with {type:"json"};
import {cargoCompilerOutputPathsV1,requireCargoCompilerOutputPathsV1} from "../🟦️.ts";

test("all original admitted compiler units fit the compact backing on every host",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true});
 expect(ajv.compile(schema)(fixture)).toBe(true);
 const physical=ajv.compile({type:"array",maxItems:fixture.maximumUnits*2,items:{type:"integer",minimum:0,maximum:255}});
 for(const root of fixture.roots){
  const paths=root.flavor==="win32"?win32:posix;
  const build=paths.join(root.root,fixture.partition,"m/u/l",fixture.lane,"b");
  let longest=0;
  for(const row of [...fixture.admitted,...fixture.dependencies,...fixture.additional]){
   const unit={profile:fixture.profile,platform:fixture.platform,...row};
   for(const path of cargoCompilerOutputPathsV1(build,unit,root.flavor as "posix"|"win32")){
    expect(paths.isAbsolute(path)).toBe(true);
    expect(physical([...Buffer.from(path,"utf16le")]),path).toBe(true);
    longest=Math.max(longest,path.length);
   }
  }
  expect(longest).toBeLessThanOrEqual(fixture.maximumUnits);
 }
});

test("a transitive overlong compiler output is refused before compilation",()=>{
 const unit={packageName:"semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests",crateName:"semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests",profile:fixture.profile,platform:fixture.platform};
 expect(()=>requireCargoCompilerOutputPathsV1(posix.join("/workspace/semio",".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l",fixture.lane,"b"),[{...unit,packageName:"short",crateName:"short"},unit])).toThrow();
 expect(()=>requireCargoCompilerOutputPathsV1(posix.join("/workspace/semio",fixture.partition,"m/u/l",fixture.lane,"b"),[unit])).not.toThrow();
});

test("strict storage resolves the checkout-relative Rust partition",async()=>{
 const root=resolve("/workspace/semio");
 const owner=await import("../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🟦️.ts");
 expect(owner.repositoryStrictStorageV1(root).directory).toBe(resolve(root,fixture.partition));
});
