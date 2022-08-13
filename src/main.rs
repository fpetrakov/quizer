use clap::Parser;
use ini::Ini;
use std::io;

// TODO: add default colors to output
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

            println!("Question {:?}:", section_name);
            println!("{:?}", question);

            let user_answer = get_user_answer();

            if user_answer == right_answer {
                println!("You are absolutely right!");
                current_score += 1;
            } else {
                println!("Unfortunately, you are wrong!");
            }
        }

        println!(
            "You managed to answer {} out of {} questions",
            current_score, max_score
        );

        println!("Would you like to play again? If so type 'y' ");
        let user_answer = get_user_answer();
        if user_answer == "y" {
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
