use std::env;

mod _01;
mod _02;
mod _03;
mod _04;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = "./inputs".to_string();
    let base_folder = args.get(1).unwrap_or(&default);

    println!("Base: {:?}", base_folder);

    println!("_01");
    let _01_file = format!("{}/01.txt", base_folder);
    _01::execute(_01_file);

    println!("_02");
    let _02_file = format!("{}/02.txt", base_folder);
    _02::execute(_02_file);

    println!("_03");
    let _03_file = format!("{}/03.txt", base_folder);
    _03::execute(_03_file);

    println!("_04");
    let _04_file = format!("{}/04.txt", base_folder);
    _04::execute(_04_file);
}
