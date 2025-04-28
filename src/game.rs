use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode},
    queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use serde::Serialize;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[derive(Serialize)]
pub struct SessionStats {
    pub typed: usize,
    pub correct: usize,
    pub accuracy: f64,
    pub wpm: f64,
}

fn word_diff(input: &str, expected: &str) -> String {
    format!("{} → {}", input, expected)
}

pub async fn start_typing_session(words: Vec<String>) -> SessionStats {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    queue!(stdout, Hide).unwrap();
    stdout.flush().unwrap();

    let mut typed = 0;
    let mut correct = 0;
    let mut word_buffer = String::new();

    let target_lines: Vec<Vec<String>> = words
        .chunks(5)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut typed_lines: Vec<Vec<Option<(String, bool, String)>>> = vec![vec![None; 5]];
    let mut current_line = 0;
    let mut current_word_index = 0;

    let mut start_time: Option<Instant> = None;
    let time_limit = Duration::from_secs(30);

    loop {
        let elapsed = start_time.map_or(Duration::ZERO, |s| s.elapsed());
        let time_up = elapsed >= time_limit;

        if time_up && word_buffer.is_empty() {
            break;
        }

        queue!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
        let mut y_cursor = 0;

        for (line_index, target_line) in target_lines.iter().enumerate() {
            if line_index > current_line {
                break;
            }

            queue!(stdout, cursor::MoveTo(0, y_cursor)).unwrap();
            for (word_index, target_word) in target_line.iter().enumerate() {
                let typed_word = typed_lines[line_index][word_index].as_ref();

                match typed_word {
                    Some((typed_str, true, _)) => {
                        queue!(stdout, SetForegroundColor(Color::Green)).unwrap();
                        write!(stdout, "{} ", typed_str).unwrap();
                    }
                    Some((typed_str, false, expected)) => {
                        queue!(stdout, SetForegroundColor(Color::Red)).unwrap();
                        write!(stdout, "{} ", word_diff(typed_str, expected)).unwrap();
                    }
                    None => {
                        if line_index == current_line && word_index == current_word_index {
                            let mut with_cursor = String::new();
                            let expected_chars: Vec<char> = target_word.chars().collect();
                            let buffer_chars: Vec<char> = word_buffer.chars().collect();
                            for i in 0..expected_chars.len() {
                                if i == buffer_chars.len() {
                                    with_cursor.push('|');
                                }
                                if i < buffer_chars.len() {
                                    with_cursor.push(buffer_chars[i]);
                                } else {
                                    with_cursor.push(expected_chars[i]);
                                }
                            }
                            if buffer_chars.len() >= expected_chars.len() {
                                with_cursor.push('|');
                            }
                            queue!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
                            write!(stdout, "{} ", with_cursor).unwrap();
                        } else {
                            queue!(stdout, SetForegroundColor(Color::DarkGrey)).unwrap();
                            write!(stdout, "{} ", target_word).unwrap();
                        }
                    }
                }
            }
            queue!(stdout, ResetColor).unwrap();
            y_cursor += 1;
        }

        queue!(stdout, cursor::MoveTo(0, y_cursor)).unwrap();
        writeln!(stdout, "Time left: {}s", 30 - elapsed.as_secs()).unwrap();
        queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
        stdout.flush().unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if start_time.is_none() {
                    start_time = Some(Instant::now());
                }

                match key.code {
                    KeyCode::Char(' ') => {
                        if current_line < target_lines.len()
                            && current_word_index < target_lines[current_line].len()
                        {
                            let expected = &target_lines[current_line][current_word_index];
                            let is_correct = word_buffer == *expected;
                            typed += 1;
                            if is_correct {
                                correct += 1;
                            }
                            typed_lines[current_line][current_word_index] =
                                Some((word_buffer.clone(), is_correct, expected.clone()));
                            word_buffer.clear();
                            current_word_index += 1;

                            if current_word_index == 5 {
                                current_line += 1;
                                current_word_index = 0;
                                if current_line < target_lines.len() {
                                    typed_lines.push(vec![None; 5]);
                                }
                            }

                            if time_up {
                                break;
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        word_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        word_buffer.pop();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    queue!(stdout, cursor::MoveTo(0, current_line as u16 + 2), Clear(ClearType::FromCursorDown), Show).unwrap();
    stdout.flush().unwrap();

    let total_secs = start_time.map_or(0.0, |s| s.elapsed().as_secs_f64());
    let accuracy = if typed == 0 { 0.0 } else { (correct as f64 / typed as f64) * 100.0 };
    let wpm = (correct as f64 / total_secs) * 60.0;

    SessionStats {
        typed,
        correct,
        accuracy,
        wpm,
    }
}
