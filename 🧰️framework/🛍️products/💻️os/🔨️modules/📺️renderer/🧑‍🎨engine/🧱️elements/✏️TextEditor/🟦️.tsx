
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/✏️TextEditor/component.tsx
/** 📝️ `✏️TextEditor` — the text/code document scene host: wasm editor-engine canvas surface
 * (grammar highlighting, hover tokens, F2 rename with multi-span preview, alt-click completions),
 * its context menu and keyboard-editing wiring, and the SSR-safe `Textarea`-based fallback. */
// #region 🔌️Adapters
import { useEffect, useMemo, useRef, useState } from "react";
import { WasmCanvas } from "@semio-tech/framework/canvas/react-renderer";
import { cn, ContextMenuController, glassClass, Textarea, useLabel } from "@semio-tech/ui-react";
import { textEditorActions, type ActionDescriptor, type ComponentSceneHostProps, type ContextMenuItemSpec, type PluginContextMenuRequest, type TextEditorScene } from "@semio-tech/framework";
import { useLocalDocumentOwnerV1, useLocalDocumentWindowV1 } from "../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { useClient } from "@semio-tech/framework-os-node-graph-rs/index";
import { openSurfaceContextMenu, mapContextMenuSpecs } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { parseSceneJsonField } from "../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { TextPeerCaretsOverlayV1 } from "../👕️canvas-presence/🟦️.tsx";
import type { EditorDiagnostic, GrammarToken, PickTarget, SpanRange, TextEditorExplicitDraftSettings, TextEditorExplicitDraftState, WasmEditorSurfaceProps } from "./🏷️editor-types/🟦️.ts";
import { acquireRetainedTextEditorDraftV1, publishRetainedTextEditorDraftV1, retainedTextEditorDraftKeyV1 } from "./📌️retained-drafts/🟦️.ts";
import { HighlightedBuffer, ignoreInputSinkChange } from "./🖍️buffer-and-edit-helpers/🟦️.tsx";
import { documentHistoryChord, isTextEditingKey } from "./🚫️edit-refusals-and-keys/🟦️.ts";
import { parseJsonOr } from "./📤️outbox-and-scene-packets/🟦️.ts";
import { acceptTextEditorExplicitDraftCancellation, beginTextEditorExplicitDraft, changeTextEditorExplicitDraft, createTextEditorExplicitDraft, parseTextEditorExplicitDraftSettings, reconcileTextEditorExplicitDraft, refuseTextEditorExplicitDraftCancellation, requestTextEditorExplicitDraftCancellation, settleTextEditorExplicitDraft, startTextEditorExplicitDraftOperation, textEditorExplicitDraftAction, textEditorExplicitDraftFailureV1 } from "./✍️explicit-draft-state/🟦️.ts";
import { useEditorSurfaceState } from "./🧰️surface-state-and-delivery/🟦️.ts";
import { useEditorSessionPresence } from "./👥️session-presence-and-caret/🟦️.ts";
import { useEditorCompletionsRenameAndMenu, type EditorCore3 } from "./🔤️completions-rename-and-menu/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️TextEditorHost
//#region WasmEditorSurface
/** 🖋️ The canvas text editor. It never paints on its own: the session {@link WasmCanvas} hands it is the
 * frame-demanding handle, where every call invalidates and the canvas paints once at the next frame. Its handlers used to
 * call `renderFrame` (a synchronous paint) after every change on top of that, and every echo of its own edit was synced
 * again even when its pack was byte-identical to the one already applied (the selection round trip), so one keystroke
 * painted 3.7–4.1 times — writer 4.13, trinity jack 3.67 per key, measured by the latency gate's paint hook (ticket
 * 26/09/23 F2). A scene pack is now synced once per session.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏱️interaction-latency/🟦️.ts */
export function WasmEditorSurface(props: WasmEditorSurfaceProps) {
  const core = {} as EditorCore3;
  Object.assign(core, useEditorSurfaceState(props));
  Object.assign(core, useEditorSessionPresence(core));
  Object.assign(core, useEditorCompletionsRenameAndMenu(core));
  const { surfaceId } = props;
  const { applyCompletion, cancelRename, caretScreenPosition, commitRename, completionIndex, completions, completionsOpen, contextMenu, contextMenuTitleLabel, descriptionId, dismissContextMenu, dispatch, editorLabel, emitSelection, invalid, lastHoverRangeRef, moveExplicitHistory, newlineGates, onSessionReady, openCompletions, presenceFrame, presenceWindowId, projectPeerCaret, publishCaretPresence, readOnly, readOnlyRef, renameDraft, renameInfo, renamePosition, requestContextMenu, scene, sessionEpoch, sessionFactory, sessionRef, setCompletionIndex, setCompletionsOpen, setContextMenu, setPresenceFrame, shellContextMenuFallback, sinkRef, startRename, surfaceRef, textEditorMenuKeysByActionId, updateRenamePreview , presenceRuntimeKey } = core;

  return (
    <div ref={surfaceRef} className="relative min-h-0 flex-1">
      <WasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <TextPeerCaretsOverlayV1 windowId={presenceWindowId} project={projectPeerCaret} frame={presenceFrame + sessionEpoch} lineHeightPx={16} locale={typeof document !== "undefined" ? document.documentElement.lang : undefined} />
      <div
        className="absolute inset-0"
        onPointerDown={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          if (event.detail >= 2) {
            session.selectSpanAtScreen(sx, sy);
            emitSelection();
            return;
          }
          session.pointerDownScreen(sx, sy, event.button);
          emitSelection();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          try {
            const hover = JSON.parse(session.hoverTokenRangeJson()) as SpanRange | null;
            const changed = (hover?.start ?? null) !== (lastHoverRangeRef.current?.start ?? null) || (hover?.end ?? null) !== (lastHoverRangeRef.current?.end ?? null);
            if (changed) {
              lastHoverRangeRef.current = hover;
              if (hover) {
                session.setHoverRange(hover.start, hover.end);
                dispatch(textEditorActions.hover, { start: hover.start, end: hover.end });
              }
            }
          } catch {}
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          emitSelection();
        }}
        onPointerCancel={() => {
          const session = sessionRef.current;
          if (!session) return;
          session.pointerCancelScreen();
        }}
        onWheel={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          event.preventDefault();
          session.wheelScrollScreen(event.deltaY);
          dispatch("setCamera", { camera: JSON.parse(session.cameraJson()) });
          publishCaretPresence();
          setPresenceFrame((frame) => frame + 1);
        }}
        onContextMenu={(event) => {
          event.preventDefault();
          event.stopPropagation();
          void (async () => {
          const session = sessionRef.current;
          if (!session) return;
          dismissContextMenu();
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          session.pointerDownScreen(sx, sy, 0);
          session.pointerUpScreen(sx, sy, 0);
          emitSelection();
          if (event.altKey && completions.length > 0) {
            openCompletions();
            return;
          }
          const pickTargets = parseJsonOr<readonly PickTarget[]>(session.pickTargetsAtScreenJson(sx, sy), []);
          const hasSelection = session.anchor() !== session.caret();
          const localActions: Record<string, () => void> = {
            requestCompletions: openCompletions,
            selectToken: () => {
              session.selectSpanAt(session.caret());
              emitSelection();
            },
            selectLine: () => {
              const caret = session.caret();
              const buffer = scene.buffer;
              const lines = buffer.split("\n");
              let offset = 0;
              let lineIndex = 0;
              for (let i = 0; i < lines.length; i++) {
                const next = offset + (lines[i]?.length ?? 0) + 1;
                if (caret < next || i === lines.length - 1) {
                  lineIndex = i;
                  break;
                }
                offset = next;
              }
              offset = 0;
              for (let i = 0; i < lineIndex; i++) offset += (lines[i]?.length ?? 0) + 1;
              const lineLength = lines[lineIndex]?.length ?? 0;
              session.setSelectionRange(offset, offset + lineLength);
              emitSelection();
            },
            selectAll: () => {
              session.setSelectionRange(0, scene.buffer.length);
              emitSelection();
            },
            commitRename: () => {
              if (renameInfo) startRename();
            },
            cut: () => {
              document.execCommand("cut");
            },
            copy: () => {
              document.execCommand("copy");
            },
            paste: () => {
              document.execCommand("paste");
            },
            formatDocument: () => dispatch(textEditorActions.formatDocument ?? "formatDocument"),
            lintDocument: () => dispatch("lintDocument"),
          };
          for (const target of pickTargets) {
            localActions[`pick:${target.domain}:${target.id}`] = () => {};
          }
          const dispatchTextEditorMenu = (action: string, args?: Record<string, unknown>) => {
            if (localActions[action]) {
              localActions[action]!();
              return;
            }
            dispatch(action, args);
          };
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "textEditor", args: null },
              surface: {
                surfaceId,
                kind: "textEditor",
                hits: pickTargets.map((target) => ({ domain: target.domain, id: target.id, label: target.label })),
                selection: [],
                text: {
                  caret: session.caret(),
                  hasSelection,
                  word: undefined,
                  canRename: renameInfo != null,
                  hasCompletions: completions.length > 0,
                },
              },
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => mapContextMenuSpecs(specs, dispatchTextEditorMenu, textEditorMenuKeysByActionId),
            shellContextMenuFallback,
          );
          setContextMenu({ position: { x: event.clientX, y: event.clientY }, ...menu });
          })();
        }}
      >
        {renameDraft ? (
          <input
            className={cn("pointer-events-auto absolute z-50 min-w-[12rem] rounded border border-border px-2 py-1 font-mono text-xs text-foreground shadow-md", glassClass)}
            data-level="pane"
            style={renamePosition ? { left: renamePosition.x, top: renamePosition.y - 4 } : { left: 12, top: 12 }}
            value={renameDraft.text}
            autoFocus
            onChange={(event) => updateRenamePreview(event.target.value)}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === "Escape") {
                event.preventDefault();
                cancelRename();
                return;
              }
              if (event.key === "Enter") {
                event.preventDefault();
                commitRename();
              }
            }}
            onBlur={commitRename}
          />
        ) : null}
        {completionsOpen && completions.length > 0
          ? (() => {
              const session = sessionRef.current;
              const position = session ? caretScreenPosition(session) : null;
              return (
                <div className={cn("pointer-events-auto absolute z-50 max-h-48 min-w-40 overflow-auto rounded border border-border p-1 shadow-md", glassClass)} data-level="menu" style={position ? { left: position.x, top: position.y + 18 } : { left: 12, top: 12 }}>
                  {completions.map((item, index) => (
                    <button
                      key={`${item.label}-${index}`}
                      type="button"
                      className={`block w-full rounded px-2 py-1 text-left font-mono text-[0.6875rem] ${index === completionIndex ? "bg-accent text-accent-foreground" : "hover:bg-active-base"}`}
                      onPointerDown={(event) => event.stopPropagation()}
                      onClick={() => applyCompletion(item)}
                    >
                      <span>{item.label}</span>
                      {item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
                    </button>
                  ))}
                </div>
              );
            })()
          : null}
      </div>
      <ContextMenuController title={contextMenuTitleLabel} open={contextMenu != null} position={contextMenu?.position ?? null} items={contextMenu?.items ?? []} onOpenChange={(open) => !open && dismissContextMenu()} />
      <textarea
        ref={sinkRef}
        className="absolute inset-0 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground opacity-0"
        value={scene.buffer}
        onChange={ignoreInputSinkChange}
        onCompositionEnd={(event) => {
          const session = sessionRef.current;
          if (!session || readOnlyRef.current || event.data.length === 0) return;
          session.insertText(event.data);
          emitSelection();
        }}
        onPaste={(event) => {
          const session = sessionRef.current;
          const pasted = event.clipboardData.getData("text/plain");
          event.preventDefault();
          if (!session || readOnlyRef.current || pasted.length === 0) return;
          session.replaceSelection(pasted);
          emitSelection();
        }}
        onCopy={(event) => {
          const selected = sessionRef.current?.selectionText() ?? "";
          if (selected.length === 0) return;
          event.preventDefault();
          event.clipboardData.setData("text/plain", selected);
        }}
        onCut={(event) => {
          const session = sessionRef.current;
          const selected = session?.selectionText() ?? "";
          if (!session || selected.length === 0) return;
          event.preventDefault();
          event.clipboardData.setData("text/plain", selected);
          if (readOnlyRef.current) return;
          session.replaceSelection("");
          emitSelection();
        }}
        onKeyDown={(event) => {
          if (event.nativeEvent.isComposing || event.key === "Process") return;
          const history = documentHistoryChord(event);
          if (history) {
            event.preventDefault();
            if (moveExplicitHistory(history)) return;
            dispatch(history, {});
            return;
          }
          if (readOnlyRef.current && isTextEditingKey(event)) {
            event.preventDefault();
            return;
          }
          const session = sessionRef.current;
          if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch("submit", {});
            return;
          }
          if ((event.key === " " || event.code === "Space") && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            openCompletions();
            return;
          }
          if (event.key === "F2" && renameInfo) {
            event.preventDefault();
            startRename();
            return;
          }
          if (event.key.toLowerCase() === "a" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            session?.selectAll();
            emitSelection();
            return;
          }
          if (event.key.toLowerCase() === "f" && event.shiftKey && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch(textEditorActions.formatDocument, {});
            return;
          }
          if (!session) return;

          if (completionsOpen && completions.length > 0) {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              setCompletionIndex((index) => (index + 1) % completions.length);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              setCompletionIndex((index) => (index - 1 + completions.length) % completions.length);
              return;
            }
            if (event.key === "Tab" || event.key === "Enter") {
              event.preventDefault();
              applyCompletion(completions[completionIndex] ?? completions[0]!);
              return;
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setCompletionsOpen(false);
              return;
            }
          }

          const extend = event.shiftKey;
          if (event.key === "ArrowLeft") {
            event.preventDefault();
            session.moveLeft(extend);
            emitSelection();
            return;
          }
          if (event.key === "ArrowRight") {
            event.preventDefault();
            session.moveRight(extend);
            emitSelection();
            return;
          }
          if (event.key === "ArrowUp") {
            event.preventDefault();
            session.moveUp(extend);
            emitSelection();
            return;
          }
          if (event.key === "ArrowDown") {
            event.preventDefault();
            session.moveDown(extend);
            emitSelection();
            return;
          }
          if (event.key === "Home") {
            event.preventDefault();
            session.moveLineStart(extend);
            emitSelection();
            return;
          }
          if (event.key === "End") {
            event.preventDefault();
            session.moveLineEnd(extend);
            emitSelection();
            return;
          }
          if (event.key === "Tab") {
            event.preventDefault();
            session.insertText(session.tabInsertText());
            emitSelection();
            return;
          }
          if (event.key === "Enter") {
            event.preventDefault();
            const allowed = newlineGates == null || newlineGates.has(session.caret());
            if (allowed) {
              session.insertText("\n");
              emitSelection();
            }
            return;
          }
          if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
            event.preventDefault();
            session.insertText(event.key);
            emitSelection();
            return;
          }
          if (event.key === "Backspace") {
            event.preventDefault();
            session.backspace();
            emitSelection();
            return;
          }
          if (event.key === "Delete") {
            event.preventDefault();
            session.deleteForward();
            emitSelection();
          }
        }}
        spellCheck={false}
        aria-readonly={readOnly}
        data-read-only={readOnly ? "" : undefined}
        aria-label={editorLabel}
        aria-describedby={descriptionId}
        aria-invalid={invalid || undefined}
      />
    </div>
  );
}

//#endregion WasmEditorSurface
//#region TextEditorHost
export function TextEditorHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.textEditor;
  const isClient = useClient();
  const localDocumentOwner = useLocalDocumentOwnerV1();
  const localDocumentWindow = useLocalDocumentWindowV1() ?? node.surfaceId;
  const explicitDraft = useMemo(() => parseTextEditorExplicitDraftSettings(scene?.settingsJson), [scene?.settingsJson]);
  const draftScope = JSON.stringify([node.controllerId, node.surfaceId, explicitDraft?.editAction, explicitDraft?.editArgument, Object.entries(explicitDraft?.editArguments ?? {}).filter(([key]) => key !== "revision")]);
  const publication = { scope: draftScope, buffer: scene?.buffer ?? "", revision: explicitDraft?.publicationRevision ?? "" };
  const publicationRef = useRef(publication);
  publicationRef.current = publication;
  const mountedRef = useRef(true);
  const retainsExplicitDraft = explicitDraft !== null && localDocumentOwner?.active === true;
  const retainedDraft = useMemo(() => retainsExplicitDraft && localDocumentOwner
    ? acquireRetainedTextEditorDraftV1(localDocumentOwner, retainedTextEditorDraftKeyV1(localDocumentWindow, draftScope), createTextEditorExplicitDraft(draftScope, publication.buffer, publication.revision))
    : null, [draftScope, localDocumentOwner, localDocumentWindow, retainsExplicitDraft]);
  const [draftState, setDraftState] = useState<TextEditorExplicitDraftState>(() => retainedDraft?.state ?? createTextEditorExplicitDraft(draftScope, publication.buffer, publication.revision));
  const draftStateRef = useRef(draftState);
  const updateDraft = (next: TextEditorExplicitDraftState) => {
    draftStateRef.current = next;
    if (retainedDraft) publishRetainedTextEditorDraftV1(retainedDraft, next);
    if (mountedRef.current) setDraftState(next);
  };
  useEffect(() => {
    if (!retainedDraft) return;
    const receive = (next: TextEditorExplicitDraftState) => {
      if (!mountedRef.current) return;
      draftStateRef.current = next;
      setDraftState(next);
    };
    retainedDraft.listeners.add(receive);
    draftStateRef.current = retainedDraft.state;
    setDraftState(retainedDraft.state);
    return () => { retainedDraft.listeners.delete(receive); };
  }, [retainedDraft]);
  useEffect(() => {
    if (!scene || !explicitDraft) return;
    updateDraft(reconcileTextEditorExplicitDraft(draftStateRef.current, draftScope, scene.buffer, explicitDraft.publicationRevision));
  }, [draftScope, explicitDraft?.publicationRevision, scene?.buffer]);
  useEffect(() => {
    mountedRef.current = true;
    return () => { mountedRef.current = false; };
  }, []);
  const tokens = useMemo((): readonly GrammarToken[] => {
    if (!scene?.tokensJson) return [];
    try {
      return parseSceneJsonField<GrammarToken[]>(scene.tokensJson);
    } catch {
      return [];
    }
  }, [scene?.tokensJson]);
  const diagnostics = useMemo((): readonly EditorDiagnostic[] => {
    if (!scene?.diagnosticsJson) return [];
    try {
      return parseSceneJsonField<EditorDiagnostic[]>(scene.diagnosticsJson);
    } catch {
      return [];
    }
  }, [scene?.diagnosticsJson]);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const documentPlaceholderLabel = useLabel("ui.host.documentPlaceholder");
  const languageDocumentLabel = useLabel("ui.host.languageDocument", { language: scene?.language });

  if (!scene) return <div className="semio-text-editor-empty">{emptySceneLabel}</div>;

  const currentDraft = draftState.surfaceId === draftScope ? draftState : createTextEditorExplicitDraft(draftScope, scene.buffer, publication.revision);
  const draft = explicitDraft ? currentDraft.draft : scene.buffer;
  const pending = currentDraft.pending !== null;
  const failure = explicitDraft && (currentDraft.conflicted || currentDraft.failed) ? currentDraft.conflicted ? explicitDraft.conflictLabel : textEditorExplicitDraftFailureV1(explicitDraft, currentDraft.diagnostic) : null;
  const statusId = `${node.surfaceId}.draft-status`;
  const displayedScene = explicitDraft ? { ...scene, buffer: draft } : scene;
  const changeDraft = (text: string) => {
    updateDraft(changeTextEditorExplicitDraft(draftStateRef.current, draftScope, scene.buffer, text, publication.revision));
  };
  const discardDraft = () => {
    if (!draftStateRef.current.pending) updateDraft(createTextEditorExplicitDraft(draftScope, scene.buffer, publication.revision));
  };
  const applyDraft = async () => {
    if (!explicitDraft) return;
    const before = reconcileTextEditorExplicitDraft(draftStateRef.current, draftScope, scene.buffer, publication.revision);
    const next = beginTextEditorExplicitDraft(before);
    if (next === before || !next.pending) return;
    const owner = next.pending;
    updateDraft(next);
    let outcome: unknown;
    try {
      outcome = await onAction({ ...textEditorExplicitDraftAction(explicitDraft, owner.value, owner.changes), controllerId: node.controllerId }, { started: (operation) => {
        const current = retainedDraft?.state ?? draftStateRef.current;
        updateDraft(startTextEditorExplicitDraftOperation(current, owner, operation));
      } });
    } catch { outcome = null; }
    const latest = publicationRef.current;
    const settled = settleTextEditorExplicitDraft(retainedDraft?.state ?? draftStateRef.current, owner, outcome, latest.scope, latest.buffer, latest.revision);
    if (retainedDraft?.owner.active) publishRetainedTextEditorDraftV1(retainedDraft, settled);
    else if (mountedRef.current) updateDraft(settled);
  };
  const cancelDraft = async () => {
    const before = retainedDraft?.state ?? draftStateRef.current;
    const owner = before.pending;
    const operation = before.operation;
    if (!owner || !operation) return;
    const requested = requestTextEditorExplicitDraftCancellation(before, owner);
    if (requested === before) return;
    updateDraft(requested);
    let outcome: unknown;
    try { outcome = await operation.cancel(); } catch { outcome = null; }
    const current = retainedDraft?.state ?? draftStateRef.current;
    const latest = publicationRef.current;
    const accepted = typeof outcome === "object" && outcome !== null && (outcome as { readonly kind?: unknown }).kind === "applied";
    updateDraft(accepted ? acceptTextEditorExplicitDraftCancellation(current, owner) : refuseTextEditorExplicitDraftCancellation(current, owner, latest.scope, latest.buffer, latest.revision));
  };

  return (
    <div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col ui-surface" data-level="base" data-surface-id={node.surfaceId} data-dirty={explicitDraft && currentDraft.dirty ? "true" : "false"} aria-busy={explicitDraft && pending || undefined}>
      {isClient ? (
        <WasmEditorSurface scene={displayedScene} controllerId={node.controllerId} surfaceId={node.surfaceId} onAction={onAction} explicitDraft={explicitDraft} explicitDraftDirty={currentDraft.dirty} descriptionId={failure ? statusId : undefined} invalid={Boolean(failure)} onDraftChange={changeDraft} requestContextMenu={requestContextMenu} />
      ) : (
        <div className="relative min-h-0 flex-1">
          <HighlightedBuffer buffer={draft} tokens={tokens} />
          <Textarea
            className="relative min-h-0 flex-1 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground"
            id={`${node.surfaceId}.editor`}
            lazy
            rows={24}
            value={draft}
            placeholder={scene.language ? languageDocumentLabel : documentPlaceholderLabel}
            aria-label={scene.language ? languageDocumentLabel : documentPlaceholderLabel}
            aria-describedby={failure ? statusId : undefined}
            aria-invalid={Boolean(failure) || undefined}
            onLazyChange={(value) => explicitDraft ? changeDraft(value) : onAction({ controllerId: node.controllerId, action: textEditorActions.edit, args: { surfaceId: node.surfaceId, text: value } })}
          />
        </div>
      )}
      {explicitDraft ? (
        <div className="flex flex-wrap items-center justify-end gap-2 border-t border-border px-3 py-2">
          {failure ? <span id={statusId} role="alert" className="mr-auto break-words text-xs text-destructive">{failure}</span> : null}
          {pending ? <span role="status" className="mr-auto text-xs text-muted-foreground">{explicitDraft.applyingLabel}</span> : null}
          {pending ? <button type="button" className="shrink-0 rounded border border-border px-3 py-1 text-xs text-foreground disabled:opacity-50" disabled={currentDraft.operation === null || currentDraft.cancelling} aria-busy={currentDraft.cancelling || undefined} onClick={() => void cancelDraft()}>{explicitDraft.cancelLabel}</button> : null}
          <button type="button" className="shrink-0 rounded border border-border px-3 py-1 text-xs text-foreground disabled:opacity-50" disabled={!currentDraft.dirty || pending} onClick={discardDraft}>{explicitDraft.discardLabel}</button>
          <button type="button" className="shrink-0 rounded bg-accent px-3 py-1 text-xs text-accent-foreground disabled:opacity-50" disabled={!currentDraft.dirty || currentDraft.conflicted || pending} onClick={() => void applyDraft()}>{explicitDraft.applyLabel}</button>
        </div>
      ) : null}
      {diagnostics.length > 0 ? (
        <div className="border-t border-border px-3 py-2 text-[0.6875rem] text-muted-foreground">
          {diagnostics.slice(0, 4).map((diag, index) => (
            <div key={`${diag.start}-${diag.end}-${index}`} className="truncate">
              {diag.severity ? `[${diag.severity}] ` : ""}
              {diag.message}
            </div>
          ))}
        </div>
      ) : null}
    </div>
  );
}
//#endregion TextEditorHost
//#endregion 🔖️TextEditorHost
