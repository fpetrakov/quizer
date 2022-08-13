use clap::Parser;
use colored::Colorize;
use ini::Ini;
use std::io;

// TODO: add color attr to question in .ini
// TODO: update Readme
// TODO: add custom win message
// TODO: add custom lose message
// TODO: finish Readme

const LONG_ABOUT: &str = "
To run quizer create .ini file with questions and answers, for example:

[1]
Question='What is my favorite color?'
Answer='Red'

[2]
Question='What is my name?'
Answer='Fedya'
";

/// Rust app that creates cli quiz for you!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Args {
    /// Path to .ini file with questions and answers
    path: String,
}

fn main() {
    let args = Args::parse();
    let conf = Ini::load_from_file(args.path).unwrap();

    let max_score = conf.sections().count();
    let mut current_score = 0;

    loop {
        for (sec, _) in &conf {
            let section_name = sec.unwrap();
            let section = conf.section(Some(section_name)).unwrap();
            let question = section.get("Question").unwrap();
            let right_answer = section.get("Answer").unwrap().to_lowercase();

            println!("{}", format!("Question {:?}:", section_name).cyan());
            println!("{}", format!("{:?}", question).blue());

            let user_answer = get_user_answer();

            if user_answer == right_answer {
                println!("{}", format!("You are absolutely right!").bright_green());
                current_score += 1;
            } else {
                println!("{}", format!("Unfortunately, you are wrong!").red());
            }
        }

        println!(
            "{}",
            format!(
                "You managed to answer {} out of {} questions",
                current_score, max_score
            )
            .purple()
        );

        println!("{}", format!("Type 'y' to play again").blue());
        let user_answer = get_user_answer();
        if user_answer == "y" {
            current_score = 0;
            continue;
        } else {
            break;
        }
    }
}

fn get_user_answer() -> String {
    let mut user_answer = String::new();
    io::stdin().read_line(&mut user_answer).unwrap();
    user_answer.trim_end().to_lowercase()
}
