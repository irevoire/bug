#![feature(str_split_once)]
#![feature(let_chains)]

fn main() {
    let mut bug = "lalacoollolo".to_string();
    if !bug.contains("🔢") && (let Some((start, end)) = bug.split_once("1234")) {
        bug = format!("{}🔢{}", start, end);
    }

    println!("{}", bug);
}
