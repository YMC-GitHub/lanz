// use std::env;
use std::process;

// use find_text;
// mod argv;

fn main() {
    // println!("[zero] hi, zero!");
    // code(read-argv): read argv from cli
    let argv: Vec<String> = find_text::read_argv();

    // - code(read-argv): use bng! macro to output the contents of the array read
    // dbg!(argv);

    // let (_api, _method, file_name) = parse_argv(&argv);
    // // rule(rust): &String -> String
    // // code(read-text): read text from file location
    // read_text(file_name.to_string());

    // let flag = parse_flag(&argv);
    // read_text(flag.file_path.to_string());

    // let flag = Flag::new(&argv);
    // println!("[{}] {}", flag.api, flag.method);
    // read_text(flag.file_path.to_string());

    // $env:RUST_BACKTRACE=1
    let flag = find_text::Flag::parse(&argv).unwrap_or_else(|err| {
        println!("[zero] problem parsing flag: {err}");
        process::exit(1);
    });
    println!("[{}] {} {} in {}", flag.api, flag.method, flag.query, flag.file_path);
    // let _ = call_flag(flag);

    // code(chore): the use of if-let makes the code simpler and more readable because we don't care about the OK value returned by run.
    if let Err(e) = find_text::run(flag) {
        println!("[zero] call flag error: {e}");
        process::exit(1);
    }
}
