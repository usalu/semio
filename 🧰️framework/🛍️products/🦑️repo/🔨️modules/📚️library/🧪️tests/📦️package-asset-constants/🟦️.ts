import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

/** 📦️ The Rust leaves that declare one package asset as the constants `<PREFIX>_ASSET_NAME`, `<PREFIX>_ASSET_SIZE_BYTES` and `<PREFIX>_ASSET_SHA256`. */
const CONSTANT_LEAVES: readonly Readonly<{ file: string; prefix: string }>[] = [
  { file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🦀️.rs", prefix: "DANCING_GIF" },
  { file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🦀️.rs", prefix: "BACHELOR_THESIS" },
];

/** 🔏️ The digest sidecars (`name`, `size`, `sha256` per asset, three lines each) that a deferred example or a served schema document embeds in place of its payload. */
const DIGEST_SIDECARS: readonly string[] = [
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🖼️assets/🌱️metabolism/🏙️metabolism-model/🔏️.digest",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🖼️assets/🌙️dream/🔏️.digest",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔏️.digest",
];

type Declared = Readonly<{ name: string; sizeBytes: number; sha256: string }>;

const rustConstant = (source: string, name: string): string => {
  const match = new RegExp(`pub const ${name}: (?:&str|u64) = ("[^"]*"|\\d+);`, "u").exec(source);
  if (!match) throw new Error(`missing constant ${name}`);
  return match[1]!.startsWith('"') ? (JSON.parse(match[1]!) as string) : match[1]!;
};

const sidecarRows = (text: string): Declared[] => {
  const lines = text.split("\n").filter((line) => line.length > 0);
  if (lines.length === 0 || lines.length % 3 !== 0) throw new Error("a digest sidecar holds name, size and digest per asset");
  const rows: Declared[] = [];
  for (let index = 0; index < lines.length; index += 3) rows.push({ name: lines[index]!, sizeBytes: Number(lines[index + 1]), sha256: lines[index + 2]! });
  return rows;
};

describe("package asset declarations match the files they declare", () => {
  const root = getWorkspaceRoot();

  const checkAgainstFile = (declared: Declared): void => {
    expect(declared.name.startsWith("/") || declared.name.split("/").includes("..")).toBe(false);
    const path = join(root, declared.name);
    expect(statSync(path).size).toBe(declared.sizeBytes);
    expect(createHash("sha256").update(readFileSync(path)).digest("hex")).toBe(declared.sha256);
  };

  for (const { file, prefix } of CONSTANT_LEAVES) {
    test(`${prefix}: the Rust constants are the independently computed size and SHA-256 of the named file`, () => {
      const source = readFileSync(join(root, file), "utf8");
      checkAgainstFile({ name: rustConstant(source, `${prefix}_ASSET_NAME`), sizeBytes: Number(rustConstant(source, `${prefix}_ASSET_SIZE_BYTES`)), sha256: rustConstant(source, `${prefix}_ASSET_SHA256`) });
    });
  }

  for (const sidecar of DIGEST_SIDECARS) {
    test(`${sidecar.split("/").slice(-4, -1).join("/")}: every digest row is the independently computed size and SHA-256 of the named file`, () => {
      for (const row of sidecarRows(readFileSync(join(root, sidecar), "utf8"))) checkAgainstFile(row);
    });
  }

  test("a corrupted digest is refused", () => {
    const [row] = sidecarRows(readFileSync(join(root, DIGEST_SIDECARS[0]!), "utf8"));
    expect(() => checkAgainstFile({ ...row!, sha256: "0".repeat(64) })).toThrow();
  });
});
