import { type FemDslStatement, parseFemDsl } from "../🔤️dsl-parser/🟦️.ts";

//#region 🔖️SnapshotModel
export type FemVec2 = readonly [number, number];

export type FemSupport = { readonly id: string; readonly nodeId: string; readonly fixed: readonly string[] };

export type FemMaterial = { readonly id: string; readonly name: string; readonly e: number; readonly nu: number; readonly rho: number };

export type FemSection = { readonly id: string; readonly name: string; readonly area: number; readonly iy: number };

export type FemAnalysisSettings = { readonly modalCount: number; readonly bucklingCount: number; readonly deformationScale: number };

/** 🏋️ One applied load, flattened across `FemLoad`'s three variants (`◻️2d/🦀️.rs`'s `Nodal`/`MemberUdl`/`Area`). */
export type FemLoad = {
  readonly kind: string;
  readonly id: string;
  readonly nodeId?: string;
  readonly dof?: string;
  readonly value?: number;
  readonly elementId?: string;
  readonly wx?: number;
  readonly wy?: number;
  readonly regionId?: string;
  readonly solidId?: string;
  readonly pressure?: number;
};

export type FemLoadCase = { readonly id: string; readonly name: string; readonly loads: readonly FemLoad[]; readonly selfWeight: boolean };

export type FemCombination = { readonly id: string; readonly name: string };

export type Fem2dNode = { readonly id: string; readonly x: number; readonly y: number };

export type Fem2dElement = { readonly kind: string; readonly id: string; readonly start: string; readonly end: string; readonly materialId: string; readonly sectionId: string };

export type Fem2dRegion = { readonly id: string; readonly name: string; readonly outline: readonly FemVec2[]; readonly holes: readonly FemVec2[]; readonly thickness: number; readonly materialId: string; readonly meshSize: number };

/** ◻️ The subset of `Fem2dSnapshot` (`🗿️artifacts/◻️2d/🦀️.rs`) the 2D stories render. */
export type Fem2dSnapshot = {
  readonly nodes: readonly Fem2dNode[];
  readonly elements: readonly Fem2dElement[];
  readonly supports: readonly FemSupport[];
  readonly regions: readonly Fem2dRegion[];
  readonly materials: readonly FemMaterial[];
  readonly sections: readonly FemSection[];
  readonly loadCases: readonly FemLoadCase[];
  readonly combinations: readonly FemCombination[];
  readonly analysis: FemAnalysisSettings;
};

export type Fem3dNode = { readonly id: string; readonly x: number; readonly y: number; readonly z: number };

export type Fem3dElement = { readonly kind: string; readonly id: string; readonly start: string; readonly end: string; readonly materialId: string; readonly sectionId: string; readonly roll: number };

export type Fem3dSolid = { readonly id: string; readonly name: string; readonly outline: readonly FemVec2[]; readonly holes: readonly FemVec2[]; readonly baseZ: number; readonly height: number; readonly layers: number; readonly meshSize: number; readonly materialId: string; readonly axis: "x" | "y" | "z" };

/** 🧊️ The subset of `Fem3dSnapshot` (`🗿️artifacts/🧊️3d/🦀️.rs`) the 3D stories render. */
export type Fem3dSnapshot = {
  readonly nodes: readonly Fem3dNode[];
  readonly elements: readonly Fem3dElement[];
  readonly supports: readonly FemSupport[];
  readonly solids: readonly Fem3dSolid[];
  readonly materials: readonly FemMaterial[];
  readonly sections: readonly FemSection[];
  readonly loadCases: readonly FemLoadCase[];
  readonly combinations: readonly FemCombination[];
  readonly analysis: FemAnalysisSettings;
};

//#endregion 🔖️SnapshotModel

//#region 🔖️Projections
function text(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function num(value: unknown, fallback = 0): number {
  return typeof value === "number" ? value : fallback;
}

function flag(value: unknown, fallback = false): boolean {
  return typeof value === "boolean" ? value : fallback;
}

function strings(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.map((entry) => String(entry)) : [];
}

function polygon(value: unknown): readonly FemVec2[] {
  return Array.isArray(value) ? value.filter((entry): entry is number[] => Array.isArray(entry) && entry.length >= 2 && entry.every((n) => typeof n === "number")).map((entry) => [entry[0]!, entry[1]!] as const) : [];
}

function analysis(statements: readonly FemDslStatement[] | undefined): FemAnalysisSettings {
  const fields = statements?.[0]?.fields ?? {};
  return { modalCount: num(fields.modalCount, 3), bucklingCount: num(fields.bucklingCount, 3), deformationScale: num(fields.deformationScale, 1) };
}

function supports(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemSupport[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), nodeId: text(row.nodeId), fixed: strings(row.fixed) }));
}

function materials(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemMaterial[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), e: num(row.e), nu: num(row.nu), rho: num(row.rho) }));
}

function sections(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemSection[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), area: num(row.area), iy: num(row.iy) }));
}

function combinations(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemCombination[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name) }));
}

/** 🏋️ A `loads:BLOCK` cell's statement run → the flattened `FemLoad` rows the structure layers draw vectors for. The `member-udl` keyword is accepted in both its DSL (`member-udl`) and camelCase spellings. */
function loads(cell: unknown): readonly FemLoad[] {
  if (!Array.isArray(cell)) return [];
  return (cell as readonly FemDslStatement[]).map((statement) => {
    const keyword = statement.keyword ?? "";
    const fields = statement.fields;
    return {
      kind: keyword === "member-udl" ? "memberUdl" : keyword,
      id: text(fields.id),
      ...(fields.nodeId === undefined ? {} : { nodeId: text(fields.nodeId) }),
      ...(fields.dof === undefined ? {} : { dof: text(fields.dof) }),
      ...(fields.value === undefined ? {} : { value: num(fields.value) }),
      ...(fields.elementId === undefined ? {} : { elementId: text(fields.elementId) }),
      ...(fields.wx === undefined ? {} : { wx: num(fields.wx) }),
      ...(fields.wy === undefined ? {} : { wy: num(fields.wy) }),
      ...(fields.regionId === undefined ? {} : { regionId: text(fields.regionId) }),
      ...(fields.solidId === undefined ? {} : { solidId: text(fields.solidId) }),
      ...(fields.pressure === undefined ? {} : { pressure: num(fields.pressure) }),
    };
  });
}

function loadCases(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemLoadCase[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), loads: loads(row.loads), selfWeight: flag(row.selfWeight) }));
}

/** ◻️ `fem.fem2d.dsl` → the `Fem2dSnapshot` slice the 2D stories render. */
export function parseFem2dDsl(dslText: string): Fem2dSnapshot {
  const doc = parseFemDsl(dslText);
  return {
    nodes: (doc.tables.nodes ?? []).map((row) => ({ id: text(row.id), x: num(row.x), y: num(row.y) })),
    elements: (doc.blocks.elements ?? []).map((statement) => ({
      kind: statement.keyword ?? "beam",
      id: text(statement.fields.id),
      start: text(statement.fields.start),
      end: text(statement.fields.end),
      materialId: text(statement.fields.materialId),
      sectionId: text(statement.fields.sectionId),
    })),
    supports: supports(doc.tables.supports),
    regions: (doc.tables.regions ?? []).map((row) => ({
      id: text(row.id),
      name: text(row.name),
      outline: polygon(row.outline),
      holes: polygon(row.holes),
      thickness: num(row.thickness),
      materialId: text(row.materialId),
      meshSize: num(row.meshSize, 1),
    })),
    materials: materials(doc.tables.materials),
    sections: sections(doc.tables.sections),
    loadCases: loadCases(doc.tables["load-cases"]),
    combinations: combinations(doc.tables.combinations),
    analysis: analysis(doc.blocks.analysis),
  };
}

/** 🧊️ `fem.fem3d.dsl` → the `Fem3dSnapshot` slice the 3D stories render. */
export function parseFem3dDsl(dslText: string): Fem3dSnapshot {
  const doc = parseFemDsl(dslText);
  return {
    nodes: (doc.tables.nodes ?? []).map((row) => ({ id: text(row.id), x: num(row.x), y: num(row.y), z: num(row.z) })),
    elements: (doc.blocks.elements ?? []).map((statement) => ({
      kind: statement.keyword ?? "frame",
      id: text(statement.fields.id),
      start: text(statement.fields.start),
      end: text(statement.fields.end),
      materialId: text(statement.fields.materialId),
      sectionId: text(statement.fields.sectionId),
      roll: num(statement.fields.roll),
    })),
    supports: supports(doc.tables.supports),
    solids: (doc.tables.solids ?? []).map((row) => ({
      id: text(row.id),
      name: text(row.name),
      outline: polygon(row.outline),
      holes: polygon(row.holes),
      baseZ: num(row.baseZ),
      height: num(row.height),
      layers: num(row.layers, 1),
      meshSize: num(row.meshSize, 1),
      materialId: text(row.materialId),
      axis: (text(row.axis) === "x" || text(row.axis) === "y" ? text(row.axis) : "z") as "x" | "y" | "z",
    })),
    materials: materials(doc.tables.materials),
    sections: sections(doc.tables.sections),
    loadCases: loadCases(doc.tables["load-cases"]),
    combinations: combinations(doc.tables.combinations),
    analysis: analysis(doc.blocks.analysis),
  };
}

/** 🕳️ `crate::artifacts::fem2d::schema::empty_fem2d_snapshot()`'s story twin — what `setActiveExample` loads for any id other than the bundled example's own. */
export const EMPTY_FEM2D_SNAPSHOT: Fem2dSnapshot = { nodes: [], elements: [], supports: [], regions: [], materials: [], sections: [], loadCases: [], combinations: [], analysis: { modalCount: 3, bucklingCount: 3, deformationScale: 1 } };

/** 🕳️ `Fem3dSnapshot::default()`'s story twin — what fem3d's `setActiveExample` loads for any id other than `"default"`. */
export const EMPTY_FEM3D_SNAPSHOT: Fem3dSnapshot = { nodes: [], elements: [], supports: [], solids: [], materials: [], sections: [], loadCases: [], combinations: [], analysis: { modalCount: 3, bucklingCount: 3, deformationScale: 1 } };

//#endregion 🔖️Projections
