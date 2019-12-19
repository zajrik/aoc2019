use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn part2() {
    let input: Vec<u32> = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    fn process_data(mut data: Vec<u32>, noun: u32, verb: u32) -> u32 {
        data[1] = noun;
        data[2] = verb;

        let mut i: usize = 0;

        loop {
            match data[i] {
                1 => {
                    let val1: u32 = data[data[i + 1] as usize];
                    let val2: u32 = data[data[i + 2] as usize];
                    let idx: usize = data[i + 3] as usize;
                    data[idx] = val1 + val2;
                }
                2 => {
                    let val1: u32 = data[data[i + 1] as usize];
                    let val2: u32 = data[data[i + 2] as usize];
                    let idx: usize = data[i + 3] as usize;
                    data[idx] = val1 * val2;
                }
                99 => break,
                _ => panic!("Invalid opcode"),
            }

            i += 4;
        }

        data[0]
    };

    let (sender, receiver): (Sender<(u32, u32)>, Receiver<(u32, u32)>) = mpsc::channel();

    for i in 0..=99 {
        for j in 0..=99 {
            let data: Vec<u32> = input.clone();
            let s: Sender<(u32, u32)> = sender.clone();

            thread::spawn(move || {
                if process_data(data, i, j) == 19690720 {
                    s.send((i, j)).unwrap();
                }
            });
        }
    }

    let result: (u32, u32) = receiver.recv().unwrap();

    println!("{}", result.0 * 100 + result.1);
}
