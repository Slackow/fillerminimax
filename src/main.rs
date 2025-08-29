use crate::game::filler;
use crate::game::filler::{Cell, Filler};
use crate::minimax::eval;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, ErrorKind, Write};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::thread::spawn;
use std::time::Duration;

mod game;
mod minimax;
static PRINT_DOTS: AtomicBool = AtomicBool::new(false);
fn main() {
    spawn(|| {
        loop {
            if PRINT_DOTS.load(Relaxed) {
                print!(".");
                stdout().lock().flush().unwrap_or(());
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    let mut filler = read_board()
        .expect("Failure to read stdin")
        .unwrap_or_else(Filler::new);
    let options: String = Cell::CELLS
        .iter()
        .enumerate()
        .map(|(a, x)| format!("{}: {}, ", a + 1, x))
        .collect();
    loop {
        println!(
            "{filler}\n{}, {}; {}, {}\nPick {options}",
            filler.p1.0, filler.p1.1, filler.p2.0, filler.p2.1
        );
        let mut line = String::new();
        if stdin().read_line(&mut line).is_err() {
            continue;
        }
        let Some(&pick) = line
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|i| Cell::CELLS.get(i.checked_sub(1)?))
        else {
            continue;
        };
        if pick == filler.p1.0 || pick == filler.p2.0 {
            continue;
        }
        filler = filler.do_move(pick);
        if filler.is_over() {
            println!("Game over \n{filler}\n{:?}, {:?}", filler.p1, filler.p2);
            break;
        }
        // minimax!
        PRINT_DOTS.store(true, Relaxed);
        let mut rated_options: Vec<_> = filler
            .get_options()
            .into_iter()
            .map(|option| (
                option,
                minimax::minimax(filler.do_move(option), 19, i8::MIN, i8::MAX),
            ))
            .collect();
        PRINT_DOTS.store(false, Relaxed);
        println!();
        let lowest_option = rated_options.iter().min_by_key(|(_, rating)| rating);
        if let Some(&(_, lowest_rating)) = lowest_option {
            rated_options.retain(|&(_, rating)| rating <= lowest_rating);
        }
        if let Some(next_filler) = rated_options
            .into_iter()
            .map(|(option, _)| filler.do_move(option))
            .min_by_key(eval)
        {
            filler = next_filler;
        }
        if filler.is_over() {
            println!("Game over \n{filler}\n{:?}, {:?}", filler.p1, filler.p2);
            break;
        }
    }
}

fn read_board() -> Result<Option<Filler>, Box<dyn Error>> {
    let mut vec = Vec::<[Cell; filler::COLS]>::with_capacity(filler::ROWS);
    for row in stdin().lock().lines().take(filler::ROWS) {
        let row = row?;
        if row.is_empty() {
            return Ok(None);
        }
        let row: Vec<Cell> = row
            .chars()
            .into_iter()
            .filter_map(Cell::from_input)
            .collect();
        vec.push(row.try_into().map_err(|_| {
            std::io::Error::new(ErrorKind::InvalidData, "Incorrect number of columns")
        })?);
    }
    let board: [[Cell; filler::COLS]; filler::ROWS] = vec
        .try_into()
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidData, "Incorrect number of rows"))?;
    Ok(Some(Filler::from(board)))
}
