import { toolJobMemberStoreProducerExact, type ToolJobMemberStoreProducerSources } from "@semio-tech/framework-os/modules/store/initialization/owners/member/source";
import { policyReadFileSafe } from "@semio-tech/repo-lib/inventory";
import { policyReadMemberStoreProducerSources } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { semioMemberStoreProducerSourceSelfTests } from "../../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🧪️tests/🟦️.ts";

/** 🍙️ Runs the semio member-store producer source mutation lane against the repository's actual producer sources; returns the executed case count. */
export function verifySemioMemberStoreProducerSource(repoRoot: string): number {
  return semioMemberStoreProducerSourceSelfTests(
    (candidate) => toolJobMemberStoreProducerExact(candidate as ToolJobMemberStoreProducerSources),
    policyReadMemberStoreProducerSources(repoRoot),
    JSON.parse(policyReadFileSafe(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧬️schema/🔣️.json")),
  );
}
