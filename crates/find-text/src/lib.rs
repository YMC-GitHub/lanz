// #[allow(unused_variables)]

use std::env;
use std::error::Error;
use std::fs;

pub fn read_argv() -> Vec<String> {
    let argv: Vec<String> = env::args().collect();
    return argv;
}

// fn parse_argv(argv: &[String]) -> (&str, &str, &str) {
//     let api = &argv[1];
//     let method = &argv[2];
//     let file_name = &argv[3];

//     // println!("[zero] api {}", api);
//     // println!("[zero] method {}", method);
//     // println!("[zero] file {}", file_name);

//     // rule(str-transform-in-rust): &String -> &str
//     (api, method, file_name)
// }
pub fn parse_argv(argv: &[String]) -> (String, String, String, String) {
    let api = argv[1].clone();
    let method = argv[2].clone();
    let query = argv[3].clone();
    let file_path = argv[4].clone();
    // let feature = argv[5].clone();

    (api, method, query, file_path)
}

// pub mod argv;
// pub mod nanz;

pub struct Flag {
    pub api: String,
    pub method: String,
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

// code(flag-value-type): instead of storing a reference type like &str, it stores a String string.
// code(flag-value-type): flag does not borrow external strings
// code(flag-value-type): with ownership of the internal string.

pub fn parse_flag(argv: &[String]) -> Flag {
    // code(chore): directly replicate target data in its entirety without having to worry about ownership, borrowing, etc.
    // perf(chore): it has its disadvantages, that is, there is a certain performance loss.

    // perf(chore): is it a serious project? Is it not good to use clone directly for toy projects?

    // perf(chore): it depends on whether the code path is a hot path.
    // perf(chore): the hot path is obviously executed more times, and the hot path is worth using the better performance implementation.
    // parse_argv
    let (api, method, query, file_path) = parse_argv(argv);
    // let ignore_case = env::var("IGNORE_CASE").is_ok();
    // let ignore_case_flag = env::var("IGNORE_CASE").ok();
    // let ignore_case = match ignore_case_flag.as_ref().map(String::as_ref) {
    //     None => false,
    //     Some("0") => false,
    //     Some(_) => true,
    // };
    let ignore_case: bool = std::env::var("IGNORE_CASE").map_or_else(
        |_| {
            // if env don't exist, try read ignore case in CLI args
            argv.iter()
                .any(|arg| arg.to_lowercase() == "-i" || arg.to_lowercase() == "--ignore-case")
        },
        |s| s == "0" || s.to_lowercase() == "false", // if env U=IGNORE_CASE exist, check his value
    );
    Flag { api, method, query, file_path, ignore_case }
}

// code(flag-parse): returns a Result: contains a Flag instance on success and an error message on failure.
impl Flag {
    // fn new(argv: &[String]) -> Flag {
    //     if argv.len() < 3 {
    //         // code(chore): anic is less appropriate for outputting user-friendly information, it is better suited to informing developers
    //         panic!("[zero] not enough argv");
    //     }
    //     parse_flag(argv)
    // }

    pub fn parse(argv: &[String]) -> Result<Flag, &'static str> {
        if argv.len() < 3 {
            // code(chore): anic is less appropriate for outputting user-friendly information, it is better suited to informing developers
            // panic!("[zero] not enough argv");
            return Err("[zero] not enough argv");
        }
        Ok(parse_flag(argv))
    }
}

// code(chore): our program doesn't need to return anything, but in order to satisfy Result<T,E>, we use Ok(()) to return a unit type ().
// code(chore): the important of Box<dyn Error>
// code(chore): with Box<dyn Error> , we don't need to specify a specific error type, otherwise you'll also need to look at the error type returned by fs::read_to_string
// code(chore): Otherwise you also need to look at the error type returned by fs::read_to_string and copy it to our run function return
// code(chore): with Box<dyn Error> , we don't need to specify a specific error type, otherwise you'll also need to look at the error type returned by fs::read_to_string
// code(chore): the main thing is that once we do this, it means that we can't uniformly handle errors when we call them at higher levels.
// code(chore): other functions can also return this feature object, and then the caller can use a uniform way to handle the Box returned by different functions.<dyn Error>
pub fn run(flag: Flag) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(flag.file_path)?;

    // println!("[zero] text:\n{contents}");
    // Ok(())
    let text = read_text(flag.file_path)?;

    let stra = if flag.ignore_case { findi(&flag.query, &text) } else { find(&flag.query, &text) };

    for line in stra {
        println!("{line}")
    }
    Ok(())
}

pub fn read_text(loc: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(loc)?;

    // println!("[zero] text:\n{contents}");
    Ok(contents)
}

pub fn find<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut stra = Vec::new();

    // vec![]
    for line in contents.lines() {
        // do sth with line
        if line.contains(query) {
            stra.push(line);
        }
    }
    stra
}

pub fn findi<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut stra = Vec::new();

    // vec![]
    let query = query.to_lowercase();
    for line in contents.lines() {
        // do sth with line
        if line.to_lowercase().contains(&query) {
            stra.push(line);
        }
    }
    stra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_text_in_line() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // exp and act
        assert_eq!(vec!["safe, fast, productive."], find(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], findi(query, contents));
    }
}
