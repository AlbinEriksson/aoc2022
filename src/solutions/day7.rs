use std::{slice, fs::File};

use super::Solver;

enum Entry<'a> {
    File(u32, &'a str),
    Dir(FileTree<'a>)
}

struct FileTree<'a> {
    name: &'a str,
    entries: Vec<Entry<'a>>,
    total_size: u32
}

impl<'a> FileTree<'a> {
    fn new(name: &str) -> FileTree {
        FileTree { name, entries: vec!(), total_size: 0 }
    }

    fn populate(&mut self, io: &mut slice::Iter<TerminalIO<'a>>) {
        loop {
            match io.next() {
                Some(TerminalIO::ChangeDir("/")) => panic!("Unexpected 'cd /' in file tree"),
                Some(TerminalIO::ChangeDir("..")) => break,
                Some(TerminalIO::ChangeDir(name)) => {
                    let tree = self.entries
                        .iter_mut()
                        .find_map(|entry| match entry {
                            Entry::Dir(tree) if tree.name == *name => Some(tree),
                            _ => None
                        })
                        .unwrap();
                    tree.populate(io);
                    self.total_size += tree.total_size;
                }
                Some(TerminalIO::List) => (),
                Some(TerminalIO::Dir(name)) => {
                    self.entries.push(Entry::Dir(FileTree::new(name)))
                },
                Some(TerminalIO::File(size, name)) => {
                    self.entries.push(Entry::File(*size, name));
                    self.total_size += *size;
                },
                None => break
            }
        }
    }

    fn iter_subtrees(&self) -> FileTreeIter {
        FileTreeIter { iter_stack: vec!(self.entries.iter()) }
    }
}

struct FileTreeIter<'a> {
    iter_stack: Vec<slice::Iter<'a, Entry<'a>>>
}

impl<'a> Iterator for FileTreeIter<'a> {
    type Item = &'a FileTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_stack.is_empty() {
                return None;
            }
            match self.iter_stack.last_mut().unwrap().next() {
                Some(Entry::Dir(tree)) => {
                    self.iter_stack.push(tree.entries.iter());
                    return Some(tree);
                },
                Some(Entry::File(_, _)) => continue,
                None => {
                    self.iter_stack.pop();
                }
            }
        }
    }
}

enum TerminalIO<'a> {
    ChangeDir(&'a str),
    List,
    Dir(&'a str),
    File(u32, &'a str)
}

pub struct Day7<'a> {
    terminal: Vec<TerminalIO<'a>>
}

impl<'a> Day7<'a> {
    fn create_file_system(&self) -> FileTree {
        let mut io = self.terminal.iter();
        match io.next().unwrap() {
            TerminalIO::ChangeDir("/") => (),
            _ => panic!("Expected first command to be 'cd /'")
        }
        let mut tree = FileTree::new("/");
        tree.populate(&mut io);
        tree
    }
}

const INPUT: &str = include_str!("../../input/day7");

impl<'a> Solver for Day7<'a> {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day7 { terminal: vec!() }
    }

    fn reset(&mut self) {
        self.terminal.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            if line.starts_with('$') {
                if &line[2..4] == "cd" {
                    self.terminal.push(TerminalIO::ChangeDir(&line[5..]));
                } else if &line[2..4] == "ls" {
                    self.terminal.push(TerminalIO::List);
                }
            } else if line.starts_with("dir") {
                self.terminal.push(TerminalIO::Dir(&line[4..]));
            } else {
                let mut words = line.split(' ');
                let file_size: u32 = words.next().unwrap().parse().unwrap();
                let file_name = words.next().unwrap();
                self.terminal.push(TerminalIO::File(file_size, file_name));
            }
        }
    }

    fn solve_part1(&self) -> u32 {
        self.create_file_system()
            .iter_subtrees()
            .map(|tree| tree.total_size)
            .filter(|size| *size <= 100000)
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        let fs = self.create_file_system();
        let disk_capacity: u32 = 70_000_000;
        let update_size: u32 = 30_000_000;
        let to_delete = fs.total_size - (disk_capacity - update_size);
        fs
            .iter_subtrees()
            .map(|tree| tree.total_size)
            .filter(|size| *size >= to_delete)
            .min()
            .unwrap()
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Sum of directories of under total size 100000: {part1}");
        println!("Size of smallest directory to free space: {part2}");
    }
}
