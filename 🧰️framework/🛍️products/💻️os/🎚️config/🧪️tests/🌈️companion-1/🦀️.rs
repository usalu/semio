//! 🌈️ Companion tests of os-config (1 of 2): configuration schema, mutation and text IO tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_config::*;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod opening_config {
    pub use semio_framework_os_config::opening_config::*;

    #[path = "."]
    pub mod component {
        pub use semio_framework_os_config::opening_config::testkit::component::*;
        use semio_framework::{AppRef, AppRole};
        use semio_framework_artifact_reference::ArtifactDialect;
        use semio_framework_value_derive::{FromValue, ToValue};
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use semio_framework_value::DslValue;

        #[path = "../../🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;

        #[path = "../../🧬️schema/🎨️ui-preferences/🧪️tests/🌱️owned-customization/🦀️.rs"]
        mod owned_customization_tests;
    }

    #[path = "."]
    pub mod mutations {
        pub use semio_framework_os_config::opening_config::mutations::*;

        #[path = "."]
        pub mod admit_local_document {
            pub use semio_framework_os_config::opening_config::mutations::admit_local_document::*;
            use semio_framework_os_config::opening_config::mutations::retire_local_document::RetireLocalDocument;
            use semio_framework_os_config::opening_config::mutations::LocalCatalogConfigMutation;
            use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value::ToValue;
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/📥️admit-local-document/🧪️tests/📥️lists-a-persisted-studio-beside-an-imported-one/🦀️.rs"]
            mod tests_lists_a_persisted_studio_beside_an_imported_one;

            #[path = "../../🧬️schema/🧬️mutations/📥️admit-local-document/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod attach_local_folder {
            pub use semio_framework_os_config::opening_config::mutations::attach_local_folder::*;
            use semio_framework_os_config::opening_config::mutations::detach_local_folder::DetachLocalFolder;
            use semio_framework_os_config::opening_config::mutations::LocalFoldersConfigMutation;
            use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value::ToValue;
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/📎️attach-local-folder/🧪️tests/📎️remembers-the-folder-beside-another-document/🦀️.rs"]
            mod tests_remembers_the_folder_beside_another_document;

            #[path = "../../🧬️schema/🧬️mutations/📎️attach-local-folder/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod change_merge_policy {
            pub use semio_framework_os_config::opening_config::mutations::change_merge_policy::*;
            use semio_framework_os_config::opening_config::mutations::MergePolicyConfigMutation;
            use protocol::{Mutation, MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value_derive::{FromValue, ToValue};

            #[path = "../../🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/🪪️tightens-the-authority-to-vigilant/🦀️.rs"]
            mod tests_tightens_the_authority_to_vigilant;

            #[path = "../../🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod clear_default_app {
            pub use semio_framework_os_config::opening_config::mutations::clear_default_app::*;
            use semio_framework_os_config::opening_config::{OpeningDiff, OpeningPreferences};
            use semio_framework_os_config::opening_config::mutations::set_default_app::SetDefaultApp;
            use semio_framework_os_config::opening_config::mutations::OpeningConfigMutation;
            use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework::AppRole;
            use semio_framework_artifact_reference::ArtifactDialect;
            use semio_framework_value_derive::{FromValue, ToValue};
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/🦀️.rs"]
            mod tests_unpins_the_cad_editor_and_keeps_the_viewer_pin;

            #[path = "../../🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod detach_local_folder {
            pub use semio_framework_os_config::opening_config::mutations::detach_local_folder::*;
            use semio_framework_os_config::opening_config::mutations::attach_local_folder::{attach_local_folder, LocalFolderBindings, LocalFoldersDiff};
            use semio_framework_os_config::opening_config::mutations::LocalFoldersConfigMutation;
            use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value_derive::{FromValue, ToValue};
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/✂️detach-local-folder/🧪️tests/✂️forgets-the-folder-and-keeps-its-sibling/🦀️.rs"]
            mod tests_forgets_the_folder_and_keeps_its_sibling;

            #[path = "../../🧬️schema/🧬️mutations/✂️detach-local-folder/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod retire_local_document {
            pub use semio_framework_os_config::opening_config::mutations::retire_local_document::*;
            use semio_framework_os_config::opening_config::mutations::admit_local_document::{admit_local_document, LocalCatalog, LocalCatalogDiff};
            use semio_framework_os_config::opening_config::mutations::LocalCatalogConfigMutation;
            use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value_derive::{FromValue, ToValue};
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/📤️retire-local-document/🧪️tests/📤️unlists-the-studio-and-keeps-its-sibling/🦀️.rs"]
            mod tests_unlists_the_studio_and_keeps_its_sibling;

            #[path = "../../🧬️schema/🧬️mutations/📤️retire-local-document/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod set_default_app {
            pub use semio_framework_os_config::opening_config::mutations::set_default_app::*;
            use semio_framework_os_config::opening_config::{OpeningDiff, OpeningPreferences};
            use semio_framework_os_config::opening_config::mutations::clear_default_app::ClearDefaultApp;
            use semio_framework_os_config::opening_config::mutations::OpeningConfigMutation;
            use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework::{AppRef, AppRole};
            use semio_framework_artifact_reference::ArtifactDialect;
            use semio_framework_value_derive::{FromValue, ToValue};

            #[path = "../../🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/✏️repins-the-cad-editor-to-the-drafting-app/🦀️.rs"]
            mod tests_repins_the_cad_editor_to_the_drafting_app;

            #[path = "../../🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod sign_in {
            pub use semio_framework_os_config::opening_config::mutations::sign_in::*;
            use semio_framework_os_config::opening_config::mutations::IdentityConfigMutation;
            use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value::{FromValue, ToValue};
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🪪️replaces-the-active-session-with-a-second-account/🦀️.rs"]
            mod tests_replaces_the_active_session_with_a_second_account;

            #[path = "../../🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod sign_out {
            pub use semio_framework_os_config::opening_config::mutations::sign_out::*;
            use semio_framework_os_config::opening_config::mutations::sign_in::{sign_in, IdentityDiff, IdentitySetting};
            use semio_framework_os_config::opening_config::mutations::IdentityConfigMutation;
            use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
            use semio_framework_value_derive::{FromValue, ToValue};
            use serde::{Deserialize, Serialize};

            #[path = "../../🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🪪️clears-the-active-session/🦀️.rs"]
            mod tests_clears_the_active_session;

            #[path = "../../🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }

        #[path = "."]
        pub mod ui_preferences {
            pub use semio_framework_os_config::opening_config::mutations::ui_preferences::*;

            #[path = "."]
            pub mod set_appearance {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_appearance::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🌗️set-appearance/🧪️tests/✏️sets-appearance/🦀️.rs"]
                mod tests_sets_appearance;
            }
        }
    }
}
