import { CHANNEL_VERSION_PIN_PATH, channelVersionCensus, channelVersionCensusRoots, writeChannelVersionConsumers, type ChannelVersionSourceViewV1 } from "./🔍️census/🟦️.ts";
import { admitChannelVersionContributionPathV1, admitChannelVersionContributionsV1, type ChannelVersionConsumerV1, type ChannelVersionContributionOwnerV1 } from "./📣️contributions/🟦️.ts";
export { admitChannelVersionContributionsV1 } from "./📣️contributions/🟦️.ts";
export type { ChannelVersionConsumerV1, ChannelVersionContributionOwnerV1 } from "./📣️contributions/🟦️.ts";
/** 🔖️Owns the app-engine channel pin and admits contributions from present source owners. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { BundleScript, declaredComponentKind, discoverCatalogPackages, getWorkspaceRoot, loadCatalogTaxonomy, registryCatalogInputView, type RegistryCatalogInputView, type DiscoveredPackage } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { parseComponentPackageId } from "../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME } from "../../🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts";

const OS = "🧰️framework/🛍️products/💻️os";
const MOD = `${OS}/🔨️modules`;

export type ChannelVersionDescribeOwnerV1 = Readonly<{ ownerRel: string; manifest: string }>;

/** 🏭️Classifies an exact descriptor output against its supplied source-owner declarations. */
export function channelVersionIsDescribeOutputV1(path: string, owners: readonly ChannelVersionDescribeOwnerV1[]): boolean {
  const matching = owners.filter(owner => path === `${owner.ownerRel}/${DESCRIPTOR_JSON_FILENAME}` || path === `${owner.ownerRel}/${DESCRIPTOR_PACK_FILENAME}`);
  if (matching.length > 1) throw Error("Describe output has ambiguous source ownership");
  const owner = matching[0];
  if (!owner || declaredComponentKind(owner.manifest) === undefined) return false;
  parseComponentPackageId(owner.manifest, `${owner.ownerRel}/Cargo.toml`);
  return true;
}

/** 📄️ The source files a version literal is written in: Rust, TypeScript and JSON. */
export const CHANNEL_VERSION_CENSUS_EXTENSIONS: readonly string[] = ["rs", "ts", "tsx", "mts", "json", "jsonc"];

/** 📇️Discovers only contribution declarations owned by physically present package manifests. */
function contributedChannelConsumersV1(packages: readonly DiscoveredPackage[], view: RegistryCatalogInputView): readonly ChannelVersionConsumerV1[] {
  const owners: ChannelVersionContributionOwnerV1[] = [];
  for (const pkg of packages) {
    if (pkg.lang !== "🟦️typescript" && pkg.lang !== "🦀️rust") continue;
    const manifest = view.readText(pkg.manifestPath);
    const document = (pkg.lang === "🦀️rust" ? Bun.TOML.parse(manifest) : JSON.parse(manifest)) as { package?: { metadata?: { semio?: Record<string, unknown> } }; semio?: Record<string, unknown> };
    const declaration = pkg.lang === "🦀️rust" ? document.package?.metadata?.semio?.["channel-version-consumers"] : document.semio?.channelVersionConsumers;
    if (declaration === undefined) continue;
    const path = `${pkg.ownerRel}/${admitChannelVersionContributionPathV1(declaration)}`;
    const parts = path.split("/");
    for (let index = 1; index < parts.length; index++) if (view.kind(parts.slice(0, index).join("/")) !== "directory") throw Error(`Channel contribution has a missing or symbolic-link ancestor: ${path}`);
    if (view.kind(path) !== "file") throw Error(`Declared channel contribution is not a regular file: ${path}`);
    owners.push({ ownerRoot: pkg.ownerRel, document: JSON.parse(view.readText(path)) });
  }
  return admitChannelVersionContributionsV1(owners);
}

/** 🗂️Reads current source candidates while excluding exact owner-declared producer outputs. */
function channelVersionCandidateFiles(repoRoot: string, consumers: readonly ChannelVersionConsumerV1[], packages: readonly DiscoveredPackage[], view: RegistryCatalogInputView): readonly string[] {
  const roots = channelVersionCensusRoots(packages, consumers);
  const pathspecs = roots.flatMap(root => CHANNEL_VERSION_CENSUS_EXTENSIONS.map(extension => `:(glob)${root}/**/*.${extension}`));
  const listed = spawnSync("git", ["-c", "core.quotePath=false", "grep", "-l", "-z", "--untracked", "-E", "appChannelVersion|CHANNEL_VERSION|app_channel_version|6170704368616e6e656c56657273696f6e", "--", ...pathspecs], { cwd: repoRoot, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (listed.status !== 0 && listed.status !== 1) throw Error(`channel-version census: git grep failed (${listed.status}): ${listed.stderr}`);
  const owners = packages.filter(pkg => pkg.lang === "🦀️rust").map(pkg => ({ ownerRel: pkg.ownerRel, manifest: view.readText(pkg.manifestPath) }));
  return listed.stdout.split("\0").filter(path => path.length > 0 && !channelVersionIsDescribeOutputV1(path, owners) && path !== CHANNEL_VERSION_PIN_PATH && !path.startsWith(`${MOD}/🧑‍💻dev/🔖️channel-version/`) && !path.startsWith(`${MOD}/🧑‍💻dev/🧪️tests/🔖️channel-version/`));
}

/** 📖️Constructs one source snapshot for a channel check or write operation. */
export function loadChannelVersionSourceV1(repoRoot: string): ChannelVersionSourceViewV1 {
  const view = registryCatalogInputView(repoRoot, loadCatalogTaxonomy());
  const packages = discoverCatalogPackages(repoRoot, loadCatalogTaxonomy(), view);
  const consumers = contributedChannelConsumersV1(packages, view);
  return { pin: pinnedChannelVersion(repoRoot), consumers, candidates: channelVersionCandidateFiles(repoRoot, consumers, packages, view), readText: path => view.readText(path) };
}

/** 📌️ The version the pin owns. */
export function pinnedChannelVersion(repoRoot: string): number {
  const pin = JSON.parse(readFileSync(join(repoRoot, CHANNEL_VERSION_PIN_PATH), "utf8")) as { channelVersion?: unknown };
  if (!Number.isInteger(pin.channelVersion) || (pin.channelVersion as number) < 1) throw new Error(`${CHANNEL_VERSION_PIN_PATH} carries no positive integer channelVersion`);
  return pin.channelVersion as number;
}

/** 🔖️ `channel-version <generate [--guest]|check>` — the generator and the census of the one channel version authority. */
export class ChannelVersionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [verb, ...rest] = segments;
    if (verb === "test-contributions") {
      if (rest.length) throw Error("test-contributions accepts no arguments");
      const { proveChannelVersionContributionsV1, proveChannelVersionContributionCensusV1, proveIndependentChannelVersionContributionsV1 } = await import("./📣️contributions/🧪️tests/🟦️.ts");
      await proveIndependentChannelVersionContributionsV1();
      console.log(`[channel-version] outward contributions: ${proveChannelVersionContributionsV1() + proveChannelVersionContributionCensusV1()} vectors passed`);
      return;
    }
    if (verb === "test-describe-ownership") {
      if (rest.length) throw Error("test-describe-ownership accepts no arguments");
      const { proveChannelVersionDescribeOwnershipV1 } = await import("./🧪️tests/🟦️.ts");
      console.log(`[channel-version] describe ownership: ${proveChannelVersionDescribeOwnershipV1()} vectors passed`);
      return;
    }
    if (verb !== "generate" && verb !== "check") throw Error("usage: channel-version <generate [--guest]|check>");
    const repoRoot = getWorkspaceRoot();
    const source = loadChannelVersionSourceV1(repoRoot);
    if (verb === "generate") {
      const { written, refused } = writeChannelVersionConsumers(source, { guest: rest.includes("--guest"), writeText: (path, text) => writeFileSync(join(repoRoot, path), text) });
      for (const path of written) console.log(`[channel-version] wrote ${path}`);
      for (const line of refused) console.error(`[channel-version] refused ${line}`);
    } else if (verb !== "check") {
      throw new Error("usage: channel-version <generate [--guest]|check>");
    }
    const { pin, findings } = channelVersionCensus(source);
    for (const finding of findings) console.error(`[channel-version] ${finding.problem} ${finding.path}: ${finding.detail}`);
    console.log(`[channel-version] pin ${pin}: ${source.consumers.length} registered consumers, ${findings.length} finding(s)`);
    if (findings.length > 0) process.exitCode = 1;
  }
}
