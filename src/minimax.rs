use crate::game::filler::{Filler, Turn};

pub fn minimax(filler: Filler, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
    if depth == 0 || filler.is_over() {
        return eval(&filler);
    }

    let maximizing = matches!(filler.turn, Turn::P1);

    let mut options = filler
        .get_options()
        .into_iter()
        .map(|option| filler.do_move(option))
        .collect::<Vec<_>>();
    if maximizing {
        options.sort_unstable_by_key(|f| -eval(f));
    } else {
        options.sort_unstable_by_key(eval);
    }
    let mut acc_eval = if maximizing { i8::MIN } else { i8::MAX };
    for option in options {
        let eval = minimax(option, depth - 1, alpha, beta);
        if maximizing {
            acc_eval = i8::max(acc_eval, eval);
            alpha = i8::max(alpha, eval);
        } else {
            acc_eval = i8::min(acc_eval, eval);
            beta = i8::min(beta, eval);
        }
        if beta <= alpha {
            break;
        }
    }
    acc_eval
}

pub fn eval(filler: &Filler) -> i8 {
    (filler.p1.1 as i8) - (filler.p2.1 as i8)
}
