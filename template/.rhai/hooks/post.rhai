if variable::get("os-arch").starts_with("windows") {
    print("⚠️  post-hook: skipping final 'cargo fmt' on windows");
} else {
    system::command("cargo", ["fmt"]);
}

file::delete(".rhai");
