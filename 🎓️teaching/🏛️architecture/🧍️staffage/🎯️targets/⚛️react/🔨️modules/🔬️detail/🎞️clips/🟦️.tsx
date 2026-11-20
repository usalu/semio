/** 🎞️ Poses and clips as selectable cards, and the timeline of a clip: phases, events, contact spans and tracks, with a text alternative.
 *
 * Keyboard: arrow keys move through a card group; on the timeline Left and Right seek one frame, Enter or Space on an event marker seeks to it.
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/radio/ — radio group model
 */
import { namedPoses } from "../../🔎️search/🟦️.ts";
import { useRef, type KeyboardEvent, type ReactElement } from "react";
import type { Asset, Clip, ClipEvent, Text } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Cell, Column, Note, Section, SectionTitle, SharedBorderRowCell, Tag, cn } from "../../🪟️chrome/🟦️.tsx";
import type { Selection } from "../../💾️persistence/🟦️.ts";
import { DataTable, Row, num } from "../🧱️tables/🟦️.tsx";

type Card = { readonly id: string; readonly title: Text; readonly description: Text; readonly facts: readonly string[]; readonly badge?: string };

function CardGroup(props: { readonly label: string; readonly cards: readonly Card[]; readonly checked: string | null; readonly onPick: (id: string) => void }): ReactElement {
  const { language } = useArchive();
  const refs = useRef<(HTMLButtonElement | null)[]>([]);
  const index = Math.max(0, props.cards.findIndex((card) => card.id === props.checked));
  const onKey = (event: KeyboardEvent, at: number): void => {
    const step = event.key === "ArrowRight" || event.key === "ArrowDown" ? 1 : event.key === "ArrowLeft" || event.key === "ArrowUp" ? -1 : 0;
    if (step === 0) return;
    event.preventDefault();
    const next = (at + step + props.cards.length) % props.cards.length;
    refs.current[next]?.focus();
    props.onPick(props.cards[next]!.id);
  };
  return (
    <Column role="radiogroup" aria-label={props.label}>
      {props.cards.map((card, at) => (
        <button key={card.id} ref={(element) => void (refs.current[at] = element)} type="button" role="radio" aria-checked={card.id === props.checked} tabIndex={at === index ? 0 : -1} data-card={card.id} onKeyDown={(event) => onKey(event, at)} data-shared-border-cell="" onClick={() => props.onPick(card.id)} className={cn("flex min-w-0 flex-col items-start gap-single border-0 p-double text-left transition-colors motion-reduce:transition-none", card.id === props.checked ? "bg-active-base text-active-foreground" : "bg-panel hover:bg-hover-panel")}>
          <span className="flex w-full items-baseline justify-between gap-single">
            <span className="text-sm font-semibold">{card.title[language]}</span>
            {card.badge === undefined ? null : <Tag className="bg-current/15 text-current">{card.badge}</Tag>}
          </span>
          <span className="text-xs leading-normal opacity-80">{card.description[language]}</span>
          <span className="font-mono text-[0.65rem] opacity-80">{card.facts.join(" · ")}</span>
        </button>
      ))}
    </Column>
  );
}

/** 🧍️ Poses and clips of an asset; `selection` null means its default clip. */
export function PosesAndClips(props: { readonly asset: Asset; readonly selection: Selection | null; readonly onSelect: (selection: Selection) => void }): ReactElement {
  const { asset, selection, onSelect } = props;
  const { t, language } = useArchive();
  const clipId = selection === null ? asset.presentation.defaultClip : selection.kind === "clip" ? selection.clip : null;
  const poseId = selection?.kind === "pose" ? selection.pose : null;
  const clips: Card[] = asset.clips.map((clip) => ({
    id: clip.id,
    title: clip.title,
    description: clip.description,
    facts: [`${num(clip.durationS, language)} s`, t("detail.clips.fps", { fps: clip.fps }), t(clip.loop ? "detail.clips.loop" : "detail.clips.once"), t(clip.rootMotion === "plant" ? "detail.clips.plant" : "detail.clips.noRoot"), ...(clip.advanceM === undefined ? [] : [t("detail.clips.advance", { distance: num(clip.advanceM, language) })])],
    ...(clip.id === asset.presentation.defaultClip ? { badge: t("detail.clips.default") } : {}),
  }));
  const poses: Card[] = namedPoses(asset).map((pose) => ({ id: pose.id, title: pose.title, description: pose.description, facts: [pose.ground === "auto" ? t("detail.poses.groundAuto") : t("detail.poses.groundAuthored"), t("detail.poses.contacts", { count: pose.contacts.length })], ...(pose.id === asset.presentation.defaultPose ? { badge: t("detail.clips.default") } : {}) }));
  return (
    <Section aria-labelledby="detail-poses" data-section="poses">
      <SectionTitle id="detail-poses">{t("detail.poses.title")}</SectionTitle>
      <CardGroup label={t("detail.poses.poses")} cards={poses} checked={poseId} onPick={(pose) => onSelect({ kind: "pose", pose })} />
      <SectionTitle level={3}>{t("detail.clips.title")}</SectionTitle>
      <CardGroup label={t("detail.clips.clips")} cards={clips} checked={clipId} onPick={(clip) => onSelect({ kind: "clip", clip })} />
    </Section>
  );
}

const EVENT_GLYPHS: Readonly<Record<ClipEvent["kind"], string>> = { "contact-on": "▼", "contact-off": "▲", ignition: "✦", shutdown: "■", "load-on": "⊕", "load-off": "⊖", thermal: "☼", motion: "→", operating: "⚙" };

/** ⏳️ The timeline of a clip with a cursor at `timeS`; `onSeek` receives seconds. */
export function ClipTimeline(props: { readonly asset: Asset; readonly clip: Clip; readonly timeS: number; readonly onSeek: (timeS: number) => void }): ReactElement {
  const { asset, clip, timeS, onSeek } = props;
  const { t, language } = useArchive();
  const { durationS, fps } = clip;
  const pct = (at: number): string => `${(Math.min(Math.max(at, 0), durationS) / durationS) * 100}%`;
  const trackName = (target: string): string => {
    const [kind, id = "", field = ""] = target.split(":");
    if (kind === "activityMet") return t("detail.track.activity");
    if (kind === "emitter") return asset.thermal.emitters.find((emitter) => emitter.id === id)?.title[language] ?? id.replaceAll("-", " ");
    if (kind === "region") return `${asset.thermal.regions.find((region) => region.id === id)?.title[language] ?? id.replaceAll("-", " ")} · ${field}`;
    if (kind === "load") return t("detail.track.load", { name: asset.poses.flatMap((pose) => pose.loads).find((load) => load.id === id)?.title[language] ?? id.replaceAll("-", " ") });
    if (kind === "operating") return t("detail.track.operating", { name: id.replaceAll("-", " ") });
    return target;
  };
  const seek = (at: number): void => onSeek(Math.min(Math.max(at, 0), durationS));
  const frames = Math.round(durationS * fps);
  const seconds = Array.from({ length: Math.floor(durationS) + 1 }, (_, second) => second);
  const anchors = [...new Set(clip.contacts.map((span) => span.anchor))];
  const label = (event: ClipEvent): string => `${event.title[language]}, ${t(`detail.event.${event.kind}`)}, ${num(event.t, language)} s`;
  return (
    <Section aria-labelledby="detail-timeline" data-section="timeline">
      <SectionTitle id="detail-timeline">{t("detail.timeline.title", { clip: clip.title[language] })}</SectionTitle>
      <SharedBorderRowCell className="px-double py-single"><input type="range" aria-label={t("detail.timeline.label")} aria-describedby="detail-timeline-help" aria-valuetext={`${num(timeS, language)} s`} min={0} max={durationS} step={1 / fps} value={Math.min(Math.max(timeS, 0), durationS)} onChange={(event) => seek(Number(event.target.value))} className="min-h-6 w-full accent-[var(--accent)]" /></SharedBorderRowCell>
      <div role="group" aria-label={t("detail.timeline.events")} data-shared-border-cell="" data-border-probe-art="" onClick={(event) => { const box = event.currentTarget.getBoundingClientRect(); if (box.width > 0 && !(event.target instanceof Element && event.target.closest("[data-event]"))) seek(((event.clientX - box.left) / box.width) * durationS); }} className="bg-panel px-double py-single">
        <div className="relative">
        <div className="relative h-6" aria-hidden="true">
          {frames <= 150 ? Array.from({ length: frames + 1 }, (_, frame) => <span key={frame} className="absolute top-0 h-1.5 border-l border-border" style={{ left: pct(frame / fps) }} />) : null}
          {seconds.map((second) => <span key={second} className="absolute top-0 h-3 border-l border-foreground/70 pl-px font-mono text-[0.65rem] text-muted-foreground" style={{ left: pct(second) }}><span className="relative top-3">{second} s</span></span>)}
        </div>
        <div className="relative mt-double h-6" data-row="phases">
          {clip.phases.map((phase) => <span key={phase.id} data-phase={phase.id} className="absolute inset-y-0 overflow-hidden border border-foreground/60 bg-muted/50 px-single text-[0.65rem] leading-6" style={{ left: pct(phase.fromS), width: `calc(${pct(phase.toS)} - ${pct(phase.fromS)})` }}>{phase.title[language]}</span>)}
        </div>
        <div className="relative mt-single h-7" data-row="events">
          {clip.events.map((event, at) => (
            <button key={at} type="button" data-shared-border-frame="" data-event={event.kind} aria-label={label(event)} title={label(event)} onClick={() => seek(event.t)} className="absolute top-0 min-h-6 min-w-6 -translate-x-1/2 border border-border bg-background text-xs leading-none" style={{ left: pct(event.t) }}>
              <span aria-hidden="true">{EVENT_GLYPHS[event.kind]}</span>
            </button>
          ))}
        </div>
        {anchors.map((anchor) => (
          <div key={anchor} className="relative mt-single h-3 bg-muted/30" data-contact={anchor} title={asset.anchors.find((candidate) => candidate.id === anchor)?.title[language] ?? anchor}>
            {clip.contacts.filter((span) => span.anchor === anchor).map((span, at) => <span key={at} className="absolute inset-y-0 bg-[repeating-linear-gradient(135deg,currentColor_0_2px,transparent_2px_5px)] text-foreground/70" style={{ left: pct(span.fromS), width: `calc(${pct(span.toS)} - ${pct(span.fromS)})` }} />)}
          </div>
        ))}
        <span aria-hidden="true" data-cursor="" className="pointer-events-none absolute inset-y-0 w-0 border-l-2 border-[var(--accent)]" style={{ left: pct(timeS) }} />
        </div>
      </div>
      <Note id="detail-timeline-help">{t("detail.timeline.help")}</Note>
      {clip.tracks.length === 0 ? null : <Cell><ul className="m-0 flex list-none flex-wrap gap-single p-0">{clip.tracks.map((track) => <li key={track.target}><Tag>{trackName(track.target)} · {track.keys.length} {t("detail.timeline.keys")}</Tag></li>)}</ul></Cell>}
      <details data-shared-border-cell="">
        <summary data-shared-border-frame="" data-shared-border-edges="" className="min-h-6 cursor-pointer px-double py-single text-xs">{t("detail.timeline.table")}</summary>
        <Column>
          <DataTable caption={t("detail.timeline.phases")} head={[t("detail.timeline.name"), t("detail.timeline.from"), t("detail.timeline.to")]}>
            {clip.phases.map((phase) => <Row key={phase.id} head={phase.title[language]} cells={[`${num(phase.fromS, language)} s`, `${num(phase.toS, language)} s`]} />)}
          </DataTable>
          <DataTable caption={t("detail.timeline.events")} head={[t("detail.timeline.name"), t("detail.timeline.kind"), t("detail.timeline.at")]}>
            {clip.events.map((event, at) => <Row key={at} head={event.title[language]} cells={[t(`detail.event.${event.kind}`), `${num(event.t, language)} s`]} />)}
          </DataTable>
          <DataTable caption={t("detail.timeline.contacts")} head={[t("detail.timeline.name"), t("detail.timeline.from"), t("detail.timeline.to")]}>
            {clip.contacts.map((span, at) => <Row key={at} head={asset.anchors.find((candidate) => candidate.id === span.anchor)?.title[language] ?? span.anchor} cells={[`${num(span.fromS, language)} s`, `${num(span.toS, language)} s`]} />)}
          </DataTable>
        </Column>
      </details>
    </Section>
  );
}
