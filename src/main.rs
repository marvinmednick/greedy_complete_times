//use std::env;
//use std::process; 
//use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::collections::{HashMap,BTreeMap};
//use std::thread;
use regex::Regex;
//use std::fmt;

//extern crate minheap;
//use minheap::MinHeap;
mod cmd_line;
use crate::cmd_line::CommandArgs;
use std::cmp::Ordering;


#[derive(Debug)]
pub struct Job {
    pub length: i64,
    pub weight: i64,
    pub priority: i64,
}

impl Job {

    pub fn new(weight: i64, len: i64, priority: i64) -> Self {
            Job { length: len, weight: weight, priority: priority }
    }

    pub fn set_priority(&mut self, new_pri: i64) {
        self.priority = new_pri;

    }

}
    impl PartialOrd for Job {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.priority == other.priority {
                self.weight.partial_cmp(&other.weight)
            }
            else {
                self.priority.partial_cmp(&other.priority)
            }
        }
    }

    impl Ord for Job {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.priority == other.priority {
                self.weight.cmp(&other.weight)
            }
            else {
                self.priority.cmp(&other.priority)
            }
        }
    }

    impl PartialEq for Job {
        fn eq(&self, other: &Self) -> bool {
            self.priority == other.priority && self.weight == other.weight
        }
    }

    impl Eq for Job {}

pub fn print_list(jobs : &Vec<Job>) {
        let mut count = 0;
        for j in jobs {
            println!("#{}: weight: {} len: {} pri: {}",count,j.weight,j.length,j.priority);
            count += 1;
        }

}



fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);



  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

	let mut jobs = Vec::<Job>::new();

    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();
    println!("first line is {}",line);

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
  //      println!("processing {}",line_data);

        // split the line into the vertex and the list of adjacent vertexes/weight pairs
        let re_job = Regex::new(r"\s*(?P<weight>\d+)\s+(?P<length>\d+.*$)").unwrap();
        // adjacent vertexes are in the format vertex,weight   - and regex below allows for
        // whitespace
        let caps = re_job.captures(&line_data).unwrap();
        let text1 = caps.get(1).map_or("", |m| m.as_str());
        let text2 = caps.get(2).map_or("", |m| m.as_str());
   //     println!("T1 {} T2 {}", text1, text2);

        let weight = text1.parse::<i64>().unwrap();
        let length = text2.parse::<i64>().unwrap();

        let new_job = Job::new(weight,length,weight-length);

 //       println!("Job #{}:  {:?}",_count,new_job);
        jobs.push(new_job);
    }

    //println!("Presort");
    //print_list(&jobs);
    jobs.sort();
    jobs.reverse();
    //println!("Final");
    //print_list(&jobs);


    let mut endtime = 0;
    let mut sum = 0;
    for j in &jobs {
        endtime += j.length;
        let wt_end = endtime * j.weight;
        sum += wt_end;
//        println!("Interim Result:  Endtime is {} Weighted endtime {}  - Sum {} ",endtime, wt_end, sum);
    }
    println!("Final Difference Result:  Endtime is {} Weighted endtimes {}",endtime, sum);

    for i in 0..jobs.len()  {
        let w = jobs[i].weight.clone();
        let l = jobs[i].length.clone();
        jobs[i].set_priority((w*10000)/l);
    }   
    jobs.sort();
    jobs.reverse();
    //print_list(&jobs);

    let mut endtime = 0;
    let mut sum = 0;
    for j in &jobs {
        endtime += j.length;
        let wt_end = endtime * j.weight;
        sum += wt_end;
//        println!("Interim Result:  Endtime is {} Weighted endtime {}  - Sum {} ",endtime, wt_end, sum);
    }
    println!("Final Ratio Result:  Endtime is {} Weighted endtimes {}",endtime, sum);
    

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
/*
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_outgoing(1),&[2]);
		assert_eq!(g.get_incoming(2),&[1]);
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_incoming(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.add_edge(1,2),Some(3));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[3]);
		
	}


 }
 */
