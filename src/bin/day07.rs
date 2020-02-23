
use regex::Regex;


struct Above(char, char);



fn main() {
    let input = include_str!("../../input-day07");

    let re = Regex::new(r"^Step (?P<before>\w) must be finished before step (?P<after>)\w) can begin.$").unwrap();

    re.captures(input).and_then(|cap| {
        let res: u32 = cap.name("before").map(|login| login.as_str());
    });
}
