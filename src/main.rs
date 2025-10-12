use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::os::fd::AsRawFd;
use std::process::exit;
use std::time::Duration;
use crossterm::{cursor, event, terminal, ExecutableCommand, QueueableCommand};
use crossterm::event::Event;
use crossy_terminal::map::MapState;

fn main() {
    let mut stdout = stdout();
    let mut map = MapState::new();
    stdout.execute(cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    while map.alive {
        if event::poll(Duration::from_millis(100)).unwrap() {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if key.code == event::KeyCode::Char('q') {
                        break;
                    } else if key.code.is_up() {
                        map.up();
                    } else if key.code.is_right() {
                        map.right();
                    } else if key.code.is_down() {
                        map.down();
                    } else if key.code.is_left() {
                        map.left();
                    }
                }
                _ => {}
            }
        }

        stdout.queue(terminal::BeginSynchronizedUpdate).unwrap();
        stdout.queue(cursor::MoveTo(0,0)).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        stdout.write_all(format!("Use q to quit\n\r{}", map.render()).as_bytes()).unwrap();
        stdout.queue(terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }

    terminal::disable_raw_mode().unwrap();
}
