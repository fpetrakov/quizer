use clap::Parser;
use ini::Ini;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    /// Path to file with questions and answers
    path: String,
}

fn main() {
    let args = Args::parse();

    let conf = Ini::load_from_file(args.path).unwrap();

    for (sec, _) in &conf {
        let section_name = sec.unwrap();
        let section = conf.section(Some(section_name)).unwrap();
        let question = section.get("Question").unwrap();
        let right_answer = section.get("Answer").unwrap().to_lowercase();

        println!("Question {:?}:", section_name);
        println!("{:?}", question);

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).unwrap();
        let user_answer = user_answer.trim_end().to_lowercase();

        if user_answer == right_answer {
            println!("You are absolutely right!")
        } else {
            println!("Unfortunately, you are wrong!");
        }
    }
}
