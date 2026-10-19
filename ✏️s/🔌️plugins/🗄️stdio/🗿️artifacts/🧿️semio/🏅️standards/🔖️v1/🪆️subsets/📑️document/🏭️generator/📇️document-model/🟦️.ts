/** 📇️ The semio document model every document recipe is written against: runs, styles, blocks and their builders. */

/** ✍️ One styled run of literal text. `underline` survives only into docx, `link` only into md — the
 *  asymmetry is the carriers', and every recipe below is written knowing which of the two can see it. */
export type Run = Readonly<{ text: string; bold?: boolean; italic?: boolean; underline?: boolean; link?: string }>;

/** 🎨️ One named paragraph style. WordprocessingML is the only carrier that has such a table at all. */
export type Style = Readonly<{ id: string; name: string; basedOn?: string }>;

/** 🧱 The block vocabulary this subset's serializers map from, mirrored exactly so a fixture exercises
 *  the same shapes production exports. */
export type Block =
  | { kind: "paragraph"; style?: string; runs: readonly Run[] }
  | { kind: "heading"; level: number; style?: string; runs: readonly Run[] }
  | { kind: "list"; ordered: boolean; items: readonly (readonly Block[])[] }
  | { kind: "code"; language?: string; text: string }
  | { kind: "quote"; blocks: readonly Block[] }
  | { kind: "image"; imageId: string; alt: string }
  | { kind: "page-break" };

/** 📄️ One whole document, of which each carrier encodes a different part. */
export type Doc = Readonly<{ styles: readonly Style[]; blocks: readonly Block[] }>;

/** 👪️ The families the corpus is sharded and reported by — never at artifact level. */
export type Family = "blocks" | "runs" | "styles";

/** 🧪️ One corpus entry. A recipe DESCRIBES two documents; it computes nothing and predicts nothing.
 *  `carriers` names only the carriers that actually ENCODE what this mutation writes — a carrier that
 *  cannot see the change is left out rather than given a fixture that would prove nothing.
 *  `counterexample` is a deliberately WRONG after, committed alongside the right one so the reading
 *  gate is provably two-sided. */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  mutation: string;
  property: string;
  carriers: readonly ("docx" | "md")[];
  notes: string;
  before: Doc;
  after: Doc;
  counterexample?: Doc;
}>;

/** 🧱 A plain paragraph of one unstyled run — the shape most recipes are built from. */
export function para(text: string, style?: string): Block {
  return { kind: "paragraph", runs: [{ text }], ...(style === undefined ? {} : { style }) };
}

/** 🔢️ A heading with no explicit style, so its level travels through docx's own `HeadingN` convention. */
export function heading(level: number, text: string): Block {
  return { kind: "heading", level, runs: [{ text }] };
}

/** 🎨️ One named style row. */
export function style(id: string, name: string, basedOn?: string): Style {
  return { id, name, ...(basedOn === undefined ? {} : { basedOn }) };
}

/** 📄️ The document most recipes mutate: two real styles and three blocks, all flat, so both carriers
 *  recover the same block-text sequence and the cross-family agreement check is meaningful. */
export function baseDoc(): Doc {
  return {
    styles: [style("Body", "Body Text"), style("Heading1", "Heading 1")],
    blocks: [heading(1, "The Report Title"), para("The body paragraph."), para("The closing paragraph.")],
  };
}

/** 🧬️ A structural copy with the block list rewritten by `edit`. */
export function withBlocks(doc: Doc, edit: (blocks: Block[]) => Block[]): Doc {
  return { styles: doc.styles, blocks: edit([...doc.blocks]) };
}

/** 🎨️ A structural copy with the style table rewritten by `edit`. */
export function withStyles(doc: Doc, edit: (styles: Style[]) => Style[]): Doc {
  return { styles: edit([...doc.styles]), blocks: doc.blocks };
}
