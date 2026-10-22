import { adoptStagedGraphLeavesV1 } from "../../../../../../../📥️inputs/🟨️.mjs";
import { withBuildGraphStagingV1 } from "../../../../../../../📥️inputs/💾️storage/🟨️.mjs";

/** 🧪️ One competing adopter process: waits for the common start, adopts the named staged leaves into the target tree and prints what it did. */
const { stagingRoot, targetRoot, names, startAt } = JSON.parse(process.argv[2]);
while (Date.now() < startAt);
const plan = withBuildGraphStagingV1({ root: targetRoot }, () => adoptStagedGraphLeavesV1("/unused", stagingRoot, names));
console.log(JSON.stringify(plan));
