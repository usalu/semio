//! 🔗️ `delete-fastener` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

async fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}

/// 🗑️ Deletes one fastener by id.
pub async fn delete_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = arg_str(args, "id").or_else(|| arg_str(args, "fastenerId")) else {
        return;
    };
    ctx.scene.document.fasteners.retain(|fastener| fastener.id != id);
}
