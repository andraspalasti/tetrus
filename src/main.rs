mod game;
mod tetromino;

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal,
};
use game::Game;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use tetromino::{Colors, Tetromino};

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    // setup: clear terminal and hide cursor
    execute!(
        stdout,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let game = Game::new(10, 20).unwrap();

    draw_game(&mut stdout, &game)?;
    sleep(Duration::from_millis(5000));
    stdout.flush()?;

    // teardown: reset background color,
    // reset cursor position, show cursor, clear terminal
    execute!(
        stdout,
        SetBackgroundColor(Color::Reset),
        cursor::MoveTo(0, 0),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All)
    )?;
    Ok(())
}

fn draw_game(stdout: &mut std::io::Stdout, game: &Game) -> crossterm::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;
    for row in game.board() {
        for c in row {
            execute!(stdout, SetBackgroundColor(c_to_ctc(c)), Print(" ."))?;
        }
        execute!(stdout, cursor::MoveToNextLine(1))?;
    }

    draw_piece(stdout, game.offset(), game.piece())?;

    Ok(())
}

fn draw_piece(
    stdout: &mut std::io::Stdout,
    offset: &(i32, i32),
    piece: &Tetromino,
) -> crossterm::Result<()> {
    let coords = piece.clone().into_iter();
    for (dx, dy) in coords {
        execute!(
            stdout,
            cursor::MoveTo(2 * ((offset.0 + dx) as u16), (offset.1 + dy) as u16),
            SetBackgroundColor(c_to_ctc(&piece.color())),
            Print("  ")
        )?;
    }
    Ok(())
}

/// Color to crossterm color
fn c_to_ctc(c: &Colors) -> crossterm::style::Color {
    use crossterm::style::Color::*;
    match c {
        Colors::Cyan => Cyan,
        Colors::Blue => Blue,
        Colors::Orange => DarkYellow,
        Colors::Yellow => Yellow,
        Colors::Green => Green,
        Colors::Purple => Magenta,
        Colors::Red => Red,
        _ => Reset,
    }
}
