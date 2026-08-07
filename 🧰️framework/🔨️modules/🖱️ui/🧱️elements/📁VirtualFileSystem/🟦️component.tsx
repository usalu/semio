// #region 🧲️Header
// 💻️ framework/ui/elements/📁VirtualFileSystem/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { format, formatDistanceToNow } from "date-fns";
import { type IconName } from "@semio-tech/assets";
// 🧱️core: uiDataLabel/UiLabel imported directly from 🫀️core/UiLabel, NOT via the barrel — this component
// calls uiDataLabel(...) at module top level (inside a top-level demo-fixture object literal), which
// requires a non-circular import (see 🧱️elements/🏷️UiLabel/🟦️component.tsx's header comment for why
// the barrel import caused a real bug).
import { type UiLabel, uiDataLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { TableAvatar } from "../👤Avatar/🟦️component.tsx";
import { Table } from "../🦴Skeletons/🧪️story.tsx";
import { type TableColumn, type TableProps, type HierarchicalRowData, type DragDropConfig } from "../📊Table/🟦️component.tsx";
import { type TreeSelectionMode, normalizeTreeSelectedIds, getTreeNextSelectionState } from "../🪵Tree/🟦️component.tsx";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { Icon } from "../🔣Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 📁️VirtualFileSystem
/** @emoji 🏷️ Render-agnostic descriptor presentation kinds for {@link VirtualFileSystem} columns. */
export type DescriptorKind =
  | { readonly id: string; readonly name: string; readonly description?: string; readonly presentation: "text" }
  | {
      readonly id: string;
      readonly name: string;
      readonly description?: string;
      readonly presentation: "time";
      readonly format?: "date" | "datetime" | "relative";
    }
  | { readonly id: string; readonly name: string; readonly description?: string; readonly presentation: "avatar" };

/** @emoji 🏷️ Column binding on a {@link FileNodeKind} referencing a {@link DescriptorKind}. */
export interface FileNodeDescriptor {
  readonly id: string;
  readonly descriptorKindId: string;
  readonly label?: UiLabel;
  readonly description?: string;
}

/** @emoji 📁️ File node kind registry entry (icon, labels, column descriptors). */
export interface FileNodeKind {
  readonly id: string;
  readonly name: string;
  readonly icon?: string;
  readonly description?: string;
  readonly descriptors: readonly FileNodeDescriptor[];
}

/** @emoji 📁️ Cell value for one {@link FileNodeDescriptor} column on a {@link FileNode}. */
export type FileNodeDescriptorValue = { readonly presentation: "text"; readonly text: string } | { readonly presentation: "time"; readonly iso: string } | { readonly presentation: "avatar"; readonly name: string; readonly icon?: string };

/** @emoji 📁️ Schema driving {@link VirtualFileSystem} columns and glyphs. */
export interface VirtualFileSystemSchema {
  readonly fileNodeKinds: Readonly<Record<string, FileNodeKind>>;
  readonly descriptorKinds: Readonly<Record<string, DescriptorKind>>;
  readonly descriptorColumnIds: readonly string[];
}

/** @emoji 📁️ Demo VFS descriptor kinds for stories and unit tests. */
export const VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS: Readonly<Record<string, DescriptorKind>> = {
  text: { id: "text", name: "Text", presentation: "text" },
  time: { id: "time", name: "Time", presentation: "time", format: "datetime" },
  avatar: { id: "avatar", name: "Avatar", presentation: "avatar" },
};

/** @emoji 📁️ Demo VFS file node kinds for stories and unit tests. */
export const VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS: Readonly<Record<string, FileNodeKind>> = {
  root: {
    id: "root",
    name: "Root",
    icon: "layout-grid",
    descriptors: [
      { id: "path", descriptorKindId: "text", label: uiDataLabel("Path") }, // chrome-i18n-allow: demo fixture
      { id: "fileNodeKind", descriptorKindId: "text", label: uiDataLabel("Node kind") }, // chrome-i18n-allow: demo fixture
    ],
  },
  branch: {
    id: "branch",
    name: "Branch",
    icon: "folder",
    descriptors: [
      { id: "path", descriptorKindId: "text", label: uiDataLabel("Path") }, // chrome-i18n-allow: demo fixture
      { id: "fileNodeKind", descriptorKindId: "text", label: uiDataLabel("Node kind") }, // chrome-i18n-allow: demo fixture
    ],
  },
  leaf: {
    id: "leaf",
    name: "Leaf",
    icon: "file",
    descriptors: [
      { id: "path", descriptorKindId: "text", label: uiDataLabel("Path") }, // chrome-i18n-allow: demo fixture
      { id: "fileNodeKind", descriptorKindId: "text", label: uiDataLabel("Node kind") }, // chrome-i18n-allow: demo fixture
    ],
  },
};

/** @emoji 📁️ Demo virtual file system schema for stories and unit tests. */
export const VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA: VirtualFileSystemSchema = {
  fileNodeKinds: VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
  descriptorKinds: VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  descriptorColumnIds: ["path", "fileNodeKind"],
};

/** @emoji 📁️ One node in a virtual file system tree (children may be loaded lazily by the host). */
export interface FileNode {
  readonly id: string;
  readonly fileNodeKindId: string;
  readonly name: string;
  readonly path?: string;
  readonly parentId?: string | null;
  readonly hasChildren?: boolean;
  readonly icon?: string;
  readonly descriptorValues?: Readonly<Record<string, FileNodeDescriptorValue>>;
}

/** @emoji 📁️ {@link FileNode} alias used by {@link VirtualFileSystem}. */
export type VirtualFileSystemNode = FileNode;

/** @emoji 📁️ Flattened visible row for {@link VirtualFileSystem} (only expanded branches). */
export interface VirtualFileSystemRow extends FileNode, HierarchicalRowData {
  readonly level: number;
  readonly isExpanded?: boolean;
  readonly navigateUri?: string;
}

/** @emoji 📁️ Props for {@link VirtualFileSystem} — a hierarchical {@link Table} for virtual file tree nodes. */
export interface VirtualFileSystemProps {
  readonly schema: VirtualFileSystemSchema;
  readonly rows: readonly VirtualFileSystemRow[];
  readonly selectionMode?: TreeSelectionMode;
  readonly selectedRowIds?: Set<string> | readonly string[];
  readonly defaultSelectedRowIds?: readonly string[];
  readonly onSelectionChange?: (selectedRowIds: readonly string[], context: { readonly anchorRowId?: string }) => void;
  readonly onRowClick?: (row: VirtualFileSystemRow, index: number, event: React.MouseEvent) => void;
  readonly onRowContextMenu?: (row: VirtualFileSystemRow, index: number, event: React.MouseEvent) => void;
  readonly onRowDoubleClick?: (row: VirtualFileSystemRow, index: number) => void;
  readonly onRowMouseEnter?: (row: VirtualFileSystemRow, index: number) => void;
  readonly onRowMouseLeave?: (row: VirtualFileSystemRow, index: number) => void;
  readonly rowClassName?: (row: VirtualFileSystemRow, index: number) => string;
  readonly onToggleExpand?: (rowId: string) => void;
  readonly emptyMessage?: UiLabel;
  readonly className?: string;
  readonly rowHeight?: TableProps<VirtualFileSystemRow>["rowHeight"];
  readonly dragDrop?: DragDropConfig;
  readonly extraColumns?: readonly TableColumn<VirtualFileSystemRow>[];
}

/** @emoji 📁️ Visible row order for shift-range selection in {@link VirtualFileSystem}. */
export function getVirtualFileSystemOrderedRowIds(rows: readonly VirtualFileSystemRow[]): string[] {
  return rows.map((row) => row.id);
}

/** @emoji 📁️ Normalizes selected row ids for {@link VirtualFileSystem} selection mode. */
export function normalizeVirtualFileSystemSelectedRowIds(selectedRowIds: readonly string[], selectionMode: TreeSelectionMode): string[] {
  return normalizeTreeSelectedIds([...selectedRowIds], selectionMode);
}

/** @emoji 📁️ Next selection after a row click (shift range, ctrl/cmd toggle, plain replace). */
export function getVirtualFileSystemNextSelectionState(args: {
  readonly selectionMode: TreeSelectionMode;
  readonly selectedRowIds: readonly string[];
  readonly orderedRowIds: readonly string[];
  readonly targetRowId: string;
  readonly anchorRowId?: string;
  readonly additiveKey: boolean;
  readonly rangeKey: boolean;
}): { readonly selectedRowIds: string[]; readonly anchorRowId?: string } {
  const next = getTreeNextSelectionState({
    selectionMode: args.selectionMode,
    selectedIds: [...args.selectedRowIds],
    orderedIds: [...args.orderedRowIds],
    targetId: args.targetRowId,
    anchorId: args.anchorRowId,
    additiveKey: args.additiveKey,
    rangeKey: args.rangeKey,
  });
  return { selectedRowIds: next.selectedIds, anchorRowId: next.anchorId };
}

/** @emoji 📁️ Resolves a {@link FileNodeKind} from a {@link VirtualFileSystemSchema}. */
export function resolveVirtualFileSystemFileNodeKind(schema: VirtualFileSystemSchema, fileNodeKindId: string): FileNodeKind | undefined {
  return schema.fileNodeKinds[fileNodeKindId];
}

/** @emoji 📁️ Resolves a {@link DescriptorKind} from a {@link VirtualFileSystemSchema}. */
export function resolveVirtualFileSystemDescriptorKind(schema: VirtualFileSystemSchema, descriptorKindId: string): DescriptorKind | undefined {
  return schema.descriptorKinds[descriptorKindId];
}

/** @emoji 📁️ Finds the first {@link FileNodeDescriptor} binding for a column id across all file node kinds. */
export function resolveVirtualFileSystemDescriptorBinding(schema: VirtualFileSystemSchema, descriptorColumnId: string): { readonly binding: FileNodeDescriptor; readonly descriptorKind: DescriptorKind } | undefined {
  for (const fileNodeKind of Object.values(schema.fileNodeKinds)) {
    const binding = fileNodeKind.descriptors.find((entry) => entry.id === descriptorColumnId);
    if (!binding) continue;
    const descriptorKind = schema.descriptorKinds[binding.descriptorKindId];
    if (!descriptorKind) continue;
    return { binding, descriptorKind };
  }
  return undefined;
}

/** @emoji 📁️ Builds descriptor cell values from a {@link VirtualFileSystemSchema}. */
export function buildVirtualFileSystemDescriptorValues(
  schema: VirtualFileSystemSchema,
  fileNodeKindId: string,
  options: {
    readonly path?: string;
    readonly updatedIso?: string;
    readonly createdBy?: { readonly name: string; readonly icon?: string };
    readonly textByDescriptorId?: Readonly<Record<string, string>>;
    readonly extra?: Readonly<Record<string, FileNodeDescriptorValue>>;
  } = {},
): Readonly<Record<string, FileNodeDescriptorValue>> {
  const fileNodeKind = schema.fileNodeKinds[fileNodeKindId];
  const values: Record<string, FileNodeDescriptorValue> = { ...options.extra };
  if (options.path !== undefined) values.path = { presentation: "text", text: options.path };
  if (fileNodeKind) values.fileNodeKind = { presentation: "text", text: fileNodeKind.name };
  if (options.updatedIso) values.updated = { presentation: "time", iso: options.updatedIso };
  if (options.createdBy) values.createdBy = { presentation: "avatar", name: options.createdBy.name, icon: options.createdBy.icon };
  if (options.textByDescriptorId) {
    for (const [descriptorId, text] of Object.entries(options.textByDescriptorId)) {
      values[descriptorId] = { presentation: "text", text };
    }
  }
  return values;
}

/** @emoji 📁️ Renders one descriptor cell for a {@link VirtualFileSystemRow}. */
export function renderVirtualFileSystemDescriptorCell(descriptorKind: DescriptorKind, value: FileNodeDescriptorValue | undefined): React.ReactNode {
  if (!value || value.presentation !== descriptorKind.presentation) return "";
  switch (value.presentation) {
    case "text":
      return value.text;
    case "time": {
      const parsed = Date.parse(value.iso);
      if (Number.isNaN(parsed)) return value.iso;
      const date = new Date(parsed);
      if (descriptorKind.presentation === "time" && descriptorKind.format === "relative") {
        return formatDistanceToNow(date, { addSuffix: true });
      }
      if (descriptorKind.presentation === "time" && descriptorKind.format === "date") {
        return format(date, "yyyy-MM-dd");
      }
      return format(date, "yyyy-MM-dd HH:mm");
    }
    case "avatar":
      return <TableAvatar name={value.name} icon={value.icon} />;
    default:
      return "";
  }
}

/** @emoji 📁️ Builds {@link TableColumn} entries from {@link VirtualFileSystemSchema} descriptor columns. */
export function buildVirtualFileSystemDescriptorColumns(schema: VirtualFileSystemSchema): TableColumn<VirtualFileSystemRow>[] {
  const columns: TableColumn<VirtualFileSystemRow>[] = [];
  for (const columnId of schema.descriptorColumnIds) {
    const resolved = resolveVirtualFileSystemDescriptorBinding(schema, columnId);
    if (!resolved) continue;
    const { binding, descriptorKind } = resolved;
    columns.push({
      id: columnId,
      header: binding.label ?? descriptorKind.name,
      width: descriptorKind.presentation === "avatar" ? "12%" : "14%",
      accessor: (row) => {
        const fileNodeKind = schema.fileNodeKinds[row.fileNodeKindId];
        if (!fileNodeKind?.descriptors.some((entry) => entry.id === columnId)) return "";
        return renderVirtualFileSystemDescriptorCell(descriptorKind, row.descriptorValues?.[columnId]);
      },
    });
  }
  return columns;
}

/** @emoji 📁️ Built-in icons keyed by VFS schema `icon` ids and {@link FileNodeKind} ids. */
const VIRTUAL_FILE_SYSTEM_ICON_BY_ID: Readonly<Record<string, IconName>> = {
  "layout-grid": "layout-grid",
  folder: "folder",
  file: "file-text",
  branch: "folder",
  leaf: "file-text",
  layout: "layout",
  component: "component",
  users: "users",
  landmark: "landmark",
  puzzle: "puzzle",
  link: "link",
  box: "box",
  "circle-dot": "circle-dot",
  plug: "plug",
  root: "layout-grid",
  kit: "layout-grid",
  design: "layout",
  type: "component",
  family: "users",
  typology: "landmark",
  piece: "puzzle",
  connection: "link",
  representation: "box",
  port: "circle-dot",
  connector: "plug",
  json: "file-json",
  jsonc: "file-json",
  json5: "file-json",
  yaml: "file-code",
  yml: "file-code",
  toml: "file-code",
  xml: "file-code",
  md: "file-text",
  markdown: "file-text",
  txt: "file-text",
  log: "file-text",
  pdf: "file-type",
  png: "file-image",
  jpg: "file-image",
  jpeg: "file-image",
  gif: "file-image",
  webp: "file-image",
  svg: "file-image",
  ico: "file-image",
  bmp: "file-image",
  glb: "box",
  gltf: "box",
  obj: "box",
  fbx: "box",
  stl: "box",
  usdz: "box",
  zip: "file-archive",
  tar: "file-archive",
  gz: "file-archive",
  tgz: "file-archive",
  "7z": "file-archive",
  rar: "file-archive",
  csv: "file-spreadsheet",
  tsv: "file-spreadsheet",
  xlsx: "file-spreadsheet",
  xls: "file-spreadsheet",
  ts: "file-code",
  tsx: "file-code",
  js: "file-code",
  jsx: "file-code",
  mjs: "file-code",
  cjs: "file-code",
  rs: "file-code",
  py: "file-code",
  wasm: "file-code",
  html: "file-code",
  css: "file-code",
  scss: "file-code",
  sql: "file-code",
  compose: "file-json",
};

/** @emoji 📁️ Resolves a built-in icon for a VFS schema icon id or file node kind id. */
export function resolveVirtualFileSystemSchemaIcon(iconOrKindId: string): IconName | undefined {
  return VIRTUAL_FILE_SYSTEM_ICON_BY_ID[iconOrKindId];
}

/** @emoji 📁️ Returns a built-in icon name for a generic VFS file node kind id. */
export function virtualFileSystemKindIcon(fileNodeKindId: string): IconName {
  return resolveVirtualFileSystemSchemaIcon(fileNodeKindId) ?? "file-text";
}

/** @emoji 📁️ True when a VFS row `icon` value is a remote or data URL image, not a schema icon id. */
export function isVirtualFileSystemRemoteIcon(icon: string): boolean {
  const trimmed = icon.trim();
  return trimmed.startsWith("http://") || trimmed.startsWith("https://") || trimmed.startsWith("data:") || trimmed.startsWith("/") || trimmed.startsWith("./");
}

/** @emoji 📁️ DFS-flattens visible rows: only children of expanded parents in `childrenByParentId`. */
export function buildVirtualFileSystemVisibleRows(rootId: string, childrenByParentId: ReadonlyMap<string, readonly VirtualFileSystemNode[]>, expandedIds: ReadonlySet<string>, root?: VirtualFileSystemNode): VirtualFileSystemRow[] {
  const rows: VirtualFileSystemRow[] = [];
  const visit = (node: VirtualFileSystemNode, level: number) => {
    const hasChildren = Boolean(node.hasChildren);
    const expanded = hasChildren && expandedIds.has(node.id);
    rows.push({
      ...node,
      level,
      parentId: node.parentId ?? undefined,
      hasChildren,
      isExpanded: expanded,
    });
    if (!expanded) return;
    const children = childrenByParentId.get(node.id);
    if (!children?.length) return;
    for (const child of children) visit(child, level + 1);
  };
  const rootNode = root ?? {
    id: rootId,
    fileNodeKindId: "root",
    name: rootId,
    hasChildren: childrenByParentId.has(rootId) || expandedIds.has(rootId),
  };
  const rootChildren = childrenByParentId.get(rootNode.id);
  if (rootChildren?.length) {
    for (const child of rootChildren) visit(child, 0);
  }
  return rows;
}

const VirtualFileSystemNodeGlyph: React.FC<{
  readonly schema: VirtualFileSystemSchema;
  readonly fileNodeKindId: string;
  readonly icon?: string;
  readonly name: string;
}> = ({ schema, fileNodeKindId, icon }) => {
  const kindIcon = icon ?? schema.fileNodeKinds[fileNodeKindId]?.icon;
  const glyphClass = "inline-flex size-small shrink-0 items-center justify-center";
  const schemaIcon = kindIcon ? resolveVirtualFileSystemSchemaIcon(kindIcon) : undefined;
  if (schemaIcon) {
    return (
      <span className={glyphClass}>
        <Icon icon={schemaIcon} size={14} />
      </span>
    );
  }
  if (kindIcon && isVirtualFileSystemRemoteIcon(kindIcon)) {
    return (
      <span className={`${glyphClass} overflow-hidden rounded-sm`}>
        <img src={kindIcon} alt="" className="size-full object-cover" />
      </span>
    );
  }
  if (kindIcon) {
    return (
      <span className={`${glyphClass} text-base leading-none`} aria-hidden>
        {kindIcon}
      </span>
    );
  }
  return (
    <span className={glyphClass}>
      <Icon icon={virtualFileSystemKindIcon(fileNodeKindId)} size={14} />
    </span>
  );
};

/** @emoji 📁️ Hierarchical virtual file-system table (specialized {@link Table}). */
export const VirtualFileSystem: React.FC<VirtualFileSystemProps> = ({
  schema,
  rows,
  selectionMode = "multiple",
  selectedRowIds: controlledSelectedRowIds,
  defaultSelectedRowIds = [],
  onSelectionChange,
  onRowClick,
  onRowContextMenu,
  onRowDoubleClick,
  onRowMouseEnter,
  onRowMouseLeave,
  rowClassName,
  onToggleExpand,
  emptyMessage,
  className = "",
  rowHeight = "normal",
  dragDrop,
  extraColumns = [],
}) => {
  const noFileSystemNodesLabel = useLabel("ui.common.noFileSystemNodes");
  const resolvedEmptyMessage = emptyMessage ?? noFileSystemNodesLabel;
  const nameLabel = useLabel("ui.common.name");
  const collapseLabel = useLabel("ui.common.collapse");
  const expandLabel = useLabel("ui.common.expand");
  const [uncontrolledSelectedRowIds, setUncontrolledSelectedRowIds] = reactHostPort.useState<Set<string>>(() => new Set(normalizeVirtualFileSystemSelectedRowIds(defaultSelectedRowIds, selectionMode)));
  const selectionAnchorRowIdRef = reactHostPort.useRef<string | undefined>(normalizeVirtualFileSystemSelectedRowIds(defaultSelectedRowIds, selectionMode)[0]);
  const orderedRowIds = reactHostPort.useMemo(() => getVirtualFileSystemOrderedRowIds(rows), [rows]);
  const resolvedSelectedRowIds = reactHostPort.useMemo(() => {
    if (controlledSelectedRowIds === undefined) return uncontrolledSelectedRowIds;
    return controlledSelectedRowIds instanceof Set ? controlledSelectedRowIds : new Set(controlledSelectedRowIds);
  }, [controlledSelectedRowIds, uncontrolledSelectedRowIds]);
  const applySelection = reactHostPort.useCallback(
    (next: { readonly selectedRowIds: string[]; readonly anchorRowId?: string }) => {
      const normalized = normalizeVirtualFileSystemSelectedRowIds(next.selectedRowIds, selectionMode);
      selectionAnchorRowIdRef.current = next.anchorRowId ?? normalized[normalized.length - 1];
      if (controlledSelectedRowIds === undefined) {
        setUncontrolledSelectedRowIds(new Set(normalized));
      }
      onSelectionChange?.(normalized, { anchorRowId: selectionAnchorRowIdRef.current });
    },
    [controlledSelectedRowIds, onSelectionChange, selectionMode],
  );
  const handleRowClick = reactHostPort.useCallback(
    (row: VirtualFileSystemRow, index: number, event: React.MouseEvent) => {
      const next = getVirtualFileSystemNextSelectionState({
        selectionMode,
        selectedRowIds: [...resolvedSelectedRowIds],
        orderedRowIds,
        targetRowId: row.id,
        anchorRowId: selectionAnchorRowIdRef.current,
        additiveKey: event.metaKey || event.ctrlKey,
        rangeKey: event.shiftKey,
      });
      applySelection(next);
      onRowClick?.(row, index, event);
    },
    [applySelection, onRowClick, orderedRowIds, resolvedSelectedRowIds, selectionMode],
  );
  const columns = reactHostPort.useMemo((): TableColumn<VirtualFileSystemRow>[] => {
    const base: TableColumn<VirtualFileSystemRow>[] = [
      {
        id: "name",
        header: nameLabel,
        width: "32%",
        accessor: (row) => (
          <div className="flex min-w-0 items-center gap-single" style={{ paddingInlineStart: (row.level ?? 0) * 14 }}>
            {row.hasChildren ? (
              <button
                type="button"
                data-vfs-expand
                className="inline-flex size-small shrink-0 items-center justify-center rounded text-element hover:bg-hover-interactive-fill hover:text-emphasized"
                aria-label={row.isExpanded ? collapseLabel : expandLabel}
                onClick={(event) => {
                  event.stopPropagation();
                  onToggleExpand?.(row.id);
                }}
                onDoubleClick={(event) => event.stopPropagation()}
              >
                {row.isExpanded ? "▾️" : "▸️"}
              </button>
            ) : (
              <span className="inline-block size-small shrink-0" aria-hidden />
            )}
            <VirtualFileSystemNodeGlyph schema={schema} fileNodeKindId={row.fileNodeKindId} icon={row.icon} name={row.name} />
            <span className="truncate">{row.name}</span>
          </div>
        ),
      },
      ...buildVirtualFileSystemDescriptorColumns(schema),
    ];
    return [...base, ...extraColumns];
  }, [extraColumns, onToggleExpand, schema, nameLabel, collapseLabel, expandLabel]);

  return (
    <Table<VirtualFileSystemRow>
      className={className}
      columns={columns}
      data={[...rows]}
      getRowId={(row) => row.id}
      selectedRows={resolvedSelectedRowIds}
      onRowClick={handleRowClick}
      onRowContextMenu={onRowContextMenu}
      onRowDoubleClick={onRowDoubleClick}
      onRowMouseEnter={onRowMouseEnter}
      onRowMouseLeave={onRowMouseLeave}
      rowClassName={rowClassName}
      emptyMessage={resolvedEmptyMessage}
      rowHeight={rowHeight}
      hierarchical
      dragDrop={dragDrop}
    />
  );
};

VirtualFileSystem.displayName = "VirtualFileSystem";

// #endregion 📁️VirtualFileSystem
