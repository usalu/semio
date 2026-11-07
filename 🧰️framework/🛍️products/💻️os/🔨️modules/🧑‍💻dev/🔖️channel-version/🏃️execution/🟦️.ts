/** 🏃️ `channel-version <generate [--guest]|check|test-contributions|test-describe-ownership>` — the script that drives the one channel version authority. */

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { channelVersionCensus, writeChannelVersionConsumers } from "../🔍️census/🟦️.ts";
import { loadChannelVersionSourceV1, pinnedChannelVersion, staleChannelDescribeOutputsV1 } from "../🟦️.ts";
import { runToolSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** 🔖️ `channel-version <generate [--guest]|check>` — the generator and the census of the one channel version authority. */
export class ChannelVersionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [verb, ...rest] = segments;
    if (verb === "test-contributions") {
      if (rest.length) throw Error("test-contributions accepts no arguments");
      const { proveChannelVersionContributionsV1, proveChannelVersionContributionCensusV1, proveIndependentChannelVersionContributionsV1 } = await import("../📣️contributions/🧪️tests/🟦️.ts");
      await proveIndependentChannelVersionContributionsV1();
      console.log(`[channel-version] outward contributions: ${proveChannelVersionContributionsV1() + proveChannelVersionContributionCensusV1()} vectors passed`);
      return;
    }
    if (verb === "test-describe-channel") {
      if (rest.length) throw Error("test-describe-channel accepts no arguments");
      const { proveStaleChannelDescribeOutputsV1 } = await import("../🧪️tests/🟦️.ts");
      console.log(`[channel-version] describe channel: ${proveStaleChannelDescribeOutputsV1()} vectors passed`);
      return;
    }
    if (verb === "check-descriptors") {
      if (rest.length) throw Error("check-descriptors accepts no arguments");
      const root = findWorkspaceRoot(), pin = pinnedChannelVersion(root);
      const listed = runToolSyncV1("git", ["ls-files", "-z", "--", "*/🧩️component/🔣️.json"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }).stdout.split("\0").filter(path => path.length > 0);
      const stale = staleChannelDescribeOutputsV1(listed.map(path => ({ path, text: readFileSync(join(root, path), "utf8") })), pin);
      for (const path of stale) console.error(`[channel-version] stale-descriptor-channel ${path}: regenerate with repo run workspace:components describe <plugin> --profile prototype`);
      console.log(`[channel-version] pin ${pin}: ${listed.length} committed component descriptors, ${stale.length} stale`);
      if (stale.length > 0) process.exitCode = 1;
      return;
    }
    if (verb === "test-describe-ownership") {
      if (rest.length) throw Error("test-describe-ownership accepts no arguments");
      const { proveChannelVersionDescribeOwnershipV1 } = await import("../🧪️tests/🟦️.ts");
      console.log(`[channel-version] describe ownership: ${proveChannelVersionDescribeOwnershipV1()} vectors passed`);
      return;
    }
    if (verb !== "generate" && verb !== "check") throw Error("usage: channel-version <generate [--guest]|check>");
    const repoRoot = findWorkspaceRoot();
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
