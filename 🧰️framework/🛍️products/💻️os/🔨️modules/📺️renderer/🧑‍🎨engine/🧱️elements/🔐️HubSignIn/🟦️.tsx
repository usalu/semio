// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔐️HubSignIn/component.tsx
/** 🔐️ `🔐️HubSignIn` — the shell's hub connection and sign-in pane: choose between the local
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
import { useEffect, useId, useRef, useState, type FormEvent, type ReactElement } from "../⚛️react-port/🟦️.ts";
import { Button, Input, SharedBorderRow, SharedBorderRowCell, useLabel } from "@semio-tech/ui-react";
import {
  HUB_SIGN_IN_PASSWORD_MIN_BYTES,
  LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1,
  hubSignInErrorTextV1,
  hubSignInTextV1,
  validHubSignInEmailV1,
  validHubSignInPasswordV1,
  type HubConnectionBookV1,
  type HubSessionStateV1,
  type HubSignInErrorCodeV1
} from "@semio-tech/framework/directory/sign-in";
import { hubUiLabel } from "../🔗️HubConnection/🌐️ui-labels/🟦️.ts";
import { HubFieldRow, HubRowItem, HubRowList, HubSelectField } from "../🔗️HubConnection/🪢️hub-rows/🟦️.tsx";
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
    <SharedBorderRow as="form" orientation="vertical" aria-label={groupLabel} onSubmit={submit}>
      <HubFieldRow htmlFor={`${id}-origin`} label={addressLabel}>
        <Input
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
          className="min-w-0 flex-1 text-sm"
        />
        <Button icon="plus" type="submit" variant="outline" aria-label={submitLabel} disabled={busy || typed.trim().length === 0}>{submitLabel}</Button>
      </HubFieldRow>
      {invalid ? <SharedBorderRowCell tone="destructive"><p id={`${id}-origin-error`} role="alert" className="text-xs text-destructive">{text.invalidOrigin}</p></SharedBorderRowCell> : null}
    </SharedBorderRow>
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
  const thisDeviceLabel = useLabel(hubUiLabel("os.hub.signIn.thisDevice"));
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
    <SharedBorderRow as="section" orientation="vertical" aria-labelledby={`${id}-title`} data-semio-hub-phase={session.phase} data-semio-hub-connection={session.connectionId} className="w-full">
      <SharedBorderRow as="header" orientation="vertical">
        <SharedBorderRowCell><h2 id={`${id}-title`} className="text-base font-medium">{title}</h2></SharedBorderRowCell>
        <SharedBorderRowCell><p className="text-sm text-muted-foreground">{description}</p></SharedBorderRowCell>
      </SharedBorderRow>

      <HubSelectField
        id={`${id}-hub`}
        label={hubLabel}
        value={book.selectedId}
        alias="os.hub.signIn.hub"
        disabled={busy}
        options={book.connections.map((entry) => ({ value: entry.id, label: `${entry.kind === "local-bootstrap" ? thisDeviceLabel : entry.label} — ${entry.origin}` }))}
        onValueChange={onSelectConnection}
      >
        {book.selectedId === LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 ? null : (
          <Button icon="trash-2" type="button" variant="outline" disabled={busy} aria-label={forgetLabel} onClick={() => onForgetHub(book.selectedId)}>{forgetLabel}</Button>
        )}
      </HubSelectField>

      <AddHubForm onAddHub={onAddHub} locale={locale} busy={busy} />

      {expiredText === null ? null : <SharedBorderRowCell><p role="status" aria-live="polite" className="text-sm">{expiredText}</p></SharedBorderRowCell>}

      {errorText === null ? null : (
        <SharedBorderRowCell tone="destructive" className="flex-1">
          <p ref={errorRef} tabIndex={-1} role="alert" aria-label={errorRegionLabel} data-semio-hub-error={session.error ?? ""} className="text-sm text-destructive">{errorText}</p>
        </SharedBorderRowCell>
      )}

      {session.phase === "signed-in" ? (
        <HubRowList>
          <HubRowItem>
            <SharedBorderRowCell className="flex-1"><p className="text-sm">{signedInLabel}</p></SharedBorderRowCell>
            <Button icon="lock" type="button" variant="outline" aria-label={signOutLabel} disabled={session.phase !== "signed-in"} onClick={onSignOut}>{signOutLabel}</Button>
          </HubRowItem>
        </HubRowList>
      ) : (
        <SharedBorderRowCell><p className="text-sm text-muted-foreground">{signedOutLabel}</p></SharedBorderRowCell>
      )}

      {!offered ? null : (
        <SharedBorderRow as="form" orientation="vertical" aria-label={title} onSubmit={submit}>
            <HubFieldRow htmlFor={`${id}-email`} label={emailLabel}>
            <Input
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
              className="min-w-0 flex-1 text-sm"
            />
            </HubFieldRow>
            {emailValid ? null : <SharedBorderRowCell tone="destructive"><p id={`${id}-email-error`} className="text-xs text-destructive">{text.invalidEmail}</p></SharedBorderRowCell>}
            <HubFieldRow htmlFor={`${id}-password`} label={passwordLabel}>
            <Input
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
              className="min-w-0 flex-1 text-sm"
            />
            </HubFieldRow>
            {passwordValid ? null : <SharedBorderRowCell tone="destructive"><p id={`${id}-password-error`} className="text-xs text-destructive">{text.shortPassword}</p></SharedBorderRowCell>}
          <SharedBorderRow>
            <Button id="os.hub.signIn.submit" icon="unlock" type="submit" variant="outline" aria-busy={busy} aria-label={busy ? busyLabel : submitLabel} disabled={!submittable}>{busy ? busyLabel : submitLabel}</Button>
            {busy ? <Button icon="x" type="button" variant="outline" aria-label={cancelLabel} onClick={onCancel}>{cancelLabel}</Button> : null}
          </SharedBorderRow>
        </SharedBorderRow>
      )}

      <SharedBorderRowCell><p role="status" aria-live="polite" data-semio-hub-local-only="true" className="text-xs text-muted-foreground">
        {session.offline ? `${offlineLabel} ${localOnlyLabel}` : localOnlyLabel}
      </p></SharedBorderRowCell>
    </SharedBorderRow>
  );
}
//#endregion 🔖️Pane
