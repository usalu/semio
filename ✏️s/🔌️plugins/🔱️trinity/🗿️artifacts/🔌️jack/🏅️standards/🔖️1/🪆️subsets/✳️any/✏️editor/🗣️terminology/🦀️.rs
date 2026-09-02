//! 🗣️ Trinity Jack app — locale/terminology label set.

use semio_framework_plugin::app_labels;

app_labels! {
    /// 🗣️ Complete UI label set for the Jack query app; one field per label makes every locale×terminology
    /// combination compile-checked. No distinct reuse-terminology concept for this app, so reuse repeats native.
    pub struct TrinityJackLabels {
        pieces: native_en "Pieces", native_de "Stücke", reuse_en "Pieces", reuse_de "Stücke";
        connections: native_en "Connections", native_de "Verbindungen", reuse_en "Connections", reuse_de "Verbindungen";
        fixtures: native_en "Fixtures", native_de "Fixturen", reuse_en "Fixtures", reuse_de "Fixturen";
        example_queries: native_en "Example queries", native_de "Beispielabfragen", reuse_en "Example queries", reuse_de "Beispielabfragen";
        manifest_kinds: native_en "Manifest kinds", native_de "Manifestarten", reuse_en "Manifest kinds", reuse_de "Manifestarten";
        piece: native_en "Piece", native_de "Stück", reuse_en "Piece", reuse_de "Stück";
        connection: native_en "Connection", native_de "Verbindung", reuse_en "Connection", reuse_de "Verbindung";
        connector: native_en "Connector", native_de "Verbinder", reuse_en "Connector", reuse_de "Verbinder";
        geometry: native_en "Geometry", native_de "Geometrie", reuse_en "Geometry", reuse_de "Geometrie";
        identity: native_en "Identity", native_de "Identität", reuse_en "Identity", reuse_de "Identität";
        history: native_en "History", native_de "Verlauf", reuse_en "History", reuse_de "Verlauf";
        query: native_en "Query", native_de "Abfrage", reuse_en "Query", reuse_de "Abfrage";
        window_graph: native_en "Nakagin Graph", native_de "Nakagin-Graph", reuse_en "Nakagin Graph", reuse_de "Nakagin-Graph";
        window_editor: native_en "Jack Query", native_de "Jack-Abfrage", reuse_en "Jack Query", reuse_de "Jack-Abfrage";
        window_results: native_en "Results", native_de "Ergebnisse", reuse_en "Results", reuse_de "Ergebnisse";
    }
}
