/** 🔀️ Resolved reveal arrangements: dispositions by participant, morph companions and morph target frames, centering and visibility of resolved positions.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type {
  Disposition,
  DispositionPosition,
  Arrangement,
  ParticipantEmphasis,
  Slide,
  ResolvedDisposition
} from "@semio-tech/presentation";
import {
  buildResolutionScope,
  morphId,
  resolveArrangement,
  resolveEmbodiment,
  unionDispositionPositions
} from "@semio-tech/presentation";

//#region 🔖️RevealMorph
/** 👻️ reveal.js-only morph companion role (not part of presentation core). */
export type RevealMorphCompanionKind = "source" | "target";

/** ✅️ Resolved disposition plus optional reveal.js morph companion metadata. */
export interface RevealResolvedDisposition extends ResolvedDisposition {
  readonly revealMorphCompanion?: RevealMorphCompanionKind;
  /** 📐️ Source slide frame for target ghosts: paired with {@link ResolvedDisposition.position} for frame and crop morph during auto-animate. */
  readonly revealMorphFromFrame?: DispositionPosition;
  /** 🎞️ Target slide frame for source tiles: paired with {@link ResolvedDisposition.position} when the next slide morphFrom references this participant. */
  readonly revealMorphToFrame?: DispositionPosition;
  /** 🔲️ Previous-slide morphTo slot frame (catalogue grid) for crop morph into {@link ResolvedDisposition.position}. */
  readonly revealMorphFromMorphToFrame?: DispositionPosition;
}

function primaryDispositionByParticipant(arrangement: Arrangement): Map<string, { readonly participantId: string; readonly embodimentId: string; readonly emphasis: ParticipantEmphasis }> {
  const map = new Map<string, { readonly participantId: string; readonly embodimentId: string; readonly emphasis: ParticipantEmphasis }>();
  for (const disposition of arrangement.dispositions) {
    if (!map.has(disposition.participantId)) {
      map.set(disposition.participantId, disposition);
    }
  }
  return map;
}

function dispositionByParticipant(arrangement: Arrangement): Map<string, Disposition> {
  const map = new Map<string, Disposition>();
  for (const disposition of arrangement.dispositions) {
    if (!map.has(disposition.participantId)) {
      map.set(disposition.participantId, disposition);
    }
  }
  return map;
}

function revealMorphCompanionFromMorphFrom(scope: ReturnType<typeof buildResolutionScope>, sourceSlide: Slide, arrangement: Arrangement, options?: { readonly morphLineTargets?: boolean }): RevealResolvedDisposition[] {
  const sourceByParticipant = primaryDispositionByParticipant(sourceSlide.arrangement);
  const sourceDeclByParticipant = dispositionByParticipant(sourceSlide.arrangement);
  const morphLineTargets = options?.morphLineTargets ?? true;
  const companions: RevealResolvedDisposition[] = [];
  for (const disposition of arrangement.dispositions) {
    for (const slot of disposition.morphFrom ?? []) {
      const sourceDisposition = sourceByParticipant.get(slot.participantId);
      const sourceDecl = sourceDeclByParticipant.get(slot.participantId);
      const participant = scope.participants.get(slot.participantId);
      if (!participant) {
        throw new Error(`morphFrom slot references unknown participant "${slot.participantId}".`);
      }
      const embodimentId = slot.embodimentId ?? sourceDisposition?.embodimentId;
      if (!embodimentId) {
        throw new Error(`morphFrom slot for "${slot.participantId}" needs embodimentId (or a source disposition with embodimentId).`);
      }
      companions.push({
        participant,
        embodiment: resolveEmbodiment(scope, embodimentId),
        emphasis: sourceDisposition?.emphasis ?? "active",
        embodimentId,
        morphId: morphId(participant.id),
        position: slot.position,
        revealMorphFromFrame: sourceDecl?.position,
        revealMorphCompanion: "target",
      });
    }
  }
  return companions;
}

function revealMorphCompanionFromMorphTo(scope: ReturnType<typeof buildResolutionScope>, targetSlide: Slide, arrangement: Arrangement): RevealResolvedDisposition[] {
  const targetByParticipant = primaryDispositionByParticipant(targetSlide.arrangement);
  const companions: RevealResolvedDisposition[] = [];
  for (const disposition of arrangement.dispositions) {
    for (const slot of disposition.morphTo ?? []) {
      const targetDisposition = targetByParticipant.get(slot.participantId);
      const participant = scope.participants.get(slot.participantId);
      if (!participant) {
        throw new Error(`morphTo slot references unknown participant "${slot.participantId}".`);
      }
      const embodimentId = slot.embodimentId ?? targetDisposition?.embodimentId;
      if (!embodimentId) {
        throw new Error(`morphTo slot for "${slot.participantId}" needs embodimentId (or a target disposition with embodimentId).`);
      }
      companions.push({
        participant,
        embodiment: resolveEmbodiment(scope, embodimentId),
        emphasis: targetDisposition?.emphasis ?? disposition.emphasis,
        embodimentId,
        morphId: morphId(participant.id),
        position: slot.position,
        revealMorphCompanion: "source",
      });
    }
  }
  return companions;
}

function revealMorphToFrameByParticipant(nextSlide: Slide): Map<string, DispositionPosition> {
  const map = new Map<string, DispositionPosition>();
  for (const disposition of nextSlide.arrangement.dispositions) {
    for (const slot of disposition.morphFrom ?? []) {
      if (!map.has(slot.participantId)) {
        map.set(slot.participantId, slot.position);
      }
    }
  }
  return map;
}

function revealMorphFromMorphToFrameByParticipant(previousSlide: Slide): Map<string, DispositionPosition> {
  const map = new Map<string, DispositionPosition>();
  for (const disposition of previousSlide.arrangement.dispositions) {
    for (const slot of disposition.morphTo ?? []) {
      if (!map.has(slot.participantId)) {
        map.set(slot.participantId, slot.position);
      }
    }
  }
  return map;
}

/** 🐾️ Resolves an arrangement and appends reveal.js morph companions for one-to-many / many-to-one. */
export function resolveRevealArrangement(scope: ReturnType<typeof buildResolutionScope>, arrangement: Arrangement, context: { readonly previousSlide?: Slide; readonly nextSlide?: Slide }): RevealResolvedDisposition[] {
  const resolved = resolveArrangement(scope, arrangement) as RevealResolvedDisposition[];
  const companions: RevealResolvedDisposition[] = [];
  if (context.previousSlide !== undefined) {
    companions.push(...revealMorphCompanionFromMorphFrom(scope, context.previousSlide, arrangement));
  }
  const hasMorphTo = arrangement.dispositions.some((disposition) => (disposition.morphTo?.length ?? 0) > 0);
  if (context.nextSlide !== undefined && hasMorphTo) {
    companions.push(...revealMorphCompanionFromMorphTo(scope, context.nextSlide, arrangement));
  }
  const morphToFrames = context.nextSlide !== undefined ? revealMorphToFrameByParticipant(context.nextSlide) : undefined;
  const morphFromMorphToFrames = context.previousSlide !== undefined ? revealMorphFromMorphToFrameByParticipant(context.previousSlide) : undefined;
  const withMorphFrames = resolved.map((disposition) => {
    if (disposition.revealMorphCompanion !== undefined || disposition.position === undefined) {
      return disposition;
    }
    let updated = disposition;
    const revealMorphToFrame = morphToFrames?.get(disposition.participant.id);
    if (revealMorphToFrame !== undefined) {
      updated = { ...updated, revealMorphToFrame };
    }
    const revealMorphFromMorphToFrame = morphFromMorphToFrames?.get(disposition.participant.id);
    if (revealMorphFromMorphToFrame !== undefined) {
      updated = { ...updated, revealMorphFromMorphToFrame };
    }
    return updated;
  });
  return [...withMorphFrames, ...companions];
}

export function isRevealMorphCompanionOnly(disposition: RevealResolvedDisposition): boolean {
  return disposition.revealMorphCompanion !== undefined;
}

function visibleRevealArrangementPositions(resolved: readonly RevealResolvedDisposition[]): DispositionPosition[] {
  const positions: DispositionPosition[] = [];
  for (const disposition of resolved) {
    if (isRevealMorphCompanionOnly(disposition)) {
      continue;
    }
    if (disposition.style?.opacity === 0) {
      continue;
    }
    if (disposition.position) {
      positions.push(disposition.position);
    }
  }
  return positions;
}

/** ⊕ Centers visible placements; omits reveal-only morph companions. */
export function centerRevealResolvedArrangement(resolved: readonly RevealResolvedDisposition[]): RevealResolvedDisposition[] {
  const positions = visibleRevealArrangementPositions(resolved);
  if (positions.length === 0) {
    return [...resolved];
  }
  const bounds = unionDispositionPositions(positions);
  const offset = {
    x: (1 - bounds.width) / 2 - bounds.x,
    y: (1 - bounds.height) / 2 - bounds.y,
    width: 0,
    height: 0,
  };
  const epsilon = 1e-6;
  if (Math.abs(offset.x) < epsilon && Math.abs(offset.y) < epsilon) {
    return [...resolved];
  }
  return resolved.map((disposition) => {
    if (!disposition.position || isRevealMorphCompanionOnly(disposition)) {
      return disposition;
    }
    return {
      ...disposition,
      position: {
        x: disposition.position.x + offset.x,
        y: disposition.position.y + offset.y,
        width: disposition.position.width,
        height: disposition.position.height,
      },
    };
  });
}
//#endregion 🔖️RevealMorph
