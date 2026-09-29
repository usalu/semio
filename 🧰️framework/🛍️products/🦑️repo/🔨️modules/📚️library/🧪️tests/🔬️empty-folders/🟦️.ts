import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { cleanCollectEmptyFolderRemovals, cleanDirectoryIsEmpty } from "../../🧼️workspace-cleanup/🔍️empty-folders/🟦️.ts";

describe("empty folder clean discovery", () => {
  test("detects leaf and nested empty directories", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-empty-folders-"));
    try {
      mkdirSync(join(root, "nested", "leaf"), { recursive: true });
      mkdirSync(join(root, "solo"));
      expect(cleanDirectoryIsEmpty(join(root, "solo"))).toBe(true);
      expect(cleanDirectoryIsEmpty(join(root, "nested", "leaf"))).toBe(true);
      expect(cleanDirectoryIsEmpty(join(root, "nested"))).toBe(false);
      const removals = cleanCollectEmptyFolderRemovals(root, []).map((row) => row.path.replaceAll("\\", "/"));
      expect(removals).toContain("solo");
      expect(removals).toContain("nested/leaf");
      expect(removals).not.toContain("nested");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
