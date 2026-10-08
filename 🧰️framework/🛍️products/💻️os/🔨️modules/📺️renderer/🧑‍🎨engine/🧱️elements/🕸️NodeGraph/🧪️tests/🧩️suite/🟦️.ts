import Ajv from "ajv/dist/2020.js";
import viewport from "../../../../../../../../../🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json";
import sceneContract from "../../../../../../../../../🔨️modules/🗺️surface/🕸️node-graph/📡️scene/🧬️schema/🔣️.json";
import inputContract from "../../📡️scene/🧬️schema/🔣️.json";
import inputCorpus from "../../📡️scene/🧫️fixtures/🔣️.json";
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{parse as parseToml}from"@iarna/toml";
import refusalCorpus from "../../⚠️refusal/🧫️fixtures/🔣️.json";
type CargoManifest={package:{name:string};lib?:{name?:string};dependencies:Record<string,{path?:string}>};
test("OS graph public refusal interfaces use their defining general packages", () => {
  const owner = resolve(import.meta.dir, "../..");
  const manifestSource = readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const manifestValue = Bun.TOML.parse(manifestSource);
  expect(manifestValue).toEqual(parseToml(manifestSource));
  const manifest = manifestValue as CargoManifest;
  expect(manifest.dependencies[refusalCorpus.packErrorProvider]?.path).toBe("../../../../../../../../../🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust");
  const providerSource = readFileSync(resolve(owner, "📦️packages/🦀️rust", manifest.dependencies[refusalCorpus.packErrorProvider]!.path!, "Cargo.toml"), "utf8");
  const providerValue = Bun.TOML.parse(providerSource);
  expect(providerValue).toEqual(parseToml(providerSource));
  const provider = providerValue as CargoManifest;
  expect(provider.package.name).toBe(refusalCorpus.packErrorProvider);
  expect(provider.lib?.name ?? provider.package.name.replaceAll("-", "_")).toBe("semio_framework_pack_error");
  const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
  expect(source).toMatch(/^use semio_framework_pack_error::PackError;/m);
  expect(source).toContain("Json(ValueError)");
  expect(source).toContain("Pack(PackError)");
  expect(source).not.toMatch(/pub fn from_json\(value: &Value\)/);
  expect(source).not.toContain("impl From<serde_json::Error> for NodeGraphError");
});


test("OS scene receiving input has variable grants and explicit frame ownership",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(viewport).addSchema(sceneContract).addSchema(inputContract);
 const request=ajv.getSchema(inputContract.$id)!;
 for(const format of inputCorpus.formats)expect(request({format,inputBytes:1024,maximumOwnedBytes:inputCorpus.maximumOwnedBytes,maximumDepth:inputCorpus.maximumDepth,maximumItems:inputCorpus.maximumItems})).toBe(true);
 for(const format of ["Auto","body",null])expect(request({format,inputBytes:1024,maximumOwnedBytes:inputCorpus.maximumOwnedBytes,maximumDepth:inputCorpus.maximumDepth,maximumItems:inputCorpus.maximumItems})).toBe(false);
 const grant=ajv.getSchema(inputContract.$id+"#/$defs/InputGrant")!;
 for(const maximumUnits of inputCorpus.units)for(const maximumCapacityBytes of [0,1024])for(const cancelled of [false,true])expect(grant({maximumUnits,maximumCapacityBytes,cancelled})).toBe(true);
 for(const maximumUnits of [-1,0.5,null])expect(grant({maximumUnits,maximumCapacityBytes:1024,cancelled:false})).toBe(false);
 const step=ajv.getSchema(inputContract.$id+"#/$defs/InputStep")!;
 for(const units of inputCorpus.units)expect(step({units,copiedBytes:Math.max(0,units-1),admittedBytes:units===0?0:1024})).toBe(true);
 for(const copiedBytes of [-1,0.5,null])expect(step({units:1,copiedBytes,admittedBytes:1024})).toBe(false);
 const progress=ajv.getSchema(inputContract.$id+"#/$defs/Progress")!;
 for(const phase of inputContract.$defs.Progress.properties.phase.enum)expect(progress({phase,inputBytes:0,totalInputBytes:1024,admittedBytes:0,complete:false})).toBe(true);
});
