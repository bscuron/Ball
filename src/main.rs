#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate ncurses;
use ncurses::{curs_set, endwin, getmaxx, getmaxy, initscr, stdscr, CURSOR_VISIBILITY, mvprintw, refresh};
use std::time::Duration;
use std::{thread, time};

const DELAY: Duration = time::Duration::from_millis(25);
const BALL_RADIUS: f32 = 10.5;
const FORCE_GRAVITY: f32 = 0.01;
const CHAR_EMPTY: char = ' ';
const CHAR_BALL: char = 'X';

struct Ball {
    pos: Vec<f32>,
    vel: Vec<f32>,
    acc: Vec<f32>,
    radius: f32,
}

fn main() {
    ncurses_init();
    let rows: i32 = getmaxy(stdscr());
    let cols: i32 = getmaxx(stdscr());

    let mut quit: bool = false;
    let mut ball: Ball = Ball {
        pos: vec![(cols / 2) as f32, (rows / 2) as f32],
        acc: vec![0.5, 0.0],
        vel: vec![0.0, 0.0],
        radius: BALL_RADIUS,
    };
    while !quit {
        update(&mut ball, rows, cols);
        show(&mut ball, rows, cols);
        thread::sleep(DELAY);
    }
    ncurses_term();
}

fn update(ball: &mut Ball, rows: i32, cols: i32) {
    add(&mut ball.acc, &vec![0.0, FORCE_GRAVITY]);
    add(&mut ball.vel, &ball.acc);
    add(&mut ball.pos, &ball.vel);

    if ball.pos[1] >= rows as f32 - ball.radius - 1.0 {
        ball.pos[1] = rows as f32 - ball.radius - 1.0;
        ball.vel[1] *= -1.0;
    }

    if ball.pos[1] <= ball.radius - 1.0 {
        ball.pos[1] = ball.radius - 1.0;
        ball.vel[1] *= -1.0;
    }

    if ball.pos[0] >= cols as f32 - ball.radius - 1.0 {
        ball.pos[0] = cols as f32 - ball.radius - 1.0;
        ball.vel[0] *= -1.0;
    }


    if ball.pos[0] <= ball.radius - 1.0 {
        ball.pos[0] = ball.radius - 1.0;
        ball.vel[0] *= -1.0;
    }

    mul(&mut ball.acc, &vec![0.5, 0.5]);
}

fn show(ball: &mut Ball, rows: i32, cols: i32) {
    for y in 0..rows {
        for x in 0..cols {
            let dx: f32 = ball.pos[0] - x as f32;
            let dy: f32 = ball.pos[1] - y as f32;
            let dist_squared: f32 = dx * dx + dy * dy;
            let radius_squared: f32 = ball.radius * ball.radius;
            let c: char = match dist_squared <= radius_squared {
                true => CHAR_BALL,
                false => CHAR_EMPTY,
            };
            mvprintw(y, x, &c.to_string());
        }
    }
    refresh();
}


fn mul(va: &mut Vec<f32>, vb: &Vec<f32>) {
    assert_eq!(va.len(), vb.len());
    for i in 0..va.len() {
        va[i] *= vb[i];
    }
}

fn add(va: &mut Vec<f32>, vb: &Vec<f32>) {
    assert_eq!(va.len(), vb.len());
    for i in 0..va.len() {
        va[i] += vb[i];
    }
}

fn ncurses_init() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn ncurses_term() {
    endwin();
}
