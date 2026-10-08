//! 🖥 Native application with its explicit installed-service inventory.
fn main() {
    semio_s_dev_services::run_native_v1(semio_s_dev_gis_services::service_contributions_v1());
}
