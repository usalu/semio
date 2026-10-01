//! 🗣️ S Studio app — locale × terminology label set (constitutional: ui/Terminology).

use semio_framework_plugin::app_labels;

//#region 🔖️Terminology
// 🗣️ Every operation/view-action/shell-action's German translation lives directly at its
// `.mutation()`/`.view_action()`/`.shell_action()` call site in `create_space_app`'s manifest.
app_labels! {
    pub struct SStudioLabels {
        apps_section: native_en "Apps", native_de "Apps", reuse_en "Apps", reuse_de "Apps";
        media_vfs_empty_message: native_en "No app instances in the workflow.", native_de "Keine App-Instanzen im Workflows.", reuse_en "No app instances in the workflow.", reuse_de "Keine App-Instanzen im Workflows.";
        add_parameter: native_en "Add Parameter", native_de "Parameter hinzufügen", reuse_en "Add Parameter", reuse_de "Parameter hinzufügen";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        min: native_en "Min", native_de "Min", reuse_en "Min", reuse_de "Min";
        max: native_en "Max", native_de "Max", reuse_en "Max", reuse_de "Max";
        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        add_option: native_en "Add option", native_de "Option hinzufügen", reuse_en "Add option", reuse_de "Option hinzufügen";
        new_option_placeholder: native_en "New option", native_de "Neue Option", reuse_en "New option", reuse_de "Neue Option";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        node_id: native_en "Node id", native_de "Knoten-ID", reuse_en "Node id", reuse_de "Knoten-ID";
        label: native_en "Label", native_de "Beschriftung", reuse_en "Label", reuse_de "Beschriftung";
        direct_value: native_en "Direct value", native_de "Direkter Wert", reuse_en "Direct value", reuse_de "Direkter Wert";
        workflow_node: native_en "Workflow node", native_de "Workflow-Knoten", reuse_en "Workflow node", reuse_de "Workflow-Knoten";
        workflow_nodes: native_en "Workflow nodes", native_de "Workflow-Knoten", reuse_en "Workflow nodes", reuse_de "Workflow-Knoten";
        app_instance: native_en "App instance", native_de "App-Instanz", reuse_en "App instance", reuse_de "App-Instanz";
        app_instances: native_en "App instances", native_de "App-Instanzen", reuse_en "App instances", reuse_de "App-Instanzen";
        select_hint: native_en "Select workflow nodes in the canvas.", native_de "Wähle Workflow-Knoten im Arbeitsbereich aus.", reuse_en "Select workflow nodes in the canvas.", reuse_de "Wähle Workflow-Knoten im Arbeitsbereich aus.";
        program_prefix: native_en "Program", native_de "Programm", reuse_en "Program", reuse_de "Programm";
        app_prefix: native_en "App", native_de "App", reuse_en "App", reuse_de "App";
        instance_id_prefix: native_en "Instance id", native_de "Instanz-ID", reuse_en "Instance id", reuse_de "Instanz-ID";
        bound_value_prefix: native_en "Bound value", native_de "Gebundener Wert", reuse_en "Bound value", reuse_de "Gebundener Wert";
        active_app: native_en "Active app", native_de "Aktive App", reuse_en "Active app", reuse_de "Aktive App";
        window_workflow: native_en "Workflow", native_de "Workflow", reuse_en "Workflow", reuse_de "Workflow";
        window_media_vfs: native_en "Media VFS", native_de "Media-VFS", reuse_en "Media VFS", reuse_de "Media-VFS";
        window_compiled_dag: native_en "Compiled DAG", native_de "Kompilierter DAG", reuse_en "Compiled DAG", reuse_de "Kompilierter DAG";
        toggle_on: native_en "On", native_de "An", reuse_en "On", reuse_de "An";
        toggle_off: native_en "Off", native_de "Aus", reuse_en "Off", reuse_de "Aus";
        mixed_placeholder: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
        parameter_count_suffix: native_en "parameter(s)", native_de "Parameter", reuse_en "parameter(s)", reuse_de "Parameter";
        media_node_count_label: native_en "node(s)", native_de "Knoten", reuse_en "node(s)", reuse_de "Knoten";
        app_instance_count_label: native_en "app instance(s)", native_de "App-Instanz(en)", reuse_en "app instance(s)", reuse_de "App-Instanz(en)";
        context_open_instance: native_en "Open instance", native_de "Instanz öffnen", reuse_en "Open instance", reuse_de "Instanz öffnen";
        context_duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        context_copy: native_en "Copy", native_de "Kopieren", reuse_en "Copy", reuse_de "Kopieren";
        context_paste: native_en "Paste", native_de "Einfügen", reuse_en "Paste", reuse_de "Einfügen";
        context_rename_label: native_en "Rename label…", native_de "Bezeichnung umbenennen…", reuse_en "Rename label…", reuse_de "Bezeichnung umbenennen…";
        context_remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        context_select_all: native_en "Select all", native_de "Alle auswählen", reuse_en "Select all", reuse_de "Alle auswählen";
        context_clear_selection: native_en "Clear selection", native_de "Auswahl aufheben", reuse_en "Clear selection", reuse_de "Auswahl aufheben";
        context_reorganize: native_en "Reorganize", native_de "Neu anordnen", reuse_en "Reorganize", reuse_de "Neu anordnen";
    }
}
//#endregion 🔖️Terminology
