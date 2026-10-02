/** 🧩️ Splits complete leading emoji graphemes while retaining the original remainder. */
const PATH_EMOJI_SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

export function leadingEmojiIdentity(value: string): Readonly<{ emoji: string; rest: string; first: string }> {
  let emoji = "", first = "";
  for (const { segment } of PATH_EMOJI_SEGMENTER.segment(value.normalize("NFC"))) {
    if (!/[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(segment)) break;
    if (!first) first = segment;
    emoji += segment;
  }
  return { emoji, rest: value.slice(emoji.length), first };
}
