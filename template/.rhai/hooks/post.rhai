file::delete(".rhai");
file::rename("include.gitignore", ".gitignore");
system::command("cargo", ["fmt"]);