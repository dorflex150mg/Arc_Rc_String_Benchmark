use std::rc::Rc;
use std::sync::Arc;
use rand::{
    distributions::Alphanumeric,
    thread_rng, 
    Rng,
};
use std::time::{self, Duration};
use std::fmt::{self, Display};

const BENCHMARK_LEN: usize = 64;
const ESSAYS: usize = 2048;

const TINY: usize = 1;
const SMALL: usize = 32;
const MEDIUM: usize = 1024;
const LARGE: usize = 8192;
const XL: usize = 32_768;

enum StrType {
    StringType,
    RcType,
    ArcType,
}

struct Stats {
    mean: u128,
    max: u128,
    min: u128,
}

impl Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mean: \t{}, \tmin: \t{}, \tmax: \t{}\t", self.mean, self.min, self.max)
    }
}

fn generate_random_string(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}


fn read_benchmark_str(str_type: StrType, essays: usize, size: usize) -> Stats {
    let mut sum = 0;
    let mut max = 0;
    let mut min = std::u128::MAX;
    for _ in 0..essays {
        let rand_string: String = generate_random_string(size);
        let difference = match str_type {
            StrType::StringType => {
                let start = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                let b = rand_string.clone();
                let end = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                print!("b: {} ", b);
                end - start
            },
            StrType::RcType => {
                let rc: Rc<str> = rand_string.into(); 
                let start = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                let b = rc.clone();
                let end = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                print!("b: {} ", b);
                end - start
            },
            StrType::ArcType => {
                let arc: Arc<str> = rand_string.into(); 
                let start = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                let b = arc.clone();
                let end = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
                print!("b: {} ", b);
                end - start
            },
        };

        if difference > max {
            max = difference
        }
        if difference < min {
            min = difference
        }
        sum += difference;
    }
    let n_essays: u128 = essays.try_into().unwrap();
    Stats { 
        mean: sum/n_essays, 
        max, 
        min,
    }
}
        

fn main() {

    let s_mean_string = read_benchmark_str(StrType::StringType, ESSAYS, SMALL);
    let s_mean_rc = read_benchmark_str(StrType::RcType, ESSAYS, SMALL);
    let s_mean_arc = read_benchmark_str(StrType::ArcType, ESSAYS, SMALL);
    let m_mean_string = read_benchmark_str(StrType::StringType, ESSAYS, MEDIUM);
    let m_mean_rc = read_benchmark_str(StrType::RcType, ESSAYS, MEDIUM);
    let m_mean_arc = read_benchmark_str(StrType::ArcType, ESSAYS, MEDIUM);
    let l_mean_string = read_benchmark_str(StrType::StringType, ESSAYS, LARGE);
    let l_mean_rc = read_benchmark_str(StrType::RcType, ESSAYS, LARGE);
    let l_mean_arc = read_benchmark_str(StrType::ArcType, ESSAYS, LARGE);
    let x_mean_string = read_benchmark_str(StrType::StringType, ESSAYS, XL);
    let x_mean_rc = read_benchmark_str(StrType::RcType, ESSAYS, XL);
    let x_mean_arc = read_benchmark_str(StrType::ArcType, ESSAYS, XL);
    println!("\nsmall string:\t {}", s_mean_string);
    println!("small rc:\t {}", s_mean_rc);
    println!("small arc:\t {}", s_mean_arc);
    println!("medium string:\t {}", m_mean_string);
    println!("medium rc:\t {}", m_mean_rc);
    println!("medium arc:\t {}", m_mean_arc);
    println!("large string:\t {}", l_mean_string);
    println!("large rc:\t {}", l_mean_rc);
    println!("large arc:\t {}", l_mean_arc);
    println!("xl string:\t {}", x_mean_string);
    println!("xl rc:\t\t {}", x_mean_rc);
    println!("xl arc:\t\t {}", x_mean_arc);
}
