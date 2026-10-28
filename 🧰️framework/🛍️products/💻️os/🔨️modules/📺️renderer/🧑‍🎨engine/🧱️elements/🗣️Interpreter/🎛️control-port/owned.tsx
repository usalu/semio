/** 🪢️ Shared ordinary views borrow exact owned subscriptions and acknowledge them after commit. */
import { useCallback, useLayoutEffect, useSyncExternalStore, type ReactNode } from "react";
import { useOwnedRead } from "../../📃️UiDocumentStore/📖️owned/🟦️.tsx";
import { OwnedStagedControls, type StagedOwnedSnapshot, type StagedOwnedSubscription, type StagedOwnedView } from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents/owned.ts";
import type { UiControlContext, UiControlPort } from "./🟦️.ts";
import { OrdinaryControlView } from "./🟦️.tsx";
import type { UiNodeRecord } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import { fieldAssociation } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/📝️Field/🔗️association/🟦️.ts";
import { stagedBodyNode } from "./body.ts";
import { controlLabelTarget } from "./label-target.ts";
import { controlDomId } from "../🪪️stable-dom-ids/🟦️.ts";
import { UiContainerFrame } from "./container.tsx";
import { TextView } from "../🔤️display-views/🟦️.tsx";

type OwnedControlProps = { readonly owner: OwnedStagedControls; readonly id: number; readonly view: StagedOwnedView };

/** 🎟️ Only a committed original status read may install the staged publication proof. */
export function OwnedStagedStatusView({ owner, id, view }: OwnedControlProps) {
  const { snapshot, subscription } = useOwnedRead<StagedOwnedSnapshot, StagedOwnedSubscription>(owner.reads, id);
  const record = snapshot && subscription ? owner.control(subscription, snapshot, view).read(id) : undefined;
  useLayoutEffect(() => { if (snapshot && subscription && record && stagedBodyNode(record, "status")) owner.publishStatus(subscription, snapshot, view); }, [owner, snapshot, subscription, view, record]);
  return null;
}

/** 🎛️ Retains no copied record or argument tree; the port rechecks the rendered read on every event. */
export function OwnedStagedControlView({ owner, id, view, localDocumentOwner, domScope, field }: OwnedControlProps & Pick<UiControlContext, "localDocumentOwner" | "domScope" | "field">) {
  const { snapshot, subscription } = useOwnedRead<StagedOwnedSnapshot, StagedOwnedSubscription>(owner.reads, id);
  useSyncExternalStore(owner.subscribe, owner.presentation, owner.presentation);
  if (!snapshot || !subscription) return null;
  const controls = owner.control(subscription, snapshot, view);
  const record = controls.read(id);
  return record && stagedBodyNode(record, "control") ? <OrdinaryControlView record={record} context={{ controls, localDocumentOwner, domScope, field }} /> : null;
}

type BodyContext = Pick<UiControlContext, "localDocumentOwner" | "domScope">;

function useBodyNode({ owner, id, view }: OwnedControlProps) {
  const { snapshot, subscription } = useOwnedRead<StagedOwnedSnapshot, StagedOwnedSubscription>(owner.reads, id);
  const controls = snapshot && subscription ? owner.control(subscription, snapshot, view) : undefined;
  return { controls, record: controls?.read(id) };
}

function OwnedFieldControl({ owner, id, view, field, disabled, localDocumentOwner, domScope }: OwnedControlProps & BodyContext & { readonly field: UiNodeRecord; readonly disabled: boolean }) {
  const read = useBodyNode({ owner, id, view });
  useSyncExternalStore(owner.subscribe, owner.presentation, owner.presentation);
  if (!read.record || !read.controls || !stagedBodyNode(read.record, "control") || field.component.type !== "container") return null;
  const record = read.record;
  const port = read.controls;
  const controls: UiControlPort = disabled ? { identity: port.identity, surface: port.surface, read: port.read, subscribe: port.subscribe, value: port.value, disabled: () => true, dispatch: () => {} } : port;
  const context: UiControlContext = { controls, localDocumentOwner, domScope };
  const target = controlLabelTarget(context, record);
  const association = fieldAssociation(domScope, target, Boolean(field.component.description), Boolean(field.component.error));
  const fieldContext: UiControlContext = { controls, localDocumentOwner, domScope, field: { label: association.label, describedBy: association.describedBy, error: association.error, required: field.component.required ?? undefined, invalid: Boolean(field.component.error) } };
  return <UiContainerFrame record={field} id={domScope!} controlId={target}>{record.component.type === "text" ? <div id={controlDomId(context, record)} role="group" aria-labelledby={association.label} aria-describedby={association.describedBy}><TextView record={record} /></div> : <OrdinaryControlView record={record} context={fieldContext} />}</UiContainerFrame>;
}

function OwnedBodyField(props: OwnedControlProps & BodyContext & { readonly disabled: boolean }) {
  const { record, controls } = useBodyNode(props);
  if (!record || !controls || !stagedBodyNode(record, "field")) return null;
  const id = controlDomId({ controls, domScope: props.domScope }, record);
  return <OwnedFieldControl owner={props.owner} id={record.children[0]!} view={props.view} field={record} disabled={props.disabled || record.disabled} localDocumentOwner={props.localDocumentOwner} domScope={id} />;
}

function OwnedBodyRoot(props: OwnedControlProps & BodyContext) {
  const { record, controls } = useBodyNode(props);
  if (!record || !controls || !stagedBodyNode(record, "form")) return null;
  const id = controlDomId({ controls, domScope: props.domScope }, record);
  return <UiContainerFrame record={record} id={id}>{record.children.map((child, index) => index === 0 ? <OwnedStagedStatusView key={child} owner={props.owner} id={child} view={props.view} /> : <OwnedBodyField key={child} owner={props.owner} id={child} view={props.view} localDocumentOwner={props.localDocumentOwner} domScope={id} disabled={record.disabled} />)}</UiContainerFrame>;
}

/** 🪟️ The original surface supplies root metadata and every node read through its own subscription. */
export function OwnedStagedBodyView({ owner, localDocumentOwner, domScope, pending = null }: { readonly owner: OwnedStagedControls; readonly pending?: ReactNode } & BodyContext) {
  const subscribe = useCallback((notify: () => void) => {
    if (!owner.view()) return () => {};
    const subscription = owner.reads.subscribeView(notify);
    return () => owner.reads.unsubscribeNode(subscription);
  }, [owner]);
  const view = useSyncExternalStore(subscribe, owner.view, () => undefined);
  useSyncExternalStore(owner.subscribe, owner.presentation, owner.presentation);
  return view && view.root !== null ? <OwnedBodyRoot owner={owner} id={view.root} view={view} localDocumentOwner={localDocumentOwner} domScope={domScope} /> : pending;
}
