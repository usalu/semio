/** 👆️ Subject adapter of the gesture-recognition case: the press machine, the heat and the three path gestures of `@semio-tech/pets` replay every committed trace tick by tick and hold themselves to the committed answers.
 *
 * A path is a flat list of integers in quanta (a quarter of a pixel): the first point, then per tick either the
 * step `dx, dy` to the next point or one number `hold + n`, which repeats the latest point for `n` ticks. The sample
 * at index `k` is the pointer of tick `k`. A variant of a trace is the same trace mirrored, transposed or played
 * backwards — exact on integers, so every implementation sees the same numbers.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👆️gesture/🟦️.ts
 */
import { AssertionError } from "node:assert";
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import {
  CIRCLE_AGAINST,
  CIRCLE_CLOSE,
  CIRCLE_FAST,
  CIRCLE_HYSTERESIS,
  CIRCLE_MARGIN,
  CIRCLE_OUT_TICKS,
  CIRCLE_QUARTERS,
  CIRCLE_REACH,
  CIRCLE_REST,
  CIRCLE_ROUND,
  CIRCLE_SLOW,
  COLD,
  type Caress,
  ENOUGH_TICKS,
  type Guards,
  HEAT_CLICK,
  HEAT_ENOUGH,
  HEAT_FORGIVEN,
  HEAT_HELLO,
  HEAT_HOLD,
  HEAT_LEAK,
  HEAT_TRICK,
  HOLD_TICKS,
  IDLE,
  type PressInput,
  SCROLL_TICKS,
  SHAKE_AMPLITUDE,
  SHAKE_HYSTERESIS,
  SHAKE_PAUSE,
  SHAKE_REST,
  SHAKE_REVERSALS,
  SHAKE_SPEED,
  SHAKE_WINDOW,
  SLOP_COARSE,
  SLOP_FINE,
  STROKE_FAST,
  STROKE_HYSTERESIS,
  STROKE_LENGTH,
  STROKE_MARGIN,
  STROKE_PAUSE,
  STROKE_SEGMENTS,
  STROKE_SLANT,
  STROKE_SLOW,
  STROKE_WINDOW,
  UNGUARDED,
  hoverStep,
  noHover,
  noShaking,
  pressStep,
  shakeStep,
  warmthAfter,
} from "../../🔨️modules/👆️gesture/🟦️.ts";

const VECTORS = "shared://👆️gesture-recognition/🔣️.json";

/** 🎚️ The thresholds of the module as the vectors name them. */
export const CONSTANTS = {
  slopFine: SLOP_FINE,
  slopCoarse: SLOP_COARSE,
  holdTicks: HOLD_TICKS,
  heatClick: HEAT_CLICK,
  heatHold: HEAT_HOLD,
  heatLeak: HEAT_LEAK,
  heatHello: HEAT_HELLO,
  heatTrick: HEAT_TRICK,
  heatEnough: HEAT_ENOUGH,
  heatForgiven: HEAT_FORGIVEN,
  enoughTicks: ENOUGH_TICKS,
  scrollTicks: SCROLL_TICKS,
  circleMargin: CIRCLE_MARGIN,
  circleReach: CIRCLE_REACH,
  circleHysteresis: CIRCLE_HYSTERESIS,
  circleQuarters: CIRCLE_QUARTERS,
  circleFast: CIRCLE_FAST,
  circleSlow: CIRCLE_SLOW,
  circleRound: CIRCLE_ROUND,
  circleClose: CIRCLE_CLOSE,
  circleAgainst: CIRCLE_AGAINST,
  circleOutTicks: CIRCLE_OUT_TICKS,
  circleRest: CIRCLE_REST,
  strokeMargin: STROKE_MARGIN,
  strokeHysteresis: STROKE_HYSTERESIS,
  strokeLength: STROKE_LENGTH,
  strokeSlow: STROKE_SLOW,
  strokeFast: STROKE_FAST,
  strokeSlant: STROKE_SLANT,
  strokeSegments: STROKE_SEGMENTS,
  strokeWindow: STROKE_WINDOW,
  strokePause: STROKE_PAUSE,
  shakeAmplitude: SHAKE_AMPLITUDE,
  shakeHysteresis: SHAKE_HYSTERESIS,
  shakeSpeed: SHAKE_SPEED,
  shakeReversals: SHAKE_REVERSALS,
  shakeWindow: SHAKE_WINDOW,
  shakePause: SHAKE_PAUSE,
  shakeRest: SHAKE_REST,
};

/** 📦️ A body as the vectors write it: left, top, width and height in quanta. */
export type Box = readonly [number, number, number, number];

/** 🔔️ A cue or a signal and the tick it came at. */
export type Sighted = readonly [number, string];

/** 🐾️ A pointer path decoded into one point per tick, in quanta. */
export type Trail = { readonly xs: readonly number[]; readonly ys: readonly number[] };

/** 🖲️ One event of a committed press: the tick it arrives at (after that tick has passed), the event, and the guards that are up for it. */
export type PressEvent = { readonly at: number; readonly kind: "pressed" | "dragged" | "released" | "cancelled"; readonly x?: number; readonly y?: number; readonly pointer?: "mouse" | "pen" | "touch"; readonly guards?: readonly (keyof Guards)[] };

/** 🫳️ A committed press: its events over `ticks` ticks. */
export type PressVector = { readonly id: string; readonly ticks: number; readonly events: readonly PressEvent[]; readonly expected: readonly Sighted[] };

/** 🌡️ A committed history of attention: the caresses and the ticks they come at. */
export type WarmthVector = { readonly id: string; readonly caresses: readonly (readonly [number, Caress])[]; readonly expected: readonly (readonly [number, string, number, number, number])[] };

/** 🌀️ A committed hover gesture round or over one body: what the hand meant (`label`), the ticks between which it drew it (`span`), whether the oracle finds it clean (`robust`), and the guard that is up throughout, if any (then `of` names the same trace without it). */
export type HoverVector = { readonly id: string; readonly label: string; readonly robust: boolean; readonly span: readonly [number, number]; readonly body: Box; readonly path: readonly number[]; readonly guard?: keyof Guards; readonly of?: string; readonly expected: readonly Sighted[] };

/** 🫨️ A committed held path of a pet of some height (in quanta): a shake or a carry, described like a hover gesture. */
export type HeldVector = { readonly id: string; readonly label: string; readonly robust: boolean; readonly span: readonly [number, number]; readonly height: number; readonly path: readonly number[]; readonly guard?: keyof Guards; readonly of?: string; readonly expected: readonly Sighted[] };

/** 🧳️ A committed stretch of ordinary pointer travel over the page of bodies; `sample` marks the stretches of the fundamental level. */
export type TravelVector = { readonly id: string; readonly kind: string; readonly sample: boolean; readonly path: readonly number[] };

/** 🧫️ The committed vectors. */
export type Vectors = {
  readonly constants: Record<string, number>;
  readonly quantum: number;
  readonly hold: number;
  readonly page: readonly [number, number];
  readonly variants: number;
  readonly floors: { readonly circles: number; readonly strokes: number; readonly shakes: number };
  readonly presses: readonly PressVector[];
  readonly warmths: readonly WarmthVector[];
  readonly circles: readonly HoverVector[];
  readonly strokes: readonly HoverVector[];
  readonly shakes: readonly HeldVector[];
  readonly bodies: readonly Box[];
  readonly travels: readonly TravelVector[];
};

/** 📂️ The committed vectors of a plan. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🧵️ The points of a path, one per tick, in quanta. */
export function trailOf(path: readonly number[], hold: number): Trail {
  let x = path[0] ?? 0;
  let y = path[1] ?? 0;
  const xs = [x];
  const ys = [y];
  for (let index = 2; index < path.length; ) {
    const token = path[index]!;
    if (token >= hold) {
      for (let repeat = 0; repeat < token - hold; repeat++) {
        xs.push(x);
        ys.push(y);
      }
      index += 1;
    } else {
      x += token;
      y += path[index + 1]!;
      xs.push(x);
      ys.push(y);
      index += 2;
    }
  }
  return { xs, ys };
}

/** 🪞️ A trail in one of its sixteen variants on a page of `page` quanta: bit 0 mirrors left and right, bit 1 top and bottom, bit 2 swaps the axes, bit 3 plays it backwards. */
export function variantOf(trail: Trail, page: readonly [number, number], variant: number): Trail {
  const mirrored = (variant & 1) === 0 ? trail.xs : trail.xs.map((x) => page[0] - x);
  const flipped = (variant & 2) === 0 ? trail.ys : trail.ys.map((y) => page[1] - y);
  const xs = (variant & 4) === 0 ? mirrored : flipped;
  const ys = (variant & 4) === 0 ? flipped : mirrored;
  return (variant & 8) === 0 ? { xs, ys } : { xs: [...xs].reverse(), ys: [...ys].reverse() };
}

/** 🔲️ A body in the same variant. */
export function boxVariant(box: Box, page: readonly [number, number], variant: number): Box {
  const x = (variant & 1) === 0 ? box[0] : page[0] - box[0] - box[2];
  const y = (variant & 2) === 0 ? box[1] : page[1] - box[1] - box[3];
  return (variant & 4) === 0 ? [x, y, box[2], box[3]] : [y, x, box[3], box[2]];
}

/** 🚧️ The guards with one of them up, or none. */
function guarded(guard: keyof Guards | undefined): Guards {
  return guard === undefined ? UNGUARDED : { ...UNGUARDED, [guard]: true };
}

/** 🪶️ Every cue of the hover gestures while the pointer follows a trail past one body. */
export function hovered(trail: Trail, quantum: number, box: Box, guards: Guards): Sighted[] {
  const body = { x: box[0] / quantum, y: box[1] / quantum, width: box[2] / quantum, height: box[3] / quantum };
  const cues: Sighted[] = [];
  let hover = noHover(0);
  for (let tick = 0; tick < trail.xs.length; tick++) {
    const step = hoverStep(hover, { x: trail.xs[tick]! / quantum, y: trail.ys[tick]! / quantum }, body, tick, guards);
    hover = step.state;
    if (step.cue !== null) cues.push([tick, step.cue]);
  }
  return cues;
}

/** 🥤️ Every cue of the shake while the grip follows a trail. */
export function held(trail: Trail, quantum: number, height: number, guards: Guards): Sighted[] {
  const cues: Sighted[] = [];
  let shaking = noShaking(0);
  for (let tick = 0; tick < trail.xs.length; tick++) {
    const step = shakeStep(shaking, { x: trail.xs[tick]! / quantum, y: trail.ys[tick]! / quantum }, height / quantum, tick, guards);
    shaking = step.state;
    if (step.cue !== null) cues.push([tick, step.cue]);
  }
  return cues;
}

/** 🕹️ Every signal of a press: per tick first the passing of the tick, then the events that arrive at it, in order. */
export function pressed(vector: Pick<PressVector, "ticks" | "events">, quantum: number): Sighted[] {
  const signals: Sighted[] = [];
  let press = IDLE;
  for (let tick = 1; tick <= vector.ticks; tick++) {
    const passed = pressStep(press, { kind: "ticked" }, tick, UNGUARDED);
    press = passed.state;
    if (passed.signal !== null) signals.push([tick, passed.signal]);
    for (const event of vector.events) {
      if (event.at !== tick) continue;
      const x = (event.x ?? 0) / quantum;
      const y = (event.y ?? 0) / quantum;
      const input: PressInput = event.kind === "pressed" ? { kind: "pressed", x, y, pointer: event.pointer ?? "mouse" } : event.kind === "cancelled" ? { kind: "cancelled" } : { kind: event.kind, x, y };
      const guards = (event.guards ?? []).reduce<Guards>((up, guard) => ({ ...up, [guard]: true }), UNGUARDED);
      const step = pressStep(press, input, tick, guards);
      press = step.state;
      if (step.signal !== null) signals.push([tick, step.signal]);
    }
  }
  return signals;
}

/** 💓️ The answer to every caress of a history: the heat right after it, the tier, the run, the tricks so far and the tick until which the pet has had enough. */
export function warmed(caresses: WarmthVector["caresses"]): [number, string, number, number, number][] {
  const answers: [number, string, number, number, number][] = [];
  let warmth = COLD;
  for (const [tick, caress] of caresses) {
    warmth = warmthAfter(warmth, tick, caress);
    answers.push([warmth.heat, warmth.tier, warmth.run, warmth.tricks, warmth.until]);
  }
  return answers;
}

/** 🧾️ Refuses an answer that is not the committed one. */
function agreed<Answer>(scenario: string, id: string, produced: Answer, expected: unknown): Answer {
  if (JSON.stringify(produced) !== JSON.stringify(expected)) throw new AssertionError({ message: `${scenario}/${id}: the subject answers ${JSON.stringify(produced)}, the committed vector says ${JSON.stringify(expected)}` });
  return produced;
}

/** 📊️ How many traces of one kind there are in the four mirrored variants, on how many the gesture of their label was cued, and on how many something else was; a mirror turns a circle the other way round. Refused when anything else was cued, or when a mirror changes a tick. */
function tallied<Trace extends HoverVector | HeldVector>(traces: readonly Trace[], cuesOf: (vector: Trace, variant: number) => Sighted[]): { traces: number; detected: number; wrong: number } {
  const tally = { traces: 0, detected: 0, wrong: 0 };
  for (const vector of traces) {
    if (vector.guard !== undefined || vector.label === "carry") continue;
    const upright = cuesOf(vector, 0);
    for (let variant = 0; variant < 4; variant++) {
      const turned = (vector.label === "circle" || vector.label === "countercircle") && (variant === 1 || variant === 2);
      const wanted = turned ? (vector.label === "circle" ? "countercircle" : "circle") : vector.label;
      const cues = cuesOf(vector, variant);
      if (JSON.stringify(cues.map(([tick]) => tick)) !== JSON.stringify(upright.map(([tick]) => tick))) throw new AssertionError({ message: `detection: the mirror ${variant} of ${vector.id} is cued at other ticks than the trace itself` });
      tally.traces += 1;
      tally.detected += cues.some(([, cue]) => cue === wanted) ? 1 : 0;
      tally.wrong += cues.some(([, cue]) => cue !== wanted) ? 1 : 0;
    }
  }
  if (tally.wrong !== 0) throw new AssertionError({ message: `detection: ${tally.wrong} deliberate traces were answered with another gesture than theirs` });
  return tally;
}

/** 🔭️ Every cue ordinary travel sets off: each chosen stretch, in each of the committed variants (the sample in the first only), past every body. The projection says how much was watched and names every cue; a subject that cues anything is refused. */
export function travelled(document: Pick<Vectors, "travels" | "bodies" | "hold" | "quantum" | "page" | "variants">, sample: boolean): { pointerTicks: number; petTicks: number; cues: [string, number, number, number, string][] } {
  const cues: [string, number, number, number, string][] = [];
  let pointerTicks = 0;
  for (const travel of document.travels) {
    if (sample && !travel.sample) continue;
    const trail = trailOf(travel.path, document.hold);
    for (let variant = 0; variant < (sample ? 1 : document.variants); variant++) {
      const turned = variantOf(trail, document.page, variant);
      pointerTicks += turned.xs.length;
      document.bodies.forEach((box, body) => {
        for (const [tick, cue] of hovered(turned, document.quantum, boxVariant(box, document.page, variant), UNGUARDED)) cues.push([travel.id, variant, body, tick, cue]);
      });
    }
  }
  return { pointerTicks, petTicks: pointerTicks * document.bodies.length, cues };
}

/** 🤫️ Refuses ordinary travel that set anything off. */
function silent(travel: ReturnType<typeof travelled>): ReturnType<typeof travelled> {
  if (travel.cues.length !== 0) throw new AssertionError({ message: `travel: ordinary travel set off ${travel.cues.length} cues, the first ${JSON.stringify(travel.cues.slice(0, 5))}` });
  return travel;
}

/** 🎐️ The cues of one hover vector in one of its mirrored variants. */
function hoverCues(document: Vectors, vector: HoverVector, variant: number): Sighted[] {
  return hovered(variantOf(trailOf(vector.path, document.hold), document.page, variant), document.quantum, boxVariant(vector.body, document.page, variant), guarded(vector.guard));
}

/** 🎢️ The cues of one held vector in one of its mirrored variants. */
function heldCues(document: Vectors, vector: HeldVector, variant: number): Sighted[] {
  return held(variantOf(trailOf(vector.path, document.hold), document.page, variant), document.quantum, vector.height, guarded(vector.guard));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: CONSTANTS }) },
    presses: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.presses.map((vector) => [vector.id, agreed("presses", vector.id, pressed(vector, document.quantum), vector.expected)])) };
      },
    },
    warmth: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).warmths.map((vector) => [vector.id, agreed("warmth", vector.id, warmed(vector.caresses), vector.expected)])) }) },
    circles: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.circles.map((vector) => [vector.id, agreed("circles", vector.id, hoverCues(document, vector, 0), vector.expected)])) };
      },
    },
    strokes: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.strokes.map((vector) => [vector.id, agreed("strokes", vector.id, hoverCues(document, vector, 0), vector.expected)])) };
      },
    },
    shakes: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.shakes.map((vector) => [vector.id, agreed("shakes", vector.id, heldCues(document, vector, 0), vector.expected)])) };
      },
    },
    detection: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return {
          projection: {
            circles: tallied(document.circles, (vector, variant) => hoverCues(document, vector, variant)),
            strokes: tallied(document.strokes, (vector, variant) => hoverCues(document, vector, variant)),
            shakes: tallied(document.shakes, (vector, variant) => heldCues(document, vector, variant)),
          },
        };
      },
    },
    "travel-sample": { subject: (ctx) => ({ projection: silent(travelled(vectors(ctx), true)) }) },
    "travel-hours": { subject: (ctx) => ({ projection: silent(travelled(vectors(ctx), false)) }) },
  },
});
