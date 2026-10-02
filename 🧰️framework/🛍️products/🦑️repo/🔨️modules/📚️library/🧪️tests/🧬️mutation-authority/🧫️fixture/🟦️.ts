import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync } from "node:fs";
import { dirname, join } from "node:path";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🌱️ Locates the authored direct-mutation probe domain. */
export const mutationRoot = (root: string, area = "✏️s") => join(root, area, "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema", "🧬️mutations");

/** 🧫️ Prepares isolated fixture Git and the exact taxonomy inputs. */
export const prepareMutationFixtureRoot = (root: string): string => {
  expect(spawnSync("git", ["init", "--quiet", "--template="], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
  for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"]) {
    mkdirSync(join(root, dirname(path)), { recursive: true });
    copyFileSync(join(getWorkspaceRoot(), path), join(root, path));
  }
  return root;
};

/** 🎫️ Creates each fixture inside the caller's ticket output. */
export const mutationFixtureRoot = (prefix: string): string => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for mutation fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  return prepareMutationFixtureRoot(mkdtempSync(join(artifactRoot, prefix)));
};
