import { describe, expect, test } from "bun:test";
import { featureRow, forwardableFeatures, forwardFeatures, withDevDependencyRows } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { identifierSet } from "../../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { crateTreeWasmCfg } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
import { renderMirrorFiles } from "../../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { dropBaseUnresolvedNames, unresolvedSubjects } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { declaredNames, hostPrivate } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟢️shared/🟦️.ts";

describe("artifact crate carve", () => {
  test("prints the cfg of the wasm32 guest target, which the planner scans for code only that target compiles", () => {
    const cfg = crateTreeWasmCfg();
    expect(cfg).toContain("target_arch=wasm32");
    expect(cfg).not.toContain("unix");
  });

  test("keeps the cfg gate of a module in the wrapper and the imported leaf a mirror root declares in its place", () => {
    const gates = new Map<string, readonly string[]>([["editor::wasm", ['#[cfg(target_arch = "wasm32")]']]]);
    const spec = {
      doc: "//! mirror", newRootDir: "/tmp/mirror-x", innerAttributes: [], externLines: [], movedItemsText: "", itemImports: [], owned: [],
      imported: [{ path: ["editor", "wasm", "protocol"], crateIdent: "other_crate", visibility: "pub", order: 1, glob: false }, { path: ["editor", "wasm"], crateIdent: "third_crate", visibility: "pub", order: 0, glob: true }],
      wrapperNodes: new Map(), gates,
    };
    const text = renderMirrorFiles(spec).root;
    expect(text).toContain('#[cfg(target_arch = "wasm32")]\n    #[path = "."]\n    pub mod wasm {');
    expect(text).toContain("pub use third_crate::editor::wasm::*;");
    const without = renderMirrorFiles({ ...spec, gates: undefined }).root;
    expect(without).not.toContain("wasm32");
  });

  test("a test that only mentions a host import as a field, a path segment or its own binding does not need that import", () => {
    const host = "use crate::snapshot::{Component, Point2};\nuse crate::mep::Mep;\nuse crate::walls;\nuse crate::plan;\nfn run() {}\n";
    const body = [
      "fn graph(session: &Session) {",
      "    let walls = || 1;",
      "    assert_eq!(session.mep.len(), walls());",
      "    let row = Row { mep: 1, walls: 2 };",
      "    let node = ModelNode::Component(id);",
      "    for (local, plan) in pairs {}",
      "}",
    ].join("\n");
    expect(hostPrivate(host, identifierSet(body), new Set<string>(), body)).toEqual({ items: [], carry: [], relative: [] });
    const pattern = "fn graph(v: Value) { let Point2 { x, .. } = v else { return }; }";
    expect(hostPrivate(host, identifierSet(pattern), new Set<string>(), pattern).carry).toEqual(["use crate::snapshot::Point2;"]);
    const real = "fn graph() { let point = Point2 { x: 0.0, y: 0.0 }; let c: Component = point.into(); }";
    expect(hostPrivate(host, identifierSet(real), new Set<string>(), real).carry).toEqual(["use crate::snapshot::{Component, Point2};"]);
    const elsewhere = "fn graph() { let point = crate::snapshot::Point2 { x: 0.0, y: 0.0 }; }";
    expect(hostPrivate(host, identifierSet(elsewhere), new Set<string>(), elsewhere, ["a"]).carry).toEqual([]);
    const viaHost = "fn graph() { let point = crate::a::Point2 { x: 0.0, y: 0.0 }; }";
    expect(hostPrivate(host, identifierSet(viaHost), new Set<string>(), viaHost, ["a", "component"]).carry).toEqual(["use crate::snapshot::Point2;"]);
  });

  test("an impl of an imported trait or type does not declare that name in the body", () => {
    const body = "impl Work<App> for Job { fn run(&self) {} }\nstruct Own;\nfn helper() {}\n";
    expect([...declaredNames(body)].sort()).toEqual(["Own", "helper"]);
    const host = "use crate::retained::{Work, Step};\n";
    expect(hostPrivate(host, identifierSet(body), declaredNames(body), body).carry).toEqual(["use crate::retained::Work;"]);
  });

  test("a host import that names one of the host's own child modules is relative, not an absolute import to carry", () => {
    const host = "#[path = \"frames/🦀️.rs\"]\nmod frames;\nuse frames::Rigid;\nuse std::collections::BTreeMap;\n";
    const body = "fn walls(r: Rigid, m: BTreeMap<u8, u8>) {}";
    const found = hostPrivate(host, identifierSet(body), new Set<string>(), body);
    expect(found.relative).toEqual(["use frames::Rigid;"]);
    expect(found.carry).toEqual(["use std::collections::BTreeMap;"]);
  });

  test("an unresolved last path segment is a re-worded unresolved name too", () => {
    expect(unresolvedSubjects("E0433\tcannot find `BimViewerLabels` in `terminology`\tcould not find `BimViewerLabels` in `terminology`")).toEqual(["BimViewerLabels"]);
    expect(dropBaseUnresolvedNames(["E0432\tunresolved import `crate::viewer::bim::terminology::BimViewerLabels`"], ["E0433\tcannot find `BimViewerLabels` in `terminology`\tx", "E0433\tcannot find `Other` in `terminology`\ty"])).toEqual(["E0433\tcannot find `Other` in `terminology`\ty"]);
  });

  test("keeps the optional dependencies a testkit feature enables and forwards the feature to the carved crates", () => {
    const manifest = '[package]\nname = "x"\n\n[features]\ntestkit = ["dep:serde_json", "dep:quick-xml"]\nempty = []\n\n[dependencies]\nserde_json = { workspace = true, optional = true }\n';
    const features = forwardableFeatures(manifest);
    expect(features).toEqual([{ name: "testkit", entries: ["dep:serde_json", "dep:quick-xml"] }, { name: "empty", entries: [] }]);
    expect(featureRow(features[0]!, ["demo-a", "demo-b"])).toBe('testkit = ["dep:serde_json", "dep:quick-xml", "demo-a/testkit", "demo-b/testkit"]');
    expect(featureRow(features[1]!, [])).toBe("empty = []");
    const forwarded = forwardFeatures(manifest, features, ["demo-a"]);
    expect(forwarded).toContain('testkit = ["dep:serde_json", "dep:quick-xml", "demo-a/testkit"]');
    expect(forwarded).toContain('empty = ["demo-a/empty"]');
    expect(forwarded).toContain("[dependencies]\nserde_json = { workspace = true, optional = true }");
  });

  test("adds the carved crates as dev-dependencies of the artifact manifest, creating the section when it is absent", () => {
    const rows = ['demo-a = { path = "../a", features = ["testkit"] }'];
    expect(withDevDependencyRows("[package]\nname = \"x\"\n\n[dev-dependencies]\nserde_json.workspace = true\n", rows)).toBe('[package]\nname = "x"\n\n[dev-dependencies]\n' + rows[0] + '\nserde_json.workspace = true\n');
    expect(withDevDependencyRows("[package]\nname = \"x\"\n", rows)).toBe('[package]\nname = "x"\n\n[dev-dependencies]\n' + rows[0] + "\n");
    expect(withDevDependencyRows("[package]\n", [])).toBe("[package]\n");
  });
});
