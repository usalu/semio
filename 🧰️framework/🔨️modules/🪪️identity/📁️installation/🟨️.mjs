import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🔁️ Binds admission to an owner-read schema snapshot for persistent process revision loading. */
export function installationDirectoryParserV1(contract) {
  if (!contract || contract.type !== "string" || typeof contract.pattern !== "string" || !Number.isSafeInteger(contract.maxLength) || contract.maxLength < 1) throw new Error("Invalid installation directory schema snapshot");
  const pattern = new RegExp(contract.pattern, "u"), limit = contract.maxLength;
  return value => {
    if (typeof value !== "string" || value.length > limit * 2 || [...value].length > limit || value !== value.normalize("NFC") || !pattern.test(value)) throw new Error("Installation directory requires one explicit non-generic emoji and a portable slug");
    return value;
  };
}
const admit = installationDirectoryParserV1(schema);
const segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 🪪️ Admits one NFC physical installation identity under the shared schema authority. */
export function parseInstallationDirectoryV1(value) {
  return admit(value);
}

/** 🧩️ Derives the admitted installation's first grapheme identity without presentation selectors. */
export function installationDirectoryEmoji(value) {
  return [...segmenter.segment(parseInstallationDirectoryV1(value))][0].segment.replaceAll("\uFE0F", "");
}

/** ⚠️ Finds a file or directory sibling occupying the admitted emoji identity. */
export function installationDirectoryCollision(name, siblings) {
  const emoji = installationDirectoryEmoji(name);
  return siblings.find(sibling => [...segmenter.segment(sibling.normalize("NFC"))][0]?.segment.replaceAll("\uFE0F", "").replaceAll("\uFE0E", "") === emoji);
}
