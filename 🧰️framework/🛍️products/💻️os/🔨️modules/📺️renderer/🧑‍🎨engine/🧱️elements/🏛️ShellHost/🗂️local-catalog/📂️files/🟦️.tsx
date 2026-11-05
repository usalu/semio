import * as React from "../../../⚛️react-port/🟦️.ts";
import type { LocalDocument } from "@semio-tech/framework/config/mutations";
import { localizedLabelTextV1 } from "../../📣️replay-refusal/🟦️.ts";

const labels = {
  title: { en: "Files on this device", de: "Dateien auf diesem Gerät" },
  opening: { en: "Opening {name}…", de: "{name} wird geöffnet…" },
  cancel: { en: "Cancel", de: "Abbrechen" },
} as const;

/** 📂️ Reopens locally acknowledged generic artifacts through their owning surface. */
export function LocalFilesV1(props: Readonly<{
  documents: readonly LocalDocument[];
  locale: string;
  activeDocumentId: string | null;
  persisted: ReadonlySet<string>;
  onOpen: (document: LocalDocument, signal: AbortSignal) => Promise<void>;
}>): React.ReactElement | null {
  const [pending, setPending] = React.useState<LocalDocument | null>(null);
  const [failure, setFailure] = React.useState<string | null>(null);
  const request = React.useRef<AbortController | null>(null);
  const mounted = React.useRef(false);
  React.useEffect(() => {
    mounted.current = true;
    return () => { mounted.current = false; request.current?.abort(); request.current = null; };
  }, []);
  const text = (key: keyof typeof labels) => localizedLabelTextV1(labels[key], props.locale);
  const documents = props.documents.filter(document => document.surface !== undefined).slice().sort((left, right) => right.admittedAtMs - left.admittedAtMs || (left.documentId < right.documentId ? -1 : left.documentId > right.documentId ? 1 : 0));
  const open = async (document: LocalDocument) => {
    if (request.current !== null) return;
    const controller = new AbortController();
    request.current = controller;
    setPending(document);
    setFailure(null);
    try { await props.onOpen(document, controller.signal); }
    catch (error) { if (mounted.current && !controller.signal.aborted) setFailure(error instanceof Error ? error.message : String(error)); }
    finally {
      if (mounted.current && request.current === controller) {
        request.current = null;
        setPending(null);
      }
    }
  };
  if (documents.length === 0) return null;
  return <section aria-label={text("title")} className="pointer-events-auto rounded-sm border bg-base px-double py-double text-sm">
    <h2 className="font-semibold">{text("title")}</h2>
    <ul>{documents.map(document => <li key={document.documentId}>
      <button type="button" className="rounded-sm px-single py-single focus-visible:outline"
        data-semio-local-document-id={document.documentId}
        data-semio-local-persisted={props.persisted.has(document.documentId) ? "true" : "false"}
        aria-current={props.activeDocumentId === document.documentId ? "true" : undefined}
        disabled={pending !== null}
        onClick={() => { void open(document); }}>{document.name}</button>
    </li>)}</ul>
    {pending === null ? null : <div>
      <p role="status" aria-live="polite" aria-busy="true">{text("opening").replaceAll("{name}", pending.name)}</p>
      <button type="button" onClick={() => request.current?.abort()}>{text("cancel")}</button>
    </div>}
    {failure === null ? null : <p role="alert">{failure}</p>}
  </section>;
}
