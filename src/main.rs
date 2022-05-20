mod game;
mod tetromino;

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{self, Print},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use std::{
    io::{stdout, Stdout, Write},
    time::{Duration, Instant},
};

use tetromino::Tetromino;

use crate::game::Game;

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    enable_raw_mode()?;

    let mut game = Game::new();

    let mut time = Instant::now();

    loop {
        if is_event_available()? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        break;
                    }

                    match event.code {
                        KeyCode::Esc => break,
                        KeyCode::Left => game.move_left(),
                        KeyCode::Right => game.move_right(),
                        KeyCode::Up => game.rotate_piece(),
                        KeyCode::Down => game.move_down(),
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        execute!(stdout, Clear(ClearType::All))?;
        draw_box(&mut stdout, (0, 0), 2 + game.width() as u16 * 2, 2 + game.height() as u16)?;
        draw_game(&mut stdout, &game, (1, 1))?;

        if 800 < time.elapsed().as_millis() {
            game.tick();
            time = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(stdout, cursor::Show, LeaveAlternateScreen)
}

fn is_event_available() -> crossterm::Result<bool> {
    poll(Duration::from_millis(50))
}

fn draw_game(stdout: &mut Stdout, game: &Game, offset: (u16, u16)) -> crossterm::Result<()> {
    queue!(stdout, cursor::MoveTo(offset.0, offset.1))?;
    for row in game.board() {
        for color in row {
            queue!(
                stdout,
                style::SetBackgroundColor(color.ctcolor()),
                Print(" .")
            )?;
        }
        queue!(
            stdout,
            cursor::MoveDown(1),
            cursor::MoveToColumn(1 + offset.0)
        )?;
    }
    let piece_offset = (
        game.offset().0 + (offset.0 as i32),
        game.offset().1 + (offset.1 as i32),
    );
    draw_piece(stdout, game.piece(), piece_offset)
}

fn draw_piece(stdout: &mut Stdout, piece: &Tetromino, offset: (i32, i32)) -> crossterm::Result<()> {
    queue!(stdout, style::SetBackgroundColor(piece.color().ctcolor()))?;
    for (dx, dy) in piece.clone() {
        queue!(
            stdout,
            cursor::MoveTo(((offset.0 + dx) * 2 - 1) as u16, (offset.1 + dy) as u16),
            Print("  "),
        )?;
    }
    queue!(stdout, style::SetBackgroundColor(style::Color::Reset))?;
    stdout.flush()
}

fn draw_box(stdout: &mut Stdout, offset: (u16, u16), w: u16, h: u16) -> crossterm::Result<()> {
    // corners of the box
    queue!(
        stdout,
        style::SetBackgroundColor(style::Color::Reset),
        cursor::MoveTo(offset.0, offset.1),
        Print("┌"),
        cursor::MoveTo(offset.0 + w - 1, offset.1),
        Print("┐"),
        cursor::MoveTo(offset.0, offset.1 + h - 1),
        Print("└"),
        cursor::MoveTo(offset.0 + w - 1, offset.1 + h - 1),
        Print("┘")
    )?;

    // horizontal lines
    for i in 2..w {
        queue!(
            stdout,
            cursor::MoveTo(offset.0 + i - 1, offset.1),
            Print("─"),
            cursor::MoveTo(offset.0 + i - 1, offset.1 + h - 1),
            Print("─")
        )?;
    }

    // vertical lines
    for i in 2..h {
        queue!(
            stdout,
            cursor::MoveTo(offset.0, offset.1 + i - 1),
            Print("│"),
            cursor::MoveTo(offset.0 + w - 1, offset.1 + i - 1),
            Print("│")
        )?;
    }
    stdout.flush()
}
