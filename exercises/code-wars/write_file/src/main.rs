use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};

fn main() {
    let txt_file: Result<File, io::Error> = OpenOptions::new()
        .create(true)
        .append(true)
        .open("to_do.txt");

    match txt_file {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            let mut task: String = String::new();

            println!("Insira sua tarefa ou objetivo: ");
            io::stdin()
                .read_line(&mut task)
                .expect("Failed to read line!");

            let task_formated: String = format!("-[ ] {}\n", task.trim().to_string());

            let _ = writer.write_all(task_formated.as_bytes());
            let _ = writer.flush();

            println!("Write Operation Successful");
        }
        Err(error) => println!("{}", error),
    }
}
