import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import userEvent from "@testing-library/user-event";
import { createElement } from "react";
import { render } from "@semio-tech/ui-react/test";
import { buildVirtualFileSystemSceneRows } from "@semio-tech/ui-react";
import { VirtualFileSystemHost } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { describe, expect, it, vi } from "vitest";

type WindowFixture = { readonly windowId: string; readonly bodyKey: string; readonly surfaceKind: "block-list" | "table" | "diff-view" | "event-feed" | "text-editor" | "virtual-file-system" };
type FileRowFixture = { readonly id: string; readonly fileNodeKindId: string; readonly name: string; readonly parentId: string | null; readonly hasChildren: boolean; readonly descriptorValues: Readonly<Record<string, never>> };
type FilesFixture = {
  readonly surfaceId: string;
  readonly schema: Readonly<Record<string, unknown>>;
  readonly rows: readonly FileRowFixture[];
  readonly initialVisibleRowIds: readonly string[];
  readonly initialLevels: readonly number[];
  readonly collapsedVisibleRowIds: readonly string[];
  readonly collapsedLevels: readonly number[];
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const taxonomy = "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder";
const fixture = JSON.parse(readFileSync(resolve(repoRoot, taxonomy, "🧫️fixtures/🎬️scene-showcase/🔣️.json"), "utf8")) as { readonly windows: readonly WindowFixture[]; readonly files: FilesFixture };
const schema = JSON.parse(readFileSync(resolve(repoRoot, taxonomy, "🧬️schema/🎬️scene-showcase/🔣️.json"), "utf8"));

describe("registered Playbook scene showcase", () => {
  it("validates the authored route corpus with the independent JSON Schema oracle", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.windows.map(({ surfaceKind }) => surfaceKind)).toEqual(["block-list", "table", "diff-view", "event-feed", "text-editor", "virtual-file-system"]);
    expect(new Set(fixture.windows.map(({ windowId }) => windowId)).size).toBe(fixture.windows.length);
    expect(fixture.files.rows.every((row) => !("navigateUri" in row))).toBe(true);
    const expanded = new Set(fixture.files.rows.filter((row) => row.hasChildren).map((row) => row.id));
    const initial = buildVirtualFileSystemSceneRows(fixture.files.rows, expanded);
    expect(initial.map(({ id }) => id)).toEqual(fixture.files.initialVisibleRowIds);
    expect(initial.map(({ level }) => level)).toEqual(fixture.files.initialLevels);
  });

  it("mounts the actual Files host and keeps disclosure keyboard interaction local", async () => {
    const onAction = vi.fn();
    const view = render(
      createElement(VirtualFileSystemHost, {
        node: {
          type: "componentScene",
          surfaceId: fixture.files.surfaceId,
          controllerId: "playbook-play",
          componentKind: "virtual-file-system",
          virtualFileSystem: { schemaJson: JSON.stringify(fixture.files.schema), rowsJson: JSON.stringify(fixture.files.rows), dragDropEnabled: false },
        },
        onAction,
      }),
    );
    const visibleIds = () => [...view.container.querySelectorAll("tr[data-row-id]")].map((row) => row.getAttribute("data-row-id"));
    expect(visibleIds()).toEqual(fixture.files.initialVisibleRowIds);
    const toggle = view.container.querySelector('tr[data-row-id="step/basics"] button[data-vfs-expand]') as HTMLButtonElement;
    expect(toggle).toBeTruthy();
    expect(toggle.getAttribute("aria-expanded")).toBe("true");
    toggle.focus();
    const user = userEvent.setup();
    await user.keyboard("{Enter}");
    expect(visibleIds()).toEqual(fixture.files.collapsedVisibleRowIds);
    expect(buildVirtualFileSystemSceneRows(fixture.files.rows, new Set(["playbook"])).map(({ level }) => level)).toEqual(fixture.files.collapsedLevels);
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    await user.keyboard(" ");
    expect(visibleIds()).toEqual(fixture.files.initialVisibleRowIds);
    expect(toggle.getAttribute("aria-expanded")).toBe("true");
    expect(onAction).not.toHaveBeenCalled();
  });
});
