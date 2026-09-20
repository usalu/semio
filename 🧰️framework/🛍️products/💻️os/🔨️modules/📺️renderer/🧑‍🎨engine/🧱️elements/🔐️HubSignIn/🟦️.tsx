// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔐️HubSignIn/component.tsx
/** @emoji 🔐️ `🔐️HubSignIn` — the shell's hub connection and sign-in pane: choose between the local
 * bootstrap hub and any remote hub this device knows, sign in with an email and password against
 * `POST /auth/sessions`, cancel a slow attempt, sign out, and re-authenticate an expired session.
 * Purely presentational — every effect is a callback prop, so the same element renders under a fake
 * transport in tests and under `useHubConnection` in the shell. Ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2.
 *
 * Laws this element encodes: the app is never blocked by a hub — an unreachable or refused hub
 * leaves a visible "working on this device only" affordance; the password field is structurally
 * absent when the hub answered `credential-sign-in-disabled`; and no denial ever reveals whether an
 * account exists (AU1 §1.1's single `invalid-credentials` class).
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useId, useRef, useState, type FormEvent, type ReactElement } from "react";
import { Button, useLabel } from "@semio-tech/ui-react";
import {
  HUB_SIGN_IN_PASSWORD_MIN_BYTES,
  LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1,
  hubSignInErrorTextV1,
  hubSignInTextV1,
  validHubSignInEmailV1,
  validHubSignInPasswordV1,
  type HubConnectionBookV1,
  type HubSessionStateV1,
  type HubSignInErrorCodeV1,
} from "../../../../📇️directory/🔐️sign-in/🟦️.ts";
import { hubUiLabel } from "../🔗️HubConnection/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Types
export interface HubSignInPaneProps {
  readonly book: HubConnectionBookV1;
  readonly session: HubSessionStateV1;
  /** 🌐️ The locale the closed error table is resolved against. `en`/`de` only — an unowned locale
   * is a thrown refusal, never a silent English fallback. */
  readonly locale: string;
  readonly onSelectConnection: (id: string) => void;
  readonly onAddHub: (typed: string, label: string) => "invalid-origin" | HubSignInErrorCodeV1 | null;
  readonly onForgetHub: (id: string) => void;
  readonly onSignIn: (credential: Readonly<{ email: string; password: string }>) => void;
  readonly onCancel: () => void;
  readonly onSignOut: () => void;
}
//#endregion 🔖️Types

//#region 🔖️Admission
/** 🚦️ Whether the submit button may fire: a well-formed credential, no attempt in flight, and not
 * inside a rate-limit window the hub asked us to respect. */
export function hubSignInSubmittableV1(session: HubSessionStateV1, email: string, password: string): boolean {
  if (session.phase === "signing-in" || session.phase === "signing-out") return false;
  if (session.error === "rate-limited") return false;
  if (session.error === "password-sign-in-disabled") return false;
  return validHubSignInEmailV1(email) && validHubSignInPasswordV1(password);
}

/** 🙈️ Whether the credential form exists at all. A hub that answered `credential-sign-in-disabled`
 * has no password path, so the form is removed rather than left to fail on every press. */
export function hubSignInFormOfferedV1(session: HubSessionStateV1): boolean {
  return session.phase !== "signed-in" && session.error !== "password-sign-in-disabled";
}
//#endregion 🔖️Admission

//#region 🔖️AddHub
function AddHubForm({ onAddHub, locale, busy }: { readonly onAddHub: HubSignInPaneProps["onAddHub"]; readonly locale: string; readonly busy: boolean }): ReactElement {
  const id = useId();
  const [typed, setTyped] = useState("");
  const [label, setLabel] = useState("");
  const [invalid, setInvalid] = useState(false);
  const addressLabel = useLabel(hubUiLabel("os.hub.signIn.hubAddress"));
  const submitLabel = useLabel(hubUiLabel("os.hub.signIn.addHubSubmit"));
  const groupLabel = useLabel(hubUiLabel("os.hub.signIn.addHub"));
  const text = hubSignInTextV1(locale);

  const submit = (event: FormEvent): void => {
    event.preventDefault();
    const failure = onAddHub(typed, label);
    setInvalid(failure !== null);
    if (failure === null) {
      setTyped("");
      setLabel("");
    }
  };

  return (
    <form aria-label={groupLabel} onSubmit={submit} className="flex flex-col gap-2 sm:flex-row sm:items-end">
      <div className="flex min-w-0 flex-1 flex-col gap-1">
        <label htmlFor={`${id}-origin`} className="text-xs text-muted-foreground">{addressLabel}</label>
        <input
          id={`${id}-origin`}
          name="hubOrigin"
          type="text"
          inputMode="url"
          autoComplete="url"
          required
          value={typed}
          disabled={busy}
          aria-invalid={invalid}
          aria-describedby={invalid ? `${id}-origin-error` : undefined}
          onChange={(event) => {
            setTyped(event.target.value);
            setInvalid(false);
          }}
          className="w-full min-w-0 rounded-sm border px-single py-1 text-sm"
        />
        {invalid ? <p id={`${id}-origin-error`} role="alert" className="text-xs text-red-400">{text.invalidOrigin}</p> : null}
      </div>
      <Button icon="plus" type="submit" variant="outline" aria-label={submitLabel} disabled={busy || typed.trim().length === 0}>{submitLabel}</Button>
    </form>
  );
}
//#endregion 🔖️AddHub

//#region 🔖️Pane
/** 🔐️ The pane. One `<form>` with a real `<label>` per control and browser autofill hints, a single
 * `role="alert"` region for every denial, an `aria-busy` submit while an attempt is in flight with a
 * cancel beside it, and a permanently visible "working on this device only" line so the human can
 * see the app is not blocked. Lays out one column at phone width and two from `sm:` up. */
export function HubSignInPane({ book, session, locale, onSelectConnection, onAddHub, onForgetHub, onSignIn, onCancel, onSignOut }: HubSignInPaneProps): ReactElement {
  const id = useId();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const errorRef = useRef<HTMLParagraphElement | null>(null);
  const title = useLabel(hubUiLabel("os.hub.signIn.title"));
  const description = useLabel(hubUiLabel("os.hub.signIn.description"));
  const hubLabel = useLabel(hubUiLabel("os.hub.signIn.hub"));
  const forgetLabel = useLabel(hubUiLabel("os.hub.signIn.forgetHub"));
  const emailLabel = useLabel(hubUiLabel("os.hub.signIn.email"));
  const passwordLabel = useLabel(hubUiLabel("os.hub.signIn.password"));
  const submitLabel = useLabel(hubUiLabel("os.hub.signIn.submit"));
  const cancelLabel = useLabel(hubUiLabel("os.hub.signIn.cancel"));
  const signOutLabel = useLabel(hubUiLabel("os.hub.signIn.signOut"));
  const busyLabel = useLabel(hubUiLabel("os.hub.signIn.busy"));
  const signedOutLabel = useLabel(hubUiLabel("os.hub.signIn.signedOut"));
  const offlineLabel = useLabel(hubUiLabel("os.hub.signIn.offline"));
  const localOnlyLabel = useLabel(hubUiLabel("os.hub.signIn.localOnly"));
  const errorRegionLabel = useLabel(hubUiLabel("os.hub.signIn.errorRegion"));
  const signedInLabel = useLabel(hubUiLabel("os.hub.signIn.signedInAs"), { user: session.userId ?? "" });
  const text = hubSignInTextV1(locale);
  const busy = session.phase === "signing-in";
  const emailValid = email.length === 0 || validHubSignInEmailV1(email);
  const passwordValid = password.length === 0 || validHubSignInPasswordV1(password);
  const submittable = hubSignInSubmittableV1(session, email, password);
  const offered = hubSignInFormOfferedV1(session);
  const errorText = session.error === null ? null : session.error === "cancelled" ? text.cancelled : hubSignInErrorTextV1(locale, session.error, session.retryAfterSeconds);
  const expiredText = session.phase === "expired" ? text.expired : null;

  useEffect(() => {
    if (session.error !== null) errorRef.current?.focus();
  }, [session.error]);

  useEffect(() => {
    if (session.phase === "signed-in") setPassword("");
  }, [session.phase]);

  const submit = (event: FormEvent): void => {
    event.preventDefault();
    if (submittable) onSignIn({ email, password });
  };

  return (
    <section aria-labelledby={`${id}-title`} data-semio-hub-phase={session.phase} data-semio-hub-connection={session.connectionId} className="flex w-full min-w-0 flex-col gap-4 p-4">
      <header className="flex flex-col gap-1">
        <h2 id={`${id}-title`} className="text-base font-medium">{title}</h2>
        <p className="text-sm text-muted-foreground">{description}</p>
      </header>

      <div className="flex flex-col gap-1">
        <label htmlFor={`${id}-hub`} className="text-xs text-muted-foreground">{hubLabel}</label>
        <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
          <select
            id={`${id}-hub`}
            data-element-alias="os.hub.signIn.hub"
            value={book.selectedId}
            disabled={busy}
            onChange={(event) => onSelectConnection(event.target.value)}
            className="w-full min-w-0 rounded-sm border px-single py-1 text-sm sm:flex-1"
          >
            {book.connections.map((entry) => (
              <option key={entry.id} value={entry.id}>{`${entry.label} — ${entry.origin}`}</option>
            ))}
          </select>
          {book.selectedId === LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 ? null : (
            <Button icon="trash-2" type="button" variant="outline" disabled={busy} aria-label={forgetLabel} onClick={() => onForgetHub(book.selectedId)}>{forgetLabel}</Button>
          )}
        </div>
      </div>

      <AddHubForm onAddHub={onAddHub} locale={locale} busy={busy} />

      {expiredText === null ? null : <p role="status" aria-live="polite" className="text-sm">{expiredText}</p>}

      {errorText === null ? null : (
        <p
          ref={errorRef}
          tabIndex={-1}
          role="alert"
          aria-label={errorRegionLabel}
          data-semio-hub-error={session.error ?? ""}
          className="rounded-sm border border-red-400/40 bg-red-400/10 px-single py-1 text-sm text-red-400"
        >
          {errorText}
        </p>
      )}

      {session.phase === "signed-in" ? (
        <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
          <p className="text-sm">{signedInLabel}</p>
          <Button icon="lock" type="button" variant="outline" aria-label={signOutLabel} disabled={session.phase !== "signed-in"} onClick={onSignOut}>{signOutLabel}</Button>
        </div>
      ) : (
        <p className="text-sm text-muted-foreground">{signedOutLabel}</p>
      )}

      {!offered ? null : (
        <form aria-label={title} onSubmit={submit} className="flex flex-col gap-3">
          <div className="flex flex-col gap-1">
            <label htmlFor={`${id}-email`} className="text-xs text-muted-foreground">{emailLabel}</label>
            <input
              id={`${id}-email`}
              data-element-alias="os.hub.signIn.email"
              name="email"
              type="email"
              inputMode="email"
              autoComplete="username"
              required
              value={email}
              disabled={busy}
              aria-invalid={!emailValid}
              aria-describedby={emailValid ? undefined : `${id}-email-error`}
              onChange={(event) => setEmail(event.target.value)}
              className="w-full min-w-0 rounded-sm border px-single py-1 text-sm"
            />
            {emailValid ? null : <p id={`${id}-email-error`} className="text-xs text-red-400">{text.invalidEmail}</p>}
          </div>
          <div className="flex flex-col gap-1">
            <label htmlFor={`${id}-password`} className="text-xs text-muted-foreground">{passwordLabel}</label>
            <input
              id={`${id}-password`}
              data-element-alias="os.hub.signIn.password"
              name="password"
              type="password"
              autoComplete="current-password"
              required
              minLength={HUB_SIGN_IN_PASSWORD_MIN_BYTES}
              value={password}
              disabled={busy}
              aria-invalid={!passwordValid}
              aria-describedby={passwordValid ? undefined : `${id}-password-error`}
              onChange={(event) => setPassword(event.target.value)}
              className="w-full min-w-0 rounded-sm border px-single py-1 text-sm"
            />
            {passwordValid ? null : <p id={`${id}-password-error`} className="text-xs text-red-400">{text.shortPassword}</p>}
          </div>
          <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
            <Button id="os.hub.signIn.submit" icon="unlock" type="submit" variant="outline" aria-busy={busy} aria-label={busy ? busyLabel : submitLabel} disabled={!submittable}>{busy ? busyLabel : submitLabel}</Button>
            {busy ? <Button icon="x" type="button" variant="outline" aria-label={cancelLabel} onClick={onCancel}>{cancelLabel}</Button> : null}
          </div>
        </form>
      )}

      <p role="status" aria-live="polite" data-semio-hub-local-only="true" className="text-xs text-muted-foreground">
        {session.offline ? `${offlineLabel} ${localOnlyLabel}` : localOnlyLabel}
      </p>
    </section>
  );
}
//#endregion 🔖️Pane
