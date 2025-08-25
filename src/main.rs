use crate::game::filler::{Filler, CELLS};
use std::io::{stdin, stdout, Write};
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
    let mut filler = Filler::new();
    let options: String = CELLS.iter().enumerate().map(|(a, x)| format!("{}: {}, ", a + 1, x)).collect();
    loop {
        println!("{filler}\n{}, {}; {}, {}\nPick {options}", filler.p1.0, filler.p1.1, filler.p2.0, filler.p2.1);
        let mut line = String::new();
        if let Err(_) = stdin().read_line(&mut line) { continue }
        let Ok(pick) = line.trim().parse() else { continue };
        if !(1..=CELLS.len()).contains(&pick) { continue }
        let pick = CELLS[pick - 1];
        if pick == filler.p1.0 || pick == filler.p2.0 { continue }
        filler = filler.do_move(pick);
        if filler.is_over() {
            println!("Game over \n{filler}\n{:?}, {:?}", filler.p1, filler.p2);
            break
        }
        // minimax!
        PRINT_DOTS.store(true, Relaxed);
        let mut rated_options: Vec<_> = filler.get_options().into_iter().map(|option| (
            option, minimax::minimax(filler.do_move(option), 19, f32::NEG_INFINITY, f32::INFINITY)
        )).collect();
        PRINT_DOTS.store(false, Relaxed);
        println!();
        rated_options.sort_by(|a, b| f32::total_cmp(&a.1, &b.1));
        if let Some((option, _)) = rated_options.first() {
            filler = filler.do_move(*option);
        }
        if filler.is_over() {
            println!("Game over \n{filler}\n{:?}, {:?}", filler.p1, filler.p2);
            break
        }
    }
}