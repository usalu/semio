//! 🧹️ 🧹️ Remodeling play app commands command — `clear-geo-products`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::replace_geo_products;
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "clear-geo-products")]
pub struct ClearGeoProducts {}

pub fn handle(_payload: &ClearGeoProducts, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_geo_products(None)]))
}
