/**
 * 🌳️ The TypeScript twin of `🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs`. Both read the SAME neutral
 * fixture (`🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json`): the Rust law drives the live wgpu arena and
 * proves the published row rects are the ones `events::hit_test` reads; this one drives the metric
 * the REACT `Tree` presents rows on — `domSizePx("treeRowUiSpacing")`, the `--size-workbench` its
 * rows carry as `h-workbench` — and re-derives every fixture rect from it with a second,
 * independent implementation of the stacking rule.
 *
 * The defect both sides pin: the wgpu layout measured a tree row as its own padding (6.4 px) while
 * the painter drew it 24 px tall, so a click on the painted `Add Generation` label fell outside the
 * rectangle the hit test read (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-tree-row-hit-test-2026-09-12.md`).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { domSizePx, STYLING_METRICS } from "@semio-tech/ui-styling";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const uiRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui");

type FixtureItem = { readonly id: string; readonly hidden?: boolean; readonly defaultOpen?: boolean; readonly items?: readonly FixtureItem[] };
type FixtureSection = { readonly id: string; readonly label?: string; readonly items?: readonly FixtureItem[] };
type FixtureRect = { readonly path: readonly string[]; readonly x: number; readonly y: number; readonly width: number; readonly height: number };
type FixtureCase = { readonly name: string; readonly treeWidth: number; readonly treeHeight: number; readonly tree: { readonly sections: readonly FixtureSection[] }; readonly rects: readonly FixtureRect[] };

const law = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🌳️tree-row-rects/🔣️.json"), "utf8")) as {
  readonly metrics: { readonly rowHeightPx: number; readonly uiSpacingCompactPx: number; readonly treeRowUiSpacing: number };
  readonly cases: readonly FixtureCase[];
};

/** 📏️ The React row metric, read the way `🧱️elements/🌳️Tree/🟦️.tsx` reads it. */
const rowHeightPx = domSizePx("treeRowUiSpacing");

/** 🌳️ One item row's own height: its row plus every nested row it reveals when expanded. */
const itemHeight = (item: FixtureItem): number => {
  if (item.hidden === true) return 0;
  if (item.defaultOpen !== true || item.items === undefined) return rowHeightPx;
  return item.items.reduce((total, nested) => total + itemHeight(nested), rowHeightPx);
};

const sectionHeaderHeight = (section: FixtureSection): number => (section.label === undefined ? 0 : rowHeightPx);

const sectionHeight = (section: FixtureSection): number => (section.items ?? []).reduce((total, item) => total + itemHeight(item), sectionHeaderHeight(section));

/** 📐️ Every row's rect in its own parent's space, keyed by the fixture's own path. */
const rowRects = (entry: FixtureCase): Map<string, { x: number; y: number; width: number; height: number }> => {
  const rects = new Map<string, { x: number; y: number; width: number; height: number }>();
  const width = entry.treeWidth;
  let sectionY = 0;
  for (const section of entry.tree.sections) {
    rects.set(section.id, { x: 0, y: sectionY, width, height: sectionHeight(section) });
    let cursor = sectionHeaderHeight(section);
    const place = (items: readonly FixtureItem[], prefix: string, offset: number): number => {
      let y = offset;
      for (const item of items) {
        if (item.hidden === true) continue;
        const path = `${prefix}/${item.id}`;
        rects.set(path, { x: 0, y, width, height: itemHeight(item) });
        if (item.defaultOpen === true && item.items !== undefined) place(item.items, path, rowHeightPx);
        else for (const unreached of item.items ?? []) rects.set(`${path}/${unreached.id}`, { x: 0, y: 0, width, height: 0 });
        y += itemHeight(item);
      }
      return y;
    };
    cursor = place(section.items ?? [], section.id, cursor);
    sectionY += cursor;
  }
  return rects;
};

describe("🌳️ the tree row metric is one number in every presentation", () => {
  it("reads React's row height off the same token the fixture was pinned against", () => {
    expect(rowHeightPx).toBe(law.metrics.rowHeightPx);
    expect(law.metrics.uiSpacingCompactPx * law.metrics.treeRowUiSpacing).toBeCloseTo(law.metrics.rowHeightPx, 6);
    expect(STYLING_METRICS.dom.treeRowUiSpacing).toBe(law.metrics.treeRowUiSpacing);
    console.log(`[DEBUG] tree-row-rects react rowHeightPx=${rowHeightPx}`);
  });

  it("keeps `--size-workbench` at 7.5 ui-spacings, the multiplier React's rows are sized by", () => {
    const css = readFileSync(resolve(uiRoot, "🎨️styling/🖌️ui/🎨️.css"), "utf8");
    expect(css).toContain("--size-small: calc(5 * var(--ui-spacing));");
    expect(css).toContain("--size-workbench: calc(1.5 * var(--size-small));");
  });

  it("sizes every Tree row shell on that metric and nothing else", () => {
    const tree = readFileSync(resolve(uiRoot, "🧱️elements/🌳️Tree/🟦️.tsx"), "utf8");
    expect(tree).toContain('const treeRowHeightPx = domSizePx("treeRowUiSpacing");');
    expect(tree).toContain('"relative h-workbench min-h-workbench max-h-workbench w-full min-w-0 select-none overflow-hidden"');
  });

  it("re-derives every fixture rect from React's own metric", () => {
    for (const entry of law.cases) {
      const rects = rowRects(entry);
      const total = entry.tree.sections.reduce((sum, section) => sum + sectionHeight(section), 0);
      expect(total, entry.name).toBeCloseTo(entry.treeHeight, 6);
      for (const expected of entry.rects) {
        const found = rects.get(expected.path.join("/"));
        expect(found, `${entry.name} ${expected.path.join("/")}`).toEqual({ x: expected.x, y: expected.y, width: expected.width, height: expected.height });
      }
      console.log(`[DEBUG] tree-row-rects twin case ${entry.name}: ${entry.rects.length} rows re-derived`);
    }
  });
});

describe("🧊️ the wgpu side takes the same metric from the same token", () => {
  const wgpuRoot = resolve(uiRoot, "🎯️targets/🧊️wgpu");

  it("derives `Theme::tree_row_height` from `dom::TREE_ROW_UI_SPACING`, never a literal", () => {
    const theme = readFileSync(resolve(wgpuRoot, "🎨️theme/🦀️.rs"), "utf8");
    expect(theme).toContain("tree_row_height: chrome_px(dom::TREE_ROW_UI_SPACING),");
    const widgets = readFileSync(resolve(wgpuRoot, "🪀️widgets/🦀️.rs"), "utf8");
    expect(widgets).toContain("pub(crate) const TREE_ROW_HEIGHT: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_ROW_UI_SPACING) as f32;");
  });

  it("leaves layout as the single writer of tree row geometry", () => {
    const paint = readFileSync(resolve(wgpuRoot, "🖌️paint/🦀️.rs"), "utf8");
    expect(paint).not.toContain("const TREE_ROW_HEIGHT: f32 = 24.0;");
    expect(paint).toContain("let metrics = TreeRowMetrics::from_theme(theme);");
    const mounted = readFileSync(resolve(wgpuRoot, "📌️mounted_layout/🦀️.rs"), "utf8");
    expect(mounted).toContain("let height = tree_item_height(item, metrics);");
    expect(mounted).toContain("LayoutNodeKind::TreeRow { row: if expanded { metrics.row_height } else { 0.0 }, height, expanded }");
  });
});
