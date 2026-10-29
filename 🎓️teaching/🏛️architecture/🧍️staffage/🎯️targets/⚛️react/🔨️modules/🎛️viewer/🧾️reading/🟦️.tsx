/** 🧾️ How the viewer reads what it draws: the inspector of the value under the pointer or focus, and the legends of the analytic views with quantity, unit, range, sign convention and meaning.
 *
 * Colour is never the only channel: every legend states its quantity, unit, centre and convention in words, and the inspector prints the value with its unit.
 *
 * @see ../../🗣️labels/🟦️.ts — `valueText`
 * @see ../../../../../🔨️modules/🔗️contracts/🟦️.ts — `Legend`, `ItemValue`
 */
import type { ReactElement } from "react";
import { decimalsFor, formatNumber } from "@teaching/staffage";
import type { Asset, DrawingScene, ItemValue, Language, Legend } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Tag } from "../../🪟️chrome/🟦️.tsx";
import { valueText } from "../../🗣️labels/🟦️.ts";

/** 🔬️ What is under the pointer or focus. */
export type Inspected = { readonly value: ItemValue; readonly item: { readonly bone?: string; readonly region?: string } | null };

/** 🔬️ The inspector: the hovered or focused value with unit, quantity and what it belongs to; a hint while nothing is inspected. */
export function Inspector(props: { readonly inspected: Inspected | null; readonly language: Language; readonly hasValues: boolean; readonly asset: Asset }): ReactElement {
  const { t } = useArchive();
  const { inspected, language, hasValues } = props;
  return (
    <section aria-label={t("viewer.inspector")} className="flex min-h-20 flex-col gap-single border border-border bg-panel p-double">
      <h3 className="m-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("viewer.inspector")}</h3>
      {inspected === null ? (
        <p className="m-0 text-xs text-muted-foreground">{t(hasValues ? "viewer.inspectorHint" : "viewer.inspectorNone")}</p>
      ) : (
        <p role="status" className="m-0 flex flex-wrap items-baseline gap-double text-xs">
          <strong className="text-sm">{inspected.value.label[language]}</strong>
          <span className="font-mono text-base">{valueText(inspected.value, language)}</span>
          <Tag>{inspected.value.quantity}</Tag>
          {inspected.item?.bone === undefined ? null : <Tag title={t("viewer.bone")}>{props.asset.rig.bones.find((bone) => bone.id === inspected.item?.bone)?.title[language] ?? inspected.item.bone.replaceAll("-", " ")}</Tag>}
          {inspected.item?.region === undefined ? null : <Tag title={t("viewer.region")}>{props.asset.thermal.regions.find((region) => region.id === inspected.item?.region)?.title[language] ?? inspected.item.region.replaceAll("-", " ")}</Tag>}
        </p>
      )}
    </section>
  );
}

function bound(value: number, language: Language): string {
  return formatNumber(value, decimalsFor(Math.abs(value) || 1, 3), language);
}

/** 🏷️ One legend: a gradient bar with its end values, quantity, unit, convention and interpretation. */
function LegendView(props: { readonly legend: Legend; readonly language: Language }): ReactElement {
  const { legend, language } = props;
  const gradient = `linear-gradient(to right, ${legend.stops.map((stop) => `${stop.color} ${Math.round(Math.min(Math.max(stop.at, 0), 1) * 100)}%`).join(", ")})`;
  return (
    <figure className="m-0 flex flex-col gap-single">
      <figcaption className="text-xs font-semibold">
        {legend.title[language]} <span className="font-mono font-normal text-muted-foreground">({legend.unit})</span>
      </figcaption>
      <div aria-hidden="true" className="h-3 w-full border border-border" style={{ backgroundImage: gradient }} />
      <div className="flex justify-between font-mono text-[0.65rem]">
        <span>{bound(legend.min, language)}</span>
        <span>{bound(legend.max, language)}</span>
      </div>
      <p className="m-0 text-[0.65rem] leading-normal text-muted-foreground">{legend.convention[language]}</p>
      <p className="m-0 text-[0.65rem] leading-normal text-muted-foreground">{legend.interpretation[language]}</p>
    </figure>
  );
}

/** 🏷️ The legends of a scene, or the statement that the view has none. */
export function Legends(props: { readonly scene: DrawingScene | null; readonly language: Language; readonly patterns: boolean }): ReactElement {
  const { t } = useArchive();
  const legends = props.scene?.legends ?? [];
  return (
    <section aria-label={t("viewer.legends")} className="flex flex-col gap-double border border-border bg-panel p-double">
      <h3 className="m-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("viewer.legends")}</h3>
      {legends.length === 0 ? <p className="m-0 text-xs text-muted-foreground">{t("viewer.legendsNone")}</p> : legends.map((legend, index) => <LegendView key={`${legend.quantity}-${index}`} legend={legend} language={props.language} />)}
      {props.patterns && legends.length > 0 ? <p className="m-0 text-[0.65rem] text-muted-foreground">{t("viewer.patternsMeaning")}</p> : null}
    </section>
  );
}
