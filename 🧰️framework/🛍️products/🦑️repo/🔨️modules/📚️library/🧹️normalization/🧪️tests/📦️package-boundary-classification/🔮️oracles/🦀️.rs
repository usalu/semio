use std::{env, fs};
use syn::{parse::Parser, punctuated::Punctuated, Expr, File, Item, Stmt, Token};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    Declaration,
    Registration,
    Bootstrap,
    Thin,
    Implementation,
    Unresolved,
}

fn delegated_expression(expression: &Expr) -> bool {
    match expression {
        Expr::Call(_) | Expr::MethodCall(_) => true,
        Expr::Macro(value) => {
            let name = value.mac.path.segments.last().map(|segment| segment.ident.to_string());
            matches!(name.as_deref(), Some("register") | Some("mount") | Some("bootstrap") | Some("start") | Some("run"))
        }
        Expr::Return(value) => value.expr.as_deref().is_some_and(delegated_expression),
        Expr::Await(value) => delegated_expression(&value.base),
        Expr::Try(value) => delegated_expression(&value.expr),
        _ => false,
    }
}

fn delegated_block(block: &syn::Block, maximum: usize) -> bool {
    !block.stmts.is_empty()
        && block.stmts.len() <= maximum
        && block.stmts.iter().all(|statement| match statement {
            Stmt::Expr(expression, _) => delegated_expression(expression),
            Stmt::Macro(value) => value.semi_token.is_some(),
            _ => false,
        })
}

enum RegistrationMacro {
    Admitted,
    Implementation,
    Unresolved,
}

fn path_string(path: &syn::Path) -> String {
    path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("::")
}

fn path_argument(expression: &Expr) -> bool {
    match expression {
        Expr::Path(_) => true,
        Expr::Reference(value) => matches!(value.expr.as_ref(), Expr::Path(_)),
        _ => false,
    }
}

fn registration_macro(item: &syn::ItemMacro) -> RegistrationMacro {
    let name = path_string(&item.mac.path);
    if name == "semio_framework_plugin::plugin_exports" || name == "semio_framework_plugin::extension_exports" {
        let parser = Punctuated::<syn::Path, Token![,]>::parse_terminated;
        return if parser.parse2(item.mac.tokens.clone()).is_ok() { RegistrationMacro::Admitted } else { RegistrationMacro::Implementation };
    }
    if name == "inventory::submit" {
        let admitted = syn::parse2::<Expr>(item.mac.tokens.clone()).is_ok_and(|expression| match expression {
            Expr::Call(call) => matches!(call.func.as_ref(), Expr::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "new"))
                && call.args.len() == 1 && call.args.first().is_some_and(path_argument),
            _ => false,
        });
        return if admitted { RegistrationMacro::Admitted } else { RegistrationMacro::Implementation };
    }
    RegistrationMacro::Unresolved
}

fn classify_items(items: &[Item], maximum: usize) -> Role {
    let mut role = Role::Declaration;
    for item in items {
        let next = match item {
            Item::Use(_) | Item::ExternCrate(_) => Role::Declaration,
            Item::Type(value) if value.attrs.iter().any(|attribute| attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr")) => Role::Declaration,
            Item::Type(_) => Role::Implementation,
            Item::Mod(value) => value.content.as_ref().map_or(Role::Declaration, |(_, items)| classify_items(items, maximum)),
            Item::Macro(value) if value.mac.path.is_ident("include") => Role::Declaration,
            Item::Macro(value) => match registration_macro(value) {
                RegistrationMacro::Admitted => Role::Registration,
                RegistrationMacro::Implementation => Role::Implementation,
                RegistrationMacro::Unresolved => Role::Unresolved,
            },
            Item::Fn(value) if value.attrs.iter().any(|attribute| {
                let name = attribute.path().segments.last().map(|segment| segment.ident.to_string());
                name.as_deref().is_some_and(|name| name == "proc_macro" || name == "proc_macro_attribute" || name == "proc_macro_derive")
            }) && delegated_block(&value.block, maximum) => Role::Registration,
            Item::Fn(value) if delegated_block(&value.block, maximum) => {
                let name = value.sig.ident.to_string();
                if name == "main" || name == "start" || name == "bootstrap" { Role::Bootstrap } else { Role::Thin }
            }
            _ => Role::Implementation,
        };
        if next == Role::Implementation { return next; }
        if next == Role::Unresolved { role = next; }
        else if role != Role::Unresolved && next != Role::Declaration { role = next; }
    }
    role
}

fn main() {
    let path = env::args_os().nth(1).expect("source path");
    let maximum = env::args().nth(2).and_then(|value| value.parse().ok()).expect("maximum statements");
    let source = fs::read_to_string(path).expect("source bytes");
    let role = syn::parse_file(&source).map(|file: File| classify_items(&file.items, maximum)).unwrap_or(Role::Unresolved);
    print!("{}", match role {
        Role::Declaration => "declaration",
        Role::Registration => "registration",
        Role::Bootstrap => "bootstrap",
        Role::Thin => "thin-delegation",
        Role::Implementation => "implementation",
        Role::Unresolved => "unresolved",
    });
}
