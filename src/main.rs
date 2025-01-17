use std::process::Command;
use std::process::ExitCode;
#[derive(Debug)]
struct Arguments {
    message: String,
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(o) => o,
        Err(e) => return e,
    };

    let message = args.message;

    let output = Command::new("ojosama")
        .arg("-t")
        .arg(message)
        .output()
        .expect("ojosama が呼び出せませんでしたわ");

    let commit_message = String::from_utf8_lossy(&output.stdout).to_string();

    let is_success = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("`git commit`が実行できませんでしたわ。")
        .wait()
        .expect("`git commit`が実行されませんでしたわ。")
        .success();

    if !is_success {
        eprintln!("`git commit` に失敗してしまいましたわ。");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn parse_args() -> Result<Arguments, ExitCode> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        eprintln!(
            "引数の数が間違っておりますわ。 引数の数は1個であってほしいですわ。 実際の引数の数は {} 個でしたわ。",
            args.len()
        );
        eprintln!(r#"ダブルクオーテーション "" で メッセージを囲ってみてくださいまし。"#);
        return Err(ExitCode::from(1));
    };

    Ok(Arguments {
        message: args.remove(0),
    })
}
