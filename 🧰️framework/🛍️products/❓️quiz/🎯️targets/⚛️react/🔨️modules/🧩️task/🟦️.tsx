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

/** 📢️ A message for the polite live region of a task, and the function that replaces it. */
export function useAnnouncement(): { readonly announcement: string; readonly announce: (message: string) => void } {
  const [announcement, setAnnouncement] = useState("");
  return { announcement, announce: setAnnouncement };
}

/** 📢️ The visually hidden polite live region announcing what a keyboard or pointer action changed. */
export function LiveRegion(props: { readonly message: string }): ReactElement {
  return (
    <p className="quiz-visually-hidden" role="status" aria-live="polite" aria-atomic="true">
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
    <span className="quiz-grip" aria-hidden="true" title={title} onPointerDown={(event: PointerEvent<HTMLSpanElement>) => startPointerDrag(event, onDrop)}>
      ⠿
    </span>
  );
}
