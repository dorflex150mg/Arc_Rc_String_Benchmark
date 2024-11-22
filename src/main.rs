use std::rc::Rc;
use std::sync::Arc;
use std::fs::File;
use std::io::Write;

use rand::{
    distributions::Alphanumeric,
    thread_rng, 
    Rng,
};
use std::time::{self, Duration};
use std::fmt::{self, Display};


mod plotter;

const BENCHMARK_LEN: usize = 64;
const ESSAYS: usize = 2048;

const TINY: usize = 1;
const SMALL: usize = 32;
const SMALL_MEDIUM: usize = 256;
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

fn clone_str(str_type: StrType, essays: usize, size: usize) -> (Stats, Vec<u128>) {
    let mut values = vec![];
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
        values.push(difference);
        sum += difference;
    }
    let n_essays: u128 = essays.try_into().unwrap();
    (Stats { 
        mean: sum/n_essays, 
        max, 
        min,
    },
    values)
}


fn write_to_file(mut file: File, string: Vec<u128>, rc: Vec<u128>, arc: Vec<u128>) -> std::io::Result<()>{
    for i in 0..string.len() {
        let line = format!("{}{}{}{}{}{}",
            string[i].to_string(),
            ",",
            rc[i].to_string(),
            ",",
            arc[i].to_string(),
            "\n"
        );
        file.write_all(line.as_bytes())?;
        
    }
    Ok(())
}



fn main() -> std::io::Result<()> {

    let (s_mean_string, s_values_string) = clone_str(StrType::StringType, ESSAYS, SMALL);
    let (s_mean_rc, s_values_rc) = clone_str(StrType::RcType, ESSAYS, SMALL);
    let (s_mean_arc, s_values_arc) = clone_str(StrType::ArcType, ESSAYS, SMALL);

    let (sm_mean_string, sm_values_string) = clone_str(StrType::StringType, ESSAYS, SMALL_MEDIUM);
    let (sm_mean_rc, sm_values_rc) = clone_str(StrType::RcType, ESSAYS, SMALL_MEDIUM);
    let (sm_mean_arc, sm_values_arc) = clone_str(StrType::ArcType, ESSAYS, SMALL_MEDIUM);

    let (m_mean_string, m_values_string) = clone_str(StrType::StringType, ESSAYS, MEDIUM);
    let (m_mean_rc, m_values_rc) = clone_str(StrType::RcType, ESSAYS, MEDIUM);
    let (m_mean_arc, m_values_arc) = clone_str(StrType::ArcType, ESSAYS, MEDIUM);

//    let (l_mean_string, l_values_string) = clone_str(StrType::StringType, ESSAYS, LARGE);
//    let (l_mean_rc, l_values_rc) = clone_str(StrType::RcType, ESSAYS, LARGE);
//    let (l_mean_arc, l_values_arc) = clone_str(StrType::ArcType, ESSAYS, LARGE);
//
//    let (x_mean_string, x_values_string) = clone_str(StrType::StringType, ESSAYS, XL);
//    let (x_mean_rc, x_values_rc) = clone_str(StrType::RcType, ESSAYS, XL);
//    let (x_mean_arc, x_values_arc) = clone_str(StrType::ArcType, ESSAYS, XL);

    let small = File::create("small.csv")?;
    let small_medium = File::create("small_medium.csv")?;
    let medium = File::create("medium.csv")?;
    let large = File::create("large.csv")?;
    let xl = File::create("xl.csv")?;

    let _ = write_to_file(small, s_values_string.clone(), s_values_rc.clone(), s_values_arc.clone());
    let _ = write_to_file(small_medium, sm_values_string, sm_values_rc, sm_values_arc);
    let _ = write_to_file(medium, m_values_string, m_values_rc, m_values_arc);
//    let _ = write_to_file(large, l_values_string, l_values_rc, l_values_arc);
//    let _ = write_to_file(xl, x_values_string, x_values_rc, x_values_arc);
//
    
    let _ = plotter::plotter::plot_hist(
        s_values_string.iter().map(|v| { u32::try_from(*v).unwrap() }).collect(),
        s_values_rc.iter().map(|v| { u32::try_from(*v).unwrap() }).collect(),
        s_values_arc.iter().map(|v| { u32::try_from(*v).unwrap() }).collect(),
    );


    println!("\nsmall string:\t {}", s_mean_string);
    println!("small rc:\t {}", s_mean_rc);
    println!("small arc:\t {}", s_mean_arc);

    println!("\nsmall_medium string:\t {}", sm_mean_string);
    println!("small_medium rc:\t {}", sm_mean_rc);
    println!("small_medium arc:\t {}", sm_mean_arc);

    println!("medium string:\t {}", m_mean_string);
    println!("medium rc:\t {}", m_mean_rc);
    println!("medium arc:\t {}", m_mean_arc);

//    println!("large string:\t {}", l_mean_string);
//    println!("large rc:\t {}", l_mean_rc);
//    println!("large arc:\t {}", l_mean_arc);
//
//    println!("xl string:\t {}", x_mean_string);
//    println!("xl rc:\t\t {}", x_mean_rc);
//    println!("xl arc:\t\t {}", x_mean_arc);
    

    Ok(())
}
