use reqwest::{self, header::COOKIE};
use core::fmt;
use std::error::Error;
use std::env;
use std::fs;
use std::path::PathBuf;


fn get_cookie(exe_path: &PathBuf) -> Result<String, AocError>
{
    let env_cookie = env::var("AOC_TOKEN");
    if env_cookie.is_err() 
    {
        let mut token_path = exe_path.clone();
        token_path.push("token.txt");
        return match fs::read_to_string(token_path)
        {
            Ok(c) => Ok(c),
            Err(_) => Err(AocError::new("Unable to obtain advent of code cookie for authentication from the file token.txt or the environment variable AOC_TOKEN in the root of your project!"))
        };
    }
    
    return Ok(env_cookie.unwrap());

}

pub fn read_and_parse_input_data(year: i32, day: i32, split_over: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let exe_path = env::current_dir()?;
    let mut data_path = exe_path.clone();
    data_path.push(format!("src/day_{}/data", day));
    let mut file_path = data_path.clone();
    file_path.push(format!("y{}d{}.txt", year, day));
    // check for env var, if it doesn't exist, then read file.
    let cookie = match env::var("AOC_TOKEN") 
    {
        Ok(c) => c,
        Err(_) => {
            println!("Environment variable $AOC_TOKEN doesn't exist, trying from file token.txt in root");
            let mut token_path = exe_path;
            token_path.push("token.txt");
            fs::read_to_string(token_path)?
        }
    };

    let mut raw_puzzle_data: String;
    if !data_path.exists()
    {
        fs::create_dir(&data_path)?;
        fs::write(&file_path, "")?; // create file to store puzzle input in
    }

    if !fs::read_to_string(&file_path)?.is_empty() || cookie == "err" || fs::metadata(&file_path)?.len() != 0
    {
        println!("Reading from file.");
        raw_puzzle_data = fs::read_to_string(&file_path)?;
    }
    else {
        println!("Reading from site.");
        let client = reqwest::blocking::Client::new();
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let cookie_str = format!("session={}", cookie);
        let resp = client.get(url)
        .header(COOKIE, cookie_str)
        .send()?;
        raw_puzzle_data = resp.text()?;
        if raw_puzzle_data == "Puzzle inputs differ by user.  Please log in to get your puzzle input."
        {
            let e = Box::new(AocError::new("Help"));
            return Err(e);
        }
        fs::write(&file_path, &raw_puzzle_data)?;
    }
    let mut puzzle_data: Vec<String> = raw_puzzle_data.split(split_over).map(|s| s.to_owned()).collect();
    puzzle_data.remove(puzzle_data.len() - 1);
    Ok(puzzle_data)

}

#[derive(Debug)]
struct AocError {
    err_string: String
}

impl AocError {
    pub fn new(err_str: &str) -> Self
    {
        AocError { err_string: err_str.to_owned() }
    }
}

impl std::error::Error for AocError {}

impl fmt::Display for AocError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "An AOC error has occurred.")
    }
}