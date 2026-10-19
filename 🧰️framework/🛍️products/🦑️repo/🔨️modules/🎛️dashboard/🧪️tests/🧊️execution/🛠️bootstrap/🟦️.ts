import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync, lstatSync } from "node:fs";
import { resolve, join } from "node:path";
import ts from "typescript";
import { dashboardBuildDescendantsV1, dashboardInvocation, dashboardPackageRoot } from "../../../📦️installation/🟦️.ts";
import { createBuildBootstrapCapabilitiesV1 } from "../../../../📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import { createNativeBuildCallerCapabilitiesV1, type NativeBuildCallerCapabilitiesV1 } from "../../../../📚️library/🏗️build/📥️caller/🟦️.ts";
import callerSchema from "../../../../📚️library/🏗️build/🧬️schema/📥️caller.json";
import orchestrationSchema from "../../../../📚️library/🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json";
import executionSchema from "../../../../📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../📦️installation/🧬️schema/🚀️launch/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🧊️execution/🔣️.json" with { type: "json" };

test("dashboard bootstrap closes exact authored build and compile requests before Original birth", async () => {
  const oracle = new Ajv({ strict: true }), admit = oracle.compile(schema.definitions.BootstrapCases), owner = JSON.parse(readFileSync(new URL("../../../📦️packages/🦀️rust/📋️project.json", import.meta.url), "utf8"));
  expect(admit(fixture.bootstrap), JSON.stringify(admit.errors)).toBe(true);
  const file = new URL("../../../../📚️library/🏗️build/📥️caller/🟦️.ts", import.meta.url), source = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, source, ts.ScriptTarget.Latest, true);
  const reader = tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "readOwnedBuildInvocationV1")!;
  const compiled = ts.transpileModule(reader.getText(tree).replace(/^export\s+/u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(compiled.diagnostics).toEqual([]);
  const database = new Database(":memory:");
  try { for (const row of fixture.bootstrap) {
    expect(dashboardBuildDescendantsV1(row.args), row.name).toEqual(row.direct);
    const sql = database.query("with recursive edges(parent,child) as (select ?1||':'||target.key, json_extract(child.value,'$.project')||':'||json_extract(child.value,'$.target') from json_each(?2,'$.targets') target join json_each(target.value,'$.metadata.semio.execution.children') child), reachable(child,depth) as (select ?3,0 union all select edges.child,depth+1 from reachable join edges on edges.parent=reachable.child where depth<64) select json_group_array(json_array('run',child)) descendants from (select child from reachable where depth>0 order by depth,child)").get(owner.name, JSON.stringify(owner), row.args[1]) as { descendants: string };
    expect(JSON.parse(sql.descendants)).toEqual(row.descendants);
    for (const tool of ["nx", "bazel"] as const) for (const mode of ["prototype", "release"] as const) {
      const environment = { SEMIO_BUILD_TOOL: tool, SEMIO_BUILD_MODE: mode, CARGO_NET_OFFLINE: "true" }, initial = createNativeBuildCallerCapabilitiesV1(process.cwd(), row.args, tool, environment), grant = await createBuildBootstrapCapabilitiesV1(process.cwd(), ["repo", ...row.args], environment) as NativeBuildCallerCapabilitiesV1;
      expect(grant.arguments).toEqual(initial.arguments); expect(grant.selection).toEqual(initial.selection); expect(grant.native).toEqual(initial.native); expect([grant.tool, grant.buildMode]).toEqual([tool, mode]);
      expect(grant.launches.filter(request => request[0] === "repo")).toEqual([["repo", ...row.args], ...row.descendants.map(request => ["repo", ...request])]);
      const token = Object.freeze({ fixture: "received" }), ports = { schema: callerSchema, resolve, readIssuedScriptProcessEnvelopeV1: (value: unknown) => { expect(value).toBe(token); return { capabilities: grant }; }, validateJsonSchemaSubset: (schema: object, value: unknown) => oracle.compile(schema)(value) ? [] : ["refused"] };
      const read = new Function(...Object.keys(ports), compiled.outputText + ";return readOwnedBuildInvocationV1;")(...Object.values(ports));
      for (const request of row.descendants) expect(read(process.cwd(), token, request)).toBe(grant);
      expect(() => read(process.cwd(), token, ["run", "@semio-tech/repo-dashboard-rs:unowned"])).toThrow("Undeclared original Repo build selection");
      expect(() => read(process.cwd(), token, ["run", "@semio-tech/repo-dashboard-rs:build", "--unexpected"])).toThrow("Undeclared original Repo build selection");
    }
  } } finally { database.close(); }
  expect(dashboardInvocation(["run", "@semio-tech/repo-dashboard-rs:workflow"])).toBeUndefined();
  expect(dashboardInvocation(["run", "@semio-tech/repo-dashboard-rs:run", "--graph=stdout"])).toBeUndefined();
  expect(dashboardInvocation(["run", "@semio-tech/repo-dashboard-rs:run", "--", "workflow", "status"])).toEqual(["workflow", "status"]);
  expect(dashboardPackageRoot).toBe(resolve(import.meta.dir, "../../../📦️packages/🦀️rust"));
  console.log(`[DEBUG] Actual dashboard prebirth closure matched ${fixture.bootstrap.length} authored vectors across both backends and modes; exact reader body ports isolated private Original receipt`);
});

test("dashboard bootstrap refuses malformed or unowned authored child declarations", () => {
  const file = new URL("../../../📦️installation/🟦️.ts", import.meta.url), source = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, source, ts.ScriptTarget.Latest, true);
  const method = tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "dashboardBuildDescendantsV1")!;
  const body = ts.transpileModule(method.getText(tree).replace(/^export\s+/u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const original = JSON.parse(readFileSync(join(dashboardPackageRoot, "📋️project.json"), "utf8")), oracle = new Ajv({ strict: true }).compile(orchestrationSchema);
  for (const mutation of ["owner", "foreign-child", "missing-child", "extra-key", "missing-children"]) {
    const owner = structuredClone(original), declaration = owner.targets.run.metadata.semio.execution;
    if (mutation === "owner") owner.name = "another-owner";
    if (mutation === "foreign-child") declaration.children[0].project = "another-owner";
    if (mutation === "missing-child") declaration.children[0].target = "unowned";
    if (mutation === "extra-key") declaration.unowned = true;
    if (mutation === "missing-children") delete declaration.children;
    expect(oracle(declaration)).toBe(!["extra-key", "missing-children"].includes(mutation));
    const ports = { join, dashboardPackageRoot, lstatSync, orchestrationSchema, validateJsonSchemaSubset, readFileSync: (path: string) => { expect(path).toBe(join(dashboardPackageRoot, "📋️project.json")); return JSON.stringify(owner); } };
    const descendants = new Function(...Object.keys(ports), body + ";return dashboardBuildDescendantsV1;")(...Object.values(ports));
    expect(() => descendants(["run", "@semio-tech/repo-dashboard-rs:run"]), mutation).toThrow(/dashboard bootstrap/);
  }
  console.log("[DEBUG] Actual dashboard metadata reader refused foreign owners, missing child targets and closed-schema mutations without changing authored files");
});

test("dashboard bootstrap accepts closed source and native leaves without child authority",()=>{
 const file=new URL("../../../📦️installation/🟦️.ts",import.meta.url),source=readFileSync(file,"utf8"),tree=ts.createSourceFile(file.pathname,source,ts.ScriptTarget.Latest,true),method=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="dashboardBuildDescendantsV1")!,body=ts.transpileModule(method.getText(tree).replace(/^export\s+/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,original=JSON.parse(readFileSync(join(dashboardPackageRoot,"📋️project.json"),"utf8")),oracle=new Ajv({strict:true}).compile(executionSchema);
 for(const row of fixture.bootstrapLeaves){expect(oracle(row.declaration)).toBe(row.accepted);const owner=structuredClone(original);owner.targets.run.metadata.semio.execution=row.declaration;const ports={join,dashboardPackageRoot,lstatSync,orchestrationSchema,executionSchema,validateJsonSchemaSubset,readFileSync:()=>JSON.stringify(owner)},read=new Function(...Object.keys(ports),body+";return dashboardBuildDescendantsV1;")(...Object.values(ports));if(row.accepted)expect(read(["run","@semio-tech/repo-dashboard-rs:run"])).toEqual([]);else expect(()=>read(["run","@semio-tech/repo-dashboard-rs:run"])).toThrow("Closed dashboard bootstrap declaration required");}
});
