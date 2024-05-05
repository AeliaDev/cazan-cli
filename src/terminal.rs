use std::io::{stdout, StdoutLock, Write};
use crossterm::{cursor, ExecutableCommand, terminal};

pub struct SubTerminal {
    n_lines: u16,
    current_line: usize,
}

impl SubTerminal {
    pub fn new(n_lines: u16) -> Self {
        for _ in 0..n_lines - 1 {
            println!("\r");
        }
        for _ in 0..n_lines {
            stdout().lock().execute(cursor::MoveToPreviousLine(1)).unwrap();
        }

        Self {
            n_lines,
            current_line: 0,
        }
    }

    pub fn write_to(&mut self, text: &str, line: usize) {
        let mut stdout = stdout().lock();
        self.move_to(&mut stdout, line);
        write!(stdout, "{}", text).unwrap();
        stdout.flush().unwrap();
    }

    fn move_to(&mut self, stdout_lock: &mut StdoutLock, line: usize) {
        let line = line % self.n_lines as usize;
        if line < self.current_line {
            stdout_lock.execute(cursor::MoveUp(self.current_line as u16 - line as u16)).unwrap();
            stdout_lock.execute(cursor::MoveToColumn(0)).unwrap();
        } else {
            stdout_lock.execute(cursor::MoveDown(line as u16 - self.current_line as u16)).unwrap();
            stdout_lock.execute(cursor::MoveToColumn(0)).unwrap();
        }
        self.current_line = line;
    }

    pub fn rewrite_to(&mut self, text: &str, line: usize) {
        let mut stdout = stdout().lock();
        self.move_to(&mut stdout, line);
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        write!(stdout, "{}", text).unwrap();
        stdout.flush().unwrap();
    }

    pub fn move_to_last_line_and_new_line(&mut self) {
        self.move_to(&mut stdout().lock(), self.n_lines as usize - 1);
        println!();
    }
}