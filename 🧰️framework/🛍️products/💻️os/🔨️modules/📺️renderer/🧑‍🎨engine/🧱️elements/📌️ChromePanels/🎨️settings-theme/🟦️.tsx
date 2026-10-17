/** 🎨️ Settings chrome panel, Theme tab: the colour, spacing, geometry and number input rows, the contrast badge, the appearance groups and the theme settings tree. */
import { useState, type ReactElement } from "react";
import { Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, type ThemeAppearanceName, type ThemePaletteGroup, type TreeDataItem, type TreePanelConfig, type UiTranslationKey, borderElementClass, cn, resolveThemeAppearancePalettes } from "@semio-tech/ui-react";
import { resolveThemeGeometry, themeCompactSpacingUi, themeGeometryMetricDescription, WCAG_AA_CONTRAST, themePaintContrastPairs, type Rgba8, type WcagContrastGrade } from "@semio-tech/ui-styling";
import { shellLabel, shellLabelLocale } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { SettingsHostApi } from "../⚙️settings-general/🟦️.tsx";

//#region 🔖️os-chrome-panels
//#region SettingsPanel
function rgba8ToHex(rgba: readonly [number, number, number, number]): string {
  const [r, g, b] = rgba;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

function themeColorInputRow(id: string, label: string, hex: string, onChange: (hex: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <input id={id} type="color" className={cn(borderElementClass, "h-small w-16 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => onChange(event.target.value)} />,
  };
}

export function themeTextInputRow(id: string, label: string, value: string, onCommit: (value: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <Input id={id} lazy value={value} onLazyChange={onCommit} className="h-small w-32" />,
  };
}

/** 📏️ Keeps invalid geometry edits local and explains the supported shared length contract. */
function ThemeCompactSpacingInput({ id, host }: { id: string; host: SettingsHostApi }): ReactElement {
  const [invalid, setInvalid] = useState(false);
  const ui = themeCompactSpacingUi(host.locale);
  return <div className="flex min-w-0 flex-col">
    <Input id={id} lazy value={host.theme.spacing.compact!} onLazyChange={(compact) => {
      try { resolveThemeGeometry({ spacing: { ...host.theme.spacing, compact }, metrics: host.theme.metrics }); }
      catch { setInvalid(true); return; }
      setInvalid(false);
      host.setThemeSpacing("compact", compact);
    }} aria-invalid={invalid} aria-describedby={`${id}.format`} className="h-small w-32" />
    <p id={`${id}.format`} role={invalid ? "alert" : undefined} className="text-xs">{ui.description}</p>
  </div>;
}

/** 🔢️ Geometry metrics report invalid staged values without publishing them. */
function ThemeGeometryMetricInput({ id, host, section, name }: { id: string; host: SettingsHostApi; section: string; name: string }): ReactElement {
  const [invalid, setInvalid] = useState(false);
  const description = themeGeometryMetricDescription(section, name, host.locale);
  return <div className="flex min-w-0 flex-col">
    <Input id={id} lazy value={String(host.theme.metrics[section]![name])} onLazyChange={(text) => {
      const value = text.trim() ? Number(text) : NaN;
      try { resolveThemeGeometry({ spacing: host.theme.spacing, metrics: { ...host.theme.metrics, [section]: { ...host.theme.metrics[section], [name]: value } } }); }
      catch { setInvalid(true); return; }
      setInvalid(false);
      host.setThemeMetric(section, name, value);
    }} aria-invalid={invalid} aria-describedby={`${id}.format`} className="h-small w-32" />
    <p id={`${id}.format`} role={invalid ? "alert" : undefined} className="text-xs">{description}</p>
  </div>;
}

export function themeNumberInputRow(id: string, label: string, value: number | number[], onCommit: (value: number | number[]) => void): TreeDataItem {
  const text = Array.isArray(value) ? value.join(", ") : String(value);
  return {
    id,
    label,
    control: (
      <Input
        id={id}
        lazy
        value={text}
        onLazyChange={(value) => {
          const raw = value.trim();
          if (raw.includes(",")) {
            const parts = raw
              .split(",")
              .map((part) => Number.parseFloat(part.trim()))
              .filter((n) => !Number.isNaN(n));
            if (parts.length) onCommit(parts);
            return;
          }
          const n = Number.parseFloat(raw);
          if (!Number.isNaN(n)) onCommit(n);
        }}
        className="h-small w-32"
      />
    ),
  };
}

export function themeAlphaInput(id: string, alpha: number, onCommit: (alpha: number) => void): ReactElement {
  return (
    <Input
      id={id}
      lazy
      value={alpha.toFixed(2)}
      onLazyChange={(value) => {
        const nextAlpha = Number.parseFloat(value);
        if (!Number.isNaN(nextAlpha)) onCommit(Math.min(1, Math.max(0, nextAlpha)));
      }}
      className="h-small w-14 shrink-0"
    />
  );
}

const THEME_PALETTE_GROUP_LABEL_KEYS = {
  board: "ui.settings.theme.group.board",
  map: "ui.settings.theme.group.map",
  canvas: "ui.settings.theme.group.canvas",
  chrome: "ui.settings.theme.group.chrome",
  outcome: "ui.settings.theme.group.outcome",
  diagram: "ui.settings.theme.group.diagram",
} as const satisfies Record<ThemePaletteGroup, UiTranslationKey>;

const THEME_CONTRAST_GRADE_LABEL_KEYS = {
  aaa: "ui.settings.theme.contrast.aaa",
  aa: "ui.settings.theme.contrast.aa",
  aaLarge: "ui.settings.theme.contrast.aaLarge",
  fail: "ui.settings.theme.contrast.fail",
} as const satisfies Record<WcagContrastGrade, UiTranslationKey>;

/** ♿️ Live WCAG 2.2 verdict for ONE appearance paint: the WORST text-on-surface pair it takes part in
 * (`themePaintContrastPairs`), as the badge text, its grade, and — below AA — the inline warning sentence that
 * names the other paint of the pair. `null` for a paint in no pair (borders, non-chrome groups). The generated
 * default palette's surfaces are gated ≥ AA by `🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts`; a hand-built
 * theme has no such gate, so the editor says the ratio out loud. */
export function themeContrastBadgeText(
  palette: Readonly<Record<string, Rgba8>>,
  paintKey: string,
  gradeLabel: (grade: WcagContrastGrade) => string,
  formatRatio: (ratio: number) => string,
  warningLabel: (options: { readonly ratio: string; readonly minimum: string; readonly counterpart: string }) => string,
): { readonly text: string; readonly grade: WcagContrastGrade; readonly passesBodyText: boolean; readonly ratio: number; readonly counterpart: string; readonly warning: string | null } | null {
  const worst = themePaintContrastPairs(palette, paintKey)[0];
  if (!worst) return null;
  const ratio = formatRatio(worst.ratio);
  return {
    text: `${ratio}:1 · ${gradeLabel(worst.grade)}`,
    grade: worst.grade,
    passesBodyText: worst.passesBodyText,
    ratio: worst.ratio,
    counterpart: worst.counterpart,
    warning: worst.passesBodyText ? null : warningLabel({ ratio, minimum: formatRatio(WCAG_AA_CONTRAST), counterpart: worst.counterpart }),
  };
}

/** 🔢️ Prints a contrast ratio with two decimals in the ACTIVE shell locale (`4.50` / `4,50`) — the
 * ratio a user reads beside the swatch and the one a screen reader speaks are the same localized number. */
export function themeContrastRatioFormatter(locale: string): (ratio: number) => string {
  const format = new Intl.NumberFormat(locale ? [locale] : [], { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  return (ratio) => format.format(ratio);
}

function buildThemeAppearanceGroupItems(host: SettingsHostApi, appearance: ThemeAppearanceName, group: ThemePaletteGroup): TreeDataItem[] {
  const refs = host.theme.appearances[appearance][group];
  const resolved = resolveThemeAppearancePalettes(host.theme, appearance)[group];
  const formatRatio = themeContrastRatioFormatter(shellLabelLocale());
  return Object.keys(refs)
    .sort()
    .map((paintKey) => {
      const rgba = resolved[paintKey] ?? [0, 0, 0, 255];
      const hex = rgba8ToHex(rgba);
      const alpha = rgba[3] / 255;
      const rowId = `framework.settings.theme.appearances.${appearance}.${group}.${paintKey}`;
      const contrast = themeContrastBadgeText(resolved, paintKey, (grade) => shellLabel(THEME_CONTRAST_GRADE_LABEL_KEYS[grade]), formatRatio, (options) => shellLabel("ui.settings.theme.contrast.warning", options));
      const describedBy = contrast ? [`${rowId}.contrast`, ...(contrast.warning ? [`${rowId}.contrast-warning`] : [])].join(" ") : undefined;
      return {
        id: rowId,
        label: paintKey,
        control: (
          <div className="flex w-full flex-col gap-single">
            <div className="flex w-full items-center gap-single">
              <input
                type="color"
                className={cn(borderElementClass, "h-small w-10 shrink-0 rounded border bg-background")}
                value={hex}
                aria-label={paintKey}
                aria-describedby={describedBy}
                aria-invalid={contrast?.warning ? true : undefined}
                onChange={(event) => host.setThemeAppearancePaint(appearance, group, paintKey, event.target.value, alpha)}
              />
              {themeAlphaInput(`${rowId}.alpha`, alpha, (nextAlpha) => host.setThemeAppearancePaint(appearance, group, paintKey, hex, nextAlpha))}
              {contrast ? (
                <span
                  id={`${rowId}.contrast`}
                  data-contrast-grade={contrast.grade}
                  data-contrast-ratio={contrast.ratio}
                  data-contrast-counterpart={contrast.counterpart}
                  className={cn("shrink-0 text-xs tabular-nums", contrast.passesBodyText ? "text-muted-foreground" : "text-destructive")}
                  title={`${shellLabel("ui.settings.theme.contrast.label")}: ${contrast.text}`}
                  aria-label={`${shellLabel("ui.settings.theme.contrast.label")}: ${contrast.text}`}
                >
                  {contrast.text}
                </span>
              ) : null}
            </div>
            {contrast?.warning ? (
              <p id={`${rowId}.contrast-warning`} role="alert" data-contrast-warning="" className="text-xs text-destructive">
                {contrast.warning}
              </p>
            ) : null}
          </div>
        ),
      } satisfies TreeDataItem;
    });
}

export function buildSettingsThemeTree(host: SettingsHostApi): TreePanelConfig {
  const colorItems = Object.keys(host.theme.colors)
    .sort()
    .map((key) => themeColorInputRow(`framework.settings.theme.colors.${key}`, key, host.theme.colors[key]!, (hex) => host.setThemeColor(key, hex)));

  const spacingItems = Object.keys(host.theme.spacing)
    .sort()
    .map((key) => key === "compact" ? {
      id: `framework.settings.theme.spacing.${key}`,
      label: themeCompactSpacingUi(host.locale).label,
      control: <ThemeCompactSpacingInput id={`framework.settings.theme.spacing.${key}`} host={host} />,
    } : themeTextInputRow(`framework.settings.theme.spacing.${key}`, key, host.theme.spacing[key]!, (value) => host.setThemeSpacing(key, value)));

  const fontItems = Object.keys(host.theme.fontStacks)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.fonts.${key}`, key, host.theme.fontStacks[key]!, (value) => host.setThemeFontStack(key, value)));

  const strokeItems = Object.keys(host.theme.strokes)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.strokes.${key}`, key, host.theme.strokes[key]!, (value) => host.setThemeStroke(key, value)));

  const radiusItems = Object.keys(host.theme.radii)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.radii.${key}`, key, host.theme.radii[key]!, (value) => host.setThemeRadius(key, typeof value === "number" ? value : value[0]!)));

  const opacityItems = Object.keys(host.theme.opacities)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.opacities.${key}`, key, host.theme.opacities[key]!, (value) => host.setThemeOpacity(key, typeof value === "number" ? value : value[0]!)));

  const metricSections = Object.keys(host.theme.metrics)
    .sort()
    .map(
      (section): TreeDataItem => ({
        id: `framework.settings.theme.metrics.${section}`,
        label: section,
        defaultOpen: false,
        items: Object.keys(host.theme.metrics[section]!)
          .sort()
          .map((key) => themeGeometryMetricDescription(section, key, host.locale) ? {
            id: `framework.settings.theme.metrics.${section}.${key}`,
            label: key,
            control: <ThemeGeometryMetricInput id={`framework.settings.theme.metrics.${section}.${key}`} host={host} section={section} name={key} />,
          } : themeNumberInputRow(`framework.settings.theme.metrics.${section}.${key}`, key, host.theme.metrics[section]![key]!, (value) => host.setThemeMetric(section, key, value))),
      }),
    );

  const appearanceGroups: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome", "outcome", "diagram"];
  const appearanceItems: TreeDataItem[] = (["light", "dark"] as const).map((appearance) => ({
    id: `framework.settings.theme.appearances.${appearance}`,
    label: shellLabel(appearance === "light" ? "ui.settings.theme.appearance.light" : "ui.settings.theme.appearance.dark"),
    defaultOpen: false,
    items: appearanceGroups.map((group) => ({
      id: `framework.settings.theme.appearances.${appearance}.${group}`,
      label: shellLabel(THEME_PALETTE_GROUP_LABEL_KEYS[group]),
      defaultOpen: false,
      items: buildThemeAppearanceGroupItems(host, appearance, group),
    })),
  }));

  return {
    sections: [
      {
        id: "framework.settings.theme.select",
        label: `${shellLabel("ui.settings.theme.select")}${host.themeDirty ? ` (${shellLabel("ui.settings.theme.dirty")})` : ""}`,
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: shellLabel("ui.settings.theme.select"),
            control: (
              <Select id="framework.settings.theme.select" value={host.themeId} onValueChange={(value) => host.setThemeId(value)}>
                <SelectTrigger id="framework.settings.theme.select" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.themes.map((theme) => (
                    <SelectItem key={theme.id} value={theme.id}>
                      {theme.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input id="framework.settings.theme.saveLabel" value={host.themeSaveLabel} onChange={(event) => host.setThemeSaveLabel(event.target.value)} placeholder={shellLabel("ui.settings.theme.savePlaceholder")} className="h-small w-32" />
            ),
          },
          {
            id: "framework.settings.theme.save.action",
            label: shellLabel("ui.settings.theme.save"),
            control: (
              <Button icon="save"
                id="framework.settings.theme.save"
                text={shellLabel("ui.settings.theme.save")}
                disabled={!host.themeSaveLabel.trim()}
                onClick={() => {
                  const label = host.themeSaveLabel.trim();
                  if (!label) return;
                  host.saveTheme(label);
                  host.setThemeSaveLabel("");
                }}
              />
            ),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: shellLabel("ui.settings.theme.reset"),
            control: <Button icon="rotate-ccw" id="framework.settings.theme.reset" text={shellLabel("ui.settings.theme.reset")} disabled={!host.themeDirty && host.themeId === "semio"} onClick={() => host.resetTheme()} />,
          },
          {
            id: "framework.settings.theme.export.action",
            label: shellLabel("ui.settings.theme.export"),
            control: <Button icon="export" id="framework.settings.theme.export" text={shellLabel("ui.settings.theme.export")} onClick={() => host.exportTheme()} />,
          },
          {
            id: "framework.settings.theme.import.action",
            label: shellLabel("ui.settings.theme.import"),
            control: <Button icon="import" id="framework.settings.theme.import" text={shellLabel("ui.settings.theme.import")} onClick={() => host.importTheme()} />,
          },
          ...(host.themeId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.theme.delete.action",
                  label: shellLabel("ui.settings.theme.delete"),
                  control: <Button icon="trash-2" id="framework.settings.theme.delete" text={shellLabel("ui.settings.theme.delete")} onClick={() => host.deleteTheme(host.themeId)} />,
                },
              ]
            : []),
        ],
      },
      { id: "framework.settings.theme.colors", label: shellLabel("ui.settings.theme.colors"), defaultOpen: false, items: colorItems },
      { id: "framework.settings.theme.spacing", label: shellLabel("ui.settings.theme.spacing"), defaultOpen: false, items: spacingItems },
      { id: "framework.settings.theme.fonts", label: shellLabel("ui.settings.theme.fonts"), defaultOpen: false, items: fontItems },
      { id: "framework.settings.theme.strokes", label: shellLabel("ui.settings.theme.strokes"), defaultOpen: false, items: strokeItems },
      { id: "framework.settings.theme.radii", label: shellLabel("ui.settings.theme.radii"), defaultOpen: false, items: radiusItems },
      { id: "framework.settings.theme.opacities", label: shellLabel("ui.settings.theme.opacities"), defaultOpen: false, items: opacityItems },
      { id: "framework.settings.theme.metrics", label: shellLabel("ui.settings.theme.metrics"), defaultOpen: false, items: metricSections },
      { id: "framework.settings.theme.appearances", label: shellLabel("ui.settings.theme.appearances"), defaultOpen: false, items: appearanceItems },
    ],
  };
}
//#endregion SettingsPanel
//#endregion 🔖️os-chrome-panels
