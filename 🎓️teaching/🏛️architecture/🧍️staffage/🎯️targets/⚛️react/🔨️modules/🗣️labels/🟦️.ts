/** 🗣️ Bilingual words of the web renderer: view names and the spoken description of a figure; there is no default language.
 *
 * @see ../🧍️figure/🟦️.tsx — the `aria-label` and the value table
 */
import { VIEWS, decimalsFor, formatNumber } from "@teaching/staffage";
import type { DrawingScene, EvaluationQuery, ItemValue, Language, Text, View } from "@teaching/staffage";

/** 🖼️ The seven view names. */
export const VIEW_NAMES: Readonly<Record<View, Text>> = {
  "dark-line": { en: "dark line drawing", de: "dunkle Strichzeichnung" },
  "dark-solid": { en: "dark solid", de: "dunkler Vollton" },
  "light-line": { en: "light line drawing", de: "helle Strichzeichnung" },
  "light-solid": { en: "light solid", de: "heller Vollton" },
  thermal: { en: "thermal view", de: "Thermikansicht" },
  forces: { en: "force view", de: "Kraftansicht" },
  moments: { en: "moment view", de: "Momentenansicht" },
};

/** 🖼️ Whether a string names a view. */
export const isView = (value: string | null | undefined): value is View => (VIEWS as readonly string[]).includes(value ?? "");

const WORDS = {
  pose: { en: "pose", de: "Pose" },
  clip: { en: "clip", de: "Clip", },
  transition: { en: "transition", de: "Übergang" },
  at: { en: "at", de: "bei" },
  scale: { en: "scale", de: "Maßstab" },
  values: { en: "Analytic values", de: "Analysewerte" },
  quantity: { en: "Quantity", de: "Größe" },
  value: { en: "Value", de: "Wert" },
  item: { en: "Item", de: "Element" },
} as const satisfies Record<string, Text>;

/** 🗣️ A word of the table and label vocabulary. */
export const word = (key: keyof typeof WORDS, language: Language): string => WORDS[key][language];

/** 🕰️ A query in words: "pose standing", "clip walk at 0.50 s". */
export function describeQuery(query: EvaluationQuery, language: Language): string {
  const seconds = (timeS: number): string => `${formatNumber(timeS, 2, language)} s`;
  if (query.kind === "pose") return `${WORDS.pose[language]} ${query.pose}`;
  if (query.kind === "clip") return `${WORDS.clip[language]} ${query.clip} ${WORDS.at[language]} ${seconds(query.timeS)}`;
  return `${WORDS.transition[language]} ${query.from} → ${query.to} ${WORDS.at[language]} ${seconds(query.timeS)}`;
}

/** 🗣️ The `aria-label` of a figure: asset, query, view and scale. */
export function describeScene(scene: DrawingScene, language: Language): string {
  const { metadata } = scene;
  return `${metadata.title[language]}, ${describeQuery(metadata.query, language)}, ${VIEW_NAMES[metadata.view][language]}, ${WORDS.scale[language]} 1:${metadata.scale.denominator}`;
}

/** 🔢️ A value with its unit in the reader's number format. */
export function valueText(value: ItemValue, language: Language): string {
  return `${formatNumber(value.value, decimalsFor(Math.abs(value.value), 3), language)} ${value.unit}`;
}

/** 🧾️ The distinct valued items of a scene, in drawing order. */
export function valuedItems(scene: DrawingScene): { readonly id: string; readonly value: ItemValue; readonly bone?: string; readonly region?: string }[] {
  const seen = new Set<string>();
  const out: { id: string; value: ItemValue; bone?: string; region?: string }[] = [];
  for (const layer of scene.layers)
    for (const item of layer.items) {
      if (item.kind !== "path" || item.value === undefined || seen.has(item.id)) continue;
      seen.add(item.id);
      out.push({ id: item.id, value: item.value, ...(item.bone === undefined ? {} : { bone: item.bone }), ...(item.region === undefined ? {} : { region: item.region }) });
    }
  return out;
}
