import schema from "./🧬️schema/🔣️.json";

export type Assignment = Readonly<{
  language: "en" | "de";
  theme: "light" | "dark";
  title: string;
  subtitle?: string;
  author?: string;
  date?: string;
  course?: string;
  institution?: string;
  duration?: string;
  submission?: string;
}>;

/** ✅️ Validates the authored assignment declaration before any TeX is emitted. */
export function isAssignment(value: unknown): value is Assignment {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const record = value as Record<string, unknown>;
  if (schema.required.some(key => !Object.hasOwn(record, key))) return false;
  return Object.entries(record).every(([key, item]) => {
    if (!Object.hasOwn(schema.properties, key)) return false;
    const property = schema.properties[key as keyof typeof schema.properties];
    if (!property || typeof item !== "string") return false;
    if ("enum" in property) return property.enum.includes(item as never);
    return !("minLength" in property) || [...item].length >= property.minLength;
  });
}

/** 🔤️ Encodes plain authored metadata without allowing TeX control sequences. */
export function escapeAssignmentText(value: string): string {
  const escapes: Record<string, string> = { "\\": "\\textbackslash{}", "{": "\\{", "}": "\\}", "%": "\\%", "&": "\\&", "_": "\\_", "#": "\\#", "$": "\\$", "~": "\\textasciitilde{}", "^": "\\textasciicircum{}" };
  return value.replace(/[\\{}%&_#$~^]/g, character => escapes[character]!);
}

/** 🖨️ Infers the reusable assignment preamble while leaving domain content with its author. */
export function assignmentPreamble(assignment: Assignment): string {
  if (!isAssignment(assignment)) throw new Error("Invalid assignment");
  const lines = [String.raw`\documentclass[theme=${assignment.theme},language=${assignment.language}]{assignment}`];
  for (const key of ["title", "subtitle", "author", "date"] as const)
    if (assignment[key] !== undefined) lines.push("\\" + key + "{" + escapeAssignmentText(assignment[key]) + "}");
  const metadata = (["course", "institution", "duration", "submission"] as const)
    .filter(key => assignment[key] !== undefined)
    .map(key => key + "={" + escapeAssignmentText(assignment[key]!) + "}");
  if (metadata.length) lines.push("\\SemioAssignmentSetup{" + metadata.join(",") + "}");
  return lines.join("\n") + "\n";
}
