import {describe,expect,test} from "bun:test";
import {join} from "node:path";
import {generatedSourceProducersV1,readGeneratorContractsV1} from "../🟦️.ts";

const root=join(import.meta.dir,"../../../../../../../../..");
const styling="🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust";

/** 🧪️ The ingress selects the styling token producer for every crate that compiles the generated tokens file.
 * @see ../🟦️.ts */
describe("generated source ingress preparation",()=>{
 test("a crate depending on the styling crate gets the styling token producer",()=>{
  const producers=generatedSourceProducersV1(root,join(root,styling,"Cargo.toml"));
  expect(producers.map(producer=>producer.id)).toContain("styling-tokens");
  const tokens=producers.find(producer=>producer.id==="styling-tokens")!;
  expect(tokens.args).toEqual(["./📜️script.ts","generate"]);
  expect(tokens.cwd).toBe(styling);
 });
 test("a contract without the opt-in flag is not prepared",()=>{
  const contracts=Object.fromEntries(Object.entries(readGeneratorContractsV1(root)).map(([id,contract])=>[id,{...contract,ingressPreparation:undefined}]));
  expect(generatedSourceProducersV1(root,join(root,styling,"Cargo.toml"),contracts)).toEqual([]);
 });
 test("a manifest outside the repository selects nothing",()=>{
  expect(generatedSourceProducersV1(root,"/nonexistent/Cargo.toml",readGeneratorContractsV1(root))).toEqual([]);
 });
 test("every ingress-prepared contract names a plain repo script target",()=>{
  for(const [id,contract] of Object.entries(readGeneratorContractsV1(root)))if(contract.ingressPreparation)expect(contract.nativeConsumers?.length,id).toBeGreaterThan(0);
 });
});
