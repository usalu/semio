/** 🖥️ Display chrome panel: the host api, the windows and layout trees, the window kind icons and the Display panel tabs. */
import { type ReactNode } from "../../⚛️react-port/🟦️.ts";
import { Button, COMPOSE_WINDOW_TEMPLATE_MIME, Icon, type IconName, Input, type PanelTabNode, type TreeDataItem, type TreePanelConfig, singleTreeLeaf, windowTemplatePaletteTreeDragController, childElementId } from "@semio-tech/ui-react";
import { type NamedLayout } from "@semio-tech/framework";
import { createWorldProjectionTemplates, encodeWorldProjectionTemplateId, type WorldProjectionTemplateDescriptor } from "@semio-tech/framework/infinite/world/r3f";
import { shellLabel, shellTabIcon } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID } from "../🆔️panel-ids/🟦️.ts";

//#region 🔖️os-chrome-panels
//#region DisplayPanel
export type DisplayHostApi = {
  readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly iconId: IconName; readonly surfaceKind?: string }[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly saveCurrentLayout: (label: string) => void;
  readonly applyNamedLayout: (layoutId: string) => void;
  readonly deleteUserLayout: (layoutId: string) => void;
  readonly layoutSaveLabel: string;
  readonly setLayoutSaveLabel: (value: string) => void;
};

function groupNamedLayoutsToTreeItems(layouts: readonly NamedLayout[], onApply: (layoutId: string) => void, onDeleteUser?: (layoutId: string) => void): TreeDataItem[] {
  const root: TreeDataItem[] = [];
  const folderByKey = new Map<string, TreeDataItem>();
  const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
    id: `framework.display.layout.${entry.id}`,
    label: entry.label,
    onClick: () => onApply(entry.id),
    ...(entry.origin === "user" && onDeleteUser
      ? {
          actions: [
            {
              id: `framework.display.delete.${entry.id}`,
              icon: <Icon icon="trash-2" size="small" />,
              onClick: () => onDeleteUser(entry.id),
            },
          ],
        }
      : {}),
  });
  for (const entry of layouts) {
    if (!entry.groupPath?.length) {
      root.push(layoutLeaf(entry));
      continue;
    }
    let siblings = root;
    let pathKey = "";
    for (let index = 0; index < entry.groupPath.length; index += 1) {
      const segment = entry.groupPath[index]!;
      pathKey = pathKey ? `${pathKey}/${segment}` : segment;
      let folder = folderByKey.get(pathKey);
      if (!folder) {
        folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
        folderByKey.set(pathKey, folder);
        siblings.push(folder);
      }
      const folderItems = folder.items ?? (folder.items = []);
      if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
      else siblings = folderItems;
    }
  }
  return root;
}

/** 🪟️ Recursively converts a {@link WorldProjectionTemplateDescriptor} tree (Parallel/Perspective taxonomy)
 * into draggable {@link TreeDataItem}s for a window kind's Display "Windows" section — each node drags a
 * `{windowKindId, templateId}` payload that seeds the freshly-opened pane's initial camera (see
 * {@link registerPendingWorldProjection}/{@link decodeWorldProjectionTemplateId}). Branches keep `items`
 * so Orthographic > Plan > Top/… nesting stays expandable while Plan itself remains a drag target.
 *
 * The Display "Windows" section is always docked at the `bottom-left` panel anchor (see
 * `PanelGroup::Display`'s `anchor()`), so `Tree` always renders it with `direction="up"` — by design it
 * reverses sibling order at *every* level (`framework/ui/js/react/index.tsx`'s `direction === "up" ? [...items].reverse() : items`,
 * exercised by existing tests), so the box can grow upward from its anchor without breaking parent/child
 * nesting. We pre-reverse each level here to cancel that out, so the palette still *reads* top-to-bottom as
 * Plan > Top/Bottom/Front/Back/Left/Right (etc.) instead of backwards. */
function worldProjectionTemplatesToTreeItems(templates: readonly WorldProjectionTemplateDescriptor[], windowKindId: string, idPrefix: string): TreeDataItem[] {
  return [...templates]
    .reverse()
    .map((template) => ({
      id: childElementId(idPrefix, template.id),
      label: template.label,
      icon: displayWindowKindIcon(template.iconId as IconName),
      defaultOpen: false,
      dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId, templateId: encodeWorldProjectionTemplateId(template.args.spec) }) },
      ...(template.children?.length ? { items: worldProjectionTemplatesToTreeItems(template.children, windowKindId, childElementId(idPrefix, template.id)) } : {}),
    }));
}

function displayWindowKindIcon(iconId: IconName): ReactNode {
  return <Icon icon={iconId} size={12} className="size-tiny shrink-0" />;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
  return {
    dragAndDropController: windowTemplatePaletteTreeDragController(),
    sections: host.windowKinds.length
      ? host.windowKinds.map((kind) => ({
          id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id),
          label: kind.label,
          icon: displayWindowKindIcon(kind.iconId),
          defaultOpen: false,
          items:
            kind.surfaceKind === "world-3d"
              ? [
                  ...worldProjectionTemplatesToTreeItems(createWorldProjectionTemplates({ controllerId: kind.id }), kind.id, childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "projection")),
                  { id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "kind"), label: kind.label, icon: displayWindowKindIcon(kind.iconId), dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }) } },
                ]
              : [
                  {
                    id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "kind"),
                    label: kind.label,
                    icon: displayWindowKindIcon(kind.iconId),
                    dragData: {
                      [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }),
                    },
                  },
                ],
        }))
      : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
  const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
  const userLayouts = host.userLayouts;
  const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
  const userItems = userLayouts.length
    ? [
        {
          id: "framework.display.layout.group.saved",
          label: shellLabel("ui.display.saved"),
          defaultOpen: false,
          items: groupNamedLayoutsToTreeItems(
            userLayouts,
            (layoutId) => host.applyNamedLayout(layoutId),
            (layoutId) => host.deleteUserLayout(layoutId),
          ),
        },
      ]
    : [];
  return {
    sections: [
      {
        id: "framework.display.layout.save",
        label: shellLabel("ui.display.saveLayout"),
        defaultOpen: false,
        items: [
          {
            id: "framework.display.layout.save.label",
            label: shellLabel("ui.common.name"),
            control: <Input id="framework.display.saveLabel" value={host.layoutSaveLabel} onChange={(event) => host.setLayoutSaveLabel(event.target.value)} placeholder={shellLabel("ui.display.saveLayoutPlaceholder")} />,
          },
          {
            id: "framework.display.layout.save.action",
            label: shellLabel("ui.common.save"),
            control: (
              <Button icon="save"
                id="framework.display.save"
                text={shellLabel("ui.display.saveCurrentLayout")}
                disabled={!host.layoutSaveLabel.trim()}
                onClick={() => {
                  const label = host.layoutSaveLabel.trim();
                  if (!label) return;
                  host.saveCurrentLayout(label);
                  host.setLayoutSaveLabel("");
                }}
              />
            ),
          },
        ],
      },
      {
        id: "framework.display.layout.list",
        label: shellLabel("ui.display.layouts"),
        defaultOpen: true,
        items: [...builtinItems, ...userItems],
      },
    ],
  };
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
      icon: shellTabIcon("framework.display.windows"),
      name: shellLabel("ui.display.tab.windows"),
      order: -100,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
      icon: shellTabIcon("framework.display.layout"),
      name: shellLabel("ui.display.tab.layout"),
      order: -99,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
  ];
}
//#endregion DisplayPanel
//#endregion 🔖️os-chrome-panels
