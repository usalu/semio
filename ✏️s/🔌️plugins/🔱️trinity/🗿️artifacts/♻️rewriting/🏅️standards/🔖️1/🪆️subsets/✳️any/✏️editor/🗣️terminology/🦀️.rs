//! 🗣️ Trinity Rewriting app — locale/terminology label set.

use semio_framework_plugin::app_labels;

app_labels! {
    /// 🗣️ Complete UI label set for the Rewrite rule app; one field per label makes every locale×terminology combination compile-checked. No distinct reuse-terminology concept for this app, so reuse repeats native.
    pub struct TrinityRewritingLabels {
        pieces: native_en "Pieces", native_de "Stücke", reuse_en "Pieces", reuse_de "Stücke";
        piece: native_en "Piece", native_de "Stück", reuse_en "Piece", reuse_de "Stück";
        connection: native_en "Connection", native_de "Verbindung", reuse_en "Connection", reuse_de "Verbindung";
        connector: native_en "Connector", native_de "Verbinder", reuse_en "Connector", reuse_de "Verbinder";
        catalogue: native_en "Catalogue", native_de "Katalog", reuse_en "Catalogue", reuse_de "Katalog";
        add_to_lhs: native_en "Add to LHS", native_de "Zu LHS hinzufügen", reuse_en "Add to LHS", reuse_de "Zu LHS hinzufügen";
        add_to_rhs: native_en "Add to RHS", native_de "Zu RHS hinzufügen", reuse_en "Add to RHS", reuse_de "Zu RHS hinzufügen";
        parameters: native_en "Parameters", native_de "Parameter", reuse_en "Parameters", reuse_de "Parameter";
        geometry: native_en "Geometry", native_de "Geometrie", reuse_en "Geometry", reuse_de "Geometrie";
        identity: native_en "Identity", native_de "Identität", reuse_en "Identity", reuse_de "Identität";
        history: native_en "History", native_de "Verlauf", reuse_en "History", reuse_de "Verlauf";
        rule: native_en "Rule", native_de "Regel", reuse_en "Rule", reuse_de "Regel";
        window_before: native_en "Before", native_de "Vorher", reuse_en "Before", reuse_de "Vorher";
        window_after: native_en "After", native_de "Nachher", reuse_en "After", reuse_de "Nachher";
        window_lhs: native_en "LHS", native_de "LHS", reuse_en "LHS", reuse_de "LHS";
        window_rhs: native_en "RHS", native_de "RHS", reuse_en "RHS", reuse_de "RHS";
        window_jack: native_en "Jack", native_de "Jack", reuse_en "Jack", reuse_de "Jack";
        window_parameters: native_en "Parameters", native_de "Parameter", reuse_en "Parameters", reuse_de "Parameter";
    }
}
