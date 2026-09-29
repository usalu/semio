/** 🧩️ What every task interaction shares: its props, polite announcements of moves for screen readers, focus that
 * follows an item after it moved, and the pointer grip that starts a drag.
 */

import { useCallback, useLayoutEffect, useRef, useState, type PointerEvent, type ReactElement } from "react";
import type { Answer, SheetTask } from "@semio-tech/quiz";
import type { QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import { startPointerDrag } from "../🤏️drag/🟦️.ts";

/** 🧩️ The props of one task interaction: the presented task, the learner's current answer and where changes go. */
export interface TaskViewProps<T extends SheetTask, A extends Answer> {
  readonly task: T;
  readonly answer: A | undefined;
  readonly onAnswer: (answer: A) => void;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}

/** 🧱️ The frame of a drop zone (the pool, a category bin): a dashed hairline that the drag highlight fills. */
export const DROP_ZONE_CLASS = "flex min-w-0 flex-col gap-single border border-dashed border-normal p-double";

/** 🔽️ A select of a task: at least 24 px tall, framed by the hairline, on the page colour. */
export const SELECT_CLASS = "quiz-target border border-normal bg-background px-single text-sm text-foreground";

/** 🔘️ A small square action of a task row (move, unassign). */
export const ICON_BUTTON_CLASS =
  "quiz-target inline-flex min-w-[1.75em] cursor-pointer items-center justify-center border border-normal bg-transparent px-single text-foreground hover:bg-hover-interactive-fill aria-disabled:cursor-not-allowed aria-disabled:opacity-50";

/** 📢️ A message for the polite live region of a task, and the function that replaces it. */
export function useAnnouncement(): { readonly announcement: string; readonly announce: (message: string) => void } {
  const [announcement, setAnnouncement] = useState("");
  return { announcement, announce: setAnnouncement };
}

/** 📢️ The visually hidden polite live region announcing what a keyboard or pointer action changed. */
export function LiveRegion(props: { readonly message: string }): ReactElement {
  return (
    <p className="sr-only" role="status" aria-live="polite" aria-atomic="true">
      {props.message}
    </p>
  );
}

/** 🎯️ Asks to focus the element with a DOM id after the next render — for controls that moved or were re-created. */
export function useFocusAfterRender(): (id: string) => void {
  const pending = useRef<string | undefined>(undefined);
  useLayoutEffect(() => {
    const id = pending.current;
    if (id === undefined) return;
    pending.current = undefined;
    document.getElementById(id)?.focus();
  });
  return useCallback((id: string) => {
    pending.current = id;
  }, []);
}

/** 🔖️ A DOM id for one element of one task instance; slugs never contain the separator. */
export function elementId(scope: string, ...parts: readonly string[]): string {
  return [scope, ...parts].join("--");
}

/** 🤏️ The pointer grip of a draggable element; hidden from assistive technology, which uses the keyboard path. */
export function DragGrip(props: { readonly title: string; readonly onDrop: (zone: string) => void }): ReactElement {
  const { title, onDrop } = props;
  return (
    <span
      data-quiz-grip=""
      className="quiz-target inline-grid min-w-[1.75em] cursor-grab touch-none select-none place-items-center text-muted-foreground"
      aria-hidden="true"
      title={title}
      onPointerDown={(event: PointerEvent<HTMLSpanElement>) => startPointerDrag(event, onDrop)}
    >
      ⠿
    </span>
  );
}
