/** 🎞️ Subject adapter of the animation-sampling case: the animation module of `@semio-tech/pets` answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🎞️animation/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { BLINK_TICKS, blendPose, clipTicks, easeBezier, lidAt, sampleClip, sampleTrack } from "../../🔨️modules/🎞️animation/🟦️.ts";
import type { Pose } from "../../🔨️modules/🦴️rig/🟦️.ts";
import type { Clip, Ease, Species, Track } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🎞️animation-sampling/🔣️.json";

type Vectors = {
  readonly easings: readonly { readonly id: string; readonly ease: Ease; readonly amounts: readonly number[] }[];
  readonly tracks: readonly { readonly id: string; readonly track: Track; readonly phases: readonly number[] }[];
  readonly lengths: readonly { readonly id: string; readonly seconds: number }[];
  readonly species: Species;
  readonly poses: readonly { readonly id: string; readonly clip: string; readonly ticks: readonly number[] }[];
  readonly blends: readonly { readonly id: string; readonly from: Pose; readonly to: Pose; readonly amounts: readonly number[] }[];
  readonly lids: { readonly ticks: readonly number[] };
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** ⏱️ A clip of the given length without tracks. */
function lasting(id: string, seconds: number): Clip {
  return { id, seconds, loop: false, tracks: [] };
}

/** 🤸️ The poses of one clip of the committed species at the committed ticks. */
function posesOf(species: Species, clip: string, ticks: readonly number[]): Pose[] {
  const played = species.clips.find((candidate) => candidate.id === clip)!;
  return ticks.map((tick) => sampleClip(species, played, tick));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "bezier-easings": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).easings.map((vector) => [vector.id, vector.amounts.map((amount) => easeBezier(vector.ease, amount))])) }) },
    "track-samples": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).tracks.map((vector) => [vector.id, vector.phases.map((phase) => sampleTrack(vector.track, phase))])) }) },
    "clip-lengths": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).lengths.map((vector) => [vector.id, clipTicks(lasting(vector.id, vector.seconds))])) }) },
    "clip-poses": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return { projection: Object.fromEntries(committed.poses.map((vector) => [vector.id, posesOf(committed.species, vector.clip, vector.ticks)])) };
      },
    },
    "pose-blends": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).blends.map((vector) => [vector.id, vector.amounts.map((amount) => blendPose(vector.from, vector.to, amount))])) }) },
    "blink-lids": { subject: (ctx) => ({ projection: { blinkTicks: BLINK_TICKS, closures: vectors(ctx).lids.ticks.map((tick) => lidAt(tick)) } }) },
  },
});
