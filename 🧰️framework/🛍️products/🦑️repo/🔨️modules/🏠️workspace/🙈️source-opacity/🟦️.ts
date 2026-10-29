import { join } from "node:path";
import cachePolicy from "../../📚️library/⚡️caching/🔣️policy.json" with { type: "json" };
import descriptor from "../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json" with { type: "json" };
import retirement from "../../📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json" with { type: "json" };
import { BundleScript } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";

/** 🧾️ Projects the canonical preservation and retirement protocols into native source policy. */
export function sourceOpacityGoProjectionV1(): string {
  const protocol = descriptor.definitions.RelativePreservationProtocol.const;
  const policy = retirement.definitions.Policy.const;
  return [
    "// 🤖️ Generated from descriptor RelativePreservationProtocol and dependency retirement Policy.",
    "// Refresh with repo run @semio-tech/repo-workspace-go:generate-source-policy.",
    "",
    "package workspace",
    "",
    "const opaquePreservationPrefix = " + JSON.stringify(protocol.capsulePrefix),
    "const opaqueSourceDigestHex = " + String(protocol.capsuleDigestHex),
    "const opaqueRetirementPrefix = " + JSON.stringify(policy.markerPrefix),
    "",
    "var opaqueGeneratedDirectories = [...]string{" + cachePolicy.generatedDirectories.map(name => JSON.stringify(name)).join(", ") + "}",
    "",
  ].join("\n");
}

export class GenerateSourceOpacityScript extends BundleScript {
  run(): void {
    writeGeneratedFileIfChanged(join(this.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🗂️runtime-registry/🐹️.go"), sourceOpacityGoProjectionV1());
  }
}
