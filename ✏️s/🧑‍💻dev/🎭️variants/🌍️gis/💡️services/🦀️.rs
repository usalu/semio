//! 🧩 Outward application assembly of installed native document services.
use semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1;

/// 📦 Supplies the concrete owner inventory installed by this application.
pub fn service_contributions_v1() -> Vec<InstalledServiceContributionV1> {
    vec![semio_s_artifact_gis_gismap::inference_worker::gis_map_service_contribution_v1()]
}

#[cfg(test)]
#[path="🧪️tests/🔌️service-composition/🦀️.rs"]
mod tests;
