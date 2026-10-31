/** 🧬️ The model assumptions of an asset: the thermal and the structural inspector with parameter tables, solved values and source tags (measured, literature, estimate, illustrative).
 *
 * Sign conventions are stated where values appear: surface fluxes are positive when leaving the surface; axial force N > 0 is tension; bending moment M > 0 is sagging; joint moments are counter-clockwise positive.
 * Thermal fields are engineering estimates, not CFD; human joint loads are rigid-segment resultants, not clinical bone or muscle forces.
 * @see ../../../../../../🧬️schema/🟦️.ts — `ParameterSource`
 */
import type { ReactElement } from "react";
import { formatNumber, formatQuantity } from "@teaching/staffage";
import type { Asset, Environment, EvaluatedState, Language, ParameterSource, StructuralResult, ThermalField, ThermalRegion } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { cn } from "../../🪟️chrome/🟦️.tsx";
import { DataTable, Fact, Row, SOURCES, num, sci } from "../🧱️tables/🟦️.tsx";

const SHAPES: Readonly<Record<ParameterSource, string>> = { measured: "M12 4a8 8 0 100 16 8 8 0 000-16z", literature: "M5 5h14v14H5z", calibrated: "M12 3a9 9 0 100 18 9 9 0 000-18zm0 4a5 5 0 110 10 5 5 0 010-10z", "authored-estimate": "M12 4l9 16H3z", illustrative: "M12 3l9 9-9 9-9-9z" };

/** 🏷️ A source tag: a distinct shape, the source in words and the resolved citation as tooltip. */
export function SourceTag(props: { readonly source: ParameterSource; readonly reference?: string | undefined; readonly asset?: Asset | undefined }): ReactElement {
  const { source, reference, asset } = props;
  const { t } = useArchive();
  const citation = reference === undefined ? undefined : asset?.provenance.references.find((candidate) => candidate.id === reference)?.citation ?? reference;
  return (
    <span data-source={source} title={citation === undefined ? t(`detail.source.${source}.hint`) : `${t(`detail.source.${source}.hint`)} — ${citation}`} className={cn("inline-flex items-center gap-1 border px-single py-0.5 font-sans text-[0.65rem]", source === "illustrative" ? "border-dashed border-border" : "border-border")}>
      <svg viewBox="0 0 24 24" width="10" height="10" aria-hidden="true" focusable="false">
        <path d={SHAPES[source]} fill={source === "measured" || source === "literature" || source === "calibrated" ? "currentColor" : "none"} stroke="currentColor" strokeWidth="2.5" strokeDasharray={source === "illustrative" ? "4 3" : undefined} />
      </svg>
      {t(`detail.source.${source}`)}
      {reference === undefined ? null : <span className="text-muted-foreground">· {reference}</span>}
    </span>
  );
}

const flux = (value: number, language: Language): string => `${formatNumber(value, Math.abs(value) < 10 ? 1 : 0, language)} W/m²`;

function parameters(region: ThermalRegion, language: Language, t: ReturnType<typeof useArchive>["t"]): string[] {
  const list: string[] = [];
  const resistance = region.layers.reduce((sum, layer) => sum + layer.resistanceM2KW, 0);
  if (region.layers.length > 0) list.push(`${t("detail.param.insulation")} ${num(resistance, language)} m²K/W`);
  if (region.body !== undefined) list.push(`${t("detail.param.share")} ${num(region.body.metabolicShare * 100, language)} %`);
  if (region.leaf !== undefined) list.push(`${t("detail.param.stomatal")} ${sci(region.leaf.stomatalConductanceMaxMS, language)} m/s`);
  if (region.operating !== undefined) list.push(`${t("detail.param.operating")} ${region.operating.key} · ${num(region.operating.heatW, language)} W`);
  if (region.conditioned !== undefined) list.push(`${t("detail.param.coreTemp")} ${num(region.conditioned.interiorTempC, language)} °C`);
  if (region.fixed !== undefined) list.push(`${t("detail.param.surfaceTemp")} ${num(region.fixed.surfaceTempC, language)} °C`);
  return list;
}

function ThermalInspector(props: { readonly asset: Asset; readonly thermal: ThermalField | null; readonly environment: Environment; readonly language: Language }): ReactElement {
  const { asset, thermal, environment, language } = props;
  const { t } = useArchive();
  const model = asset.thermal;
  return (
    <div className="flex flex-col gap-double" data-inspector="thermal">
      <h3 className="m-0 text-sm font-semibold">{t("detail.thermal.title")}</h3>
      <p className="m-0 max-w-[70ch] text-xs leading-normal text-muted-foreground">{t("detail.thermal.honesty")}</p>
      <dl className="m-0 grid gap-x-4 sm:grid-cols-3 xl:grid-cols-6">
        <Fact term={t("detail.env.air")}>{num(environment.airTempC, language)} °C</Fact>
        <Fact term={t("detail.env.radiant")}>{num(environment.meanRadiantTempC, language)} °C</Fact>
        <Fact term={t("detail.env.humidity")}>{num(environment.relativeHumidity * 100, language)} %</Fact>
        <Fact term={t("detail.env.wind")}>{num(environment.windMS, language)} m/s · {num(environment.windDirectionDeg, language)}°</Fact>
        <Fact term={t("detail.env.solar")}>{num(environment.solarWM2, language)} W/m²</Fact>
        <Fact term={t("detail.env.pressure")}>{num(environment.pressurePa, language)} Pa</Fact>
      </dl>
      <DataTable caption={t("detail.thermal.regions")} head={[t("detail.col.region"), t("detail.col.kind"), t("detail.col.model"), t("detail.col.area"), t("detail.col.length"), t("detail.col.parameters"), t("detail.col.source")]}>
        {model.regions.map((region) => (
          <Row key={region.id} head={region.title[language]} cells={[region.kind, <span key="m" className="font-sans"><strong>{region.model}</strong><br />{t(`detail.model.${region.model}`)}</span>, `${num(region.areaM2, language)} m²`, `${num(region.characteristicLengthM, language)} m`, <ul key="p" className="m-0 list-none p-0 font-sans">{parameters(region, language, t).map((line) => <li key={line}>{line}</li>)}</ul>, <SourceTag key="s" source={region.source} reference={region.reference} asset={asset} />]} />
        ))}
      </DataTable>
      {model.emitters.length === 0 ? null : (
        <DataTable caption={t("detail.thermal.emitters")} head={[t("detail.col.emitter"), t("detail.col.kind"), t("detail.col.flow"), t("detail.col.rise"), t("detail.col.diameter"), t("detail.col.binding"), t("detail.col.source")]}>
          {model.emitters.map((emitter) => (
            <Row key={emitter.id} head={emitter.title[language]} cells={[emitter.kind, emitter.volumeFlowM3S === undefined ? "—" : `${sci(emitter.volumeFlowM3S, language)} m³/s`, emitter.temperatureRiseK === undefined ? "—" : `${num(emitter.temperatureRiseK, language)} K`, `${num(emitter.diameterM, language)} m`, emitter.operatingKey ?? emitter.region ?? "—", <SourceTag key="s" source={emitter.source} reference={emitter.reference} asset={asset} />]} />
          ))}
        </DataTable>
      )}
      {thermal === null ? <p className="m-0 text-xs text-muted-foreground">{t("detail.thermal.unsolved")}</p> : (
        <>
          <DataTable caption={t("detail.thermal.solved")} head={[t("detail.col.region"), t("detail.col.surfaceTemp"), t("detail.col.skin"), t("detail.col.radiant"), t("detail.col.net"), t("detail.col.convective"), t("detail.col.latent"), t("detail.col.moisture")]}>
            {model.regions.flatMap((region) => {
              const field = thermal.regions[region.id];
              return field === undefined ? [] : [<Row key={region.id} head={region.title[language]} cells={[`${num(field.surfaceTempC, language)} °C`, field.skinTempC === null ? "—" : `${num(field.skinTempC, language)} °C`, flux(field.emittedRadiantWM2, language), flux(field.netRadiativeWM2, language), flux(field.sensibleWM2, language), flux(field.latentWM2, language), `${num(field.moistureGM2H, language)} g/(m²·h)`]} />];
            })}
          </DataTable>
          <p className="m-0 text-xs text-muted-foreground">{t("detail.thermal.sign")}</p>
          {thermal.body === null ? null : (
            <dl className="m-0 grid gap-x-4 sm:grid-cols-2" data-totals="body">
              <Fact term={t("detail.thermal.core")}>{num(thermal.body.coreTempC, language)} °C</Fact>
              <Fact term={t("detail.thermal.skin")}>{num(thermal.body.meanSkinTempC, language)} °C</Fact>
            </dl>
          )}
          <dl className="m-0 grid gap-x-4 sm:grid-cols-3 xl:grid-cols-6" data-totals="thermal">
            <Fact term={t("detail.totals.radiant")}>{formatQuantity(thermal.totals.netRadiativeW, "W", language)}</Fact>
            <Fact term={t("detail.totals.convective")}>{formatQuantity(thermal.totals.sensibleW, "W", language)}</Fact>
            <Fact term={t("detail.totals.latent")}>{formatQuantity(thermal.totals.latentW, "W", language)}</Fact>
            <Fact term={t("detail.totals.moisture")}>{num(thermal.totals.moistureGH, language)} g/h</Fact>
            <Fact term={t("detail.totals.emitterSensible")}>{formatQuantity(thermal.totals.emitterSensibleW, "W", language)}</Fact>
            <Fact term={t("detail.totals.emitterLatent")}>{formatQuantity(thermal.totals.emitterLatentW, "W", language)}</Fact>
          </dl>
        </>
      )}
      <Assumptions items={model.assumptions} language={language} />
    </div>
  );
}

function Assumptions(props: { readonly items: readonly { readonly en: string; readonly de: string }[]; readonly language: Language }): ReactElement | null {
  const { t } = useArchive();
  if (props.items.length === 0) return null;
  return (
    <div>
      <h5 className="m-0 mb-single text-xs font-semibold">{t("detail.assumptions")}</h5>
      <ul className="m-0 list-disc pl-4 text-xs leading-normal">{props.items.map((item, index) => <li key={index}>{item[props.language]}</li>)}</ul>
    </div>
  );
}

function StructuralInspector(props: { readonly asset: Asset; readonly state: EvaluatedState | null; readonly structure: StructuralResult | null; readonly language: Language }): ReactElement {
  const { asset, state, structure, language } = props;
  const { t } = useArchive();
  const model = asset.structure;
  const massed = asset.rig.bones.filter((bone) => bone.mass !== undefined || bone.section !== undefined);
  return (
    <div className="flex flex-col gap-double" data-inspector="structural">
      <h3 className="m-0 text-sm font-semibold">{t("detail.structure.title")}</h3>
      {asset.rig.kind === "anatomical" ? <p className="m-0 max-w-[70ch] text-xs leading-normal text-muted-foreground">{t("detail.structure.notClinical")}</p> : null}
      <p className="m-0 text-xs text-muted-foreground">{t("detail.structure.sign")}</p>
      <dl className="m-0 grid gap-x-4 sm:grid-cols-3">
        <Fact term={t("detail.structure.gravity")}>{num(model.gravityMS2, language)} m/s²</Fact>
        <Fact term={t("detail.structure.dynamics")}>{t(`detail.dynamics.${model.dynamics}`)}</Fact>
        <Fact term={t("detail.structure.supports")}>{model.supports.length === 0 ? "—" : model.supports.map((id) => asset.anchors.find((anchor) => anchor.id === id)?.title[language] ?? id).join(", ")}</Fact>
      </dl>
      <DataTable caption={t("detail.structure.bones")} head={[t("detail.col.bone"), t("detail.col.mass"), t("detail.col.com"), t("detail.col.section"), t("detail.col.source")]}>
        {massed.map((bone) => (
          <Row key={bone.id} head={bone.title[language]} cells={[bone.mass === undefined ? "—" : `${num(bone.mass.kg, language)} kg · ${t(`detail.distribution.${bone.mass.distribution}`)}`, bone.mass === undefined ? "—" : `(${num(bone.mass.comX, language)}; ${num(bone.mass.comY, language)}) m`, bone.section === undefined ? "—" : `A ${sci(bone.section.areaM2, language)} m² · I ${sci(bone.section.secondMomentM4, language)} m⁴ · c ${num(bone.section.extremeFibreM, language)} m · ${bone.section.material}`, bone.mass === undefined ? "—" : <SourceTag key="s" source={bone.mass.source} reference={bone.mass.reference} asset={asset} />]} />
        ))}
      </DataTable>
      <DataTable caption={t("detail.structure.materials")} head={[t("detail.col.material"), t("detail.col.modulus"), t("detail.col.strength"), t("detail.col.density"), t("detail.col.conductivity"), t("detail.col.emissivity"), t("detail.col.source")]}>
        {asset.materials.map((material) => (
          <Row key={material.id} head={material.title[language]} cells={[material.elasticModulusPa === undefined ? "—" : formatQuantity(material.elasticModulusPa, "Pa", language), material.strengthPa === undefined ? "—" : formatQuantity(material.strengthPa, "Pa", language), material.densityKgM3 === undefined ? "—" : `${num(material.densityKgM3, language)} kg/m³`, material.conductivityWmK === undefined ? "—" : `${num(material.conductivityWmK, language)} W/(m·K)`, num(material.emissivity, language), <SourceTag key="s" source={material.source} reference={material.reference} asset={asset} />]} />
        ))}
      </DataTable>
      {state === null || state.loads.length === 0 ? <p className="m-0 text-xs text-muted-foreground">{t("detail.structure.noLoads")}</p> : (
        <DataTable caption={t("detail.structure.loads")} head={[t("detail.col.load"), t("detail.col.kind"), t("detail.col.fx"), t("detail.col.fy"), t("detail.col.moment")]}>
          {state.loads.map((load) => <Row key={`${load.id}-${load.at.x}-${load.at.y}`} head={load.title[language]} cells={[load.kind, formatQuantity(load.fxN, "N", language), formatQuantity(load.fyN, "N", language), formatQuantity(load.momentNm, "N·m", language)]} />)}
        </DataTable>
      )}
      {structure === null ? <p className="m-0 text-xs text-muted-foreground">{t("detail.structure.unsolved")}</p> : (
        <>
          <dl className="m-0 grid gap-x-4 sm:grid-cols-3" data-totals="structure">
            <Fact term={t("detail.structure.totalMass")}>{num(structure.totalMassKg, language)} kg</Fact>
            <Fact term={t("detail.structure.centre")}>({num(structure.centreOfMass.x, language)}; {num(structure.centreOfMass.y, language)}) m</Fact>
            <Fact term={t("detail.structure.residual")}>{formatQuantity(structure.residual.fxN, "N", language)} · {formatQuantity(structure.residual.fyN, "N", language)} · {formatQuantity(structure.residual.momentNm, "N·m", language)}</Fact>
          </dl>
          <DataTable caption={t("detail.structure.reactions")} head={[t("detail.col.anchor"), t("detail.col.fx"), t("detail.col.fy"), t("detail.col.moment"), t("detail.col.pressure"), t("detail.col.active")]}>
            {structure.reactions.map((reaction) => <Row key={reaction.anchor} head={asset.anchors.find((anchor) => anchor.id === reaction.anchor)?.title[language] ?? reaction.anchor} cells={[formatQuantity(reaction.fxN, "N", language), formatQuantity(reaction.fyN, "N", language), formatQuantity(reaction.momentNm, "N·m", language), reaction.pressurePa === null ? "—" : formatQuantity(reaction.pressurePa, "Pa", language), t(reaction.active ? "detail.yes" : "detail.no")]} />)}
          </DataTable>
          <details>
            <summary className="min-h-6 cursor-pointer text-xs">{t("detail.structure.joints")}</summary>
            <DataTable className="mt-single" caption={t("detail.structure.joints")} head={[t("detail.col.bone"), t("detail.col.axial"), t("detail.col.shear"), t("detail.col.jointMoment")]}>
              {Object.values(structure.joints).map((joint) => <Row key={joint.bone} head={asset.rig.bones.find((bone) => bone.id === joint.bone)?.title[language] ?? joint.bone} cells={[formatQuantity(joint.axialN, "N", language), formatQuantity(joint.shearN, "N", language), formatQuantity(joint.momentNm, "N·m", language)]} />)}
            </DataTable>
          </details>
        </>
      )}
      <Assumptions items={model.assumptions} language={language} />
    </div>
  );
}

/** 🧬️ The model section: thermal and structural inspector of one asset at the evaluated state. */
export function ModelSection(props: { readonly asset: Asset; readonly state: EvaluatedState | null; readonly thermal: ThermalField | null; readonly structure: StructuralResult | null; readonly environment: Environment; readonly language: Language }): ReactElement {
  const { t } = useArchive();
  return (
    <section aria-labelledby="detail-model" className="flex flex-col gap-double" data-section="model">
      <h2 id="detail-model" className="m-0 font-serif text-lg">{t("detail.model.title")}</h2>
      <ul className="m-0 flex list-none flex-wrap gap-single p-0" aria-label={t("detail.source.legend")}>
        {SOURCES.map((source) => <li key={source}><SourceTag source={source} /></li>)}
      </ul>
      <ThermalInspector asset={props.asset} thermal={props.thermal} environment={props.environment} language={props.language} />
      <StructuralInspector asset={props.asset} state={props.state} structure={props.structure} language={props.language} />
    </section>
  );
}
