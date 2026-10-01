/** 🗳️ What the others think, item by item. Sheets differ per learner, so everything is semantic, keyed by item id,
 * never by position: for a classification who put an item in which category, for a sorting where the others place it,
 * for a matching which values the others give it. The crowd shown is live — the
 * drafts of the learners thinking along in the same quiz right now, each in their presence colour — or, when nobody else
 * is, every submitted run of that quiz (the learner's own included, so it speaks of everyone). Every figure has a
 * sentence for assistive technology; nothing here is a live region, so updates never interrupt.
 *
 * @see ../../../../🧬️schema/🔣️.json — `CrowdView`, `ThinkingState`
 * @see ../../../../🔨️modules/👥️presence/🟦️.ts — `thinkingCrowd`, the live aggregation
 */

import { createContext, useContext, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { thinkingCrowd, type CrowdView, type SheetTask, type ThinkingState, type ThinkingTask } from "@semio-tech/quiz";
import type { QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import { formatNumber } from "../📏️quantity/🟦️.ts";
import { cn } from "../🪟️chrome/🟦️.tsx";
import { paintStyle } from "../👥️presence/🟦️.tsx";

/** 🎟️ One category or value the others gave an item, how often, and (live) by whom. */
export interface CrowdChoice {
  readonly key: string;
  readonly count: number;
  readonly tags: readonly string[];
}

/** 🙋️ What the others think of one item: how many answered it (and live, who), their choices by category or value,
 * and for a sorting their normalized positions (0 smallest … 1 largest; live per learner, submitted their mean). */
export interface ItemCrowd {
  readonly item: string;
  readonly answers: number;
  readonly tags: readonly string[];
  readonly choices: readonly CrowdChoice[];
  readonly positions: readonly { readonly position: number; readonly tag?: string }[];
}

/** 🗳️ A crowd to show: `live` drafts of `count` learners thinking along, or `count` `submitted` runs; `items` answers
 * what the crowd thinks of the items of one sheet task (of one dimension for a matching). */
export interface Crowd {
  readonly source: "live" | "submitted";
  readonly count: number;
  readonly items: (task: SheetTask, dimension?: string) => ReadonlyMap<string, ItemCrowd>;
}

function byCount(left: CrowdChoice, right: CrowdChoice): number {
  const [a, b] = [Number(left.key), Number(right.key)];
  if (right.count !== left.count) return right.count - left.count;
  if (Number.isFinite(a) && Number.isFinite(b)) return a - b;
  return left.key < right.key ? -1 : left.key > right.key ? 1 : 0;
}

/** 🫧️ The items of a live crowd task. */
export function liveItems(tasks: readonly ThinkingTask[], dimension?: string): ReadonlyMap<string, ItemCrowd> {
  const task = tasks.find((entry) => entry.dimension === dimension);
  return new Map(
    (task?.items ?? []).map((item) => [
      item.item,
      {
        item: item.item,
        answers: item.tags.length,
        tags: item.tags,
        choices: (item.votes ?? []).map((vote) => ({ key: vote.key, count: vote.tags.length, tags: vote.tags })).sort(byCount),
        positions: (item.positions ?? []).map((position) => ({ position: position.position, tag: position.tag })),
      },
    ]),
  );
}

/** 📚️ The items of a submitted crowd task. */
export function submittedItems(view: CrowdView, task: string, dimension?: string): ReadonlyMap<string, ItemCrowd> {
  const entry = view.tasks.find((candidate) => candidate.task === task && candidate.dimension === dimension);
  return new Map(
    (entry?.items ?? []).map((item) => [
      item.item,
      {
        item: item.item,
        answers: item.answers,
        tags: [],
        choices: (item.counts ?? []).map((count) => ({ key: count.key, count: count.count, tags: [] })).sort(byCount),
        positions: item.meanPosition === undefined ? [] : [{ position: item.meanPosition }],
      },
    ]),
  );
}

/** 🗳️ The crowd to show: the live one while anyone else thinks along (`others`, the thinking states of the others),
 * else the submitted one once a run of the quiz was submitted, else none. */
export function chooseCrowd(others: readonly ThinkingState[], submitted: CrowdView | undefined): Crowd | undefined {
  const thinkers = new Set(others.map((state) => state.tag)).size;
  if (thinkers > 0) return { source: "live", count: thinkers, items: (task, dimension) => liveItems(thinkingCrowd(others, task), dimension) };
  if (submitted !== undefined && submitted.runs > 0) return { source: "submitted", count: submitted.runs, items: (task, dimension) => submittedItems(submitted, task.id, dimension) };
  return undefined;
}

/** 📍️ The place (1 … `total`) a normalized position stands for, rounded half up to one decimal. */
export function crowdPlace(position: number, total: number): number {
  return Math.round((1 + position * Math.max(0, total - 1)) * 10) / 10;
}

/** ⚖️ The mean of the positions of an item, if any. */
export function meanPosition(item: ItemCrowd | undefined): number | undefined {
  if (item === undefined || item.positions.length === 0) return undefined;
  return item.positions.reduce((sum, entry) => sum + entry.position, 0) / item.positions.length;
}

interface CrowdContextValue {
  readonly crowd: Crowd | undefined;
  readonly paint: (tag: string) => CSSProperties | undefined;
}

const CrowdContext = createContext<CrowdContextValue>({ crowd: undefined, paint: () => undefined });

/** 🗳️ Makes `crowd` what the task views below show (none: the learner thinks alone); `paint` colours a learner by tag. */
export function CrowdProvider(props: { readonly crowd: Crowd | undefined; readonly colours?: ReadonlyMap<string, number>; readonly children: ReactNode }): ReactElement {
  const { colours } = props;
  const paint = (tag: string): CSSProperties | undefined => {
    const colour = colours?.get(tag);
    return colour === undefined ? undefined : paintStyle(colour);
  };
  return <CrowdContext.Provider value={{ crowd: props.crowd, paint }}>{props.children}</CrowdContext.Provider>;
}

/** 🗳️ The crowd the task views show, if any, and how to colour a learner. */
export function useCrowd(): CrowdContextValue {
  return useContext(CrowdContext);
}

/** 🏷️ Where a crowd comes from: live with how many think along, or submitted with how many runs. */
export function CrowdSource(props: { readonly crowd: Crowd; readonly text: QuizText }): ReactElement {
  const { crowd, text } = props;
  return (
    <p data-crowd-source={crowd.source} className="m-0 flex flex-wrap items-center gap-x-single text-xs text-muted-foreground">
      <span aria-hidden="true">👥</span>
      <span className="font-semibold text-foreground">{text(crowd.source === "live" ? "quiz.crowd.liveTitle" : "quiz.crowd.pastTitle")}</span>
      <span>· {text(crowd.source === "live" ? "quiz.crowd.thinking" : "quiz.crowd.runs", { count: crowd.count })}</span>
    </p>
  );
}

function Dots(props: { readonly tags: readonly string[] }): ReactElement | null {
  const { paint } = useCrowd();
  const painted = props.tags.flatMap((tag) => {
    const style = paint(tag);
    return style === undefined ? [] : [{ tag, style }];
  });
  if (painted.length === 0) return null;
  return (
    <span aria-hidden="true" className="inline-flex items-center gap-[0.125rem]">
      {painted.map(({ tag, style }) => (
        <span key={tag} className="quiz-online !m-0" style={style} />
      ))}
    </span>
  );
}

/** 🎟️ How the others answered one item by category or value: compact counts (live with each learner's colour), most
 * given first, and the sentence. */
export function CrowdChoices(props: { readonly item: ItemCrowd | undefined; readonly subject: string; readonly label: (key: string) => string; readonly text: QuizText; readonly className?: string }): ReactElement | null {
  const { item, subject, label, text } = props;
  const live = useCrowd().crowd?.source !== "submitted";
  if (item === undefined || item.choices.length === 0) return null;
  const choices = item.choices.map((entry) => text("quiz.crowd.choice", { label: label(entry.key), count: entry.count })).join(", ");
  return (
    <p data-crowd-item={item.item} className={cn("quiz-crowd m-0 flex flex-wrap items-center gap-single text-xs text-muted-foreground", props.className)}>
      <span className="sr-only">{text(live ? "quiz.crowd.item" : "quiz.crowd.itemAll", { item: subject, choices })}</span>
      <span aria-hidden="true">👥</span>
      {item.choices.map((entry) => (
        <span key={entry.key} aria-hidden="true" className="quiz-nowrap inline-flex items-center gap-[0.1875rem] border border-normal px-single">
          <Dots tags={entry.tags} />
          {text("quiz.crowd.choice", { label: label(entry.key), count: entry.count })}
        </span>
      ))}
    </p>
  );
}

/** 📍️ Where the others place one item of a sorting: their markers on a short track (live in their colours, submitted
 * the mean), the average place, and the sentence. */
export function CrowdPosition(props: { readonly item: ItemCrowd | undefined; readonly total: number; readonly subject: string; readonly text: QuizText; readonly locale: QuizLocale; readonly className?: string }): ReactElement | null {
  const { item, total, subject, text, locale } = props;
  const { paint, crowd } = useCrowd();
  const mean = meanPosition(item);
  if (item === undefined || mean === undefined) return null;
  const position = formatNumber(crowdPlace(mean, total), locale);
  return (
    <p data-crowd-item={item.item} className={cn("quiz-crowd m-0 flex items-center gap-single text-xs text-muted-foreground", props.className)}>
      <span className="sr-only">{text(crowd?.source === "submitted" ? "quiz.crowd.positionAll" : "quiz.crowd.position", { item: subject, position, total })}</span>
      <span aria-hidden="true">👥</span>
      <span aria-hidden="true" className="quiz-crowd-track">
        {item.positions.map((entry, index) => (
          <span key={entry.tag ?? index} className="quiz-crowd-marker" style={{ ["--quiz-crowd-at" as string]: String(entry.position), ...(entry.tag === undefined ? {} : paint(entry.tag)) } as CSSProperties} />
        ))}
      </span>
      <span aria-hidden="true" className="quiz-nowrap tabular-nums">
        {text("quiz.crowd.positionShort", { position })}
      </span>
    </p>
  );
}
