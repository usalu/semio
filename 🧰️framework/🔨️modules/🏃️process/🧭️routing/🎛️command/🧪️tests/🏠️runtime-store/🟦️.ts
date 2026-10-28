/** 🧪️ Tests for routing command runtime-store. */
import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, relative, resolve, sep} from "node:path";

const root = (() => { for (let at = import.meta.dir; ; at = dirname(at)) { if (existsSync(join(at, "bunfig.toml")) && existsSync(join(at, "nx.json"))) return at; if (dirname(at) === at) throw Error("workspace root not found"); } })();
const git = (...args: string[]) => spawnSync("git", ["-c", "core.quotepath=false", ...args], {cwd: root, encoding: "utf8"});
const configuration = join(root, "🧰️framework/⚙️configuration/🏃️process/🔣️.json");
const policy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));

/** 🏠️ The command store holds runtime state (captures, leases, build targets): it lives outside every source owner tree, ignored by version control and never tracked; git is the independent oracle. */
test("the store of the command configuration is an ignored directory of the workspace cache, never a source tree", () => {
  const store = resolve(dirname(configuration), JSON.parse(readFileSync(configuration, "utf8")).store), local = relative(root, store).split(sep).join("/");
  expect(local.startsWith(".cache/"), `${local} lives in the workspace cache`).toBe(true);
  expect(git("check-ignore", "-q", "--", local).status, `${local} is ignored`).toBe(0);
  expect(git("ls-files", "--", local).stdout.trim(), `${local} is not tracked`).toBe("");
  expect(policy.generatedDirectories, "the cache root is a generated directory of every key").toContain(".cache");
});

test("every runtime state directory of the policy is ignored, untracked and no generator writes into it", () => {
  expect(policy.runtimeStateDirectories.length).toBeGreaterThan(0);
  const outputs: string[] = Object.values<any>(taxonomy.generatorContracts).flatMap((contract) => (contract.outputRoots ?? []).map((row: {path: string}) => row.path));
  for (const path of policy.runtimeStateDirectories as string[]) {
    expect(git("check-ignore", "-q", "--", path).status, `${path} is ignored`).toBe(0);
    expect(git("ls-files", "--", path).stdout.trim(), `${path} is not tracked`).toBe("");
    expect(outputs.filter((output) => output === path || output.startsWith(path + "/") || path.startsWith(output + "/")), `${path} is no declared generator output`).toEqual([]);
  }
});
