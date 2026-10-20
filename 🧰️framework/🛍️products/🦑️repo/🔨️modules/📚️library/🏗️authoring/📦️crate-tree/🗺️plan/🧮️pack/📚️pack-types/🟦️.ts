/** 📚️ The shared types of the artifact crate pack planner: the crate under construction and the moved root item unit. */
import type { SliceInfo } from "../../🧾️model/🟦️.ts";
import { MeasureUnion } from "../../../📏️measure-union/🟦️.ts";

/** 📦️ One crate under construction: the carve components it holds, their slice infos and the measured lib and test unions. */
export type PackedGroup = { components: number[]; infos: SliceInfo[]; lib: MeasureUnion; test: MeasureUnion };

/** 🚚️ One moved root item unit: its names, exports, source blocks and the crate that hosts it. */
export type MovedUnit = { index: number; names: Set<string>; exports: Set<string>; blocks: { start: number; end: number }[]; text: string; module: string; edgeSlices: Set<string>; shimRefs: Set<string>; dependsOn: Set<number>; crossNames: Map<number, string[]>; host: number };
