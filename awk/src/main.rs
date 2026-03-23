use std::process::Command;

fn basic() {
    let csv_file_path = "data/employees.csv";

    println!("---- 行全体を出力 ----", );
    let output = Command::new("awk")
        .arg("{print $0}")
        .arg(csv_file_path)
        .output()
        .expect("awk の実行に失敗");
    
    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn main() {
    basic();
}
