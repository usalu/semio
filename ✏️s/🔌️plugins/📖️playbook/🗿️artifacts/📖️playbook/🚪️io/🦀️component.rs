//! playbook IO stdio matrix
pub fn register() {
    crate::artifacts::playbook::io::import::deserializers::artifacts::docx::register();
    crate::artifacts::playbook::io::export::serializers::artifacts::docx::register();
    crate::artifacts::playbook::io::import::deserializers::artifacts::json::register();
    crate::artifacts::playbook::io::export::serializers::artifacts::json::register();
    crate::artifacts::playbook::io::import::deserializers::artifacts::md::register();
    crate::artifacts::playbook::io::export::serializers::artifacts::md::register();
    crate::artifacts::playbook::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::playbook::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::playbook::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::playbook::io::export::serializers::artifacts::txt::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
