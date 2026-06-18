use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct NomoExtension;

impl zed::Extension for NomoExtension {
    fn new() -> Self {
        NomoExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        // Prefer an explicit override, then fall back to looking up `nomo-lsp` on
        // the worktree's PATH.
        let path = worktree
            .which("nomo-lsp")
            .ok_or_else(|| {
                "could not find `nomo-lsp` on PATH. Build it with `cargo install --path nomo-lsp` \
                 and ensure it is on your PATH."
                    .to_string()
            })?;

        Ok(Command {
            command: path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(NomoExtension);
