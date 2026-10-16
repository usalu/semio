//! 🍎️ Companion test crate tests-01 of the plugin SDK partition (hosts: none).
//!
//! 🧭️ WP1 left this companion empty on purpose: every WP1 test that cannot stay in its crate names `cfg(test)` fixtures of the facade (`test_app_mutation_fixture`, `publication_fixture`, `tool_jobs::test_*`) or items of layers above its crate, which a dev-dependency cannot reach; those tests run as hub test hosts of the facade (`🪝️mounts/🌱️wp1-foundation/🧪️test-hosts` and the `app_*_tests` mounts of hub 1) until the facade fixtures are public API (WP7 re-homes them here).
