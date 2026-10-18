const EMOJI_PRESENTATION_SELECTOR_RE = /[\uFE0E\uFE0F]/g;

/** 🎨️Normalizes emoji text and emoji presentation variants for stable identity comparisons. */
export function normalizeEmojiPresentation(value: string): string {
  return value.replace(EMOJI_PRESENTATION_SELECTOR_RE, "");
}
