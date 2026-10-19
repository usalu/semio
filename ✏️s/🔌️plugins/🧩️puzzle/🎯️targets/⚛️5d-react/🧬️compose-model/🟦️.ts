/** 🧬️ The unified 5d compose model and the flat record it is composed from. */

/** ⚓️ Part root plane policy aligned with {@link https://github.com/usalu/semio/blob/main/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs} puzzle5d schema. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🧱️ Merged 5d part for topology flatten (sketchpad `2d` / `3d` presentation keys). */
export type Puzzle5dComposePart = {
  readonly id: string;
  readonly anchor?: Puzzle5dPartAnchor;
  readonly partKind?: string;
  readonly "2d": Record<string, unknown>;
  readonly "3d": Record<string, unknown>;
  readonly grips?: readonly Record<string, unknown>[];
};

/** 🔗️ Fastener row with eight transform params + diagram offsets. */
export type Puzzle5dComposeFastener = {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly fastenerKind?: string;
  readonly gap?: number;
  readonly shift?: number;
  readonly rise?: number;
  readonly rotation?: number;
  readonly turn?: number;
  readonly tilt?: number;
  readonly x?: number;
  readonly y?: number;
};

/** 🧩 Unified compose model produced by {@link compose5d}. */
export type Puzzle5dComposeModel = {
  readonly schema?: string;
  readonly domain?: string;
  readonly parts: Puzzle5dComposePart[];
  readonly fasteners: Puzzle5dComposeFastener[];
};

export type FlatRecord = Record<string, unknown>;
