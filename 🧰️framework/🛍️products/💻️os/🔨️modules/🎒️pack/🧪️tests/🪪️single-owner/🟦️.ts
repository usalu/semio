/** 🪪️ Language-neutral single-owner law: the os pack defines no item of the framework record codec, and the record wire tags have exactly one owner. */
import assert from "node:assert/strict";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

type Contract = {format:number;ownerPackage:string;ownerRoots:string[];osPackFiles:string[];adapters:string[];tagOwner:string;tagCount:number;retiredCrates:string[]};

//#region 🔣️Scan
const DEFINITION = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:fn|struct|enum|trait|const|static|type|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)/gm;
const TAG = /^\s*pub const (TAG_[A-Z0-9_]+):/gm;
const here = (relativePath: string): string => resolve(fileURLToPath(new URL(".", import.meta.url)), relativePath);
const rustFiles = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap(row => row.name === "node_modules" || row.name === "target" ? [] : row.isDirectory() ? rustFiles(join(dir, row.name)) : row.name.endsWith(".rs") ? [join(dir, row.name)] : []);
const matches = (path: string, rx: RegExp): string[] => [...readFileSync(path, "utf8").matchAll(rx)].map(match => match[1]!);
const production = (path: string): boolean => !path.split("/").some(segment => segment === "🧪️tests" || segment === "🧪️testing");
//#endregion 🔣️Scan

//#region 🧪️Law
export function testPackSingleOwner(): void {
  const contract = JSON.parse(readFileSync(here("../🧫️fixtures/🪪️single-owner/🔣️.json"), "utf8")) as Contract;
  assert.equal(contract.format, 1);
  const os = here("../..");
  const frameworkPack = here("../../../../../../🔨️modules/🎒️pack");
  const ownerFiles = contract.ownerRoots.flatMap(dir => rustFiles(join(frameworkPack, dir))).filter(production);
  const owned = new Set(ownerFiles.flatMap(file => matches(file, DEFINITION)));
  const manifest = Bun.TOML.parse(readFileSync(join(frameworkPack, "🌱️value/📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { package: { name: string } };
  assert.equal(manifest.package.name, contract.ownerPackage);
  assert.ok(owned.size > 300, `owner definitions ${owned.size}`);
  for (const name of contract.adapters) assert.ok(owned.has(name), `adapter ${name} shadows a framework definition by design`);
  const copies = contract.osPackFiles.flatMap(file => matches(join(os, file), DEFINITION).filter(name => owned.has(name) && !contract.adapters.includes(name)));
  assert.deepEqual(copies, [], "the os pack redefines framework record codec items");
  for (const file of contract.osPackFiles) assert.ok(!readFileSync(join(os, file), "utf8").includes("mod value"), `${file} mounts a record codec copy`);
  for (const crate of contract.retiredCrates) assert.equal(existsSync(join(frameworkPack, crate)), false, `${crate} is a retired copy`);
  const tagOwner = join(frameworkPack, contract.tagOwner);
  const holders = [...rustFiles(frameworkPack), ...rustFiles(os)].filter(file => matches(file, TAG).length > 0).map(file => relative(frameworkPack, file));
  assert.deepEqual(holders, [relative(frameworkPack, tagOwner)], "wire tags are defined outside their owner");
  const tags = matches(tagOwner, TAG);
  assert.equal(tags.length, contract.tagCount);
  assert.equal(new Set(tags).size, tags.length);
  const independent = Bun.spawnSync(["git", "-c", "core.quotepath=false", "grep", "--no-index", "-h", "-o", "-E", "pub const TAG_[A-Z0-9_]+:", "--", frameworkPack, os], { stdout: "pipe", stderr: "ignore" }).stdout.toString().split("\n").filter(Boolean);
  assert.equal(independent.length, contract.tagCount, "git grep disagrees with the scan");
}
//#endregion 🧪️Law
