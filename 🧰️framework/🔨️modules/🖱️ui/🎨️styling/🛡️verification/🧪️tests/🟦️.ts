import assert from "node:assert/strict";
import Ajv from "ajv";
import ts from "typescript";
import colorString from "color-string";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import * as verification from "../🟦️.ts";

/** 🛡️Proves explicit owner scopes and source rules against AJV and TypeScript syntax. */
export function proveStylingVerificationContractV1(): number {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  assert.equal(ajv.getSchema(`${schema.$id}#/$defs/CasesV1`)!(corpus), true);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/ScopeV1`)!;
  const applicationSource = ajv.getSchema(`${schema.$id}#/$defs/ApplicationPathV1`)!;
  const colorSource = ajv.getSchema(`${schema.$id}#/$defs/ColorApplicationPathV1`)!;
  const failures: string[] = [];
  const manifestOracle = ajv.getSchema(`${schema.$id}#/$defs/SourceManifestV1`)!;
  for (const row of corpus.sources) {
    const input = row.input as { roots: string[]; files: string[] };
    const canonical = (paths: string[]): boolean => paths.every(path => path.normalize("NFC") === path) && new Set(paths).size === new Set(paths.map(path => path.toLowerCase())).size;
    assert.equal(manifestOracle(input) && canonical(input.roots) && canonical(input.files), row.accepted, row.id + ": AJV source oracle");
    let accepted = false, files: readonly string[] = [];
    try { files = verification.admitStylingSourceManifestV1(input).files; accepted = true; } catch {}
    if (accepted !== row.accepted || JSON.stringify(files) !== JSON.stringify(row.files)) failures.push(row.id);
  }
  for (const row of corpus.scopes) {
    const roots = (row.input as { roots: string[] }).roots;
    const oracle = validate(row.input) && roots.every(path => path.normalize("NFC") === path) && new Set(roots).size === new Set(roots.map(path => path.toLowerCase())).size;
    assert.equal(oracle, row.accepted, `${row.id}: AJV path oracle`);
    let accepted = false, actual: readonly string[] = [];
    try { actual = verification.admitStylingScanScopeV1(row.input).roots; accepted = true; } catch {}
    if (accepted !== row.accepted || JSON.stringify(actual) !== JSON.stringify(row.roots)) failures.push(row.id);
  }
  for (const row of corpus.scans) {
    const files = row.files as Record<string, string>;
    const source = { roots: row.roots, files: Object.keys(files), readText: (path: string) => files[path]! };
    const literals: string[] = [], colorLiterals: string[] = [];
    for (const [path, text] of Object.entries(files)) {
      if (!row.roots.some(root => path.startsWith(`${root}/`)) || !applicationSource(path)) continue;
      const walk = (node: ts.Node): void => { if (ts.isStringLiteralLike(node)) { literals.push(node.text); if (colorSource(path)) colorLiterals.push(node.text); } ts.forEachChild(node, walk); };
      walk(ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX));
    }
    assert.equal(literals.some(value => /\[(?:-?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+))px\]/u.test(value)), row.expected.px.length > 0, `${row.id}: TypeScript sizing oracle`);
    const colors = colorLiterals.flatMap(value => value.match(/[^\s]*#[^\s]*|\b(?:rgba?|hsla?)\([^)]*\)/g) ?? []);
    assert.equal(colors.some(value => colorString.get(value) !== null), row.expected.color.length > 0, `${row.id}: TypeScript and color-string oracle`);
    for (const kind of ["px", "color"] as const) {
      try {
        const findings = verification.collectStylingViolationsV1(source, kind);
        assert.equal(ajv.compile({ type: "array", items: { $ref: `${schema.$id}#/$defs/ViolationV1` } })(findings), true);
        assert.deepEqual(findings.map(finding => finding.kind), row.expected[kind]);
      }
      catch { failures.push(`${row.id}:${kind}`); }
    }
  }
  assert.deepEqual(failures, [], "Styling verification differs from the portable corpus");
  return corpus.scopes.length + corpus.sources.length + corpus.scans.length;
}

/** 🟢️Executes the same portable source contract through independent esbuild and Node. */
export async function proveIndependentStylingVerificationContractV1(): Promise<void> {
  const { build } = await import("esbuild");
  const path = resolve(import.meta.dirname, "🟦️.ts");
  const products = resolve(import.meta.dirname, "../../../../../🛍️products") + "/";
  const program = `import { proveStylingVerificationContractV1 } from ${JSON.stringify(path)}; process.stdout.write(JSON.stringify({ vectors: proveStylingVerificationContractV1() }));`;
  const result = await build({ stdin: { contents: program, resolveDir: import.meta.dirname, loader: "ts" }, bundle: true, platform: "node", format: "esm", external: ["typescript"], write: false, plugins: [{ name: "framework-product-removal", setup(builder) {
    builder.onLoad({ filter: /.*/ }, input => input.path.replaceAll("\\", "/").startsWith(products.replaceAll("\\", "/")) ? { errors: [{ text: "General styling imports a concrete framework product: " + input.path }] } : undefined);
  } }] });
  const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(result.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  assert.equal(node.exitCode, 0, Buffer.from(node.stderr).toString());
  assert.deepEqual(JSON.parse(Buffer.from(node.stdout).toString()), { vectors: corpus.scopes.length + corpus.sources.length + corpus.scans.length });
}
