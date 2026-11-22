/** 🧲️ The introduction info box docks onto the border lines beneath it; every case is a row of the fixture the wgpu chrome reads too. */
import { describe, expect, it } from "vitest";
import cases from "../../../🧬️schema/📐️chrome-parity-constants/🧫️introduction-docking.json";
import constants from "../../../🧬️schema/📐️chrome-parity-constants/🧮️values.json";
import { INTRODUCTION_CLEAR_RANGE_PX, INTRODUCTION_DOCK_REACH_PX, INTRODUCTION_INFO_BOX_GAP_PX, clearIntroductionInfoBoxNearMisses, dockIntroductionInfoBox } from "./🟦️.ts";

describe("introduction chrome constants", () => {
  it("read the schema shared with the wgpu chrome", () => {
    expect(INTRODUCTION_INFO_BOX_GAP_PX).toBe(constants.introduction.infoBoxGapPx);
    expect(INTRODUCTION_DOCK_REACH_PX).toBe(constants.introduction.dockReachPx);
    expect(INTRODUCTION_CLEAR_RANGE_PX).toBe(constants.introduction.clearRangePx);
    expect(INTRODUCTION_INFO_BOX_GAP_PX).toBe(0);
  });
});

describe("dockIntroductionInfoBox", () => {
  it.each(cases.dock)("$id", (row) => {
    expect(dockIntroductionInfoBox(row.position, row.box, row.lines, row.reach)).toEqual(row.expected);
  });
});

describe("clearIntroductionInfoBoxNearMisses", () => {
  it.each(cases.clear)("$id", (row) => {
    expect(clearIntroductionInfoBoxNearMisses(row.position, row.own, row.lines, row.reach, row.range)).toEqual(row.expected);
  });
});
