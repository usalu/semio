/** 🌬️ The air controls of the viewer: the environment the thermal view and its air flow are solved in, the wind, the turbulence and wind profile, and the detail of the flow marks, in four sections a reader opens and closes.
 *
 * Every number is one {@link QuantityRow}: a label that names its unit, a field in the reader's number format (a comma or a point is read), a slider over the same range and a button back to its default; a value that was set by hand is marked "changed" in words, and a section says how many of its values are. The default of an environment value is the value of the chosen reference environment, of a condition or a mark detail the one of the solver.
 * The reference environment select applies a preset whole and names it; once a value is set by hand it reads "changed by hand" and names the preset it started from.
 * The sections are disclosure buttons (`aria-expanded`), every control is native and at least 24 px high, and nothing is said by colour alone. The controls take the translator and the language as props, so they need no context.
 *
 * @see ../🧭️wind/🟦️.tsx — the wind dial
 * @see ../../🌤️environments/🟦️.ts — the quantities of the environment
 * @see ../../🌊️flow/🟦️.ts — the conditions of the flow and the mark detail
 */
import { useId, useState, type ReactElement, type ReactNode } from "react";
import { formatNumber } from "@teaching/staffage";
import type { Asset, Environment, FlowMarkDetail, Language, Slug } from "@teaching/staffage";
import type { Key, Translate } from "../../🌐️i18n/🟦️.ts";
import { ENVIRONMENT_PRESETS, ENVIRONMENT_QUANTITY, environmentOf, shownValue, storedValue, type EnvironmentOverride, type Quantity } from "../../🌤️environments/🟦️.ts";
import { FLOW_DEFAULTS, FLOW_QUANTITIES, MARK_DEFAULTS, MARK_QUANTITIES, type FlowSettings } from "../../🌊️flow/🟦️.ts";
import { Bar, Button, Column, Icon, Note, Segmented, SelectField, SharedBorderRow, SharedBorderRowCell, cn } from "../../🪟️chrome/🟦️.tsx";
import { WindDial, type DialView } from "../🧭️wind/🟦️.tsx";

const LABEL = "text-[0.65rem] uppercase tracking-wide text-muted-foreground";

/** 🏷️ The label of a quantity with its unit. */
export const quantityLabel = (t: Translate, entry: Quantity): string => (entry.unit === "" ? t(`quantity.${entry.key}` as Key) : t("quantity.label", { name: t(`quantity.${entry.key}` as Key), unit: entry.unit }));

/** 🔢️ One number of the air: label with unit, field, slider and the way back to its default. `value` is the stored value in effect, `fallback` the stored default, `changed` whether the reader set it. */
export function QuantityRow(props: { readonly entry: Quantity; readonly value: number; readonly fallback: number; readonly changed: boolean; readonly onChange: (stored: number | undefined) => void; readonly t: Translate; readonly language: Language; readonly disabled?: boolean; readonly note?: string }): ReactElement {
  const { entry, value, fallback, changed, onChange, t, language } = props;
  const id = useId();
  const [draft, setDraft] = useState<string | null>(null);
  const name = t(`quantity.${entry.key}` as Key);
  const label = quantityLabel(t, entry);
  const shown = shownValue(entry, value);
  const text = (stored: number): string => `${formatNumber(shownValue(entry, stored), entry.digits, language)}${entry.unit === "" ? "" : ` ${entry.unit}`}`;
  const read = (typed: string): number | null => {
    const parsed = typed.trim() === "" ? Number.NaN : Number(typed.trim().replace(",", "."));
    return Number.isFinite(parsed) && parsed >= entry.min && parsed <= entry.max ? parsed : null;
  };
  const valid = draft === null || read(draft) !== null;
  const set = (next: number): void => {
    const stored = storedValue(entry, next);
    onChange(stored === fallback ? undefined : stored);
  };
  return (
    <SharedBorderRowCell data-quantity={entry.key} {...(changed ? { "data-changed": "" } : {})} className={cn("flex-col items-stretch gap-single px-double py-single", props.disabled && "opacity-60")}>
      <div className="flex flex-wrap items-center justify-between gap-single">
        <label htmlFor={id} className={LABEL}>
          {label}
          {changed ? <span className="ml-1 normal-case tracking-normal text-foreground">· {t("air.changed")}</span> : null}
        </label>
        <span className="flex items-center gap-single">
          <input
            id={id}
            type="text"
            inputMode="decimal"
            disabled={props.disabled}
            value={draft ?? formatNumber(shown, entry.digits, language)}
            aria-invalid={valid ? undefined : true}
            aria-describedby={`${id}-default`}
            onChange={(event) => {
              setDraft(event.target.value);
              const parsed = read(event.target.value);
              if (parsed !== null) set(parsed);
            }}
            onBlur={() => setDraft(null)}
            className={cn("min-h-6 w-20 border border-border bg-transparent px-single text-right font-mono text-xs text-foreground", !valid && "text-destructive")}
          />
          <Button variant="quiet" icon="rotate-ccw" disabled={!changed || props.disabled} aria-label={t("quantity.reset", { name })} title={`${t("quantity.reset", { name })} (${text(fallback)})`} onClick={() => { setDraft(null); onChange(undefined); }} />
        </span>
      </div>
      <input type="range" min={entry.min} max={entry.max} step={entry.step} value={shown} disabled={props.disabled} aria-label={t("quantity.slider", { name: label })} aria-valuetext={text(value)} onChange={(event) => set(Number(event.target.value))} className="min-h-6 w-full accent-[var(--accent)]" />
      <span id={`${id}-default`} className="text-[0.65rem] text-muted-foreground">{`${t("quantity.default", { value: text(fallback) })}${props.note === undefined ? "" : ` ${props.note}`}`}</span>
    </SharedBorderRowCell>
  );
}

/** 🪗️ A section of controls behind a disclosure button; `count` is how many of its values the reader changed. */
export function Fold(props: { readonly name: string; readonly title: string; readonly count: number; readonly open: boolean; readonly onToggle: (open: boolean) => void; readonly t: Translate; readonly children: ReactNode; readonly className?: string }): ReactElement {
  const id = useId();
  return (
    <SharedBorderRow orientation="vertical" role="group" aria-labelledby={`${id}-title`} data-air-section={props.name} {...(props.open ? { "data-open": "" } : {})} className={props.className}>
      <Bar>
        <Button grouped id={`${id}-title`} aria-expanded={props.open} {...(props.open ? { "aria-controls": `${id}-body` } : {})} onClick={() => props.onToggle(!props.open)} className="flex-1 justify-between">
          <span>{props.count > 0 ? props.t("air.sectionChanged", { name: props.title, count: props.count }) : props.title}</span>
          <Icon icon={props.open ? "chevron-up" : "chevron-down"} size="small" className="shrink-0" />
        </Button>
      </Bar>
      {props.open ? <Column id={`${id}-body`}>{props.children}</Column> : null}
    </SharedBorderRow>
  );
}

/** 🌬️ The state the air controls read and change: a part of the viewer state. */
export type AirState = { readonly preset: Slug | null; readonly environment: EnvironmentOverride; readonly flowSettings: FlowSettings; readonly flowDetail: FlowMarkDetail };

/** ⚙️ The props of {@link AirControls}: `thermal` whether the thermal view is shown (the environment acts on it only), `flowable` whether the asset has an air flow to act on, `dial` the viewpoint the wind dial is drawn for with its name. */
export type AirControlsProps = { readonly asset: Asset; readonly air: AirState; readonly onChange: (patch: Partial<AirState>) => void; readonly t: Translate; readonly language: Language; readonly thermal: boolean; readonly flowable: boolean; readonly dial: { readonly view: DialView; readonly name: string }; readonly open?: Partial<Record<AirSection, boolean>> };

/** 🗂️ The four sections. */
export const AIR_SECTIONS = ["environment", "wind", "turbulence", "marks"] as const;

/** 📤️ One of {@link AIR_SECTIONS}. */
export type AirSection = (typeof AIR_SECTIONS)[number];

const ENVIRONMENT_ROWS = ["airTempC", "meanRadiantTempC", "skyTempC", "groundTempC", "relativeHumidity", "solarWM2", "diffuseFraction", "pressurePa"] as const satisfies readonly (keyof Environment)[];
const without = <T extends object>(values: T, key: keyof T): T => {
  const { [key]: _dropped, ...rest } = values;
  return rest as T;
};
const withValue = <T extends object>(values: T, key: keyof T, value: number | undefined): T => (value === undefined ? without(values, key) : { ...values, [key]: value });

/** 🌬️ The four sections of the air. */
export function AirControls(props: AirControlsProps): ReactElement {
  const { asset, air, onChange, t, language, thermal, flowable } = props;
  const [open, setOpen] = useState<Readonly<Record<AirSection, boolean>>>({ environment: true, wind: true, turbulence: false, marks: false, ...props.open });
  const toggle = (section: AirSection) => (next: boolean): void => setOpen((before) => ({ ...before, [section]: next }));
  const { environment, base, presetId, changed } = environmentOf(asset, air.preset, air.environment);
  const presetName = presetId === null ? t("viewer.env.presetOwn") : (ENVIRONMENT_PRESETS.find((preset) => preset.id === presetId)?.title[language] ?? presetId);
  const selected = changed ? "custom" : (presetId ?? "own");
  const setEnvironment = (key: keyof Environment, value: number | undefined): void => onChange({ environment: withValue(air.environment, key, value) });
  const environmentRow = (key: keyof Environment): ReactElement => {
    const radiant = key === "skyTempC" || key === "groundTempC";
    return <QuantityRow key={key} entry={ENVIRONMENT_QUANTITY[key]} value={environment[key] ?? environment.meanRadiantTempC} fallback={base[key] ?? environment.meanRadiantTempC} changed={air.environment[key] !== undefined} onChange={(value) => setEnvironment(key, value)} t={t} language={language} {...(radiant && base[key] === undefined ? { note: t("viewer.env.followsRadiant") } : {})} />;
  };
  const settings = air.flowSettings;
  const logarithmic = settings.profile === "logarithmic";
  const flowRow = (key: keyof typeof FLOW_DEFAULTS, disabled = false): ReactElement => <QuantityRow key={key} entry={FLOW_QUANTITIES.find((entry) => entry.key === key)!} value={settings[key] ?? FLOW_DEFAULTS[key]} fallback={FLOW_DEFAULTS[key]} changed={settings[key] !== undefined} onChange={(value) => onChange({ flowSettings: withValue(settings, key, value) })} t={t} language={language} disabled={disabled || !flowable} />;
  const count = (keys: readonly string[], values: object): number => keys.filter((key) => (values as Record<string, unknown>)[key] !== undefined).length;
  const windCount = count(["windMS", "windDirectionDeg"], air.environment);
  const turbulenceKeys = ["turbulenceIntensity", "turbulenceFloorMS", "lagrangianTimeS", "profile", "roughnessM", "referenceHeightM", "lifeS", "seed"] as const;
  return (
    <>
      <Fold name="environment" title={t("viewer.environment")} count={count(ENVIRONMENT_ROWS, air.environment) + (air.preset === null ? 0 : 1)} open={open.environment} onToggle={toggle("environment")} t={t}>
        <Note className="text-[0.65rem]">{t(thermal ? "viewer.environmentHelp" : "viewer.environmentOnlyThermal")}</Note>
        <Bar>
          <SelectField
            label={t("viewer.env.preset")}
            value={selected}
            options={[{ value: "own", label: t("viewer.env.presetOwn") }, ...ENVIRONMENT_PRESETS.map((preset) => ({ value: preset.id as string, label: preset.title[language] })), ...(changed ? [{ value: "custom", label: t("viewer.env.presetChanged", { name: presetName }) }] : [])]}
            onChange={(id) => id !== "custom" && onChange({ preset: id === "own" ? null : (id as Slug), environment: {} })}
          />
        </Bar>
        <Note className="text-[0.65rem]" tone="plain"><span role="status" data-environment-status={selected}>{changed ? t("viewer.env.presetChanged", { name: presetName }) : t("viewer.env.presetActive", { name: presetName })}</span></Note>
        {ENVIRONMENT_ROWS.map(environmentRow)}
        <Bar><Button grouped icon="rotate-ccw" disabled={!changed && air.preset === null} onClick={() => onChange({ preset: null, environment: {} })}>{t("viewer.env.reset")}</Button></Bar>
      </Fold>
      <Fold name="wind" title={t("air.wind")} count={windCount} open={open.wind} onToggle={toggle("wind")} t={t}>
        <WindDial view={props.dial.view} viewName={props.dial.name} bearingDeg={environment.windDirectionDeg} speedMS={environment.windMS} t={t} language={language} onChange={(wind) => onChange({ environment: withValue(withValue(air.environment, "windMS", wind.speedMS === base.windMS ? undefined : wind.speedMS), "windDirectionDeg", wind.bearingDeg === base.windDirectionDeg ? undefined : wind.bearingDeg) })} />
        {environmentRow("windMS")}
        {environmentRow("windDirectionDeg")}
        <Bar><Button grouped icon="rotate-ccw" disabled={windCount === 0} onClick={() => onChange({ environment: without(without(air.environment, "windMS"), "windDirectionDeg") })}>{t("air.reset")}</Button></Bar>
      </Fold>
      <Fold name="turbulence" title={t("air.turbulence")} count={count(turbulenceKeys, settings)} open={open.turbulence} onToggle={toggle("turbulence")} t={t} className={cn(!flowable && "opacity-70")}>
        <Note className="text-[0.65rem]">{t(flowable ? "air.conditionsHelp" : thermal ? "flow.none" : "air.flowOnly")}</Note>
        <Segmented label={t("wind.profile")} value={logarithmic ? "logarithmic" : "uniform"} onChange={(profile) => onChange({ flowSettings: profile === "logarithmic" ? { ...settings, profile } : without(settings, "profile") })} options={[{ value: "uniform", label: t("wind.profile.uniform") }, { value: "logarithmic", label: t("wind.profile.logarithmic") }]} />
        <Note className="text-[0.65rem]">{t("wind.profileHelp")}</Note>
        {flowRow("roughnessM", !logarithmic)}
        {flowRow("referenceHeightM", !logarithmic)}
        {flowRow("turbulenceIntensity")}
        {flowRow("turbulenceFloorMS")}
        {flowRow("lagrangianTimeS")}
        {flowRow("lifeS")}
        {flowRow("seed")}
        <Bar><Button grouped icon="rotate-ccw" disabled={count(turbulenceKeys, settings) === 0} onClick={() => onChange({ flowSettings: {} })}>{t("air.reset")}</Button></Bar>
      </Fold>
      <Fold name="marks" title={t("air.marks")} count={count(MARK_QUANTITIES.map((entry) => entry.key), air.flowDetail)} open={open.marks} onToggle={toggle("marks")} t={t} className={cn(!flowable && "opacity-70")}>
        <Note className="text-[0.65rem]">{t(flowable ? "air.marksHelp" : thermal ? "flow.none" : "air.flowOnly")}</Note>
        {MARK_QUANTITIES.map((entry) => <QuantityRow key={entry.key} entry={entry} value={air.flowDetail[entry.key] ?? MARK_DEFAULTS[entry.key]} fallback={MARK_DEFAULTS[entry.key]} changed={air.flowDetail[entry.key] !== undefined} onChange={(value) => onChange({ flowDetail: withValue(air.flowDetail, entry.key, value) })} t={t} language={language} disabled={!flowable} />)}
        <Bar><Button grouped icon="rotate-ccw" disabled={Object.keys(air.flowDetail).length === 0} onClick={() => onChange({ flowDetail: {} })}>{t("air.reset")}</Button></Bar>
      </Fold>
    </>
  );
}
