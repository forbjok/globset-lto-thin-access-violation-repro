use std::path::Path;

fn main() {
    let root_path = Path::new(r#"E:\temp"#);

    let ignore_globset = globset::GlobSetBuilder::new()
        .add(globset::Glob::new("**/.git").unwrap())
        .build().unwrap();

    dbg!("BEFORE");

    // This will crash with an access violation when built in release with lto = "thin".
    ignore_globset.is_match(&root_path);

    dbg!("AFTER");
}
