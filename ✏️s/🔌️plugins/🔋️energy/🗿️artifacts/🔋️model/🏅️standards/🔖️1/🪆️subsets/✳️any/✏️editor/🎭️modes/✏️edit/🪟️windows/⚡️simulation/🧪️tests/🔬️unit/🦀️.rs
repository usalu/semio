
use super::*;

#[test]
fn actions_are_localized_and_registered_as_interactive() {
    let definition = definition();
    assert_eq!(definition.actions.len(), 7);
    assert!(definition.actions.iter().any(|action| action.id == crate::editor::model::SET_RUN_PERIOD_ACTION_ID), "the run period is edited here, so it must be declared here");
    assert!(definition.actions.iter().all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
    assert_eq!(definition.label, LocalizedLabel::native("Energy simulation", "Energiesimulation"));
    for action in &definition.actions {
        assert!(
            semio_framework::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, semio_framework::Locale::En) != action.label.resolve(terminology, semio_framework::Locale::De)),
            "action {} is not really translated",
            action.id
        );
    }
}

#[test]
fn an_idle_window_still_renders_the_editable_run_settings() {
    let model = crate::model::Model::default();
    let node = render(None, EnergySimulationConfigProjection::DEFAULT, &model, false);
    let text = format!("{node:?}");
    assert!(text.contains("energy-settings"), "the settings block must render without a live run");
    assert!(text.contains("energy-setting-run-period"));
    assert!(text.contains("energy-keyboard-start"));
}

#[test]
fn both_authored_languages_produce_different_text() {
    let model = crate::model::Model::default();
    let english = format!("{:?}", render(None, EnergySimulationConfigProjection::DEFAULT, &model, false));
    let german = format!("{:?}", render(None, EnergySimulationConfigProjection::DEFAULT, &model, true));
    assert_ne!(english, german);
    assert!(german.contains("Laufeinstellungen"));
    assert!(english.contains("Run settings"));
}

#[test]
fn every_worker_stage_is_authored_in_both_languages() {
    let stages = [
        EnergyJobStage::Validate,
        EnergyJobStage::ResolveWeather,
        EnergyJobStage::Precompute,
        EnergyJobStage::InitializeZones,
        EnergyJobStage::InitializeSurfaces,
        EnergyJobStage::InitializeWarmupHistory,
        EnergyJobStage::WarmupTimestep,
        EnergyJobStage::WarmupConvergence,
        EnergyJobStage::StartRun,
        EnergyJobStage::RunZoneTimestep,
        EnergyJobStage::AggregateZone,
        EnergyJobStage::AggregateFacility,
        EnergyJobStage::PublishTimestep,
        EnergyJobStage::Finalize,
        EnergyJobStage::Size,
        EnergyJobStage::FinalizeSummaries,
        EnergyJobStage::FinalizeMetrics,
        EnergyJobStage::FinalizeEconomics,
        EnergyJobStage::BuildResults,
        EnergyJobStage::PublishFinal,
        EnergyJobStage::EncodeOutput,
        EnergyJobStage::Complete,
    ];
    for stage in stages {
        assert_ne!(stage_text(stage, false), stage_text(stage, true), "stage {stage:?} is not really translated");
    }
}
