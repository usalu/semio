/** 🔣️ Original UTF16 source span, independent of semantic interpretation. */
export interface EcmaSourceSpan {
  readonly text: string;
  readonly start: number;
  readonly end: number;
}

/** 🪙️ First-party lexical evidence shared by runtime and repository readers. */
export interface EcmaToken extends EcmaSourceSpan {
  readonly kind: "identifier" | "string" | "number" | "template" | "regex" | "punctuation" | "invalid" | "eof";
  readonly expressions?: readonly EcmaSourceSpan[];
}

/** 🪢️ One original destructured binding identity. */
export interface EcmaObjectBinding { readonly imported: string; readonly local: string; }
/** 🔗️ One original imported binding identity. */
export interface EcmaImportBinding { readonly local: string; readonly imported: string; readonly runtime: boolean; readonly module: string; }
/** 🍃️ One original expression property. */
export interface EcmaProperty { readonly key?: EcmaExpression; readonly value: EcmaExpression; readonly computed?: boolean; }
/** 🌱️ One declared binding and original initializer. */
export interface EcmaDeclaration { readonly pattern: EcmaPattern; readonly initializer: EcmaExpression; }

/** 🧶️ Original source binding pattern. */
export interface EcmaPattern {
  readonly start: number;
  readonly end: number;
  readonly names: readonly string[];
  readonly defaults: boolean;
  readonly destructured: boolean;
  readonly objectBindings?: readonly EcmaObjectBinding[];
}

/** 🌿️ First-party expression syntax with original UTF16 range. */
export interface EcmaExpression {
  readonly start: number;
  readonly end: number;
  readonly kind: string;
  readonly name?: string;
  readonly operator?: string;
  readonly value?: string;
  readonly object?: EcmaExpression;
  readonly property?: EcmaExpression | string;
  readonly optional?: boolean;
  readonly callee?: EcmaExpression;
  readonly arguments?: readonly EcmaExpression[];
  readonly elements?: readonly EcmaExpression[];
  readonly properties?: readonly EcmaProperty[];
  readonly left?: EcmaExpression;
  readonly right?: EcmaExpression;
  readonly condition?: EcmaExpression;
  readonly whenTrue?: EcmaExpression;
  readonly whenFalse?: EcmaExpression;
  readonly parameters?: readonly EcmaPattern[];
  readonly body?: EcmaExpression | readonly EcmaStatement[];
  readonly expressions?: readonly EcmaExpression[];
}

/** 🌳️ First-party statement syntax with original UTF16 range. */
export interface EcmaStatement {
  readonly start: number;
  readonly end: number;
  readonly kind: string;
  readonly imports?: readonly EcmaImportBinding[];
  readonly name?: string;
  readonly base?: EcmaExpression;
  readonly parameters?: readonly EcmaPattern[];
  readonly body?: readonly EcmaStatement[];
  readonly declarations?: readonly EcmaDeclaration[];
  readonly expression?: EcmaExpression;
  readonly then?: EcmaStatement;
  readonly otherwise?: EcmaStatement;
  readonly initializer?: EcmaPattern;
  readonly iterable?: EcmaExpression;
  readonly statement?: EcmaStatement;
}

