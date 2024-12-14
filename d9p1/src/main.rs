use std::collections::{ HashMap, HashSet };
use std::fmt::write;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let mut mem = MemoryBlock::from_file(Path::new("input"));
    mem.sort();
    dbg!(mem.checksum());
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
enum Memory {
    Free,
    FileBlock { fid: u128 },
}

struct MemoryBlock {
    memory: Vec<Memory>,
    sorted: bool
}

impl MemoryBlock {
    pub fn from_file(p: &Path) -> Self {
        let mut mem_count: usize = 0;
        let mut mem_id: u128 = 0;
        let mut mem: Vec<Memory> = Vec::new();

        let file: Vec<u128> = read_to_string(p)
            .unwrap()
            .trim()
            .chars()
            .map(|num_str| num_str.to_digit(10).unwrap() as u128)
            .collect();

        for (idx, val) in file.iter().enumerate() {
            match idx % 2 == 0 {
                true => { 
                    for _i in 0..*val {
                        mem.push(Memory::FileBlock { fid: mem_id });
                        mem_count += 1
                    }
                    mem_id += 1;
                },
                false => {
                    for _i in 0..*val {
                        mem.push(Memory::Free);
                        mem_count += 1;
                    }
                }
            };
        };

        MemoryBlock {
            memory: mem,
            sorted: false,
        }
    }

    pub fn sort(&mut self) {
        loop {
            let lf = self.last_file().unwrap();
            let ff = self.first_free().unwrap();

            if lf < ff {
                break;
            }

            let temp = self.memory[lf];
            self.memory[ff] = temp;
            self.memory[lf] = Memory::Free;
        }

        self.sorted = true;
    }

    fn last_file(&self) -> Option<usize> {
        self.memory.iter().rposition(|x| matches!(x, Memory::FileBlock {..}))
    }

    fn first_free(&self) -> Option<usize> {
        self.memory.iter().position(|x| matches!(x, Memory::Free))
    }
    
    pub fn checksum(&self) -> i128 {
        if !self.sorted {
            panic!("MemoryBlock must be sorted before calculating checksum")
        };

        let mut checksum: i128 = 0;

        for (i, f) in self.memory.iter().enumerate() {
            match f {
                Memory::FileBlock { fid } => checksum += i as i128 * *fid as i128,
                Memory::Free => (),
            }
        };

        checksum
    }
}
