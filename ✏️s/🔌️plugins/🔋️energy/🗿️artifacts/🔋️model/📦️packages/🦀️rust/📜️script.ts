#!/usr/bin/env bun
/** 📦️ energy model Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runWeatherChecks } from "../../../../📦️packages/🟦️typescript/🧩️extensions/🌦️epw/🧪️testing/🔬️unit/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { runWeatherChronologyChecks } from "../../../../🔨️modules/⚡️simulation/⚙️engine/📍️site/📅️chronology/🧪️tests/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, scriptInvocationBudget } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runRepositoryExactCargoLaws as runOwnerExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";


/** 🔒️ Refreshes the artifact workspace lock through its original Repository command owner. */
class EnergyLockScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Energy lock accepts no arguments");
    await runRepositoryCommand(this.invocation, "cargo", ["update", "--workspace", "--manifest-path", resolve(this.root, "Cargo.toml")], this.repoRoot, "energy:lock", { version: 1, maximumElapsedMilliseconds: scriptInvocationBudget(this.invocation, 120000), output: { maximumBytes: 8388608, maximumLines: 65536 } });
  }
}

/** 🧩️ Routes native laws to their split energy owner packages. */
async function runExactCargoLaws(request: Omit<Parameters<typeof runOwnerExactCargoLaws>[0], "preparation">) {
  const plugin = "✏️s/🔌️plugins/🔋️energy";
  const artifact = `${plugin}/🗿️artifacts/🔋️model`;
  const subset = `${artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
  const owners = [
    ["kernel::", "semio-s-plugin-energy-engine-kernel", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel`],
    ["precompute::", "semio-s-plugin-energy-engine-precompute", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute`],
    ["sim::", "semio-s-plugin-energy-engine-sim", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧪️sim`],
    ["site::", "semio-s-plugin-energy-engine-site", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/📍️site`],
    ["epw::", "semio-s-plugin-energy-epw", `${plugin}/🧩️extensions/🌦️epw`],
    ["energy_simulation_session::", "semio-s-artifact-energy-model-session", `${subset}/🧵️simulation-session`],
    ["editor::model::results::", "semio-s-artifact-energy-model-editor-results", `${subset}/✏️editor/📊️results`],
  ] as const;
  const manifestPaths: Record<string, string> = {};
  const groups = request.groups.flatMap(group => group.laws.map(original => {
    const checkpoint = original.endsWith("weather_checkpoint_rejects_foreign_config_and_preserves_same_model_authority");
    const authoring = original.endsWith("pv_authored_thermal_mutations_preserve_original_rows_inverse_and_refusal");
    const owner = owners.find(([prefix]) => original.startsWith(prefix));
    if (!owner) throw Error(`Energy law has no split owner: ${original}`);
    const packageName = checkpoint ? "semio-s-plugin-energy-engine-sim-restore" : authoring ? "semio-s-artifact-energy-model-mutations" : owner[1];
    const directory = checkpoint ? `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️restore` : authoring ? `${subset}/🧬️schema/🧬️mutations` : owner[2];
    const law = checkpoint ? `tests::${original.slice(original.lastIndexOf("::") + 2)}` : authoring ? `optical_field_tests::${original.slice(original.lastIndexOf("::") + 2)}` : original.slice(owner[0].length);
    manifestPaths[packageName] = resolve(request.cwd, directory, "📦️packages/🦀️rust/Cargo.toml");
    return { ...group, package: packageName, target: { kind: "lib" as const, name: packageName.replaceAll("-", "_") }, laws: [law] };
  }));
  return runOwnerExactCargoLaws({ ...request, preparation: configuredRepositoryCargoPreparationPolicyV1(), manifestPaths, groups });
}

class EnergySolarVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!['solar-source','solar-native'].includes(segments[0]!) || segments.length > 2 || segments.length === 2 && segments[1] !== 'capture') throw new Error('Energy solar verification requires solar-source, solar-native or an explicit capture');
    const module = await import("../../../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧪️tests/🔆️pv-plane/🟦️.ts");
    if (segments[1] === 'capture') {
      const fixture = await import("../../../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧫️fixtures/🔆️pv-plane/🔣️.json");
      for (const row of fixture.default.cases) console.log(JSON.stringify({id:row.id,expected:module.calculateSolarPvReference(row)}));
      return;
    }
    const source = resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧪️tests/🔆️pv-plane/🟦️.ts");
    await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'energy:solar-source-typecheck',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
    module.verifySolarPvOracle();
    if (segments[0] === 'solar-source') return;
    const receipts = await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{'semio-s-artifact-energy-model':resolve(this.root,'Cargo.toml')},cwd:this.repoRoot,groups:[{package:'semio-s-artifact-energy-model',target:{kind:'lib',name:'semio_s_artifact_energy_model'},cargoArgs:['--locked'],laws:['kernel::tests::solar_pv::pv_kernel_preserves_independent_plane_components_weather_and_authored_efficiency','kernel::tests::solar_pv::pv_authored_thermal_mutations_preserve_original_rows_inverse_and_refusal']}]});
    console.log(`[DEBUG] Energy PV exact native passed ${receipts[0]!.assertions} law; executable ${receipts[0]!.sha256}`);
  }
}

class EnergyThermalVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy thermal verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:thermal-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts");module.verifyThermalBalanceOracle();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["kernel::tests::retained_thermal_free_constrained_and_partial_delivery_match_independent_balance","kernel::tests::retained_thermal_fault_refuses_physical_commit_and_restored_foreign_dimensions"]}]});
  console.log("[DEBUG] Energy retained thermal exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);
 }
}

class EnergyPublicationVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy publication verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📮️publication/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:publication-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📮️publication/🧪️tests/🟦️.ts");module.verifyEnergyPublicationOracle();
  const results=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");results.verifyEnergyResultReadOracle(this.repoRoot);
  await runOwnedCommand(process.execPath,["test",resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️close/🧪️tests/🟦️.ts")],this.repoRoot,"energy:terminal-owners-source",scriptInvocationBudget(this.invocation,15000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  if(segments[0]==="source")return;
  const session=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📦️packages/🦀️rust/Cargo.toml"),sim=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📦️packages/🦀️rust/Cargo.toml");
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model-session":session,"semio-s-plugin-energy-engine-sim":sim},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model-session",target:{kind:"lib"},cargoArgs:["--locked"],laws:["publication::tests::energy_original_prepared_esf2_ranges_match_sqlite_buffer_fixture_and_cancel_every_frontier","publication::tests::energy_original_surface_wire_matches_independent_fixed_width_fixture","publication::tests::energy_original_tick_publication_preserves_all_paid_source_page_and_close_frontiers"]},{package:"semio-s-plugin-energy-engine-sim",target:{kind:"lib"},cargoArgs:["--locked"],laws:["original_terminal_retirement::tests::original_terminal_backings_are_retired_before_energy_authority_drop"]}]});
  console.log("[DEBUG] Energy original publication exact native passed "+receipts.reduce((total,receipt)=>total+receipt.assertions,0)+" laws");
 }
}

class EnergyWeatherVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy weather verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/📅️chronology/🧪️tests/🟦️.ts")],this.repoRoot,"energy:weather-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  runWeatherChecks();runWeatherChronologyChecks();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["energy_simulation_session::tests::input_identity_progress_matches_neutral_bytes_unknown_total_and_original_counter","sim::tests::input_identity_original_full_fields_match_serde_and_paid_cancellation_frontiers","sim::tests::weather_job_original48_midnight_and_chronology_refusals_match_neutral_sources","sim::tests::weather_checkpoint_rejects_foreign_config_and_preserves_same_model_authority","site::chronology::tests::authored_weather_chronology_matches_neutral_midnight_leap_rollover_and_refusal_oracles","epw::tests::portable_weather_vectors_match_csv_and_serde_oracles","epw::tests::weather_record_derives_correct_fields_from_stdio_snapshot"]}]});
  console.log("[DEBUG] Energy weather exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);
 }
}


/** 🔲️ Retained original radiosity verifies independent matrix coefficients before exact native continuation. */
class EnergyEnclosureVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy enclosure verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🔲️enclosure/⚡️exchange/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:enclosure-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../../../🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🔲️enclosure/⚡️exchange/🧪️tests/🟦️.ts");module.verifyEnclosureExchangeOracle();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["precompute::enclosure_exchange::tests::enclosure_exchange_matches_portable_radiosity_without_emissivity_or_size_floors","precompute::retirement_tests::original_precompute_retirement_preserves_denied_owners_and_physical_receipts","sim::tests::energy_precompute_close_delegates_original_factory_quotes_and_physical_receipts"]}]});console.log("[DEBUG] Energy retained enclosure exact native passed "+receipts[0]!.assertions+" law; executable "+receipts[0]!.sha256);
 }
}

/** 📊️ Indexed original result reads verify portable large publications through independent SQL and Buffer. */
class EnergyResultsVerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("Energy results verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:results-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");module.verifyEnergyResultReadOracle(this.repoRoot);
  if(segments[0]==="native"){const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["editor::model::results::indexed::tests::energy_indexed_native_reads_use_prepared_original_partitions_and_locales"]}]});console.log("[DEBUG] Energy original indexed results exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);}
 }
}

/** 🧺️ Verifies caller-owned batch custody before exact native numerical continuation. */
class EnergyBatchVerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("Energy batch verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧺️batch/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:batch-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"energy:batch-source",scriptInvocationBudget(this.invocation,15000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  if(segments[0]==="source")return;
  const plugin="✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim";
  await runOwnerExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),preparation:configuredRepositoryCargoPreparationPolicyV1(),cwd:this.repoRoot,manifestPaths:{"semio-s-plugin-energy-engine-sim":resolve(this.repoRoot,plugin,"📦️packages/🦀️rust/Cargo.toml"),"semio-s-plugin-energy-engine-sim-restore":resolve(this.repoRoot,plugin,"♻️restore/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:"semio-s-plugin-energy-engine-sim",target:{kind:"lib"},cargoArgs:["--locked"],laws:["batch::tests::original_batch_slots_and_foreign_roots_keep_exact_custody","tests::p7c3_commit_lease_ack_is_the_exact_terminal_detach_witness"]},{package:"semio-s-plugin-energy-engine-sim-restore",target:{kind:"lib"},cargoArgs:["--locked"],laws:["tests::p7c2_restore_stale_step_and_install_preserve_exact_replay_authority","tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology"]}]});
 }
}








/** 🔒️ Refreshes the artifact workspace lock through its original Repository command owner. */
class EnergyLockScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Energy lock accepts no arguments");
    await runRepositoryCommand(this.invocation, "cargo", ["update", "--workspace", "--manifest-path", resolve(this.root, "Cargo.toml")], this.repoRoot, "energy:lock", { version: 1, maximumElapsedMilliseconds: scriptInvocationBudget(this.invocation, 120000), output: { maximumBytes: 8388608, maximumLines: 65536 } });
  }
}

/** 🧩️ Routes native laws to their split energy owner packages. */
async function runExactCargoLaws(request: Omit<Parameters<typeof runOwnerExactCargoLaws>[0], "preparation">) {
  const plugin = "✏️s/🔌️plugins/🔋️energy";
  const artifact = `${plugin}/🗿️artifacts/🔋️model`;
  const subset = `${artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
  const owners = [
    ["kernel::", "semio-s-plugin-energy-engine-kernel", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel`],
    ["precompute::", "semio-s-plugin-energy-engine-precompute", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute`],
    ["sim::", "semio-s-plugin-energy-engine-sim", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧪️sim`],
    ["site::", "semio-s-plugin-energy-engine-site", `${plugin}/🔨️modules/⚡️simulation/⚙️engine/📍️site`],
    ["epw::", "semio-s-plugin-energy-epw", `${plugin}/🧩️extensions/🌦️epw`],
    ["energy_simulation_session::", "semio-s-artifact-energy-model-session", `${subset}/🧵️simulation-session`],
    ["editor::model::results::", "semio-s-artifact-energy-model-editor-results", `${subset}/✏️editor/📊️results`],
  ] as const;
  const manifestPaths: Record<string, string> = {};
  const groups = request.groups.flatMap(group => group.laws.map(original => {
    const checkpoint = original.endsWith("weather_checkpoint_rejects_foreign_config_and_preserves_same_model_authority");
    const authoring = original.endsWith("pv_authored_thermal_mutations_preserve_original_rows_inverse_and_refusal");
    const owner = owners.find(([prefix]) => original.startsWith(prefix));
    if (!owner) throw Error(`Energy law has no split owner: ${original}`);
    const packageName = checkpoint ? "semio-s-plugin-energy-engine-sim-restore" : authoring ? "semio-s-artifact-energy-model-mutations" : owner[1];
    const directory = checkpoint ? `${plugin}/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️restore` : authoring ? `${subset}/🧬️schema/🧬️mutations` : owner[2];
    const law = checkpoint ? `tests::${original.slice(original.lastIndexOf("::") + 2)}` : authoring ? `optical_field_tests::${original.slice(original.lastIndexOf("::") + 2)}` : original.slice(owner[0].length);
    manifestPaths[packageName] = resolve(request.cwd, directory, "📦️packages/🦀️rust/Cargo.toml");
    return { ...group, package: packageName, target: { kind: "lib" as const, name: packageName.replaceAll("-", "_") }, laws: [law] };
  }));
  return runOwnerExactCargoLaws({ ...request, preparation: configuredRepositoryCargoPreparationPolicyV1(), manifestPaths, groups });
}

class EnergySolarVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!['solar-source','solar-native'].includes(segments[0]!) || segments.length > 2 || segments.length === 2 && segments[1] !== 'capture') throw new Error('Energy solar verification requires solar-source, solar-native or an explicit capture');
    const module = await import("../../../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧪️tests/🔆️pv-plane/🟦️.ts");
    if (segments[1] === 'capture') {
      const fixture = await import("../../../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧫️fixtures/🔆️pv-plane/🔣️.json");
      for (const row of fixture.default.cases) console.log(JSON.stringify({id:row.id,expected:module.calculateSolarPvReference(row)}));
      return;
    }
    const source = resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧪️tests/🔆️pv-plane/🟦️.ts");
    await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'energy:solar-source-typecheck',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
    module.verifySolarPvOracle();
    if (segments[0] === 'solar-source') return;
    const receipts = await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{'semio-s-artifact-energy-model':resolve(this.root,'Cargo.toml')},cwd:this.repoRoot,groups:[{package:'semio-s-artifact-energy-model',target:{kind:'lib',name:'semio_s_artifact_energy_model'},cargoArgs:['--locked'],laws:['kernel::tests::solar_pv::pv_kernel_preserves_independent_plane_components_weather_and_authored_efficiency','kernel::tests::solar_pv::pv_authored_thermal_mutations_preserve_original_rows_inverse_and_refusal']}]});
    console.log(`[DEBUG] Energy PV exact native passed ${receipts[0]!.assertions} law; executable ${receipts[0]!.sha256}`);
  }
}

class EnergyThermalVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy thermal verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:thermal-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts");module.verifyThermalBalanceOracle();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["kernel::tests::retained_thermal_free_constrained_and_partial_delivery_match_independent_balance","kernel::tests::retained_thermal_fault_refuses_physical_commit_and_restored_foreign_dimensions"]}]});
  console.log("[DEBUG] Energy retained thermal exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);
 }
}

class EnergyPublicationVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy publication verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📮️publication/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:publication-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📮️publication/🧪️tests/🟦️.ts");module.verifyEnergyPublicationOracle();
  const results=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");results.verifyEnergyResultReadOracle(this.repoRoot);
  await runOwnedCommand(process.execPath,["test",resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️close/🧪️tests/🟦️.ts")],this.repoRoot,"energy:terminal-owners-source",scriptInvocationBudget(this.invocation,15000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  if(segments[0]==="source")return;
  const session=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📦️packages/🦀️rust/Cargo.toml"),sim=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📦️packages/🦀️rust/Cargo.toml");
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model-session":session,"semio-s-plugin-energy-engine-sim":sim},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model-session",target:{kind:"lib"},cargoArgs:["--locked"],laws:["publication::tests::energy_original_prepared_esf2_ranges_match_sqlite_buffer_fixture_and_cancel_every_frontier","publication::tests::energy_original_surface_wire_matches_independent_fixed_width_fixture","publication::tests::energy_original_tick_publication_preserves_all_paid_source_page_and_close_frontiers"]},{package:"semio-s-plugin-energy-engine-sim",target:{kind:"lib"},cargoArgs:["--locked"],laws:["original_terminal_retirement::tests::original_terminal_backings_are_retired_before_energy_authority_drop"]}]});
  console.log("[DEBUG] Energy original publication exact native passed "+receipts.reduce((total,receipt)=>total+receipt.assertions,0)+" laws");
 }
}

class EnergyWeatherVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy weather verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/📅️chronology/🧪️tests/🟦️.ts")],this.repoRoot,"energy:weather-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  runWeatherChecks();runWeatherChronologyChecks();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["energy_simulation_session::tests::input_identity_progress_matches_neutral_bytes_unknown_total_and_original_counter","sim::tests::input_identity_original_full_fields_match_serde_and_paid_cancellation_frontiers","sim::tests::weather_job_original48_midnight_and_chronology_refusals_match_neutral_sources","sim::tests::weather_checkpoint_rejects_foreign_config_and_preserves_same_model_authority","site::chronology::tests::authored_weather_chronology_matches_neutral_midnight_leap_rollover_and_refusal_oracles","epw::tests::portable_weather_vectors_match_csv_and_serde_oracles","epw::tests::weather_record_derives_correct_fields_from_stdio_snapshot"]}]});
  console.log("[DEBUG] Energy weather exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);
 }
}


/** 🔲️ Retained original radiosity verifies independent matrix coefficients before exact native continuation. */
class EnergyEnclosureVerifyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(!["source","native"].includes(segments[0]!)||segments.length!==1)throw Error("Energy enclosure verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🔲️enclosure/⚡️exchange/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:enclosure-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../../../🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🔲️enclosure/⚡️exchange/🧪️tests/🟦️.ts");module.verifyEnclosureExchangeOracle();
  if(segments[0]==="source")return;
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["precompute::enclosure_exchange::tests::enclosure_exchange_matches_portable_radiosity_without_emissivity_or_size_floors","precompute::retirement_tests::original_precompute_retirement_preserves_denied_owners_and_physical_receipts","sim::tests::energy_precompute_close_delegates_original_factory_quotes_and_physical_receipts"]}]});console.log("[DEBUG] Energy retained enclosure exact native passed "+receipts[0]!.assertions+" law; executable "+receipts[0]!.sha256);
 }
}

/** 📊️ Indexed original result reads verify portable large publications through independent SQL and Buffer. */
class EnergyResultsVerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("Energy results verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:results-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  const module=await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts");module.verifyEnergyResultReadOracle(this.repoRoot);
  if(segments[0]==="native"){const receipts=await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),manifestPaths:{"semio-s-artifact-energy-model":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-s-artifact-energy-model",target:{kind:"lib",name:"semio_s_artifact_energy_model"},cargoArgs:["--locked"],laws:["editor::model::results::indexed::tests::energy_indexed_native_reads_use_prepared_original_partitions_and_locales"]}]});console.log("[DEBUG] Energy original indexed results exact native passed "+receipts[0]!.assertions+" laws; executable "+receipts[0]!.sha256);}
 }
}

/** 🧺️ Verifies caller-owned batch custody before exact native numerical continuation. */
class EnergyBatchVerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("Energy batch verification requires source or native");
  const source=resolve(this.repoRoot,"✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧺️batch/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"energy:batch-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"energy:batch-source",scriptInvocationBudget(this.invocation,15000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
  if(segments[0]==="source")return;
  const plugin="✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim";
  await runOwnerExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),preparation:configuredRepositoryCargoPreparationPolicyV1(),cwd:this.repoRoot,manifestPaths:{"semio-s-plugin-energy-engine-sim":resolve(this.repoRoot,plugin,"📦️packages/🦀️rust/Cargo.toml"),"semio-s-plugin-energy-engine-sim-restore":resolve(this.repoRoot,plugin,"♻️restore/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:"semio-s-plugin-energy-engine-sim",target:{kind:"lib"},cargoArgs:["--locked"],laws:["batch::tests::original_batch_slots_and_foreign_roots_keep_exact_custody","tests::p7c3_commit_lease_ack_is_the_exact_terminal_detach_witness"]},{package:"semio-s-plugin-energy-engine-sim-restore",target:{kind:"lib"},cargoArgs:["--locked"],laws:["tests::p7c2_restore_stale_step_and_install_preserve_exact_replay_authority","tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology"]}]});
 }
}

/** 🧩️ Every crate of the energy model artifact family (its workspace members), checked and tested together. */
const ENERGY_MODEL_PACKAGES = [
  "semio-s-artifact-energy-model",
  "semio-s-artifact-energy-model-diff",
  "semio-s-artifact-energy-model-editor",
  "semio-s-artifact-energy-model-editor-commands",
  "semio-s-artifact-energy-model-editor-config",
  "semio-s-artifact-energy-model-editor-edit",
  "semio-s-artifact-energy-model-editor-interaction",
  "semio-s-artifact-energy-model-editor-panels",
  "semio-s-artifact-energy-model-editor-results",
  "semio-s-artifact-energy-model-examples",
  "semio-s-artifact-energy-model-export",
  "semio-s-artifact-energy-model-import",
  "semio-s-artifact-energy-model-inferences",
  "semio-s-artifact-energy-model-io",
  "semio-s-artifact-energy-model-mut-air-loop",
  "semio-s-artifact-energy-model-mut-battery",
  "semio-s-artifact-energy-model-mut-boundary",
  "semio-s-artifact-energy-model-mut-construction",
  "semio-s-artifact-energy-model-mut-day-schedule",
  "semio-s-artifact-energy-model-mut-daylight-zone",
  "semio-s-artifact-energy-model-mut-enclosure",
  "semio-s-artifact-energy-model-mut-equipment-gain",
  "semio-s-artifact-energy-model-mut-fault",
  "semio-s-artifact-energy-model-mut-fenestration",
  "semio-s-artifact-energy-model-mut-finish",
  "semio-s-artifact-energy-model-mut-fixed-schedule",
  "semio-s-artifact-energy-model-mut-glazing",
  "semio-s-artifact-energy-model-mut-humidistat",
  "semio-s-artifact-energy-model-mut-ideal-loads",
  "semio-s-artifact-energy-model-mut-infiltration",
  "semio-s-artifact-energy-model-mut-leakage",
  "semio-s-artifact-energy-model-mut-lighting-gain",
  "semio-s-artifact-energy-model-mut-load-center",
  "semio-s-artifact-energy-model-mut-material",
  "semio-s-artifact-energy-model-mut-model",
  "semio-s-artifact-energy-model-mut-outdoor-air",
  "semio-s-artifact-energy-model-mut-people-gain",
  "semio-s-artifact-energy-model-mut-plant-loop",
  "semio-s-artifact-energy-model-mut-pv-system",
  "semio-s-artifact-energy-model-mut-refrigeration",
  "semio-s-artifact-energy-model-mut-room-air",
  "semio-s-artifact-energy-model-mut-run",
  "semio-s-artifact-energy-model-mut-setpoint",
  "semio-s-artifact-energy-model-mut-shading",
  "semio-s-artifact-energy-model-mut-shading-device",
  "semio-s-artifact-energy-model-mut-shw-system",
  "semio-s-artifact-energy-model-mut-site",
  "semio-s-artifact-energy-model-mut-sizing",
  "semio-s-artifact-energy-model-mut-solar-thermal",
  "semio-s-artifact-energy-model-mut-space",
  "semio-s-artifact-energy-model-mut-space-list",
  "semio-s-artifact-energy-model-mut-surface",
  "semio-s-artifact-energy-model-mut-thermostat",
  "semio-s-artifact-energy-model-mut-time-series",
  "semio-s-artifact-energy-model-mut-ventilation",
  "semio-s-artifact-energy-model-mut-water-system",
  "semio-s-artifact-energy-model-mut-week-schedule",
  "semio-s-artifact-energy-model-mut-window-layer",
  "semio-s-artifact-energy-model-mut-year-schedule",
  "semio-s-artifact-energy-model-mut-zone",
  "semio-s-artifact-energy-model-mut-zone-equipment",
  "semio-s-artifact-energy-model-mutation-fixtures",
  "semio-s-artifact-energy-model-mutations",
  "semio-s-artifact-energy-model-schema",
  "semio-s-artifact-energy-model-session",
  "semio-s-artifact-energy-model-snapshot",
  "semio-s-artifact-energy-model-viewer",
];

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-energy-model", { packages: ENERGY_MODEL_PACKAGES, commands:{"verify-batch":EnergyBatchVerifyScript,"verify-results":EnergyResultsVerifyScript,"verify-enclosure":EnergyEnclosureVerifyScript,"verify-weather":EnergyWeatherVerifyScript,verify:EnergySolarVerifyScript,"verify-thermal":EnergyThermalVerifyScript,"verify-publication":EnergyPublicationVerifyScript,lock:EnergyLockScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], twins: [{ name: "epw-weather", run: runWeatherChecks }] });
