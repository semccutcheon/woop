use std::collections::VecDeque;
use std::io::{BufReader, Read, BufRead, Write};
use std::fs::{File, read};


fn open_input() -> BufReader<File> {
    let f = File::open("test.txt").expect("Unable to open file");
    BufReader::new(f)
}
#[derive(Debug, Clone)]
struct DedupError;


struct Window<T> {
    window: VecDeque<T>,
    window_size: usize,
}

impl<T: PartialEq> Window<T>
{
    fn new(size: usize) -> Self {
        Window {
            window: VecDeque::new(),
            window_size: size,
        }
    }

    fn pop_from_window(&mut self) {
        for _ in 0..self.window_size {
            self.window.pop_back();
        }
    }

    fn contains_duplicates(&mut self) -> bool {
        for i in 0..self.window_size {
            if self.window[i] == self.window[i + self.window_size] {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    fn push(&mut self, s: T) -> Option<T> {
        self.window.push_back(s);
        if self.window.len() == 2 * self.window_size {
            if self.contains_duplicates() {
                self.pop_from_window();
                return None;
            }
            else {
                let emitted_line = self.window.pop_front().expect("window did not contain a value");
                return Some(emitted_line);
            }
        } else {
            return None;
        }

    }

    fn flush(&mut self) -> impl Iterator<Item = T> + '_ {
        return self.window.drain(..);
    }

}

fn dedup() {
    // Should be a circular buffer or something which re-uses an array
    let mut f = File::create("out.txt").expect("Unable to open output file");
    let mut window: Window<String> = Window::new(5);
    for line in open_input().lines().map(|l| l.unwrap()) {
        if let Some(emitted_line) = window.push(line) {
            writeln!(f, "{}", emitted_line).expect("Problem writing out to file");
        }
    }

    // Flush the window out
    for emitted_line in window.flush() {
        writeln!(f, "{}", emitted_line).expect("Problem writing out to file");
    }
}

fn main() {
    dedup();
}
