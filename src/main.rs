use std::env;   
use std::fs;
use std::string;

fn main() {
    let file_path = "poem.txt";
    println!("In file {file_path}");  
    read_a_file(file_path.to_string());


}

fn read_a_file (file: String) {
    let content =  fs::read_to_string(&file).expect("Unable to open file");
    println!("The content of the file {}: \n {}", &file, content)
}