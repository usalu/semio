import schema from "./🧬️schema/🔣️.json";

export type Thesis = Readonly<{
  language: "en" | "de";
  theme: "light" | "dark";
  title: string;
  author: string;
  subtitle?: string;
  date?: string;
  university?: string;
  faculty?: string;
  institute?: string;
  discipline?: string;
  awardedDegree?: string;
  supervisor?: string;
  reviewers?: string;
  submission?: string;
  defense?: string;
  place?: string;
  orcid?: string;
  programme?: string;
  studentId?: string;
  keywords?: string;
  declarationText?: string;
  declarationTitle?: string;
  dedicationTitle?: string;
  abbreviationsTitle?: string;
} & ({ degree: "bachelor" | "master" | "doctoral"; thesisType?: string } | { degree: "custom"; thesisType: string })>;

/** ✅️ Validates authored thesis metadata against the owned schema before TeX inference. */
export function isThesis(value: unknown): value is Thesis {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const record = value as Record<string, unknown>;
  if (schema.required.some(key => !Object.hasOwn(record, key))) return false;
  if (record.degree === schema.if.properties.degree.const && schema.then.required.some(key => !Object.hasOwn(record, key))) return false;
  return Object.entries(record).every(([key, item]) => {
    if (!Object.hasOwn(schema.properties, key) || typeof item !== "string") return false;
    const property = schema.properties[key as keyof typeof schema.properties];
    if ("enum" in property) return property.enum.includes(item as never);
    return [...item].length >= property.minLength && (!("pattern" in property) || new RegExp(property.pattern, "u").test(item));
  });
}

/** 🔤️ Encodes authored metadata as literal text without permitting TeX control sequences. */
export function escapeThesisText(value: string): string {
  const escapes: Record<string, string> = { "\\": "\\textbackslash{}", "{": "\\{", "}": "\\}", "%": "\\%", "&": "\\&", "_": "\\_", "#": "\\#", "$": "\\$", "~": "\\textasciitilde{}", "^": "\\textasciicircum{}" };
  return value.replace(/[\\{}%&_#$~^]/g, character => escapes[character]!);
}

/** 🖨️ Infers a reusable thesis preamble while leaving manuscript content with its author. */
export function thesisPreamble(thesis: Thesis): string {
  if (!isThesis(thesis)) throw new Error("Invalid thesis");
  const lines = [String.raw`\documentclass[type=thesis,theme=${thesis.theme},language=${thesis.language}]{semio}`];
  for (const key of ["title", "subtitle", "author", "date"] as const)
    if (thesis[key] !== undefined) lines.push("\\" + key + "{" + escapeThesisText(thesis[key]) + "}");
  const metadata = ["thesis-level={" + thesis.degree + "}"];
  for (const key of ["awardedDegree", "thesisType", "university", "faculty", "institute", "discipline", "supervisor", "reviewers", "submission", "defense", "place", "orcid", "programme", "studentId", "keywords", "declarationText", "declarationTitle", "dedicationTitle", "abbreviationsTitle"] as const)
    if (thesis[key] !== undefined) metadata.push((key === "awardedDegree" ? "degree" : key.replace(/[A-Z]/g, letter => "-" + letter.toLowerCase())) + "={" + escapeThesisText(thesis[key]) + "}");
  lines.push("\\SemioSetup{" + metadata.join(",") + "}");
  return lines.join("\n") + "\n";
}
