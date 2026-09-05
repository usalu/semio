import schema from "./📐️directory.schema.json";

const pattern = new RegExp(schema.pattern, "u");
const segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 🪪️Validates an authored installation basename and returns its folded sibling emoji identity. */
export function installationDirectoryEmoji(name: unknown): string {
  if (typeof name !== "string" || [...name].length > schema.maxLength || name !== name.normalize("NFC") || !pattern.test(name)) throw new Error("Installation directory requires one explicit non-generic emoji and a portable slug");
  return [...segmenter.segment(name)][0].segment.replaceAll("\uFE0F", "");
}

/** ⚠️Finds a file or directory sibling occupying the declared emoji identity. */
export function installationDirectoryCollision(name: string, siblings: readonly string[]): string | undefined {
  const emoji = installationDirectoryEmoji(name);
  return siblings.find((sibling) => [...segmenter.segment(sibling.normalize("NFC"))][0]?.segment.replaceAll("\uFE0F", "").replaceAll("\uFE0E", "") === emoji);
}
