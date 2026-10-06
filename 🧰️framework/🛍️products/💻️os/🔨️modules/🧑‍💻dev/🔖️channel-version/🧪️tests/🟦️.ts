import assert from "node:assert/strict";

import TOML from "@iarna/toml";
import corpus from "../🧫️fixtures/🔣️.json";
import { channelVersionIsDescribeOutputV1 } from "../🟦️.ts";

/** 🔬️Checks output ownership against strict JSON schema and independent Cargo TOML admission. */
export function proveChannelVersionDescribeOwnershipV1(): number {
  const failures: string[] = [];
  for (const row of corpus.cases) {
    const observe = (run: () => boolean): boolean | "refused" => { try { return run(); } catch { return "refused"; } };
    const oracle = observe(() => row.owners.some(owner => {
      if (row.path !== `${owner.ownerRel}/🔣️.json` && row.path !== `${owner.ownerRel}/🛂️.descriptor.semio`) return false;
      const metadata = (TOML.parse(owner.manifest).package as { metadata?: { component?: { package?: string }; semio?: { "component-kind"?: string } } }).metadata;
      const kind = metadata?.semio?.["component-kind"];
      if (kind === undefined) return false;
      if (!["plugin", "extension"].includes(kind) || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(metadata?.component?.package ?? "")) throw Error("invalid component declaration");
      return true;
    }));
    assert.equal(oracle, row.expected, `${row.id}: independent TOML oracle`);
    const actual = observe(() => channelVersionIsDescribeOutputV1(row.path, row.owners));
    if (actual !== row.expected) failures.push(`${row.id}: expected=${row.expected} actual=${actual}`);
  }
  assert.deepEqual(failures, [], "Describe output ownership differs from the portable corpus");
  return corpus.cases.length;
}
