use std::io::{self, BufRead};
use std::fs::File;
use rand::Rng;
use std::env;


fn stream_sample(filename : &str, n : usize) -> io::Result<Vec<String>> {

    // 1. initilaize
    let mut candidates : Vec<String> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    let fp = io::BufReader::new(File::open(filename)?);
    let mut fp_lines = fp
        .lines()
        .map(|rl|rl.unwrap())
        .enumerate()
        .map(|(a,b)|{(a+1,b)});

    // 2. prime candidates
    for _ in 0..n {
        let (_, line) = fp_lines.next().expect("no more lines to consume");
        candidates.push(line);
    }


    // 3. skim through file
    for (idx, line) in fp_lines {
        let p_select = n as f32 / idx as f32;
        let sample : f32 = rng.gen();
        if sample <= p_select {
            // selected 
            let replace_idx = rng.gen_range(0..n);
            candidates[replace_idx] = line;
        }
    }

    // 4. return results
    return Ok(candidates)
}


fn main() {

    let args = env::args().collect::<Vec<String>>();

    if let Ok(candidates) = stream_sample(&args[1], 10) {
        for l in candidates {
            println!("{}", l);
        }
    }
}
