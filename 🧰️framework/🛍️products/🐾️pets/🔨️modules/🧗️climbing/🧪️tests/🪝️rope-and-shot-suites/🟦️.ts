/** 🪝️ Unit suite of the climbing module, the grappling rope: shots, the hook flight, the haul and what a shot holds. */
import { describe, expect, it } from "vitest";
import { type Perch, type Rect, type Size } from "../../../../🧬️schema/🟦️.ts";
import { segmentHits } from "../../../🏞️terrain/🟦️.ts";
import { smoothstep } from "../../../📐️trigonometry/🟦️.ts";
import { REEL_CAP, REEL_LEAST, REEL_RAMP, REEL_SPEED, reelStep } from "../../../🪢️swing/🟦️.ts";
import { HOOK_INSET, HOOK_LIFT, HOOK_RETURN, HOOK_SPEED, MANTLE_INSET, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_ELEVATION, ROPE_FOLLOW, ROPE_LONG, ROPE_MARGIN, ROPE_MISS_OVERSHOOT, ROPE_RISE, ROPE_SHORT, ZIP_RAMP, ZIP_SLANT, ZIP_SPEED, haulOf, haulStep, haulTicks, hookStep, hookTicks, landingFor, missOf, shotFor, shotHolds, swayStep, zipStep } from "../../🟦️.ts";
import { along, FLOOR, keepout, LARGE, perch, PET, type Shot, SMALL, solid, STAGES, stream } from "../🧰️climbing-test-kit/🟦️.ts";

describe("shotFor", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const box = solid(300, 500, 380, 420);
  const from = (x: number, perches: readonly Perch[] = [FLOOR, shelf], keepouts: readonly Rect[] = [box], size: Size = PET) => shotFor({ x, y: 480 }, perches, keepouts, size);

  it("aims from the muzzle in front of the feet at a point just above the corner of the edge", () => {
    const shot = from(280)!;
    expect(shot.surface).toBe("shelf");
    expect(shot.facing).toBe(1);
    expect(shot.muzzle).toEqual({ x: 280 + MUZZLE_FORWARD * 40, y: 480 - MUZZLE_HEIGHT * 48 });
    expect(shot.hook).toEqual({ x: 306 + HOOK_INSET, y: 380 - HOOK_LIFT });
    expect(Math.abs(shot.length - Math.hypot(shot.hook.x - shot.muzzle.x, shot.hook.y - shot.muzzle.y))).toBeLessThan(1e-12);
    const mirrored = from(520)!;
    expect(mirrored.facing).toBe(-1);
    expect(mirrored.muzzle).toEqual({ x: 520 - MUZZLE_FORWARD * 40, y: 480 - MUZZLE_HEIGHT * 48 });
    expect(mirrored.hook).toEqual({ x: 494 - HOOK_INSET, y: 379 });
    expect(mirrored.length).toBeCloseTo(shot.length, 9);
  });

  it("takes the point of the edge nearest to the feet when that is nearer than both corners, and the middle of an edge too short for two", () => {
    expect(from(400)!.hook).toEqual({ x: 400, y: 379 });
    expect(from(400)!.facing).toBe(1);
    expect(from(400)!.reel).toBe("zip");
    expect(from(440)!.hook).toEqual({ x: 440, y: 379 });
    expect(from(311)!.hook).toEqual({ x: 310, y: 379 });
    expect(from(311)!.facing).toBe(-1);
    expect(from(309)!.hook).toEqual({ x: 310, y: 379 });
    expect(shotFor({ x: 400, y: 480 }, [perch("stub", 380, 420, 380)], [], PET)).toMatchObject({ hook: { x: 384, y: 379 }, facing: -1 });
    expect(shotFor({ x: 300, y: 480 }, [perch("stub", 380, 420, 380)], [], PET)).toMatchObject({ hook: { x: 384, y: 379 }, facing: 1 });
    expect(shotFor({ x: 400, y: 480 }, [perch("pin", 398, 404, 380)], [], PET)!.hook).toEqual({ x: 401, y: 379 });
  });

  it("reels a steep rope straight in and swings on a slanted one — unless the swing would carry it into a keep-out under the hook", () => {
    const steep = from(310)!;
    const slanted = from(240)!;
    expect(Math.abs(steep.hook.x - steep.muzzle.x)).toBeLessThanOrEqual(ZIP_SLANT * (steep.muzzle.y - steep.hook.y));
    expect(steep.reel).toBe("zip");
    expect(Math.abs(slanted.hook.x - slanted.muzzle.x)).toBeGreaterThan(ZIP_SLANT * (slanted.muzzle.y - slanted.hook.y));
    expect(slanted.reel).toBe("swing");
    const drop = slanted.hook.y + slanted.length;
    expect(from(240, [FLOOR, shelf], [box, keepout(296, drop - 10, 208, 20)])).toEqual({ ...slanted, reel: "zip" });
    expect(from(240, [FLOOR, shelf], [box, keepout(296, drop + ROPE_MARGIN + 0.5, 208, 20)])).toEqual(slanted);
    expect(from(240, [FLOOR, shelf], [box, keepout(slanted.hook.x + ROPE_MARGIN + 0.5, 430, 20, 20)])).toEqual(slanted);
  });

  it("needs a perch high enough above the feet, a rope of the right length and a rope steep enough", () => {
    const high = (y: number, size: Size = PET) => shotFor({ x: 400, y: 480 }, [perch("stub", 380, 420, y)], [], size);
    expect(high(480 - ROPE_RISE * 48 + 0.5)).toBeNull();
    expect(high(413)).not.toBeNull();
    expect(high(413)!.length).toBeGreaterThanOrEqual(ROPE_SHORT * 48);
    expect(high(414)).toBeNull();
    expect(high(66)).not.toBeNull();
    expect(high(66)!.length).toBeLessThanOrEqual(ROPE_LONG * 48);
    expect(high(65)).toBeNull();
    expect(high(480 - MUZZLE_HEIGHT * 56 - 420, LARGE)).not.toBeNull();
    expect(high(480 - MUZZLE_HEIGHT * 56 - 420, PET)).toBeNull();
    expect(high(480, PET)).toBeNull();
    expect(high(520, PET)).toBeNull();
    expect(from(240)).not.toBeNull();
    expect(from(140)).toBeNull();
    const flat = shotFor({ x: 100, y: 420 }, [perch("shelf", 180, 494, 380)], [], PET);
    expect(flat).toBeNull();
    for (const x of [200, 240, 280, 330, 400, 470, 520, 560]) {
      const shot = from(x);
      if (shot === null) continue;
      expect(shot.length >= ROPE_SHORT * 48 && shot.length <= ROPE_LONG * 48).toBe(true);
      expect(shot.muzzle.y - shot.hook.y).toBeGreaterThanOrEqual(ROPE_ELEVATION * shot.length);
    }
  });

  it("passes over what lies under its hook and is blocked by anything else on its line", () => {
    const clear = from(280)!;
    expect(segmentHits(clear.muzzle, clear.hook, box, ROPE_MARGIN)).toBe(true);
    expect(from(280, [FLOOR, shelf], [box, keepout(310, 380, 100, 30)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(310, 384, 100, 30)])).toBeNull();
    expect(from(280, [FLOOR, shelf], [box, keepout(300, 420, 16, 30)])).toBeNull();
    expect(from(280, [FLOOR, shelf], [box, keepout(312, 340, 100, 80)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(311.5, 340, 100, 80)])).toBeNull();
    const reach = along(clear.muzzle, clear.hook, 422);
    expect(from(280, [FLOOR, shelf], [box, keepout(reach - ROPE_MARGIN - 0.5 - 20, 400, 20, 20)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(reach - ROPE_MARGIN + 0.5 - 20, 400, 20, 20)])).toBeNull();
  });

  it("chooses the shot with the least rope and detour among all perches, the first one among equals", () => {
    const upper = perch("upper", 306, 494, 350);
    expect(from(400, [FLOOR, upper, shelf], [])!.surface).toBe("shelf");
    expect(from(400, [FLOOR, shelf, upper], [])!.surface).toBe("shelf");
    expect(from(400, [upper], [])!.surface).toBe("upper");
    expect(from(400, [upper, shelf])!.surface).toBe("shelf");
    expect(from(400, [upper])).toBeNull();
    const twin = perch("twin", 306, 494, 380);
    expect(from(400, [twin, shelf], [])!.surface).toBe("twin");
    expect(from(400, [shelf, twin], [])!.surface).toBe("shelf");
    expect(from(480, [perch("left", 306, 400, 380), perch("right", 560, 654, 380)], [])!.surface).toBe("left");
    expect(from(480, [perch("right", 560, 654, 380), perch("left", 306, 400, 380)], [])!.surface).toBe("right");
    expect(from(481, [perch("left", 306, 400, 380), perch("right", 560, 654, 380)], [])!.surface).toBe("right");
    expect(from(400, [], [])).toBeNull();
    expect(from(400, [FLOOR], [])).toBeNull();
  });

  it("keeps every law on generated stages", () => {
    const next = stream(43);
    let granted = 0;
    let refused = 0;
    let swings = 0;
    for (let stage = 0; stage < STAGES; stage++) {
      const size = [PET, SMALL, LARGE][next(3)]!;
      const feet = { x: 100 + next(1600) / 4, y: 300 + next(800) / 4 };
      const perches = Array.from({ length: 1 + next(3) }, (_, index) => {
        const x0 = next(2000) / 4;
        return perch(`p${index}`, x0, x0 + 20 + next(800) / 4, 100 + next(1400) / 4);
      });
      const keepouts = [...perches.map((edge) => solid(edge.x0, edge.x1, edge.y, edge.y + 40)), ...Array.from({ length: next(3) }, () => keepout(next(2400) / 4, 100 + next(1600) / 4, next(240) / 4, next(240) / 4))];
      const shot = shotFor(feet, perches, keepouts, size);
      if (shot === null) {
        refused++;
        continue;
      }
      granted++;
      if (shot.reel === "swing") swings++;
      const label = `stage ${stage}`;
      const target = perches.find((edge) => edge.surface === shot.surface)!;
      expect(feet.y - target.y, label).toBeGreaterThanOrEqual(ROPE_RISE * size.height);
      expect(shot.hook.y, label).toBe(target.y - HOOK_LIFT);
      expect(target.x1 - target.x0 >= 2 * HOOK_INSET ? shot.hook.x >= target.x0 + HOOK_INSET && shot.hook.x <= target.x1 - HOOK_INSET : shot.hook.x === (target.x0 + target.x1) / 2, label).toBe(true);
      expect(shot.muzzle, label).toEqual({ x: feet.x + shot.facing * MUZZLE_FORWARD * size.width, y: feet.y - MUZZLE_HEIGHT * size.height });
      expect(shot.facing, label).toBe(shot.hook.x < feet.x ? -1 : 1);
      expect(Math.abs(shot.length - Math.hypot(shot.hook.x - shot.muzzle.x, shot.hook.y - shot.muzzle.y)), label).toBeLessThan(1e-9);
      expect(shot.length >= ROPE_SHORT * size.height && shot.length <= ROPE_LONG * size.height, label).toBe(true);
      expect(shot.muzzle.y - shot.hook.y, label).toBeGreaterThanOrEqual(ROPE_ELEVATION * shot.length);
      let dropped = false;
      for (const blocker of keepouts) {
        const under = blocker.width > 0 && blocker.height > 0 && blocker.x <= shot.hook.x && shot.hook.x <= blocker.x + blocker.width && blocker.y <= target.y && target.y <= blocker.y + blocker.height;
        if (!under) expect(segmentHits(shot.muzzle, shot.hook, blocker, ROPE_MARGIN), label).toBe(false);
        if (!under && segmentHits(shot.hook, { x: shot.hook.x, y: shot.hook.y + shot.length }, blocker, ROPE_MARGIN)) dropped = true;
      }
      expect(shot.reel, label).toBe(Math.abs(shot.hook.x - shot.muzzle.x) <= ZIP_SLANT * (shot.muzzle.y - shot.hook.y) || dropped ? "zip" : "swing");
      expect(shotHolds(shot, perches, keepouts), label).toEqual(shot);
    }
    expect(granted).toBeGreaterThan(STAGES / 12);
    expect(swings).toBeGreaterThan(STAGES / 60);
    expect(refused).toBeGreaterThan(STAGES / 4);
  });
});

describe("the hook", () => {
  const muzzle = { x: 292, y: 448.8 };
  const hook = { x: 330, y: 379 };
  const length = Math.hypot(38, 69.8);

  it("flies on a straight line at a constant speed and ends on its target exactly", () => {
    const ticks = hookTicks(muzzle, hook, HOOK_SPEED);
    expect(ticks).toBe(Math.ceil(length / 10));
    expect(hookStep(muzzle, hook, HOOK_SPEED, 0)).toEqual(muzzle);
    expect(hookStep(muzzle, hook, HOOK_SPEED, -2)).toEqual(muzzle);
    for (let tick = 1; tick < ticks; tick++) {
      const tip = hookStep(muzzle, hook, HOOK_SPEED, tick);
      expect(Math.abs(Math.hypot(tip.x - muzzle.x, tip.y - muzzle.y) - 10 * tick), `tick ${tick}`).toBeLessThan(1e-9);
      expect(Math.abs((tip.x - muzzle.x) * (hook.y - muzzle.y) - (tip.y - muzzle.y) * (hook.x - muzzle.x)), `tick ${tick}`).toBeLessThan(1e-9);
    }
    expect(hookStep(muzzle, hook, HOOK_SPEED, ticks)).toEqual(hook);
    expect(hookStep(muzzle, hook, HOOK_SPEED, ticks + 5)).toEqual(hook);
  });

  it("comes back twice as fast after a miss, from wherever it was", () => {
    const tip = hookStep(muzzle, hook, HOOK_SPEED, 5);
    const ticks = hookTicks(tip, muzzle, HOOK_RETURN);
    expect(ticks).toBe(3);
    expect(Math.hypot(hookStep(tip, muzzle, HOOK_RETURN, 1).x - tip.x, hookStep(tip, muzzle, HOOK_RETURN, 1).y - tip.y)).toBeCloseTo(20, 9);
    expect(hookStep(tip, muzzle, HOOK_RETURN, ticks)).toEqual(muzzle);
  });

  it("needs no tick for no distance and one tick for anything up to a step", () => {
    expect(hookTicks(muzzle, muzzle, HOOK_SPEED)).toBe(0);
    expect(hookStep(muzzle, muzzle, HOOK_SPEED, 0)).toEqual(muzzle);
    expect(hookStep(muzzle, muzzle, HOOK_SPEED, 1)).toEqual(muzzle);
    expect(hookTicks({ x: 0, y: 0 }, { x: 6, y: 8 }, HOOK_SPEED)).toBe(1);
    expect(hookTicks({ x: 0, y: 0 }, { x: 6, y: 8.5 }, HOOK_SPEED)).toBe(2);
    expect(hookTicks({ x: 0, y: 0 }, { x: 60, y: 80 }, HOOK_SPEED)).toBe(10);
    expect(hookStep({ x: 0, y: 0 }, { x: 60, y: 80 }, HOOK_SPEED, 9)).toEqual({ x: 54, y: 72 });
  });
});

describe("the haul", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const shot: Shot = shotFor({ x: 400, y: 520 }, [shelf], [], PET)!;

  it("begins at rest where the actor stood when the hook bit, its feet under its hands as they stood under the muzzle", () => {
    expect(shot.reel).toBe("zip");
    const start = haulOf(shot, shot.length, PET);
    expect(start.rope).toBe(shot.length);
    expect(start.before).toEqual(start.hand);
    expect(Math.hypot(start.hand.x - shot.muzzle.x, start.hand.y - shot.muzzle.y)).toBeLessThan(1e-9);
    expect(Math.abs(start.x - 400)).toBeLessThan(1e-9);
    expect(Math.abs(start.y - 520)).toBeLessThan(1e-9);
    const hung = haulOf(shot, 0, PET);
    expect(hung).toEqual({ rope: 0, hand: shot.hook, before: shot.hook, x: shot.hook.x - MUZZLE_FORWARD * 40, y: shot.hook.y + MUZZLE_HEIGHT * 48 });
    const half = haulOf(shot, shot.length / 2, PET);
    expect(Math.abs(half.x - (start.x + hung.x) / 2)).toBeLessThan(1e-9);
    expect(Math.abs(half.y - (start.y + hung.y) / 2)).toBeLessThan(1e-9);
    const mirrored = shotFor({ x: 420, y: 520 }, [perch("stub", 380, 420, 380)], [], PET)!;
    expect(mirrored.facing).toBe(-1);
    expect(haulOf(mirrored, 0, PET).x).toBe(mirrored.hook.x + MUZZLE_FORWARD * 40);
  });

  it("gathers its speed over the ramp, reels at the zip speed along the taut line and ends where the hoist begins", () => {
    const end = REEL_LEAST * 48;
    let haul = haulOf(shot, shot.length, PET);
    let ticks = 0;
    while (haul.rope > end && ticks < 500) {
      const next = zipStep(shot, haul, ticks, PET);
      expect(Math.abs(haul.rope - next.rope - Math.min((ZIP_SPEED * smoothstep((ticks + 1) / ZIP_RAMP)) / 64, haul.rope - end)), `tick ${ticks}`).toBeLessThan(1e-9);
      expect(next).toEqual({ ...haulOf(shot, next.rope, PET), before: haul.hand });
      expect(haulStep(shot, haul, ticks, PET)).toEqual(next);
      haul = next;
      ticks++;
    }
    expect(haul.rope).toBe(end);
    expect(ticks).toBe(haulTicks(shot, PET));
    expect(ticks).toBe(ZIP_RAMP + Math.ceil((shot.length - end - (5.5 * ZIP_SPEED) / 64) / (ZIP_SPEED / 64)));
    expect(zipStep(shot, haul, ticks, PET).rope).toBe(end);
  });

  it("leaves a rope alone that is no longer than its end", () => {
    const short = shotFor({ x: 400, y: 450.2 }, [shelf], [], PET)!;
    expect(short.length).toBeLessThan(REEL_LEAST * 48);
    expect(haulTicks(short, PET)).toBe(0);
    expect(zipStep(short, haulOf(short, short.length, PET), 0, PET).rope).toBe(short.length);
    expect(zipStep(shot, haulOf(shot, 20, PET), 30, PET).rope).toBe(20);
    expect(swayStep(shot, haulOf(shot, 20, PET), 30, PET).rope).toBe(20);
  });

  it("swings on a slanted rope as the pendulum of the swing module, the feet under the hands", () => {
    const slanted = shotFor({ x: 250, y: 480 }, [FLOOR, shelf], [], PET)!;
    const least = REEL_LEAST * 48;
    expect(slanted.reel).toBe("swing");
    let haul = haulOf(slanted, slanted.length, PET);
    let ticks = 0;
    let lowest = haul.hand.y;
    let crossed = false;
    while (haul.rope > least && ticks < 500) {
      const reel = reelStep(slanted.hook, haul.hand, haul.before, haul.rope, least, ticks);
      const next = swayStep(slanted, haul, ticks, PET);
      expect(next).toEqual({ rope: reel.length, hand: reel.bob, before: haul.hand, x: reel.bob.x - MUZZLE_FORWARD * 40, y: reel.bob.y + MUZZLE_HEIGHT * 48 });
      expect(haulStep(slanted, haul, ticks, PET)).toEqual(next);
      expect(Math.hypot(next.hand.x - slanted.hook.x, next.hand.y - slanted.hook.y), `tick ${ticks}`).toBeLessThan(next.rope + 1e-9);
      expect(Math.hypot(next.hand.x - haul.hand.x, next.hand.y - haul.hand.y), `tick ${ticks}`).toBeLessThan(REEL_CAP / 64 + 1e-9);
      expect(haul.rope - next.rope, `tick ${ticks}`).toBeCloseTo(Math.min((REEL_SPEED * Math.min(ticks + 1, REEL_RAMP)) / REEL_RAMP / 64, haul.rope - least), 9);
      lowest = Math.max(lowest, next.hand.y);
      crossed = crossed || next.hand.x > slanted.hook.x;
      haul = next;
      ticks++;
    }
    expect(haul.rope).toBe(least);
    expect(ticks).toBe(haulTicks(slanted, PET));
    expect(ticks).toBe(REEL_RAMP - 1 + Math.ceil((slanted.length - least - (4.5 * REEL_SPEED) / 64) / (REEL_SPEED / 64)));
    expect(crossed).toBe(true);
    expect(lowest).toBeGreaterThan(slanted.muzzle.y);
    expect(lowest).toBeLessThan(slanted.hook.y + slanted.length);
  });

  it("lands inside the hook, towards the middle of the perch and never beyond its ends", () => {
    expect(landingFor(shot, shelf, PET)).toEqual({ x: 400 + 20 + MANTLE_INSET, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 330, y: 379 } }, shelf, PET)).toEqual({ x: 358, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 470, y: 379 } }, shelf, PET)).toEqual({ x: 442, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 400, y: 379 } }, perch("stub", 380, 420, 380), PET)).toEqual({ x: 420, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 401, y: 379 } }, perch("stub", 380, 420, 380), PET)).toEqual({ x: 380, y: 380 });
  });
});

describe("shotHolds", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const box = solid(300, 500, 380, 420);
  const shot = shotFor({ x: 280, y: 480 }, [FLOOR, shelf], [box], PET)!;

  it("holds as it is when nothing moved and follows its edge up and down", () => {
    expect(shotHolds(shot, [FLOOR, shelf], [box])).toEqual(shot);
    for (const dy of [-ROPE_FOLLOW, -3, 5, ROPE_FOLLOW]) {
      const held = shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 + dy)], [solid(300, 500, 380 + dy, 420 + dy)])!;
      expect(held.hook).toEqual({ x: 310, y: 379 + dy });
      expect(Math.abs(held.length - Math.hypot(310 - shot.muzzle.x, 379 + dy - shot.muzzle.y))).toBeLessThan(1e-12);
      expect({ ...held, hook: shot.hook, length: shot.length }).toEqual(shot);
    }
  });

  it("loses an edge that moved too far, left the hook behind or vanished", () => {
    expect(shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 + ROPE_FOLLOW + 0.5)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 - ROPE_FOLLOW - 0.5)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 310.5, 494, 380)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 310, 494, 380)], [])).not.toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("other", 306, 494, 380)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR], [])).toBeNull();
  });

  it("loses its line to a keep-out that appears on it", () => {
    expect(shotHolds(shot, [FLOOR, shelf], [box, keepout(300, 420, 16, 30)])).toBeNull();
    expect(shotHolds(shot, [FLOOR, shelf], [box, keepout(100, 420, 16, 30)])).toEqual(shot);
  });

  it("aims a shot that is meant to miss past the nearer end of the perch", () => {
    expect(missOf(shot, shelf)).toEqual({ x: 306 - ROPE_MISS_OVERSHOOT, y: 379 });
    expect(missOf({ ...shot, hook: { x: 470, y: 379 } }, shelf)).toEqual({ x: 494 + ROPE_MISS_OVERSHOOT, y: 379 });
    expect(missOf({ ...shot, hook: { x: 400, y: 379 } }, shelf)).toEqual({ x: 292, y: 379 });
  });
});
