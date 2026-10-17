//! 🧪️ `apply-animation` inverse law on its committed wrapped-arm vector: the wrapped mutation is undone by its own inverse rows, replayed
//! last-to-first, and those rows' diffs sum to the negative of the forward diff.

use semio_s_artifact_stdio_semio::io::text::mutations::decode_semio_mutation_json;
use semio_s_artifact_stdio_semio::io::text::snapshot::decode_semio_snapshot_json;
use semio_framework_test_fixtures::fixture_text;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🎞️apply-animation-applied/⬅️before.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🎞️apply-animation-applied/🦠️mutation.json"));

#[semio_framework_async_macros::async_test]
async fn the_wrapped_animation_mutation_inverts_and_sums_to_the_negative_diff() {
    let base = decode_semio_snapshot_json((*BEFORE)).expect("committed before-envelope decodes");
    let mutation = decode_semio_mutation_json((*MUTATION)).expect("committed wrapped mutation decodes");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
