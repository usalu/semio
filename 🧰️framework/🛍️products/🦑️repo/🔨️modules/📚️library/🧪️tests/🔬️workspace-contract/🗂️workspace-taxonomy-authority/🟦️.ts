import { describe, expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { resolveWorkspaceTaxonomyAuthority, resolveWorkspaceTaxonomyAuthorityFromDirectory } from "../../../🔍️discovery/🧭️workspace-taxonomy-locator/🟦️.ts";
import { ownedFilesystemEntries, type OwnedFilesystemEntry } from "../../🔍️filesystem/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

const OWNED_FILESYSTEM_DISCOVERY_GOLDEN = join(contractRoot, "../🧫️fixtures/🔍️filesystem/🔣️.json");

describe("owned filesystem discovery", () => {
  test("sorts hostile Unicode and hidden nodes by bytes without following directory symlinks", () => {
    const golden = JSON.parse(readFileSync(OWNED_FILESYSTEM_DISCOVERY_GOLDEN, "utf8")) as {
      schemaVersion: number;
      directories: string[];
      files: { path: string; content: string }[];
      symlinks: { path: string; target: string }[];
      expected: OwnedFilesystemEntry[];
    };
    const root = mkdtempSync(join(tmpdir(), "owned-filesystem-discovery-"));
    try {
      expect(golden.schemaVersion).toBe(1);
      for (const path of golden.directories) mkdirSync(join(root, path), { recursive: true });
      for (const file of golden.files) {
        const target = join(root, file.path);
        mkdirSync(resolve(target, ".."), { recursive: true });
        writeFileSync(target, file.content);
      }
      for (const link of golden.symlinks) symlinkSync(process.platform === "win32" ? join(root, link.target) : link.target, join(root, link.path), process.platform === "win32" ? "junction" : "dir");
      const entries = ownedFilesystemEntries(root);
      expect(entries).toEqual(golden.expected);
      expect(entries.some(({ path }) => path.startsWith("alias/"))).toBe(false);
      expect(ownedFilesystemEntries(join(root, "absent"))).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

const WORKSPACE_TAXONOMY_FIXTURE = join(contractRoot, "../🧫️fixtures/🌳️workspace-taxonomy/🔣️.json");

function workspaceTaxonomyFixtureRoot(): { readonly root: string; readonly retained: boolean } {
  const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR?.trim();
  const base = artifactDirectory ? resolve(getWorkspaceRoot(), artifactDirectory) : realpathSync(tmpdir());
  mkdirSync(base, { recursive: true });
  return { root: mkdtempSync(join(base, "tx-")), retained: Boolean(artifactDirectory) };
}

describe("workspace taxonomy authority", () => {
  test("resolves only a lexical, no-follow metadata.semio.taxonomy locator", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      childLocator: string;
      cases: readonly { name: string; manifest: object; target: "regular" | "file-symlink" | "directory-symlink" | "manifest-symlink"; expected: "resolve" | "reject"; reason: string }[];
      startCases: readonly { name: string; entry: "root" | "linked-child" | "child" | "compose"; expected: "resolve" | "reject"; reason: string }[];
    };
    
    for (const vector of fixture.cases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot();
      const root = fixtureRoot.root;
      try {
        writeFileSync(join(root, "nx.json"), "{}\n");
        const manifestPath = join(root, "📋️project.json");
        const target = vector.name === "child-directory" || vector.name === "component-symlink" ? fixture.childLocator : fixture.canonicalLocator;
        const targetPath = join(root, target);
        if (vector.expected === "resolve" || vector.target !== "regular") {
          mkdirSync(dirname(targetPath), { recursive: true });
          writeFileSync(targetPath, "{}\n");
        }
        if (vector.target === "file-symlink") {
          const linked = `${targetPath}.linked`;
          writeFileSync(linked, "{}\n");
          rmSync(targetPath);
          symlinkSync(linked, targetPath, "file");
        }
        if (vector.target === "directory-symlink") {
          const component = join(root, "🧪️virtual");
          const linked = join(root, "🧪️linked");
          mkdirSync(join(linked, "📚️child"), { recursive: true });
          writeFileSync(join(linked, "📚️child", "🔣️taxonomy.json"), "{}\n");
          rmSync(component, { recursive: true, force: true });
          symlinkSync(linked, component, process.platform === "win32" ? "junction" : "dir");
        }
        if (vector.target === "manifest-symlink") {
          const linked = join(root, "📋️project-linked.json");
          writeFileSync(linked, JSON.stringify(vector.manifest));
          symlinkSync(linked, manifestPath, "file");
        } else writeFileSync(manifestPath, JSON.stringify(vector.manifest));
        if (vector.expected === "resolve") {
          const authority = resolveWorkspaceTaxonomyAuthority(root);
          expect(authority.relativePath).toBe(target);
          expect(relative(root, authority.taxonomyPath).replaceAll("\\", "/")).toBe(target);
          expect(lstatSync(authority.taxonomyPath).isSymbolicLink()).toBe(false);
        } else expect(() => resolveWorkspaceTaxonomyAuthority(root)).toThrow(vector.reason);
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("discovers the portable workspace and refuses symlink or compose ancestry", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      startCases: readonly { name: string; entry: "root" | "linked-child" | "child" | "compose" | "compose-parent-root" | "compose-parent-start" | "root-nul" | "root-drive-relative" | "symlink-parent"; expected: "resolve" | "reject"; reason: string }[];
    };
    for (const vector of fixture.startCases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot();
      const root = fixtureRoot.root;
      try {
        const workspace = vector.entry === "compose" ? join(root, "compose", "workspace") : join(root, "workspace");
        const target = join(workspace, fixture.canonicalLocator);
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, "{}\n");
        writeFileSync(join(workspace, "📋️project.json"), JSON.stringify({ metadata: { semio: { taxonomy: fixture.canonicalLocator } } }));
        const start = vector.entry === "linked-child" ? join(root, "linked-workspace", "child") : vector.entry === "child" ? join(workspace, "child", "nested") : vector.entry === "symlink-parent" ? `${root}${sep}linked-workspace${sep}..${sep}workspace` : vector.entry === "compose-parent-start" ? `${root}${sep}COMPOSE${sep}..${sep}workspace` : workspace;
        if (vector.entry === "linked-child") {
          mkdirSync(join(workspace, "child"), { recursive: true });
          symlinkSync(workspace, join(root, "linked-workspace"), process.platform === "win32" ? "junction" : "dir");
        }
        if (vector.entry === "symlink-parent") symlinkSync(workspace, join(root, "linked-workspace"), process.platform === "win32" ? "junction" : "dir");
        if (vector.entry === "child") mkdirSync(start, { recursive: true });
        if (vector.expected === "resolve") {
          const authority = resolveWorkspaceTaxonomyAuthorityFromDirectory(start);
          expect(authority.workspaceRoot).toBe(workspace);
        } else {
          const resolveAuthority = vector.name === "marker-pair" || vector.entry === "compose" ? () => resolveWorkspaceTaxonomyAuthority(workspace) : vector.entry === "compose-parent-root" ? () => resolveWorkspaceTaxonomyAuthority(`${root}${sep}compose${sep}..${sep}workspace`) : vector.entry === "root-nul" ? () => resolveWorkspaceTaxonomyAuthority(`${workspace}\0`) : vector.entry === "root-drive-relative" ? () => resolveWorkspaceTaxonomyAuthority("C:workspace") : () => resolveWorkspaceTaxonomyAuthorityFromDirectory(start);
          expect(resolveAuthority).toThrow(vector.reason);
        }
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("does not borrow an outer taxonomy past a malformed nested workspace anchor", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      anchorCases: readonly { name: string; project: "regular" | "directory" | "symlink"; reason: string }[];
    };
    for (const vector of fixture.anchorCases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot();
      const root = fixtureRoot.root;
      try {
        const outer = join(root, "outer-workspace");
        const outerTaxonomy = join(outer, fixture.canonicalLocator);
        mkdirSync(dirname(outerTaxonomy), { recursive: true });
        writeFileSync(outerTaxonomy, "{}\n");
        writeFileSync(join(outer, "nx.json"), "{}\n");
        writeFileSync(join(outer, "📋️project.json"), JSON.stringify({ metadata: { semio: { taxonomy: fixture.canonicalLocator } } }));
        const inner = join(outer, "nested-workspace");
        mkdirSync(join(inner, "child"), { recursive: true });
        const projectPath = join(inner, "📋️project.json");
        if (vector.project === "regular") writeFileSync(projectPath, "{}\n");
        if (vector.project === "directory") mkdirSync(projectPath);
        if (vector.project === "symlink") symlinkSync(join(outer, "📋️project.json"), projectPath, "file");
        expect(() => resolveWorkspaceTaxonomyAuthorityFromDirectory(join(inner, "child"))).toThrow(vector.reason);
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });
});
