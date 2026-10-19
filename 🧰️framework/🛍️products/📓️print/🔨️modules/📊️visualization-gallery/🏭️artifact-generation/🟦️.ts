/** 🏭️ The generated visualization artifacts: the catalogue stylesheet and labels, the gallery documents, the generated file set and `generateVizArtifacts`. */
import { existsSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import { parseVizTaxonomy, vizOptionList, type LocalizedText, type VizCatalog } from "../../../🧬️schema/🟦️.ts";
import { GENERATED_HEADER, GENERATED_NOTE, loadVizCatalog, loadVizSchema, VIZ_GALLERY_DIR, VIZ_TAXONOMY_PATH } from "../🟦️.ts";
import { vizApiReference } from "../🧾️api-reference/🟦️.ts";

//#region 🔖️Generation
const SECTION_EMOJI: Readonly<Record<string, string>> = {
  "0": "🔺️", "1": "📊️", "2": "⏱️", "3": "🎻️", "4": "🔗️", "5": "🎛️", "6": "🍰️", "7": "🌳️", "8": "🕸️", "9": "🌊️",
  "10": "🗺️", "11": "🔥️", "12": "🧾️", "13": "🔤️", "14": "⚙️", "15": "🖥️", "16": "📋️", "17": "💹️", "18": "🎚️", "19": "📉️",
  "20": "🧭️", "21": "🧬️", "22": "🩺️", "23": "❓️", "24": "🎯️", "25": "📈️", "26": "📐️", "27": "➡️", "28": "🏔️", "29": "🧊️",
  "30": "⚛️", "31": "🔌️", "32": "📶️", "33": "⚗️", "34": "🌿️", "35": "🌦️", "36": "🔧️", "37": "🏛️", "38": "♾️", "39": "🫧️",
  "40": "🚦️", "41": "🎲️", "42": "🐟️", "43": "⚽️", "44": "🎼️", "45": "🎓️", "46": "📰️", "47": "🧱️", "48": "🧩️", "49": "🪞️",
  "50": "💬️", "51": "📏️", "52": "📍️", "53": "🏁️", "54": "📅️", "55": "🪑️", "56": "♟️", "57": "💻️", "58": "🛡️", "59": "🚚️",
  "60": "🔬️", "61": "🌀️", "62": "🏎️", "63": "👣️", "64": "⛓️", "65": "🛰️", "66": "🗄️", "67": "🧠️", "68": "🧪️", "69": "📝️",
  "70": "🗳️", "71": "🫂️", "72": "📖️", "73": "🎞️", "74": "🎨️", "75": "🧮️", "76": "🗂️", "77": "🪆️", "78": "🔄️", "79": "🏗️",
};

function texEscape(value: string): string {
  return value.replaceAll("\\", "\\textbackslash{}").replaceAll("&", "\\&").replaceAll("%", "\\%").replaceAll("$", "\\$")
    .replaceAll("#", "\\#").replaceAll("_", "\\_").replaceAll("{", "\\{").replaceAll("}", "\\}").replaceAll("~", "\\textasciitilde{}").replaceAll("^", "\\textasciicircum{}");
}

/** 🏗️ Renders `semio-viz-catalog.sty`: one registration and one cover list per chart kind. */
export function renderVizCatalogStylesheet(catalog: VizCatalog): string {
  const lines = [
    GENERATED_HEADER,
    "\\NeedsTeXFormat{LaTeX2e}",
    "\\ProvidesPackage{semio-viz-catalog}[2026/09/05 v0.3.0 semio viz chart-kind catalogue (generated)]",
    "\\RequirePackage{semio-viz-plot}",
    "",
    "%region 🔖️ChartKinds",
  ];
  for (const entry of catalog.kinds) lines.push(`\\SemioVizChartKind{${entry.slug}}{${entry.family}}{data=${entry.data},${vizOptionList(entry.options)}}`);
  lines.push("%endregion 🔖️ChartKinds", "", "%region 🔖️TaxonomyCoverage");
  for (const entry of catalog.kinds) lines.push(`\\SemioVizKindCovers{${entry.slug}}{${entry.covers.join(",")}}`);
  lines.push("%endregion 🔖️TaxonomyCoverage", "");
  return lines.join("\n");
}

/** 🏷️ Renders `semio-viz-catalog-labels.sty`: every chart-kind title in both document languages. */
export function renderVizCatalogLabels(catalog: VizCatalog): string {
  const lines = [
    GENERATED_HEADER,
    "\\NeedsTeXFormat{LaTeX2e}",
    "\\ProvidesPackage{semio-viz-catalog-labels}[2026/09/05 v0.3.0 semio viz localized chart-kind titles (generated)]",
    "\\RequirePackage{expl3}",
    "\\RequirePackage{xparse}",
    "\\RequirePackage{semio-core}",
    "",
    "\\ExplSyntaxOn",
    "",
    "%region 🔖️Registry",
    "\\msg_new:nnn { semio-viz } { unknown-kind-title }",
    "  { No~title~registered~for~viz~chart~kind~'#1'. }",
    "\\prop_new:N \\g_semio_viz_kind_title_en_prop",
    "\\prop_new:N \\g_semio_viz_kind_title_de_prop",
    "",
    "% 🏷️ Registers one chart kind's title in both document languages.",
    "\\cs_new_protected:Npn \\semio_viz_kind_title_define:nnn #1#2#3 {",
    "  \\prop_gput:Nnn \\g_semio_viz_kind_title_en_prop {#1} {#2}",
    "  \\prop_gput:Nnn \\g_semio_viz_kind_title_de_prop {#1} {#3}",
    "}",
    "",
    "% 🌍 Prints a chart kind's title in the document language; there is no default language.",
    "\\cs_new_protected:Npn \\semio_viz_kind_title_use:n #1 {",
    "  \\str_if_eq:VnTF \\l_semio_language_tl { de }",
    "    { \\prop_get:NnNTF \\g_semio_viz_kind_title_de_prop {#1} \\l_tmpa_tl { \\tl_use:N \\l_tmpa_tl } { \\msg_error:nnn { semio-viz } { unknown-kind-title } {#1} } }",
    "    { \\prop_get:NnNTF \\g_semio_viz_kind_title_en_prop {#1} \\l_tmpa_tl { \\tl_use:N \\l_tmpa_tl } { \\msg_error:nnn { semio-viz } { unknown-kind-title } {#1} } }",
    "}",
    "",
    "% 🏷️ \\SemioVizKindLabel{slug}{en}{de}: one generated title registration.",
    "\\NewDocumentCommand \\SemioVizKindLabel { m m m } { \\semio_viz_kind_title_define:nnn {#1} {#2} {#3} }",
    "% 🌍 \\SemioVizKindTitle{slug}: the chart kind's title in the document language.",
    "\\NewDocumentCommand \\SemioVizKindTitle { m } { \\semio_viz_kind_title_use:n {#1} }",
    "% 🌍 \\SemioVizLocalized{en}{de}: any generated heading in the document language.",
    "\\NewDocumentCommand \\SemioVizLocalized { m m } { \\str_if_eq:VnTF \\l_semio_language_tl { de } {#2} {#1} }",
    "%endregion 🔖️Registry",
    "",
    "\\ExplSyntaxOff",
    "",
    "%region 🔖️Titles",
    "% 🏷️ The titles are registered with expl3 syntax OFF: a title is running text, and under",
    "% \\ExplSyntaxOn a space is ignored, which silently welds every multi-word title together.",
  ];
  for (const entry of catalog.kinds) lines.push(`\\SemioVizKindLabel{${entry.slug}}{${texEscape(entry.title.en)}}{${texEscape(entry.title.de)}}`);
  lines.push("%endregion 🔖️Titles", "");
  return lines.join("\n");
}

/** 📄 Renders one gallery document: every leaf of one taxonomy section, drawn through its chart kind. */
export function renderVizGalleryDocument(sectionTitle: LocalizedText, leaves: readonly { readonly group: LocalizedText; readonly leafId: string; readonly slug: string }[]): string {
  const chapter = `\\SemioVizLocalized{${texEscape(sectionTitle.en)}}{${texEscape(sectionTitle.de)}}`;
  const lines = [
    GENERATED_HEADER,
    "\\documentclass[type=report,theme=light,language=de]{semio}",
    `\\title{${chapter}}`,
    "\\author{Semio}",
    "\\date{\\today}",
    "\\begin{document}",
    `\\chapter{${chapter}}`,
  ];
  let group = "";
  for (const leaf of leaves) {
    if (leaf.group.en !== group) {
      group = leaf.group.en;
      lines.push(`\\section{\\SemioVizLocalized{${texEscape(leaf.group.en)}}{${texEscape(leaf.group.de)}}}`);
    }
    lines.push(`% viz-covers: ${leaf.leafId}`);
    lines.push(`\\begin{VizFigure}[title={\\SemioVizKindTitle{${leaf.slug}}}, width=80]`);
    lines.push(`\\special{pdf:literal direct /SemioVizLeaf${Array.from(new TextEncoder().encode(leaf.leafId), byte => byte.toString(16).padStart(2, "0")).join("")} BMC}`);
    lines.push(`\\SemioVizChart{${leaf.slug}}`);
    lines.push("\\special{pdf:literal direct EMC}");
    lines.push("\\end{VizFigure}");
  }
  lines.push("\\end{document}", "");
  return lines.join("\n");
}

/** 🧾 Every file `generate viz` writes, as repository-relative paths with their content. */
export function vizGeneratedFiles(): readonly { readonly path: string; readonly content: string }[] {
  const catalog = loadVizCatalog();
  const schema = loadVizSchema();
  const md = readFileSync(VIZ_TAXONOMY_PATH, "utf8");
  const sections = new Map((schema["x-semio-taxonomy-sections"] as { section: string; title: LocalizedText }[]).map((entry) => [entry.section, entry.title] as const));
  const groups = new Map((schema["x-semio-taxonomy-groups"] as { title: LocalizedText }[]).map((entry) => [entry.title.en, entry.title] as const));
  const slugByLeaf = new Map<string, string>();
  for (const entry of catalog.kinds) for (const leaf of entry.covers) slugByLeaf.set(leaf, entry.slug);

  const files: { path: string; content: string }[] = [
    { path: "🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-catalog.sty", content: renderVizCatalogStylesheet(catalog) },
    { path: "🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-catalog-labels.sty", content: renderVizCatalogLabels(catalog) },
    { path: "🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-api.json", content: `${JSON.stringify({ _comment: GENERATED_NOTE, ...vizApiReference() }, null, 2)}\n` },
  ];
  const bySection = new Map<string, { group: LocalizedText; leafId: string; slug: string }[]>();
  for (const leaf of parseVizTaxonomy(md)) {
    const leafId = `${leaf.section}/${leaf.slug}`;
    const bucket = bySection.get(leaf.section) ?? [];
    bucket.push({ group: groups.get(leaf.group)!, leafId, slug: slugByLeaf.get(leafId)! });
    bySection.set(leaf.section, bucket);
  }
  for (const [section, leaves] of [...bySection].sort((left, right) => Number(left[0]) - Number(right[0]))) {
    files.push({
      path: `🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/${SECTION_EMOJI[section]}viz-${section}.tex`,
      content: renderVizGalleryDocument(sections.get(section)!, leaves),
    });
  }
  return files;
}

/** 🏗️ Writes the generated catalogue packages and gallery documents. */
export function generateVizArtifacts(repoRoot: string): readonly string[] {
  const files = vizGeneratedFiles();
  const written = new Set<string>();
  for (const file of files) {
    const path = join(repoRoot, file.path);
    if (!existsSync(path) || readFileSync(path, "utf8") !== file.content) writeFileSync(path, file.content, "utf8");
    written.add(basename(file.path));
  }
  for (const name of readdirSync(VIZ_GALLERY_DIR)) {
    if (written.has(name)) continue;
    rmSync(join(VIZ_GALLERY_DIR, name));
  }
  return files.map((file) => file.path);
}
//#endregion 🔖️Generation
