import * as React from "../../⚛️react-port/🟦️.ts";
import { localizedLabelTextV1 } from "../📣️replay-refusal/🟦️.ts";
import { isUnsavedV1, saveFailureTextV1, type SavedStateV1 } from "../../🛠️ShellHelpers/💾️saved-state/🟦️.ts";

export const CLOSE_GUARD_LABELS_V1 = {
  title: { en: "Unsaved changes", de: "Nicht gespeicherte Änderungen" },
  body: { en: "“{name}” has changes that were not saved. Close anyway and lose them?", de: "„{name}“ hat Änderungen, die nicht gespeichert wurden. Trotzdem schließen und sie verlieren?" },
  keep: { en: "Keep editing", de: "Weiter bearbeiten" },
  discard: { en: "Close and discard", de: "Schließen und verwerfen" },
  unsaved: { en: "Unsaved changes", de: "Nicht gespeicherte Änderungen" },
  retry: { en: "Retry saving", de: "Speichern wiederholen" },
  saving: { en: "Saving “{name}” before closing…", de: "„{name}“ wird vor dem Schließen gespeichert…" },
  cancel: { en: "Cancel", de: "Abbrechen" },
} as const;

/** 🗣️ One guard label in `locale`, with `{name}` filled; an unknown locale renders every language. */
export function closeGuardTextV1(key: keyof typeof CLOSE_GUARD_LABELS_V1, locale: string, name = ""): string {
  return localizedLabelTextV1(CLOSE_GUARD_LABELS_V1[key], locale).replaceAll("{name}", name);
}

/** 🚪️ Accessible confirmation shown when a close could lose edits the store never acknowledged; focus starts on the safe choice and Escape keeps the document. */
export function DocumentCloseDialog(props: Readonly<{ name: string; locale: string; failure: Readonly<{ code: string }> | null; saving?: boolean; onKeep(): void; onDiscard(): void }>): React.ReactElement {
  const keepRef = React.useRef<HTMLButtonElement>(null);
  const uid = React.useId();
  const titleId = `${uid}-title`;
  const bodyId = `${uid}-body`;
  React.useEffect(() => keepRef.current?.focus(), []);
  return (
    <div
      role="alertdialog"
      aria-modal="true"
      aria-labelledby={titleId}
      aria-describedby={bodyId}
      data-semio-close-guard=""
      onKeyDown={(event) => { if (event.key === "Escape") props.onKeep(); }}
      className="pointer-events-auto absolute left-1/2 top-1/3 z-50 -translate-x-1/2 rounded-sm border bg-base px-double py-double text-sm shadow-sm"
    >
      <p id={titleId} className="font-semibold">{closeGuardTextV1("title", props.locale)}</p>
      <p id={bodyId} aria-busy={props.saving === true}>
        {props.saving === true ? closeGuardTextV1("saving", props.locale, props.name) : closeGuardTextV1("body", props.locale, props.name)}
        {props.failure === null || props.saving === true ? null : ` ${saveFailureTextV1(props.failure, props.locale)}`}
      </p>
      <button type="button" ref={keepRef} onClick={props.onKeep}>{closeGuardTextV1(props.saving === true ? "cancel" : "keep", props.locale)}</button>
      {props.saving === true ? null : <button type="button" className="ml-single" onClick={props.onDiscard}>{closeGuardTextV1("discard", props.locale)}</button>}
    </div>
  );
}

/** 🔴️ The per-window unsaved marker, tied to the acknowledged save state and never to a text draft; a typed write failure is worded beside a retry. */
export function UnsavedMarker(props: Readonly<{ saved: SavedStateV1; locale: string; onRetry?: () => void }>): React.ReactElement | null {
  if (!isUnsavedV1(props.saved) || props.saved.kind !== "unsaved") return null;
  const failure = props.saved.failure;
  return (
    <span data-semio-unsaved-marker="" role="status" aria-live="polite">
      <span aria-hidden="true">●</span>
      <span className="sr-only">{closeGuardTextV1("unsaved", props.locale)}</span>
      {failure === null ? null : <span className="ml-single text-xs">{saveFailureTextV1(failure, props.locale)}</span>}
      {failure === null || props.onRetry === undefined ? null : <button type="button" className="ml-single underline" onClick={props.onRetry}>{closeGuardTextV1("retry", props.locale)}</button>}
    </span>
  );
}

/** 🔒️ Asks the browser to confirm leaving the page while any document holds edits the store never acknowledged. */
export function useBeforeUnloadGuardV1(unsaved: boolean): void {
  React.useEffect(() => {
    if (!unsaved) return undefined;
    const handler = (event: BeforeUnloadEvent): void => {
      event.preventDefault();
      event.returnValue = "";
    };
    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
  }, [unsaved]);
}

/** ⏱️ How long a person's close waits for the store to acknowledge before asking. */
export const DOCUMENT_CLOSE_FLUSH_TIMEOUT_MS_V1 = 10_000;

/** 🚪️ The close prompt the shell is showing: waiting for the save, or asking before a discard. */
export type ClosePromptV1 = Readonly<{ runtimeKey: string; name: string; phase: "saving" | "confirm"; failure: Readonly<{ code: string }> | null; onKeep(): void; onDiscard(): void }>;

export type UnsavedMarkerContextValueV1 = Readonly<{ saved: SavedStateV1; locale: string; onRetry?: () => void }>;

/** 🔴️ Carries the focused document's acknowledged saved state to every window header without threading it through each window builder. */
export const UnsavedMarkerContext = React.createContext<UnsavedMarkerContextValueV1>({ saved: { kind: "unknown" }, locale: "en" });

/** 🔴️ The window-chrome unsaved marker of the focused document. */
export function CurrentDocumentUnsavedMarker(): React.ReactElement | null {
  const value = React.useContext(UnsavedMarkerContext);
  return <UnsavedMarker saved={value.saved} locale={value.locale} {...(value.onRetry === undefined ? {} : { onRetry: value.onRetry })} />;
}
