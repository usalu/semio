/** 🧍️ Two-foot stance of a person-like rig: the feet share the load by the lever rule, a hand load flips which foot carries more, and the shoulder moment is load × perpendicular lever arm.
 * @see ./🐍️.py — the anastruct oracle that writes `🧫️fixtures/🧍️two-foot-stance/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🧍️two-foot-stance");
const values = doc.oracle.values as Record<string, Record<string, number | string>>;
const solved = Object.fromEntries(doc.scenarios.map((sc) => [sc.id, solveStructure(buildState(doc, sc))]));
const foot = (id: string, side: string) => solved[id]!.reactions.find((r) => r.anchor === `foot-${side}`)!;

describe("two-foot stance against anastruct", () => {
  for (const sc of doc.scenarios.filter((x) => !values[x.id]!["liftOff"])) {
    const v = values[sc.id]!;
    const result = solved[sc.id]!;
    it(`shares ${sc.id} by the lever rule`, () => {
      expect(near(foot(sc.id, "left").fyN, v["reactionLeftN"] as number, 1e-7)).toBe(true);
      expect(near(foot(sc.id, "right").fyN, v["reactionRightN"] as number, 1e-7)).toBe(true);
      expect(near(foot(sc.id, "left").pressurePa!, v["pressureLeftPa"] as number, 1e-7)).toBe(true);
      expect(near(foot(sc.id, "right").pressurePa!, v["pressureRightPa"] as number, 1e-7)).toBe(true);
      expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * 1000);
      expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 1000);
    });
    it(`transmits the oracle shoulder load in ${sc.id}`, () => {
      const joint = result.joints["arm"]!;
      expect(near(joint.momentNm, v["shoulderMomentNm"] as number, 1e-9, 1e-9)).toBe(true);
      expect(near(joint.axialN, v["shoulderAxialN"] as number, 1e-9, 1e-9)).toBe(true);
      expect(near(joint.shearN, v["shoulderShearN"] as number, 1e-9, 1e-9)).toBe(true);
      expect(near(joint.fxN, v["shoulderFxN"] as number, 1e-9, 1e-9)).toBe(true);
      expect(near(joint.fyN, v["shoulderFyN"] as number, 1e-9, 1e-9)).toBe(true);
    });
  }
  it("loads the foot on the side of the centre of mass more", () => {
    expect(foot("unloaded", "right").fyN).toBeGreaterThan(foot("unloaded", "left").fyN);
    expect(values["unloaded"]!["heavierFoot"]).toBe("right");
  });
  it("flips the heavier foot when the hand load is carried on the other side", () => {
    expect(foot("reach-right", "right").fyN).toBeGreaterThan(foot("reach-right", "left").fyN);
    expect(foot("reach-left", "left").fyN).toBeGreaterThan(foot("reach-left", "right").fyN);
    expect(values["reach-left"]!["heavierFoot"]).toBe("left");
  });
  it("lifts the far foot off and lets the sole centre of pressure carry the moment", () => {
    const result = solved["reach-far-right"]!;
    const v = values["reach-far-right"]!;
    expect(v["liftOff"]).toBe(true);
    expect(foot("reach-far-right", "left").active).toBe(false);
    expect(foot("reach-far-right", "left").fyN).toBe(0);
    expect(foot("reach-far-right", "right").active).toBe(true);
    expect(result.notes.some((n) => n.en.includes("lifted off"))).toBe(true);
    expect(foot("reach-far-right", "right").cop!.x).toBeLessThan(0.15 + 0.125);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 1000);
  });
  it("gives the shoulder a load × perpendicular lever arm moment, CCW positive", () => {
    const sc = doc.scenarios.find((s) => s.id === "reach-left")!;
    const hand = sc.loads![0]!;
    const shoulder = solved["reach-left"]!.joints["arm"]!.at;
    const arm = doc.bones.find((b) => b.id === "arm")!.mass!.kg;
    const handArm = hand.at.x - shoulder.x;
    const armCom = solved["reach-left"]!.segmentWeights.filter((f) => f.bone === "arm").reduce((a, f) => a + f.fyN * (f.at.x - shoulder.x), 0);
    expect(near(solved["reach-left"]!.joints["arm"]!.momentNm, hand.fyN * handArm + armCom, 1e-9)).toBe(true);
    expect(solved["reach-left"]!.joints["arm"]!.momentNm).toBeGreaterThan(0);
    expect(solved["reach-right"]!.joints["arm"]!.momentNm).toBeLessThan(0);
    expect(Math.abs(solved["unloaded"]!.joints["arm"]!.momentNm)).toBeLessThan(1e-9);
    expect(arm).toBe(4);
  });
  it("keeps every joint load consistent with the subtree resultant", () => {
    const result = solved["reach-left"]!;
    const trunk = result.joints["trunk"]!;
    const kids = ["head", "arm"].map((id) => result.joints[id]!);
    const own = result.segmentWeights.filter((f) => f.bone === "trunk");
    expect(near(trunk.fyN, kids.reduce((a, j) => a + j.fyN, 0) - own.reduce((a, f) => a + f.fyN, 0), 1e-9)).toBe(true);
  });
  it("states that anatomical joint values are simplified, not clinical", () => {
    const notes = solved["unloaded"]!.notes;
    expect(notes.some((n) => n.en.includes("not clinically validated"))).toBe(true);
    expect(notes.some((n) => n.de.includes("nicht klinisch validierte") || n.de.includes("keine klinisch validierten"))).toBe(true);
  });
});
