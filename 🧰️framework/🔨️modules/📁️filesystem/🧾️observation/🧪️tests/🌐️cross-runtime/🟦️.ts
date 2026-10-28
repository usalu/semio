/** 🌐️ The rooted POSIX descriptor behaves identically under Bun's native binding and Node's built-in core: same names, kinds, identities and byte digests on one shared read-only tree, same results for creation, sync and removal on fresh trees. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url)), script = join(here, "📜️script.ts");
const repositoryRoot = process.env.SEMIO_REPOSITORY_ROOT ?? resolve(here, "../../../../../../..");
const pinned = (): string | undefined => { try { return readdirSync(join(repositoryRoot, ".cache/b/t/m/bun")).map(entry => join(repositoryRoot, ".cache/b/t/m/bun", entry, "bun")).find(path => existsSync(path)); } catch { return undefined; } };
const bun = "bun" in process.versions ? process.execPath : pinned();
const posix = process.platform === "darwin" || process.platform === "linux";

test.skipIf(!posix || bun === undefined)("the descriptor capability agrees across Bun and Node on a shared tree and on fresh trees", () => {
  const base = mkdtempSync(join(tmpdir(), "nffi-descriptor-"));
  try {
    const shared = join(base, "shared");
    mkdirSync(join(shared, "nested"), { recursive: true });
    writeFileSync(join(shared, "alpha.txt"), "alpha\n");
    writeFileSync(join(shared, "bé.bin"), Buffer.from([0, 1, 2, 255]));
    writeFileSync(join(shared, "nested", "inner.txt"), "inner");
    symlinkSync("alpha.txt", join(shared, "link"));
    const results: Record<string, any> = {};
    for (const [name, executable] of [["bun", bun!], ["node", "node"]] as const) {
      const writable = join(base, `writable-${name}`);
      mkdirSync(writable);
      const run = spawnSync(executable, [script, shared, writable], { encoding: "utf8", cwd: base, env: { ...process.env, SEMIO_LANGUAGE: "en" } });
      expect(run.status, `${name}: ${run.stderr}`).toBe(0);
      results[name] = JSON.parse(run.stdout);
    }
    expect(results.node.observed.names).toEqual(["alpha.txt", "bé.bin", "link", "nested"]);
    expect(results.node.observed.entries.missing).toBe(null);
    expect(results.node.observed.entries.link.kind).toBe("link");
    expect(results.node.observed.entries.nested.kind).toBe("directory");
    expect(results.node.observed.digests["alpha.txt"]).toBe(createHash("sha256").update("alpha\n").digest("hex"));
    expect(results.node.observed).toEqual(results.bun.observed);
    expect(results.node.written).toEqual(results.bun.written);
    expect(results.node.written.inner).toEqual({ names: ["data.bin"], kind: "file", size: `${Buffer.byteLength("héllo native\n")}n`, mode: results.node.written.inner.mode, nlink: "1n" });
    expect(results.node.written.removed).toEqual([]);
    expect(readdirSync(join(base, "writable-node", "made"))).toEqual([]);
  } finally { rmSync(base, { recursive: true, force: true }); }
}, 120000);
