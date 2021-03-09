use std::env;
use std::fs;
use std::path::Path;
use percentage::Percentage;
use regex::Regex;

fn usage() {
    println!("Usage: rs-tpa-analyze <length of partition> <genome file path>\nNote, that file must not contain header!");
}

fn version(){
    println!("version: 0.1.0");
}

pub fn count_genes(genome : String, gene : char) -> usize {
    return genome.matches(gene).count();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            usage();
            return;
        },

        2 => {
            if  &args[1] == "-h" {
                usage();
                return;
            }
            if &args[1] == "-v"{
                version();
                return;
            }
        },
        3 => {
            let length = &args[1].parse::<i64>().unwrap();
            let filepath = &args[2];
            if !(Path::new(filepath).is_file()){
                panic!("Error: couldn't read file at specified filepath!");
            }
            let contents = fs::read_to_string(filepath)
                .expect("Something went wrong reading the file");
            if !(contents.chars().nth(0).unwrap() == 'A' 
                    || contents.chars().nth(0).unwrap() == 'C'
                    || contents.chars().nth(0).unwrap() == 'G' 
                    || contents.chars().nth(0).unwrap() == 'T'
                    || contents.chars().nth(0).unwrap() == 'N') {
                panic!("Error: file seems to contain a header or not a raw genome!")
            }
            println!("Parsing genome in file: {}, partition length: {}", filepath, length);
            let mut size : usize = contents.len();
            if contents.chars().last().unwrap() == '\n' {
                size = contents.len() - 1;
            }
            println!("Total genome size: {}", size);

            return ;
        },
        
        _ => {
            usage();
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts(){
        assert_eq!(count_genes("NNN".to_string(), 'N'), 3);
    }
    
}

