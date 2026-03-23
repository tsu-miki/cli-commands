use std::process::Command;

fn run_awk(command: &str, file_path: &str, description: &str) {
    print!("---- {} ----\n", description);

    let output = Command::new("awk")
        .arg(command)
        .arg(file_path)
        .output()
        .expect("awk の実行に失敗しました");

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn run_awk_with_fs(field_separator: &str, command: &str, file_path: &str, description: &str) {
    println!("---- {} ----\n", description);

    let output = Command::new("awk")
        .arg(format!("-F{}", field_separator))
        .arg(command)
        .arg(file_path)
        .output()
        .expect("awk の実行に失敗しました");

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn basic() {
    let csv_file_path = "data/employees.csv";
    let text_file_path = "data/scores.txt";

    run_awk("{print $0}", csv_file_path, "行全体を出力");
    run_awk(
        "{print $1, $3}",
        text_file_path,
        "1列目と3列目だけ出力(テキストファイル)",
    );
    run_awk_with_fs(
        ",",
        "{print $1, $3}",
        csv_file_path,
        "1列目と3列目だけ出力(CSVファイル)",
    );
}

fn main() {
    basic();
}
