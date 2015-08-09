use std::env;
use std::thread;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::sync::mpsc;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
  let args: Vec<_> = env::args().collect();

  let mut in_opts = OpenOptions::new();
  in_opts.read(true);
  let mut out_opts = OpenOptions::new();
  out_opts.write(true);

  let (tx, rx) = mpsc::channel();

  let r_s = tx.clone();
  let in_args = args.clone();
  let reading = thread::spawn(move || {
    let inP = Path::new(&in_args[1]);
    let mut inF = match in_opts.open(&inP) {
      Err(_) => panic!("Unable to open: {}", inP.display()),
      Ok(file) => file
    };
    let mut reader = BufReader::new(inF);
    loop {
      let mut s = String::new();
      reader.read_line(&mut s);
      r_s.send(s.clone());
    }
  });

  let out_args = args.clone();
  let writing = thread::spawn(move || {
    let outP = Path::new(&out_args[2]);
    let mut outF = match out_opts.open(&outP) {
      Err(_) => panic!("Unable to open: {}", outP.display()),
      Ok(file) => file
    };
    let mut outW = BufWriter::new(&outF);
    outW.write_all(b"greet.sh bump");
  });

  loop {
    let msg = match rx.recv() {
      Err(_) => panic!("Could not read"),
      Ok(m) => m
    };
    println!("got {}", msg);
  }
}

