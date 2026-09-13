/** 📽️ TypeScript, Go, and Rust entity-kind catalog renderers. */
import { entityCatalogProvenance, type EntityCatalogSource } from "../📥️source/🟦️.ts";

//#region 🔖️emit-typescript
export function emitTypeScript(source: EntityCatalogSource): string {
  const [origin, refresh] = entityCatalogProvenance(source);
  const rows = source.kinds.map((kind) => `\t{ id: ${JSON.stringify(kind.id)}, emoji: ${JSON.stringify(kind.emoji)}, iconId: ${JSON.stringify(kind.iconId)}, label: ${JSON.stringify(kind.label)}, filterable: ${kind.filterable} },`).join("\n");
  return `/** ${origin}
 * ${refresh} */
import type { EntityKind, EntityKindCatalog } from "../../🟦️";

export type { EntityKind, EntityKindCatalog };

/** 📚️ The entity-kind catalog in declaration order — the TypeScript projection of
 * \`🏷️entity-kinds/🔣️.json\`. Order is contract: the emoji index below keeps the FIRST entry. */
export const ENTITY_KINDS: EntityKindCatalog = [
${rows}
];

/** 🔍️ FIRST-WINS emoji index: when two kinds share one emoji the first declared kind wins, so 🌱️
 * resolves to \`technology-mono\` (not \`interaction-started\`) and 📝️ to \`draft\` (not \`todo\`).
 * \`new Map(ENTITY_KINDS.map((kind) => [kind.emoji, kind]))\` would be LAST-wins. */
export const ENTITY_KIND_BY_EMOJI: ReadonlyMap<string, EntityKind> = ((): ReadonlyMap<string, EntityKind> => {
\tconst index = new Map<string, EntityKind>();
\tfor (const kind of ENTITY_KINDS) if (!index.has(kind.emoji)) index.set(kind.emoji, kind);
\treturn index;
})();

/** 🔎️ Resolves one emoji prefix to its entity kind through {@link ENTITY_KIND_BY_EMOJI}. */
export function entityKindByEmoji(emoji: string): EntityKind | undefined {
\treturn ENTITY_KIND_BY_EMOJI.get(emoji);
}
`;
}
//#endregion 🔖️emit-typescript

//#region 🔖️emit-go
export function emitGo(source: EntityCatalogSource): string {
  const [origin, refresh] = entityCatalogProvenance(source);
  const rows = source.kinds.map((kind) => `\t{ID: ${JSON.stringify(kind.id)}, Emoji: ${JSON.stringify(kind.emoji)}, IconID: ${JSON.stringify(kind.iconId)}, Label: ${JSON.stringify(kind.label)}, Filterable: ${kind.filterable}},`).join("\n");
  return `// ${origin}
// ${refresh}

package client

// 🏷️EntityKind holds one entry of the entity-kind catalog (kind → emoji/icon/label/filterable).
type EntityKind struct {
\tID         string
\tEmoji      string
\tIconID     string
\tLabel      string
\tFilterable bool
}

// EntityKindCatalog is the entity-kind catalog in declaration order — the Go projection of
// 🏷️entity-kinds/🔣️.json. Named distinctly from the pre-existing EntityKinds (tree-node category
// strings) elsewhere in this package.
var EntityKindCatalog = []EntityKind{
${rows}
}

// entityKindsByEmoji is the FIRST-WINS emoji index: when two kinds share one emoji the first
// declared kind wins, so 🌱️ resolves to technology-mono and 📝️ to draft.
var entityKindsByEmoji = func() map[string]EntityKind {
\tindex := make(map[string]EntityKind, len(EntityKindCatalog))
\tfor _, kind := range EntityKindCatalog {
\t\tif _, taken := index[kind.Emoji]; !taken {
\t\t\tindex[kind.Emoji] = kind
\t\t}
\t}
\treturn index
}()

// EntityKindByEmoji resolves one emoji prefix to its entity kind through the first-wins index.
func EntityKindByEmoji(emoji string) (EntityKind, bool) {
\tkind, found := entityKindsByEmoji[emoji]
\treturn kind, found
}
`;
}
//#endregion 🔖️emit-go

//#region 🔖️emit-rust
export function emitRust(source: EntityCatalogSource): string {
  const [origin, refresh] = entityCatalogProvenance(source);
  const rows = source.kinds.map((kind) => `\tEntityKind { id: ${JSON.stringify(kind.id)}, emoji: ${JSON.stringify(kind.emoji)}, icon_id: ${JSON.stringify(kind.iconId)}, label: ${JSON.stringify(kind.label)}, filterable: ${kind.filterable} },`).join("\n");
  return `// ${origin}
// ${refresh}

/// 🏷️ One entry of the entity-kind catalog — the Rust projection of \`🔣️.json#/$defs/EntityKind\`.
pub struct EntityKind {
    pub id: &'static str,
    pub emoji: &'static str,
    pub icon_id: &'static str,
    pub label: &'static str,
    pub filterable: bool,
}

/// 📚️ The entity-kind catalog in declaration order — the Rust projection of \`🏷️entity-kinds/🔣️.json\`.
pub const ENTITY_KINDS: &[EntityKind] = &[
${rows}
];

/// 🔍️ Looks up an entity kind by its emoji prefix, FIRST-WINS: when two kinds share one emoji the
/// first declared kind wins, so 🌱️ resolves to \`technology-mono\` (not \`interaction-started\`) and
/// 📝️ to \`draft\` (not \`todo\`) — the same rule the TypeScript and Go projections apply.
pub fn entity_kind_by_emoji(emoji: &str) -> Option<&'static EntityKind> {
    ENTITY_KINDS.iter().find(|kind| kind.emoji == emoji)
}
`;
}
//#endregion 🔖️emit-rust
