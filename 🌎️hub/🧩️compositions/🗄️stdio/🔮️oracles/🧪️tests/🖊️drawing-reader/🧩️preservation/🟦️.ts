import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
import { rustTokens, rustTokenPairs } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import contract from "../../../🧫️fixtures/🖊️drawing-reader/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../../..");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");

/** 🧬️ Projects only the declared reader extraction from the captured artifact source. */
export function extractedDxfMutationSource(original: string): string {
  const ranges = contract.functions.map(({ start, end }) => {
    start = original.lastIndexOf("\n", start) + 1;
    while (start > 0) {
      const previous = original.lastIndexOf("\n", start - 2) + 1;
      if (!original.slice(previous, start).trim().startsWith("///")) break;
      start = previous;
    }
    return { start, end: end + 1 };
  });
  let source = original;
  for (const { start, end } of ranges.sort((a, b) => b.start - a.start)) source = source.slice(0, start) + source.slice(end);
  return source.replace(/\/\/\/ 📄️ Independent semantic projection[\s\S]*?#\[cfg\(feature = "oracles"\)\]\npub fn project_dxf_r12\(bytes: &\[u8\]\) -> Result<Json, String> \{\n    imp::project_dxf_r12\(bytes\)\n\}\n\n/u, "")
    .replace(/#\[cfg\(not\(feature = "oracles"\)\)\]\npub fn project_dxf_r12\(_bytes: &\[u8\]\) -> Result<Json, String> \{[^}]*\}\n/u, "");
}

/** 🛂️ Reconstructs original guards only after exact moved-body and retained-byte equality. */
export function originalDrawingSource(path: string, current: string): string {
  const original = contract.originals.find((row) => row.path === path);
  if (!original || !path.endsWith(".rs")) return current;
  if (path === contract.functions[0]!.source) {
    for (const row of contract.functions) {
      const source = readFileSync(resolve(root, row.destination), "utf8"), tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
      let body = tokens.findIndex(({ text }, index) => text === "fn" && tokens[index + 1]?.text === row.name);
      if (body < 0) throw new Error(`missing moved reader body ${row.name}`);
      while (tokens[body]?.text !== "{") body++;
      if (digest(source.slice(tokens[body]!.start, tokens[pairs.get(body)!]!.end)) !== row.sha256) throw new Error(`changed moved reader body ${row.name}`);
    }
    const support = relative(resolve(root, path, ".."), resolve(root, contract.owner, "🧰️support/🦀️.rs")).replaceAll("\\", "/");
    const mount = `\n\n#[cfg(feature = "oracles")]\n#[path = ${JSON.stringify(support)}]\nmod reference_support;`;
    const retained = current.replace(mount, "").replace("\n    use super::reference_support::{load, point_json, obj};", "");
    if (retained !== extractedDxfMutationSource(original.source)) throw new Error(`changed retained DXF mutation bytes ${path}`);
  } else {
    let expected = original.source;
    if (path.includes("/🗒️note/")) expected = expected.replaceAll("crate::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12", `${contract.library}::project_dxf_r12`);
    else if (path.includes("/🧪️tests/")) {
      expected = expected.replaceAll(", project_dxf_r12}", "}").replaceAll("use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;", `use ${contract.library}::project_dxf_r12;`);
      if (!expected.includes(`use ${contract.library}::project_dxf_r12;`)) expected = `use ${contract.library}::project_dxf_r12;\n${expected}`;
    }
    if (current !== expected) throw new Error(`changed original reader caller bytes ${path}`);
  }
  return original.source;
}
