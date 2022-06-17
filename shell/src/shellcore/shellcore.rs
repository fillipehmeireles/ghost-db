use crate::shellcore::Commands;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub async fn run() {
    let commands: Commands = Commands::new().await;
    loop {
        let mut buffer_cmd = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut buffer_cmd)
            .expect("Error on reading a command");
        let buffer = buffer_cmd.trim();
        let mut buffer_args = buffer_cmd.trim().split_whitespace();
        let buffer_main_arg: &str = buffer_args.next().unwrap_or_default();
        match buffer_main_arg {
            "insert" => {
                let key = buffer_args.next().unwrap_or_default();
                let value = buffer_args.next().unwrap_or_default();

                match commands.command_insert(key, value).await {
                    Ok(msg) => println!("{:?}", msg),
                    Err(msg_err) => println!("{:?}", msg_err),
                }
            }
            "read-all" => {
                let all_data = commands.command_read();
                for data in all_data {
                    println!("{} : {:?}", data.key(), data.value());
                }
            }
            "read-one" => {
                let id = buffer_args.next().unwrap_or_default();
                let data_storage = commands.command_read_one(id);
                match data_storage {
                    Some(data) => println!("{:?}:{:?} ", id, data),
                    None => println!("Data with ID: {} not found", id),
                }
            }
            "update" => {
                let key = buffer_args.next().unwrap_or_default();
                let value = buffer_args.next().unwrap_or_default();
                match commands.command_update(key, value).await {
                    Ok(msg) => println!("{:?}", msg),
                    Err(msg_err) => println!("{:?}", msg_err),
                }
            }
            "remove" => {
                let id = buffer_args.next().unwrap_or_default();
                match commands.command_remove(id).await {
                    Ok(msg) => println!("{:?}", msg),
                    Err(msg_err) => println!("{:?}", msg_err),
                }
            }
            "quit" => {
                println!("Thank's for using. More infos on: https://fillipemeireles.dev/GhostDB");
                break;
            }
            "help" => {
                println!(
                    "{0}\n{1}\n{2}\n{3}\n{4}",
                    "insert {KEY} {VALUE}",
                    "read-all",
                    "read-one {KEY}",
                    "update {KEY} {NEW VALUE}",
                    "remove {KEY}"
                )
            }
            &_ => {
                if buffer.len() != 0 {
                    println!("Command not found. Type Help to see options.")
                }
            }
        }
    }
}
