/** 🔸️ The virtual file system phase of registerTreeHelpers4AndMoreTests. */
import type { VirtualFileSystemNode, VirtualFileSystemRow, VirtualFileSystemSchema } from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { registerTreeHelpersTests } from "../🔶️tree-helpers/🟦️.tsx";

/** 🔹 Registers the virtual file system tests of registerTreeHelpers4AndMoreTests. */
export function registerVirtualFileSystemTests({ describe, it, buildVirtualFileSystemVisibleRows, expect, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, uiDataLabel, buildVirtualFileSystemDescriptorColumns, renderToStaticMarkup, formatVirtualFileSystemTime, VirtualFileSystem, resolveVirtualFileSystemSchemaIcon, vi, getVirtualFileSystemNextSelectionState }: Pick<ReturnType<typeof registerTreeHelpersTests>, "describe" | "it" | "buildVirtualFileSystemVisibleRows" | "expect" | "VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA" | "VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS" | "uiDataLabel" | "buildVirtualFileSystemDescriptorColumns" | "renderToStaticMarkup" | "formatVirtualFileSystemTime" | "VirtualFileSystem" | "resolveVirtualFileSystemSchemaIcon" | "vi" | "getVirtualFileSystemNextSelectionState">) {
    describe("VirtualFileSystem", () => {
      it("buildVirtualFileSystemVisibleRows only includes children of expanded parents", () => {
        const root: VirtualFileSystemNode = { id: "root", fileNodeKindId: "root", name: "Root", hasChildren: true };
        const childrenByParentId = new Map<string, readonly VirtualFileSystemNode[]>([
          [
            "root",
            [
              { id: "f1", fileNodeKindId: "branch", name: "Models", parentId: "root", hasChildren: true },
              { id: "d1", fileNodeKindId: "leaf", name: "Tower", parentId: "root", hasChildren: false },
            ],
          ],
          ["f1", [{ id: "t1", fileNodeKindId: "leaf", name: "Capsule", parentId: "f1", hasChildren: false }]],
        ]);
        const collapsed = buildVirtualFileSystemVisibleRows("root", childrenByParentId, new Set(["root"]), root);
        expect(collapsed.map((row) => row.id)).toEqual(["f1", "d1"]);
        const expanded = buildVirtualFileSystemVisibleRows("root", childrenByParentId, new Set(["root", "f1"]), root);
        expect(expanded.map((row) => row.id)).toEqual(["f1", "t1", "d1"]);
      });
  
      it("buildVirtualFileSystemDescriptorColumns renders avatar and time cells", () => {
        const schemaWithMeta: VirtualFileSystemSchema = {
          ...VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
          descriptorColumnIds: ["updated", "createdBy", "path", "fileNodeKind"],
          fileNodeKinds: {
            ...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
            root: {
              ...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.root,
              descriptors: [...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.root.descriptors, { id: "updated", descriptorKindId: "time", label: uiDataLabel("Updated") }, { id: "createdBy", descriptorKindId: "avatar", label: uiDataLabel("Created by") }],
            },
          },
        };
        const columns = buildVirtualFileSystemDescriptorColumns(schemaWithMeta, "en");
        const createdBy = columns.find((column) => column.id === "createdBy");
        expect(createdBy?.header).toBe("Created by");
        const updated = columns.find((column) => column.id === "updated");
        expect(updated?.header).toBe("Updated");
        const row: VirtualFileSystemRow = {
          id: "root:1",
          fileNodeKindId: "root",
          name: "Alpha",
          level: 0,
          descriptorValues: {
            updated: { presentation: "time", iso: "2026-05-01T12:00:00.000Z" },
            createdBy: { presentation: "avatar", name: "Ada", icon: "https://example.com/a.png" },
          },
        };
        const updatedMarkup = renderToStaticMarkup(<>{updated?.accessor(row)}</>);
        expect(updatedMarkup).toContain("2026");
        const avatarColumn = buildVirtualFileSystemDescriptorColumns(
          {
            ...schemaWithMeta,
            descriptorColumnIds: ["createdBy"],
          },
          "en",
        )[0];
        const avatarMarkup = renderToStaticMarkup(<>{avatarColumn?.accessor(row)}</>);
        expect(avatarMarkup).toContain("avatar-fallback");
        expect(avatarMarkup).toContain(">A<");
      });
  
      it("formats calendar and relative timestamps through the owned locale-explicit formatter", () => {
        const calendar = new Date(2026, 4, 1, 12, 34);
        expect(formatVirtualFileSystemTime(calendar, "date", "en")).toBe(new Intl.DateTimeFormat("en", { year: "numeric", month: "2-digit", day: "2-digit" }).format(calendar));
        expect(formatVirtualFileSystemTime(calendar, "datetime", "de")).toBe(new Intl.DateTimeFormat("de", { year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit" }).format(calendar));
        const now = new Date("2026-05-01T12:00:00.000Z");
        const future = new Date("2026-05-01T14:00:00.000Z");
        const past = new Date("2026-05-01T10:00:00.000Z");
        expect(formatVirtualFileSystemTime(future, "relative", "en", now)).toContain("2 hours");
        expect(formatVirtualFileSystemTime(past, "relative", "de", now)).toContain("2 Stunden");
      });
  
      it("renders expand affordance only for rows with children", () => {
        const markup = renderToStaticMarkup(
          <VirtualFileSystem
            schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA}
            rows={[
              { id: "root", fileNodeKindId: "root", name: "Root", level: 0, hasChildren: true, isExpanded: true },
              { id: "file", fileNodeKindId: "leaf", name: "readme.md", level: 1, hasChildren: false },
            ]}
          />,
        );
        expect(markup).toContain("data-vfs-expand");
        expect(markup).toContain("readme.md");
        expect(markup).toContain("cursor-selectable");
        expect(markup).toContain("text-element");
        expect(markup).not.toContain("text-muted-foreground");
      });
  
      it("renders file node kind vendored icons instead of avatars for schema icon ids", () => {
        const markup = renderToStaticMarkup(<VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "root", fileNodeKindId: "root", name: "Alpha", level: 0, hasChildren: false }]} />);
        expect(markup).toContain('data-icon="layout-grid"');
        expect(markup).not.toContain("avatar-fallback");
      });
  
      it("resolveVirtualFileSystemSchemaIcon maps sketchpad vfs icon ids", () => {
        expect(resolveVirtualFileSystemSchemaIcon("component")).toBe("component");
        expect(resolveVirtualFileSystemSchemaIcon("circle-dot")).toBe("circle-dot");
        expect(resolveVirtualFileSystemSchemaIcon("type")).toBe("component");
      });
  
      it("resolveVirtualFileSystemSchemaIcon maps file extension ids", () => {
        expect(resolveVirtualFileSystemSchemaIcon("glb")).toBe("box");
        expect(resolveVirtualFileSystemSchemaIcon("pdf")).toBe("file-type");
        expect(resolveVirtualFileSystemSchemaIcon("json")).toBe("file-json");
      });
  
      it("renders per-row extension icons for kit files", () => {
        const markup = renderToStaticMarkup(<VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "f1", fileNodeKindId: "leaf", name: "Tower", icon: "glb", level: 0, hasChildren: false }]} />);
        expect(markup).toContain('data-icon="box"');
        expect(markup).not.toContain("avatar-fallback");
      });
  
      it("invokes onRowDoubleClick on double-click", async () => {
        const { render } = await import("@testing-library/react");
        const userEvent = (await import("@testing-library/user-event")).default;
        const user = userEvent.setup();
        const onRowDoubleClick = vi.fn();
        const { container } = render(
          <VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "leaf-a", fileNodeKindId: "leaf", name: "Alpha", level: 0, hasChildren: false, navigateUri: "/alpha" }]} onRowDoubleClick={onRowDoubleClick} />,
        );
        const leafRow = container.querySelector('tr[data-row-id="leaf-a"]');
        expect(leafRow).toBeTruthy();
        await user.dblClick(leafRow!);
        expect(onRowDoubleClick).toHaveBeenCalledWith(expect.objectContaining({ id: "leaf-a", navigateUri: "/alpha" }), 0);
      });
  
      it("computes shift range and ctrl toggle selection for visible rows", () => {
        const orderedRowIds = ["root", "branch", "leaf-a", "leaf-b"];
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "multiple",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-b",
            anchorRowId: "root",
            additiveKey: false,
            rangeKey: true,
          }).selectedRowIds,
        ).toEqual(["root", "branch", "leaf-a", "leaf-b"]);
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "multiple",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-b",
            anchorRowId: "root",
            additiveKey: true,
            rangeKey: false,
          }).selectedRowIds,
        ).toEqual(["root", "leaf-b"]);
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "single",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-a",
            additiveKey: true,
            rangeKey: true,
          }).selectedRowIds,
        ).toEqual(["leaf-a"]);
      });
    });
}
