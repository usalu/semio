/** 🏷️ The authoring-time law over every localized label of a manifest: each terminology row of a label carries a non-blank `en` and `de`. Twin of the Rust sweep in the stdio `shipped-fleet` test, driven by the shared `localized-labels` fixture. */

const isRecord = (value: unknown): value is Readonly<Record<string, unknown>> => typeof value === "object" && value !== null && !Array.isArray(value);

/** 🔎️ Whether an object is a localized label: non-empty, every value a locale map with string `en` and `de`. */
function isLocalizedLabelShape(value: Readonly<Record<string, unknown>>): boolean {
  const rows = Object.values(value);
  return rows.length > 0 && rows.every((row) => isRecord(row) && typeof row.en === "string" && typeof row.de === "string" && Object.values(row).every((cell) => typeof cell === "string"));
}

/** 🕳️ JSON-pointer-like paths of every localized label with a blank `en`/`de` cell. */
export function blankLocalizedLabelPathsV1(value: unknown, path = ""): readonly string[] {
  if (Array.isArray(value)) return value.flatMap((item, index) => blankLocalizedLabelPathsV1(item, `${path}[${index}]`));
  if (!isRecord(value)) return [];
  if (isLocalizedLabelShape(value)) return Object.values(value).some((row) => [(row as Record<string, string>).en, (row as Record<string, string>).de].some((cell) => cell!.trim() === "")) ? [path] : [];
  return Object.entries(value).flatMap(([key, child]) => blankLocalizedLabelPathsV1(child, `${path}/${key}`));
}
