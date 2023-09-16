use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    let roasts = vec![
        "You're not stupid; you just have bad luck thinking.",
        "Remember when I asked for your opinion? Me neither.",
        "Even a kid could write better code.",
        "Really, that's the shit you wrote? Lol",
        "I am not saying I am not disappointed.",
    ];

    let fortunes = vec![
        "You will find great success in your future endeavors.",
        "A smile is your passport into the hearts of others.",
        "The best way to predict the future is to create it.",
        "You will soon embark on a great adventure.",
        "Good things come to those who wait.",
    ];

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                }
                "exit" => return,
                "roast_me" => {
                    use rand::seq::SliceRandom;
                    let random_roast = roasts.choose(&mut rand::thread_rng());
                    if let Some(roast) = random_roast {
                        println!("{}", roast);
                    }
                    previous_command = None;
                }
                "fortune" => {
                    use rand::seq::SliceRandom;
                    let random_fortune = fortunes.choose(&mut rand::thread_rng());
                    if let Some(fortune) = random_fortune {
                        println!("{}", fortune);
                    }
                    previous_command = None;
                }
                "ascii_art" => {
                    let arts = vec![
                        art::art("cow"),
                        art::art("dragon"),
                        art::art("ghost"),
                        art::art("kitty"),
                        art::art("tux"),
                    ];

                    use rand::seq::SliceRandom;
                    let random_art = arts.choose(&mut rand::thread_rng());
                    if let Some(art) = random_art {
                        println!("{}", art);
                    }
                    previous_command = None;
                }
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
