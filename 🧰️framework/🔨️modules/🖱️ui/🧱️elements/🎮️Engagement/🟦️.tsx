/** 🪟️ Owns the window engagement pane that edits the active tool parameters. */
import * as React from "react";
import { type IconName } from "@semio-tech/assets";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { type ControlIcon } from "../🔣️Icons/🟦️.tsx";
import { isInternalChromeControlId } from "../🚗️UiDriver/🟦️.tsx";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { type UiTranslationKey } from "../📚️I18n/🟦️.tsx";
import { LevelProvider } from "../🌈️Surface/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { ButtonGroup, ButtonGroupItem } from "../🔳️ButtonGroup/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../🔽️Select/🟦️.tsx";
import { Slider } from "../🎚️Slider/🟦️.tsx";
import { Stepper } from "../🪜️Stepper/🟦️.tsx";
import { Ring } from "../⭕️Ring/🟦️.tsx";
import { normalizeEngagementActionText } from "../🔍️WindowSearch/🟦️.tsx";

export interface EngagementOption {
  id: string;
  label?: UiLabel;
  icon: ControlIcon;
  pressed?: boolean;
  disabled?: boolean;
  onPress?: () => void;
}

export interface EngagementStatus {
  id: string;
  content: React.ReactNode;
}

/** 🍩️ One discrete option on an engagement {@link EngagementRingControl}. */
export interface EngagementRingOption {
  id: string;
  label: string;
  disabled?: boolean;
}

/** 🎚️ Engagement range slider for numeric values (height, distance, …). */
export interface EngagementSliderControl {
  kind: "slider";
  id?: string;
  label?: UiLabel;
  value: number;
  min: number;
  max: number;
  /** 📗️ Absolute preloaded/ready extent on the fixed `[min, max]` range. */
  ready?: number;
  step?: number;
  unit?: string;
  disabled?: boolean;
  onChange?: (value: number) => void;
  onCommit?: (value: number) => void;
}

/** 🔢️ Engagement stepper for numeric values without fixed upper bound. */
export interface EngagementStepperControl {
  kind: "stepper";
  id?: string;
  label?: UiLabel;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  disabled?: boolean;
  onChange?: (value: number) => void;
  onCommit?: (value: number) => void;
}

/** 🧫️ Engagement ring (radial dial) for angles or discrete option selection. */
export interface EngagementRingControl {
  kind: "ring";
  id?: string;
  label?: UiLabel;
  value?: string;
  options: readonly EngagementRingOption[];
  disabled?: boolean;
  onSelect?: (id: string) => void;
}

/** ⏯️ One discrete option on an engagement {@link EngagementToggleGroupControl}. */
export interface EngagementToggleGroupOption {
  id: string;
  label: string;
  icon?: IconName;
  disabled?: boolean;
}

/** 🔘️ Engagement toggle button group for small unordered enums. */
export interface EngagementToggleGroupControl {
  kind: "toggleGroup";
  id?: string;
  label?: UiLabel;
  value?: string;
  options: readonly EngagementToggleGroupOption[];
  disabled?: boolean;
  onSelect?: (id: string) => void;
}

/** 🔽️ One item on an engagement {@link EngagementSelectControl}. */
export interface EngagementSelectItem {
  id: string;
  value: string;
  label: string;
}

/** ☑️ Engagement select dropdown for large enums. */
export interface EngagementSelectControl {
  kind: "select";
  id?: string;
  label?: UiLabel;
  value?: string;
  placeholder?: UiLabel;
  items: readonly EngagementSelectItem[];
  disabled?: boolean;
  onChange?: (value: string) => void;
}

/** 🪜️ Optional engagement UI control for the active action step. */
export type EngagementControl = EngagementSliderControl | EngagementStepperControl | EngagementRingControl | EngagementToggleGroupControl | EngagementSelectControl;

/** 🏷️ i18n keys for window engagement chrome (`ui.engagement.*` in {@link uiChromeTranslationBundles}). */
export const UI_ENGAGEMENT = {
  actions: "ui.engagement.actions",
} as const satisfies Record<string, UiTranslationKey>;

/** 🔤️ Default English copy for window engagement chrome (matches `ui.engagement.*` en bundle). */
export const ENGAGEMENT_USER = {
  actionsAria: "Actions",
} as const;

/** 💬️ Floating window engagement payload with options, status, and controls. */
export interface EngagementSpec {
  /** 🎯️ Ongoing engagement: chrome stays visible; {@link options} are step transitions. */
  sessionActive?: boolean;
  options?: EngagementOption[];
  /** ⭕️ Optional slider, stepper, or ring control for the active step. */
  control?: EngagementControl;
  /** ➕️ Optional additional controls rendered below the primary control. */
  controls?: readonly EngagementControl[];
  status?: EngagementStatus[];
}

export interface EngagementProps extends EngagementSpec {
  className?: string;
}

function engagementControlLabel(control: EngagementControl): string | undefined {
  if (!control.label) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup" || control.kind === "select" || !control.unit) return control.label;
  return `${control.label} (${control.unit})`;
}

function engagementControlIsNumeric(control: EngagementControl): control is EngagementSliderControl | EngagementStepperControl {
  return control.kind === "slider" || control.kind === "stepper";
}

/** 🎛️ Renders one engagement {@link EngagementControl} using Slider, Stepper, Ring, toggle group, or Select. */
function EngagementControlView({ control }: { readonly control: EngagementControl }): React.ReactElement | null {
  const label = engagementControlLabel(control);
  const selectLabel = useLabel("ui.common.select");
  const lastNumericRef = reactHostPort.useRef(engagementControlIsNumeric(control) ? control.value : 0);
  reactHostPort.useEffect(() => {
    if (engagementControlIsNumeric(control)) lastNumericRef.current = control.value;
  }, [control]);
  if (control.kind === "slider") {
    return (
      <div data-slot="engagement-control" data-control-kind="slider" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Slider
          id={control.id ?? "engagement-control.slider"}
          value={[control.value]}
          min={control.min}
          max={control.max}
          ready={control.ready}
          step={control.step}
          disabled={control.disabled}
          onValueChange={(values) => {
            const next = values[0];
            if (next === undefined) return;
            lastNumericRef.current = next;
            control.onChange?.(next);
          }}
          onPointerUp={() => control.onCommit?.(lastNumericRef.current)}
        />
      </div>
    );
  }
  if (control.kind === "stepper") {
    return (
      <div data-slot="engagement-control" data-control-kind="stepper" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Stepper
          id={control.id ?? "engagement-control.stepper"}
          value={control.value}
          min={control.min}
          max={control.max}
          step={control.step}
          onChange={(value) => {
            lastNumericRef.current = value;
            control.onChange?.(value);
          }}
          onPointerUp={() => control.onCommit?.(lastNumericRef.current)}
        />
      </div>
    );
  }
  if (control.kind === "toggleGroup") {
    if (!control.options.length) return null;
    return (
      <div data-slot="engagement-control" data-control-kind="toggleGroup" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <ButtonGroup id={control.id} detailPanelWidthMode="fill">
          {control.options.map((option) => (
            <ButtonGroupItem
              key={option.id}
              id={option.id}
              text={option.label}
              icon={option.icon ?? "circle-dot"}
              className={cn(control.value === option.id && interactiveActiveFillClass)}
              disabled={option.disabled || control.disabled}
              onClick={() => control.onSelect?.(option.id)}
            />
          ))}
        </ButtonGroup>
      </div>
    );
  }
  if (control.kind === "select") {
    if (!control.items.length) return null;
    return (
      <div data-slot="engagement-control" data-control-kind="select" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Select id={control.id ?? "engagement-control.select"} value={control.value} onValueChange={(value) => control.onChange?.(value)} disabled={control.disabled}>
          <SelectTrigger id={control.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={control.placeholder ?? selectLabel} />
          </SelectTrigger>
          <SelectContent>
            {control.items.map((item) => (
              <SelectItem key={item.id} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
    );
  }
  if (!control.options.length) return null;
  const orbs = control.options.map((option, index) => ({
    id: option.id,
    t: control.options.length > 1 ? index / control.options.length : 0,
    disabled: option.disabled || control.disabled,
    selected: control.value === option.id,
  }));
  return (
    <div data-slot="engagement-control" data-control-kind="ring" className="flex min-w-0 flex-col items-center gap-half px-half">
      {label ? <span className="text-element text-xs">{label}</span> : null}
      <Ring id={control.id ?? "engagement-control.ring"} orbs={orbs} onOrbSelect={(orbId) => control.onSelect?.(orbId)} />
    </div>
  );
}

/** 🔝️ Top-aligned engagement: status heading, optional control, and option buttons. */
const Engagement: React.FC<EngagementProps> = ({ sessionActive = false, options, control, controls, status, className = "" }) => {
  const stepOptionsAriaLabel = useLabel(UI_ENGAGEMENT.actions);
  const primaryStepStatus = sessionActive ? status?.find((row) => row.id === "engagement-step") : undefined;
  const secondaryStatus = sessionActive ? status?.filter((row) => row.id !== "engagement-step") : status;

  const hasOptions = !!options?.length;
  const hasControl = !!control || !!controls?.length;
  const hasStatus = !!status?.length;

  if (!hasOptions && !hasControl && !hasStatus) return null;

  return (
    <LevelProvider level="pane">
      <div
        data-slot="engagement"
        data-session-active={sessionActive ? "true" : undefined}
        className={cn("pointer-events-auto flex w-full min-w-0 max-w-full flex-col gap-half", sessionActive && "bg-active-base ring-accent/35 rounded-sm ring-1 shadow-sm", className)}
      >
        {primaryStepStatus ? (
          <div data-slot="engagement-step-heading" className="text-foreground px-half text-sm font-medium leading-tight">
            {primaryStepStatus.content}
          </div>
        ) : null}
        {control ? <EngagementControlView control={control} /> : null}
        {controls?.map((row) => (
          <EngagementControlView key={row.id ?? row.label ?? row.kind} control={row} />
        ))}
        {secondaryStatus?.length ? (
          <div data-slot="engagement-status" role="status" aria-live="polite" aria-atomic="true" className="flex flex-wrap items-center justify-center gap-single text-xs text-muted-foreground">
            {secondaryStatus.map((item) => (
              <span key={item.id} data-slot="engagement-status-item">
                {item.content}
              </span>
            ))}
          </div>
        ) : null}
        {hasOptions ? (
          <div
            data-slot="engagement-options"
            data-step-options={sessionActive ? "true" : undefined}
            className="flex flex-wrap items-center justify-center gap-half"
            role="group"
            aria-label={sessionActive ? stepOptionsAriaLabel || ENGAGEMENT_USER.actionsAria : ENGAGEMENT_USER.actionsAria}
          >
            <ButtonGroup id={UI_ENGAGEMENT.actions}>
              {options!.map((option) => {
                const actionLabel = normalizeEngagementActionText(option.label ?? "");
                const optionControlId = isInternalChromeControlId(option.id) ? undefined : option.id;
                return (
                  <ButtonGroupItem key={option.id} id={optionControlId} aria-label={actionLabel} icon={option.icon} text={actionLabel} className={cn(option.pressed && interactiveActiveFillClass)} onClick={option.onPress} disabled={option.disabled} />
                );
              })}
            </ButtonGroup>
          </div>
        ) : null}
      </div>
    </LevelProvider>
  );
};

export { Engagement };
