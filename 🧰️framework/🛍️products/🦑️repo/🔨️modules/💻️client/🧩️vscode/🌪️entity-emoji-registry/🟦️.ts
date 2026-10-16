/** 🌪️ Entity emoji registry
 *
 * The entity-emoji registry projected from the framework-owned entity-kind catalog and the id regex built from it.
 */

// #region 🔌️Adapters
import { ENTITY_KIND_BY_EMOJI } from "@semio-tech/schema-ts/generated/entity-kinds";
// #endregion 🔌️Adapters

// #region 🎞️Constants
// #region 🌪️Entity Emoji Registry
// Entity Emoji Registry MUST be projected from the framework-owned entity-kind catalog
// (`framework.schema` export `EntityKindCatalog`), never restated here.
// This registry drives CodeLens detection, gutter decorations, and ID parsing.
// The CLI's AllEntityEmojis() reads the same catalog through its generated Go projection.

/**
 * Complete set of entity-identifying emojis that appear as kind prefixes in entity IDs.
 * Projected from the framework-owned entity-kind catalog; the generated index is already FIRST-WINS,
 * so 🌱️ resolves to technology-mono and 📝️ to draft. Regex patterns are derived from it.
 **/
export const ENTITY_EMOJIS: ReadonlyMap<string, string> = new Map([...ENTITY_KIND_BY_EMOJI].map(([emoji, kind]) => [emoji, kind.id]));

/**
 * Escapes a string for safe use inside a regular expression character class or alternation.
 **/
function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * Builds a regex pattern that matches any entity emoji from the registry.
 * Longer emojis are tried first to prevent partial matches (e.g. 🧑️‍💻️ before 🧑️).
 **/
export function buildEntityEmojiPattern(): string {
  const emojis = Array.from(ENTITY_EMOJIS.keys());

  emojis.sort((a, b) => b.length - a.length);
  return emojis.map(escapeRegex).join("|");
}

/**
 * Regex that matches entity IDs in text.
 * Matches either:
 * 1. Markdown link: [<emoji-id>](repo://...)
 * 2. Bare reference: <emoji-id> (emoji followed by non-whitespace, non-delimiter characters)
 * Built dynamically from the ENTITY_EMOJIS registry.
 **/
export function buildEntityIdRegex(): RegExp {
  const emojiAlt = buildEntityEmojiPattern();

  //  Group 1+2: Markdown link [<id>](repo://uri) — group(1)=id, group(2)=uri
  //  Group 3: Bare emoji-prefixed ID (emoji followed by non-delimiter text)
  return new RegExp("(?:\\[((?:" + emojiAlt + ")[^\\]]+)\\]\\((repo:\\/\\/[^)]+)\\)|((?:" + emojiAlt + ")[^\\s/\"'\\[\\]()]+))", "gu");
}

/**
 * Compiled entity ID regex, built once from the registry.
 **/
export const ENTITY_ID_REGEX = buildEntityIdRegex();
// #endregion 🌪️Entity Emoji Registry
// #endregion 🎞️Constants
