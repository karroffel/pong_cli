extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;

struct Pong {

    running: bool,

    player1_score: i32,
    player2_score: i32,

    player1_y: i32,
    player2_y: i32,

    ball_x: f64,
    ball_y: f64,

    ball_velo_x: f64,
    ball_velo_y: f64

}

const WIN_WIDTH: i32 = 80;
const WIN_HEIGHT: i32 = 24;

const PLAYER1_X: i32 = 5;
const PLAYER2_X: i32 = WIN_WIDTH - 5;

const PAD_LEN: i32 = 5;

fn main() {
    let mut tmp_player1_score = 0;
    let mut tmp_player2_score = 0;

    let mut player_scored;
    let enter_message = "Press enter to continue".to_string();

    loop {

        let mut state = Pong {

            running: true,

            player1_score: tmp_player1_score,
            player2_score: tmp_player2_score,

            player1_y: (WIN_HEIGHT - PAD_LEN) / 2 ,
            player2_y: (WIN_HEIGHT - PAD_LEN) / 2 ,

            ball_y: 5.0,
            ball_x: 20.0,

            ball_velo_x: 0.3,
            ball_velo_y: 0.05
        };

        initscr();
        noecho();
        cbreak();

        while state.running {

            render(&state);

            std::thread::sleep_ms(20);

            refresh();
            clrtoeol();

            update(&mut state);
        }

        clear();

        if tmp_player1_score < state.player1_score {
            player_scored = 1;
        } else {
            player_scored = 2;
        }

        tmp_player1_score = state.player1_score;
        tmp_player2_score = state.player2_score;

        // display points
        let s = "Player ".to_string() + &player_scored.to_string() + &" scored";
        mvprintw(WIN_HEIGHT / 2, (WIN_WIDTH - s.len() as i32) / 2, &s);
        mvprintw(WIN_HEIGHT / 2 + 1, (WIN_WIDTH - enter_message.len() as i32) / 2, &enter_message);

        while getch() != 10 {

        }

    }

}

fn ball_hit(ball_x: f64, ball_y: f64, pad_x: i32, pad_y: i32) -> bool {
    if ball_x as i32 != pad_x {
        return false;
    }

    if (ball_y as i32) >= pad_y && (ball_y as i32) < pad_y + PAD_LEN {
        return true;
    }

    false
}

fn update(state: &mut Pong) {
    timeout(1);
    let c = getch();

    if c == 'i' as i32 {
        state.player2_y -= 1;
    } else if c == 'k' as i32 {
        state.player2_y += 1;
    } else if c == 'w' as i32 {
        state.player1_y -= 1;
    } else if c == 's' as i32 {
        state.player1_y += 1;
    }


    if ball_hit(state.ball_x, state.ball_y, PLAYER1_X, state.player1_y) {
        state.ball_velo_x *= -1.0;
        state.ball_velo_y = rand::thread_rng().gen_range(-0.25, 0.25);
    }

    if ball_hit(state.ball_x, state.ball_y, PLAYER2_X, state.player2_y) {
        state.ball_velo_x *= -1.0;
        state.ball_velo_y = rand::thread_rng().gen_range(-0.25, 0.25);
    }


    if state.player1_y < 0 {
        state.player1_y = 0;
    }
    if state.player1_y > WIN_HEIGHT - PAD_LEN {
        state.player1_y = WIN_HEIGHT - PAD_LEN;
    }

    if state.player2_y < 0 {
        state.player2_y = 0;
    }
    if state.player2_y > WIN_HEIGHT - PAD_LEN {
        state.player2_y = WIN_HEIGHT - PAD_LEN;
    }


    if (state.ball_y as i32) < 1 {
        state.ball_velo_y *= -1.0;
        state.ball_y = 1.0;
    }
    if (state.ball_y as i32) >= WIN_HEIGHT {
        state.ball_velo_y *= -1.0;
        state.ball_y = (WIN_HEIGHT - 1) as f64;
    }


    if (state.ball_x as i32) < 0 {
        state.running = false;
        state.player2_score += 1;
        return;
    }
    if (state.ball_x as i32) >= WIN_WIDTH {
        state.running = false;
        state.player1_score += 1;
        return;
    }

    state.ball_x += state.ball_velo_x;
    state.ball_y += state.ball_velo_y;

    state.ball_velo_x *= 1.0005;
    if state.ball_velo_x > 1.0 {
        state.ball_velo_x = 1.0;
    } else {
        state.ball_velo_y *= 1.0005;
    }

}


fn render(state: &Pong) {
    clear();
    fn draw_pad(x: i32, y: i32) {
        for i in 0..PAD_LEN {
            mvaddch(y + i, x, '#' as u64);
        }
    }

    mvprintw(0, 0, &state.player1_score.to_string());
    mvprintw(0, WIN_WIDTH - 2, &state.player2_score.to_string());

    mvaddch(state.ball_y as i32, state.ball_x as i32, 'O' as u64);

    draw_pad(PLAYER1_X, state.player1_y);

    draw_pad(PLAYER2_X, state.player2_y);


}
