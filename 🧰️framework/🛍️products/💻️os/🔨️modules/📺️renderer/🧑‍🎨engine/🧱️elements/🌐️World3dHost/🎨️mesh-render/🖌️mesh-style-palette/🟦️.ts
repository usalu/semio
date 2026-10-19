/** 🖌️ Mesh style palette: the style kinds, the CSS-expression paint table, the live palette resolution, the style and selection-preview resolution and the semantic colors. */
import { useCallback, useState } from "react";
import {
  clearColorResolveCache,
  resolveColorHex,
  semanticVar,
  themeColorVar,
  tokenVar
} from "@semio-tech/ui-styling";
import { useCanvasAppearanceSync, useShellScopeOptional } from "@semio-tech/ui-react";
import { TOOL_RUN_PROVISIONAL_PAINT } from "../../⏯️tool-run-trace/🟦️.tsx";
import { type WorldInstanceRecord } from "../../🔭️scene-schema/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
//#region WorldMeshPaint
/** 🎨️ Mesh style kinds, in {@link resolveMeshStyle} priority order (highest first). */
export type MeshStyleKind = "disabled" | "provisional" | "celebrated" | "selected" | "highlighted" | "hovered" | "neutral";

export type MeshStyleColors = {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
};

export type MeshStylePalette = Readonly<Record<MeshStyleKind, MeshStyleColors>>;

/** 🎨️ CSS-expression paint spec per style kind, ported from the premigration puzzle 3d paint table. */
const MESH_STYLE_PAINT: Readonly<Record<MeshStyleKind, { readonly fill: string; readonly line: string; readonly emissiveIntensity: number; readonly opacity: number }>> = {
  neutral: { fill: "var(--panel)", line: semanticVar("border-normal-color"), emissiveIntensity: 0, opacity: 1 },
  hovered: { fill: semanticVar("hover-interactive-fill"), line: semanticVar("border-emphasized-color"), emissiveIntensity: 0.08, opacity: 1 },
  selected: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.35, opacity: 1 },
  highlighted: { fill: tokenVar("secondary"), line: tokenVar("secondary"), emissiveIntensity: 0.2, opacity: 1 },
  // 🎉️ Transient drop/completion paint — solid fallback for lines; shaded meshes use {@link CelebratingConicMaterial}.
  // 🟩️ The `provisional` style token: a running tool's not-yet-finalized placement (dashed outline via `ToolRunProvisionalOutline`).
  provisional: { fill: TOOL_RUN_PROVISIONAL_PAINT.fill, line: TOOL_RUN_PROVISIONAL_PAINT.line, emissiveIntensity: 0.2, opacity: TOOL_RUN_PROVISIONAL_PAINT.opacity },
  celebrated: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.55, opacity: 1 },
  disabled: { fill: "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--panel))", line: themeColorVar("muted-foreground"), emissiveIntensity: 0, opacity: 0.45 },
};

/** 🎨️ Resolves the full {@link MeshStylePalette} from live CSS custom properties (theme/dark-mode aware). */
function resolveMeshStylePalette(): MeshStylePalette {
  const resolved = {} as Record<MeshStyleKind, MeshStyleColors>;
  for (const kind of Object.keys(MESH_STYLE_PAINT) as MeshStyleKind[]) {
    const spec = MESH_STYLE_PAINT[kind];
    resolved[kind] = {
      meshColor: resolveColorHex(spec.fill),
      lineColor: resolveColorHex(spec.line),
      emissiveIntensity: spec.emissiveIntensity,
      opacity: spec.opacity,
    };
  }
  return resolved;
}

export function useMeshStylePalette(): MeshStylePalette {
  const [palette, setPalette] = useState(resolveMeshStylePalette);
  const shellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    useCallback(() => {
      clearColorResolveCache();
      setPalette(resolveMeshStylePalette());
    }, []),
    true,
    shellScope?.rootRef.current ?? undefined,
  );
  return palette;
}

/** 🎨️ Remount key for world mesh materials — R3F/three often keep the previous selected emissive until a later hover forces an update. */
export function worldMeshMaterialRevision(kind: MeshStyleKind): MeshStyleKind {
  return kind;
}

/** 🎨️ Resolves the effective style kind for an instance/component, priority: disabled → provisional → celebrated → selected → highlighted → hovered → neutral. */
export function resolveMeshStyle(state: {
  readonly disabled?: boolean;
  readonly provisional?: boolean;
  readonly celebrating?: boolean;
  readonly selected?: boolean;
  readonly highlighted?: boolean;
  readonly hovered?: boolean;
}): MeshStyleKind {
  if (state.disabled) return "disabled";
  if (state.provisional) return "provisional";
  if (state.celebrating) return "celebrated";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}

/** 🎨️ Resolves live group-selection preview paint: the new selection is active, while only objects exiting the old selection are highlighted. */
export function resolveMeshSelectionPreviewStyle(
  instance: Pick<WorldInstanceRecord, "disabled" | "provisional" | "selected" | "highlighted" | "hovered"> & { readonly celebrating?: boolean },
  previewSelected?: boolean,
): MeshStyleKind {
  const selectionExited = previewSelected === false && instance.selected === true;
  return resolveMeshStyle({
    disabled: instance.disabled,
    provisional: instance.provisional,
    celebrating: instance.celebrating,
    selected: previewSelected ?? instance.selected,
    highlighted: selectionExited || instance.highlighted,
    hovered: instance.hovered,
  });
}

/** 🔗️ An instance's effective highlight: the host's own catalog-kind hover OR the flag the guest stamped on the record
 * (an object the open time-travel draft references) — the host's hover never erases what the guest asked to show. */
export function worldInstanceHighlighted(hostHighlighted: boolean, instance: Pick<WorldInstanceRecord, "highlighted">): boolean {
  return hostHighlighted || instance.highlighted === true;
}

/** 🎨️ Slim alias over {@link MeshStylePalette} for call sites that only need the four legacy semantic colors (face/edge/vertex component overlays, markers). */
export type SemanticColors = {
  readonly mesh: string;
  readonly edge: string;
  readonly select: string;
  readonly hover: string;
  readonly edgeHover: string;
};

/** 🎨️ Maps mesh style palette fills/lines onto World3d semantic overlay colors — edge hover uses line paint so coplanar edges stay distinct from face hover fill.
 *
 * 🖱️ Component hover (a face/edge/vertex under the pointer) paints with the `highlighted` style — the
 * theme's SECONDARY token — not the `hovered` instance style: that one resolves to
 * `--hover-interactive-fill` = `--border-normal-color`, a grey a half-transparent overlay over the grey
 * mesh could not be told from the mesh at all (lowpoly face/vertex hover, 2026-09-18). Primary stays
 * selection, secondary is hover, and both are theme-owned. */
export function semanticColorsFromPalette(palette: MeshStylePalette): SemanticColors {
  return {
    mesh: palette.neutral.meshColor,
    edge: palette.neutral.lineColor,
    select: palette.selected.lineColor,
    hover: palette.highlighted.meshColor,
    edgeHover: palette.highlighted.lineColor,
  };
}
//#endregion WorldMeshPaint
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
