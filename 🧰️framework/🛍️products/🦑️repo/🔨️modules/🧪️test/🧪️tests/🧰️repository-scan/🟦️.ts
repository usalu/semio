/** 🧰️ Repository scan for the test platform laws
 *
 * The repository-wide contribution scan, oracle registry, discovered cases, oracle-purity hits and contract breaches the test platform laws read, each taken ONCE per process, with the taxonomy vocabulary helpers they share.
 */

//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { createIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuildTestInvocationV1 } from "@semio-tech/process/testing/invocation";
import { withRepositoryTestArtifactsV1 } from "../../💾️storage/🟦️.ts";
import { discoverTestContributions, profileTable, oracleImportsInProduction, discoverTestCases, loadOracleRegistry, repoRootFromHere, testTaxonomy, repositoryContractBreaches } from "../../📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

/** 🧭️ Repo-relative, forward-slashed path — the shape every discovered record carries. */
export const relativeToRepo = (root: string, target: string): string => relative(root, target).split(sep).join("/");

export const repoRoot = repoRootFromHere();

/** 💾️ Runs one body under the original finite test artifact custody: the platform refuses every ambient output root, so a test that writes or reads the test cache does it inside this callback. A named `scope` (`group/label`) gives the body its own task shard, and `exclusive` access is the custody a retirement needs. */
export const withTestArtifacts = async <T>(body: () => T | Promise<T>, access: "shared" | "exclusive" = "shared", scope?: string): Promise<T> => {
  const previous = process.env.SEMIO_TEST_OUTPUT_SCOPE;
  if (scope !== undefined) process.env.SEMIO_TEST_OUTPUT_SCOPE = scope;
  try {
    return await withBuildTestInvocationV1((original) => withRepositoryTestArtifactsV1(repoRoot, original, createIssuedResourceControlV1(original, "test:artifact-entry", 0), async () => body(), access));
  } finally {
    if (previous === undefined) delete process.env.SEMIO_TEST_OUTPUT_SCOPE;
    else process.env.SEMIO_TEST_OUTPUT_SCOPE = previous;
  }
};

/**
 * 🧩️ The repository-wide contribution scan and the registry built on it, taken ONCE for this file.
 *
 * `discoverTestContributions` walks every directory in the repository — six figures of them — and two
 * cases below point discovery at a synthetic root and then clear its memo, so a per-process cache alone
 * leaves every later reader paying the walk again. Reading both here, at module scope, means the walk
 * happens once and never inside a test's own time budget: a `30_000` ms budget then measures the
 * assertion it was written for instead of measuring a filesystem walk under whatever load the machine
 * is carrying. Nothing in this file writes into the real repository, so one scan stays the truth for
 * the whole run.
 */
export const repoContributions = discoverTestContributions(repoRoot);
export const repoRegistry = loadOracleRegistry(repoRoot);

/**
 * 🔮️ The repository's discovered cases, its oracle-purity scan and its repository-wide contract breaches, taken ONCE
 * for this file for the same reason as the contribution scan above: each is a full walk or read of the repository, and
 * several laws below each paid for them again inside their own time budget. A law now measures its assertion over
 * these outcomes; a budget no longer measures how loaded the machine is.
 */
export const repoCases = discoverTestCases(repoRoot);
export const repoOracleHits = oracleImportsInProduction(repoRoot, repoRegistry, repoCases);
export const repoWideBreaches = await withTestArtifacts(() => repositoryContractBreaches(repoRoot, repoRegistry, repoCases, repoOracleHits));

/** 🖥️ This module's own committed case — the one discovery must always find, named as it sits on disk. */
export const HOST_PROTOCOL_PARITY_CASE_DIR = `${testTaxonomy(repoRoot).testDomainPath}/${testTaxonomy(repoRoot).testsDirName}/🖥️host-protocol-parity`;

/** ⚖️ The effective profile table: framework profiles plus every one an owner contributes. */
export const contributed = (): ReadonlyMap<string, import("../../📦️packages/🟦️typescript/🟦️.ts").ComparisonProfileSpec> => profileTable(repoRegistry);

/** 🔣️ The taxonomy's own area vocabulary — these tests name no area of their own. */
const taxonomyAreas = (): Record<string, string> => JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;

/** 🚫️ Areas the taxonomy marks exempt, and the layers it marks as implementations. */
export const exemptAreas = (): string[] => Object.entries(taxonomyAreas()).filter(([, state]) => state === "exempt").map(([area]) => area);
export const implementationAreas = (): string[] => Object.entries(JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areaLayers as Record<string, string>).filter(([, layer]) => layer === "implementation").map(([area]) => area);
