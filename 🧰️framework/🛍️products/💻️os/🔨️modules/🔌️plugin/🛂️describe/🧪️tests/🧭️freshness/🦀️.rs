//! 🧭️ A committed descriptor is fresh only when it equals the assembled one; the fixture is a real emitted pair (`🧫️descriptor-freshness`), so the law needs no plugin runtime.

use super::{descriptor_freshness_fault, encode_package_descriptor};
use semio_framework::PackageDescriptor;

/// 📦️ The emitted pack bytes of the fixture descriptor.
const ASSEMBLED: &[u8] = include_bytes!("../🧫️fixtures/🧫️descriptor-freshness/🛂️.descriptor.semio");

/// 🛣️ LAW: identical pair fresh; one changed label byte stale; an undecodable committed pair and an undecodable assembled pair each a fault, so `descriptor_is_fresh` can never pass vacuously.
#[test]
fn a_committed_descriptor_is_fresh_only_when_it_equals_the_assembled_one() {
    assert_eq!(descriptor_freshness_fault(ASSEMBLED, ASSEMBLED), None);
    let mut stale: PackageDescriptor = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(ASSEMBLED).expect("fixture wire")).expect("fixture shape");
    stale.manifest.label.push('!');
    let stale = encode_package_descriptor(&stale);
    assert!(descriptor_freshness_fault(ASSEMBLED, &stale).is_some_and(|fault| fault.contains("stale")));
    assert!(descriptor_freshness_fault(ASSEMBLED, b"not a pack").is_some_and(|fault| fault.contains("committed")));
    assert!(descriptor_freshness_fault(b"not a pack", ASSEMBLED).is_some_and(|fault| fault.contains("assembled")));
}
