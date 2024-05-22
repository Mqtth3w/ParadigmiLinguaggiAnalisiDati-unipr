
const ARENA_H: i32= 400;
const ARENA_W: i32 = 400;
const BALL_H: i32 = 20;
const BALL_W: i32 = 20;

struct Ball {
    xpos: i32,
    ypos: i32,
    dx: i32,
    dy: i32,
}

fn move_ball(b: &mut Ball) {
    if 0 > b.xpos + b.dx || b.ypos + b.dy > ARENA_W - BALL_W {
        b.dx = -b.dx;
    }
    if 0 > b.ypos + b.dy || b.ypos + b.dy > ARENA_H - BALL_H {
        b.dy = -b.dy;
    }
    b.xpos += b.dx;
    b.ypos += b.dy;
}

fn main() {
    let mut ball = Ball {
        xpos: 100,
        ypos: 100,
        dx: 5,
        dy: 5,
    };

    move_ball(&mut ball);
    println!("xpos: {}, ypos: {}", ball.xpos, ball.ypos);
}
