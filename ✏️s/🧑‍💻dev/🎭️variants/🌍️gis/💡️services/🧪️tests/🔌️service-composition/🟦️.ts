import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import * as toml from "@iarna/toml";

/** 🧩 Validates the actual executable composition through independent JSON and Cargo parsers. */
export function nativeServiceCompositionLaws(repoRoot: string): void {
  const owner=resolve(import.meta.dir,"../.."), renderer=resolve(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu");
  const fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔌️service-composition/🔣️.json"),"utf8"));
  const cargo=toml.parse(readFileSync(resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as any;
  assert.equal(cargo.package.name,"semio-s-dev-gis-services");
  assert.equal(cargo.package.metadata.semio.role,"tool");
  assert.equal(cargo.bin[0].name,fixture.binary);
  assert(readFileSync(resolve(owner,"🦀️.rs"),"utf8").includes("gis_map_service_contribution_v1()"));
  assert(readFileSync(resolve(owner,"⌨️entrypoint/🦀️.rs"),"utf8").includes("run_native_v1(semio_s_dev_gis_services::service_contributions_v1())"));
  assert(readFileSync(resolve(renderer,"⌨️native-entrypoint/💾️binary/🦀️.rs"),"utf8").includes("run_native_entrypoint(Vec::new())"));
  const entrypoint=readFileSync(resolve(renderer,"⌨️native-entrypoint/🦀️.rs"),"utf8");
  assert(entrypoint.includes("run_native(&plugin_filter, modules_root, services)"));
  assert(entrypoint.includes("run_smoke(&plugin_filter, modules_root, services)"));
  assert(!/semio_s_|s\.gis/.test(entrypoint));
  console.log(`native-service-composition: cargo=1 real-owner=${fixture.services.length} generic-empty=1 passed`);
}
