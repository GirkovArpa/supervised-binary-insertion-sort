use math::round;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {
  loop {
    let mut movies: Vec<String> = Vec::new();
    if (!std::path::Path::new("./favoriteMovies.txt").exists()) {
      File::create("./favoriteMovies.txt");
    }
    let f = File::open("./favoriteMovies.txt").expect("Unable to open favoriteMovies.txt");
    let f = BufReader::new(f);
    for line in f.lines() {
      let line = line.expect("Unable to read line");
      movies.push(line);
    }
    println!("Enter item to start binary insertion sort in favoriteMovies.txt:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let title: String = input.trim().parse().unwrap();
    movies.push(title.clone());
    sort_(&mut movies, title);
    save_(&mut movies);
  }
}

fn move_(arr: &mut Vec<String>, old_index: usize, new_index: usize) {
  let removed = arr.remove(old_index);
  arr.insert(new_index, removed);
}

fn save_(movies: &mut Vec<String>) {
  fs::write("./favoriteMovies.txt", movies.join("\n"));
  println!("Updated favoriteMovies.txt!");
}

fn sort_(movies: &mut Vec<String>, title: String) {
  let mut bottom = movies.len() as f64;
  let mut top = 0f64;
  let mut middle = (movies.len() as f64) / 2f64;

  let mut comparison = movies[round::floor(middle, 0) as usize].clone();
  let mut comparisons: Vec<String> = Vec::new();
  comparisons.push(comparison.clone());

  loop {
    println!(
      "Do you like {} better than {} [{}] (Y/N)?",
      &title,
      &comparison,
      round::floor(middle, 0) + 1f64
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let response: String = input.trim().parse().unwrap();
    if response == "Y" {
      bottom = round::floor(middle, 0);
      middle = ((bottom - top) / 2f64) + top;
      comparison = (&movies)[round::floor(middle, 0) as usize].clone();
      if comparisons.contains(&comparison) {
        let index = movies.iter().position(|r| r == &title.to_string()).unwrap();
        move_(movies, index, round::ceil(middle, 0) as usize);
        break;
      }
      comparisons.push(comparison.clone());
    } else if response == "N" {
      top = round::floor(middle, 0);
      middle = ((bottom - top) / 2f64) + top;
      comparison = movies[round::floor(middle, 0) as usize].clone();
      if comparisons.contains(&comparison) || comparison == title {
        if comparison != title {
          let index = movies.iter().position(|r| r == &title.to_string()).unwrap();
          move_(movies, index, round::floor(middle, 0) as usize + 1usize);
        }
        break;
      }
      comparisons.push(comparison.clone());
    }
  }
}
