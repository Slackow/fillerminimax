use crate::game::filler::{Filler, Turn};

pub fn minimax(
    filler: Filler,
    depth: u8,
    mut alpha: f32,
    mut beta: f32
) -> f32 {
    if depth == 0 || filler.is_over() {
        return eval(&filler);
    }

    let maximizing = matches!(filler.turn, Turn::P1);

    let mut acc_eval = if maximizing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };
    let func = if maximizing { f32::max } else { f32::min };
    let mut options = filler
        .get_options()
        .into_iter()
        .map(|option| filler.do_move(option))
        .collect::<Vec<_>>();
    options.sort_by(|f1, f2| f32::total_cmp(&eval(f1), &eval(f2)));
    if maximizing {
        options.reverse();
    }
    for option in options {
        let eval = minimax(option, depth - 1, alpha, beta);
        acc_eval = func(acc_eval, eval);
        if maximizing {
            alpha = func(alpha, eval);
        } else {
            beta = func(beta, eval);
        }
        if beta <= alpha {
            break;
        }
    }
    acc_eval
}

fn eval(filler: &Filler) -> f32 {
    (filler.p1.1 as f32) - (filler.p2.1 as f32)
}
