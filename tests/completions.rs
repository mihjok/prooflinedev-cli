//! Completion scripts come from the real command definitions and never touch
//! the update cache, because a shell runs them at every startup.
use std::process::Command;

fn completions(shell: &str, home: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_proofline"))
        .args(["completions", shell])
        .env_remove("PROOFLINE_NO_UPDATE_CHECK")
        .env_remove("PROOFLINE_BASE_URL")
        .env_remove("PROOFLINE_TOKEN")
        .env("HOME", home)
        .env("XDG_CACHE_HOME", home)
        .env("LOCALAPPDATA", home)
        .output()
        .unwrap()
}

#[test]
fn every_shell_completes_commands_and_flags() {
    let home = tempfile::tempdir().unwrap();
    for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
        let output = completions(shell, home.path());
        assert!(output.status.success(), "{shell}");
        assert!(output.stderr.is_empty(), "{shell}");
        let script = String::from_utf8(output.stdout).unwrap();
        assert!(script.contains("review"), "{shell}");
        assert!(script.contains("staged"), "{shell}");
        assert!(script.contains("completions"), "{shell}");
    }
}

#[test]
fn completions_leave_no_update_cache_behind() {
    let home = tempfile::tempdir().unwrap();
    assert!(completions("bash", home.path()).status.success());
    assert_eq!(std::fs::read_dir(home.path()).unwrap().count(), 0);
}

#[test]
fn an_unknown_shell_is_refused() {
    let home = tempfile::tempdir().unwrap();
    assert!(!completions("tcsh", home.path()).status.success());
}
