/** 🎛️ The controls of the selected placement, the same in all three playgrounds: its pose or clip, where it stands and how it turns, and how it plays on the shared timeline (pause, scrub, loop, speed, start, length, clip offset).
 *
 * Playing and pausing are per placement: pausing freezes the placement at its current clip time (`holdS`) whatever the timeline does, scrubbing sets that time, playing releases it. Every edit is one event of the store and is announced; a pose or clip the asset no longer has (after a new revision of the asset) is named and the default is played instead.
 *
 * @see ../🎞️timeline/🟦️.ts — the rules the controls edit
 * @see ../🎞️timeline/🖼️panel/🟦️.tsx — the same windows on the timeline
 */
import type { ReactElement } from "react";
import { formatNumber } from "@teaching/staffage";
import type { Asset } from "@teaching/staffage";
import { useClock, type StaffageClock } from "../../⏱️clock/🟦️.ts";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Bar, Button, CheckField, Cell, Column, Note, Notice, NumberField, SelectField, SliderField, cn } from "../../🪟️chrome/🟦️.tsx";
import { TRACK_LIMITS, wrapYaw, type Placement, type Selection, type Track } from "../../💾️persistence/🟦️.ts";
import { namedPoses } from "../../🔎️search/🟦️.ts";
import { RATES } from "../../⏯️transport/🟦️.tsx";
import { clipDurationS, isMissing, readTrack } from "../🎞️timeline/🟦️.ts";

const pickValue = (pick: Selection | null): string => (pick === null ? "default" : pick.kind === "pose" ? `pose:${pick.pose}` : `clip:${pick.clip}`);
const pickFrom = (value: string): Selection | null => (value === "default" ? null : value.startsWith("pose:") ? { kind: "pose", pose: value.slice(5) } : { kind: "clip", clip: value.slice(5) });

/** 🐢️ The speeds a placement can play at. */
export const SPEEDS = [...RATES, 4] as const;

/** 🎛️ The panel of one placement; `change` is one edit with its status text. */
export function AssetPanel(props: {
  readonly placement: Placement;
  readonly asset: Asset | undefined;
  readonly title: string;
  readonly clock: StaffageClock;
  readonly timelineS: number;
  readonly mirrorAllowed: boolean;
  readonly change: (patch: Partial<Omit<Placement, "id">>, text: string) => void;
  readonly duplicate: () => void;
  readonly remove: () => void;
}): ReactElement {
  const { placement, asset, title, clock, timelineS, mirrorAllowed, change, duplicate, remove } = props;
  const { t, language } = useArchive();
  const state = useClock(clock)!;
  const clipS = asset === undefined ? 0 : clipDurationS(asset, placement.pick);
  const reading = readTrack(placement, clipS, state.timeS, timelineS);
  const missing = asset !== undefined && isMissing(asset, placement.pick);
  const seconds = (value: number): string => `${formatNumber(value, 2, language)} s`;
  const patch = (next: Partial<Track>, text: string): void => change(next, text);
  const options = [{ value: "default", label: t("playground.pick.default") }, ...(asset === undefined ? [] : namedPoses(asset)).map((pose) => ({ value: `pose:${pose.id}`, label: `${t("playground.pick.pose")}: ${pose.title[language]}` })), ...(asset?.clips ?? []).map((clip) => ({ value: `clip:${clip.id}`, label: `${t("playground.pick.clip")}: ${clip.title[language]}` }))];
  const known = new Set(options.map((option) => option.value));
  const value = pickValue(placement.pick);
  return (
    <Column as="section" aria-label={t("playground.selected", { title })} className="bg-panel" data-asset-panel={placement.id}>
      <Cell><h2 className="m-0 text-sm font-semibold">{title}</h2></Cell>
      {missing ? <Notice kind="unavailable" compact title={t("playground.asset.missing", { pick: value.slice(value.indexOf(":") + 1) })} /> : null}
      <Bar below="tablet">
        <SelectField label={t("playground.pick")} value={known.has(value) ? value : "default"} onChange={(next) => change({ pick: pickFrom(next), holdS: null }, t("playground.said.picked"))} options={options} />
        <NumberField label={t("playground.x")} unit="m" step={0.05} value={placement.xM} onChange={(xM) => xM !== null && change({ xM }, t("playground.said.moved", { title, x: formatNumber(xM, 2, language) }))} />
        <NumberField label={t("playground.z")} unit="m" step={0.05} value={placement.zM} onChange={(zM) => zM !== null && change({ zM }, t("playground.said.lateral", { title, z: formatNumber(zM, 2, language) }))} />
      </Bar>
      <Bar below="tablet">
        <NumberField label={t("playground.yaw")} unit="°" step={5} min={-180} max={180} value={placement.yawDeg} onChange={(yawDeg) => yawDeg !== null && change({ yawDeg }, t("playground.said.turned", { title, yaw: formatNumber(wrapYaw(yawDeg), 0, language) }))} />
        <CheckField label={t("playground.mirror")} disabled={!mirrorAllowed} checked={placement.mirrored} onChange={(mirrored) => change({ mirrored }, t("playground.said.mirrored"))} />
      </Bar>
      <Cell className="py-single"><h3 className="m-0 text-xs font-semibold">{t("playground.asset.playback")}</h3></Cell>
      <Bar below="tablet">
        <Button grouped icon={placement.holdS === null ? "pause" : "play"} disabled={clipS <= 0} aria-pressed={placement.holdS !== null} onClick={() => patch({ holdS: placement.holdS === null ? reading.cycleS : null }, t(placement.holdS === null ? "playground.asset.said.paused" : "playground.asset.said.playing", { title }))}>
          {t(placement.holdS === null ? "playground.asset.pauseOne" : "playground.asset.playOne", { title })}
        </Button>
        <CheckField label={t("playground.asset.loop")} checked={placement.loop} disabled={clipS <= 0} onChange={(loop) => patch({ loop }, t("playground.asset.said.loop", { title, state: t(loop ? "settings.on" : "settings.off") }))} />
        <SelectField label={t("playground.asset.speed")} value={String(placement.speed)} disabled={clipS <= 0} onChange={(speed) => patch({ speed: Number(speed) }, t("playground.asset.said.speed", { title, speed: formatNumber(Number(speed), 2, language) }))} options={[...(SPEEDS as readonly number[]).includes(placement.speed) ? [] : [placement.speed], ...SPEEDS].map((speed) => ({ value: String(speed), label: `${formatNumber(speed, speed < 1 ? 2 : 0, language)}×` }))} />
      </Bar>
      <Cell className="px-double py-single">
        <SliderField label={t("playground.asset.scrub")} value={Math.min(reading.cycleS, Math.max(clipS, 0.001))} min={0} max={Math.max(clipS, 0.001)} step={1 / 30} text={clipS <= 0 ? t("playground.asset.noTime") : `${seconds(reading.cycleS)} / ${seconds(clipS)}`} onChange={(next) => patch({ holdS: next }, t("playground.asset.said.scrubbed", { title, time: seconds(next) }))} />
      </Cell>
      <Bar below="tablet">
        <NumberField label={t("playground.asset.start")} unit="s" step={0.1} min={0} max={TRACK_LIMITS.startS.max} value={placement.startS} onChange={(startS) => startS !== null && patch({ startS }, t("playground.timeline.said.start", { title, start: seconds(startS) }))} />
        <NumberField label={t("playground.asset.length")} unit="s" step={0.1} min={TRACK_LIMITS.durationS.min} max={TRACK_LIMITS.durationS.max} value={placement.durationS ?? Math.max(timelineS - placement.startS, TRACK_LIMITS.durationS.min)} onChange={(durationS) => durationS !== null && durationS > 0 && patch({ durationS }, t("playground.timeline.said.length", { title, length: seconds(durationS) }))} />
        <CheckField label={t("playground.asset.toEnd")} checked={placement.durationS === null} onChange={(toEnd) => patch({ durationS: toEnd ? null : Math.max(timelineS - placement.startS, TRACK_LIMITS.durationS.min) }, t("playground.timeline.said.toEnd", { title }))} />
        <NumberField label={t("playground.offset")} unit="s" step={0.1} min={0} value={placement.offsetS} onChange={(offsetS) => offsetS !== null && change({ offsetS }, t("playground.said.offset"))} />
      </Bar>
      {placement.yawDeg !== 0 ? <Note className="text-[0.65rem]"><span data-yaw-note="">{t("playground.yawNote", { yaw: formatNumber(placement.yawDeg, 0, language) })}</span></Note> : null}
      {!mirrorAllowed ? <Note className="text-[0.65rem]"><span data-mirror-note="">{t("playground.mirrorOnlySide")}</span></Note> : null}
      <Bar>
        <Button grouped className={cn("flex-1")} icon="copy" onClick={duplicate}>{t("playground.duplicate")}</Button>
        <Button grouped className="flex-1" icon="trash-2" onClick={remove}>{t("playground.remove")}</Button>
      </Bar>
    </Column>
  );
}
