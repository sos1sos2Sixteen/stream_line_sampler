use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};

fn stream_sample(filename: &str, n: usize) -> io::Result<Vec<String>> {
    // 1. initilaize
    let mut candidates: Vec<String> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    let fp = io::BufReader::new(File::open(filename)?);
    let mut fp_lines = fp
        .lines()
        .map(|rl| rl.unwrap())
        .enumerate()
        .map(|(idx, line)| (idx + 1, line));

    // 2. prime candidates
    for idx in 1..=n {
        // TODO: to panic not
        if let Some((_, line)) = fp_lines.next(){
            candidates.push(line);
        }else{
            eprintln!("input file contains only {} lines, outputing all!", idx);
            break;
        }
    }

    // 3. skim through file
    for (idx, line) in fp_lines {
        let p_select = n as f32 / idx as f32;
        let sample: f32 = rng.gen();
        if sample <= p_select {
            // selected
            let replace_idx = rng.gen_range(0..n);
            candidates[replace_idx] = line;
        }
    }

    // 4. return results
    return Ok(candidates);
}

fn main() {
    let matches = App::new("line sampler")
        .version("0.1")
        .author("sixteen")
        .about("uniformly samples N lines with one pass")
        .arg(
            Arg::with_name("input")
                .required(true)
                .help("text input file"),
        )
        .arg(
            Arg::with_name("n")
                .required(true)
                .help("target n lines")
                .validator(|s| {    // ensures N is a number
                    usize::from_str_radix(&s, 10)
                        .map_err(|_| "n should be a positive integer".to_string())
                        .map(|_| ())
                }),
        )
        .get_matches();

    let input_file = matches.value_of("input").expect("no input found");
    let n_samples = usize::from_str_radix(matches.value_of("n")
        .expect("no N found"), 10)
        .expect("N must be a positive integer");

    if let Ok(candidates) = stream_sample(input_file, n_samples) {
        for l in candidates {
            println!("{}", l);
        }
    }
}

/// this code does absolutely nothing (except wasting time)
/// but it is written in github's web vscode, so its cool (maybe)
fn github_is_awsome(n: usize) -> () {
    if n > 0 {
        github_is_awsome(n - 1)
    }else{
        ()
    }
}
