//! 🛒️ Sourcing plugin — curate app: handpick and curate 3D object kinds out of a modular catalogue.

fn sourcing_setup() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<sourcing_ui::SourcingCurateApp>(sourcing::SOURCING_CURATE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "sourcing", label: "Sourcing", version: "0.1.0",
    setup: sourcing_setup,
    apps: [ sourcing_ui::create_sourcing_curate_app => sourcing_ui::SourcingCurateApp ],
}
