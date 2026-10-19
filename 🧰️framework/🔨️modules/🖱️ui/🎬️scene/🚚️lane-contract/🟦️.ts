/** 🚚️ Scene lane contract: the lane manifest entry, the lane descriptor and the spine-plus-lanes reassembly every scene document shares; a leaf, so scene types never import the lane tables that are typed by them. */
/** 🚚️ One entry of a scene spine's lane manifest (`World3dScene.lanes`, `Canvas2dScene.lanes`).
 * `bytes` is what tells a fully arrived lane from one still spread across reconcile pages; `hash`
 * (FNV-1a/64, lowercase hex, computed by the producer) is what makes the spine itself differ exactly
 * when a lane's content differs. */
export type SceneLaneRef = {
  readonly lane: string;
  readonly bytes: number;
  readonly hash: string;
};

/** 🚚️ One scene payload lane: which scene field it carries and which reserved node key its retained
 * text carrier is rooted at. */
export type SceneLane<S> = {
  readonly encoding?: "json";
  readonly lane: string;
  readonly field: keyof S;
  readonly bodyKey: string;
  readonly optional: boolean;
};

/** 🚚️ Reassembles a decoded scene spine and its lane carrier texts back into the scene every render
 * host already knows how to read — the exact inverse of the Rust `SceneDoc::split_lanes`.
 *
 * `laneTexts` is keyed by reserved carrier key ({@link SceneLane.bodyKey}). A lane the spine declares
 * but `laneTexts` does not carry is left at its spine value (an empty string for a required lane,
 * absent for an optional one) rather than guessed: a caller that wants last-known-good content across
 * a partially arrived refresh supplies it in `laneTexts` itself, which is exactly what the
 * Interpreter's per-lane cache does. Never throws. */
export function sceneFromLanes<S extends object>(spine: S, laneTexts: ReadonlyMap<string, string>, lanes: readonly SceneLane<S>[]): S {
  if (laneTexts.size === 0) return spine;
  const assembled: Record<string, unknown> = { ...(spine as Record<string, unknown>) };
  for (const lane of lanes) {
    const text = laneTexts.get(lane.bodyKey);
    if (text === undefined) continue;
    const prior = assembled[lane.field as string];
    if (lane.optional && text.length === 0 && typeof prior === "string" && prior.length > 0) continue;
    if (lane.encoding === "json") {
      try { assembled[lane.field as string] = JSON.parse(text); } catch { continue; }
    } else assembled[lane.field as string] = text;
  }
  return assembled as S;
}
