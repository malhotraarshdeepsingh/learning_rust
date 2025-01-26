use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, terminal,
};
use rand::Rng;
use std::io::stdout;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;

fn main() {
    let mut stdout = stdout();

    // setup terminal for game display
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    // initialize snake
    let mut snake: Vec<(u16, u16)> = vec![
        (WIDTH / 4, HEIGHT / 2),
        (WIDTH / 4 - 1, HEIGHT / 2),
        (WIDTH / 4 - 2, HEIGHT / 2),
    ];

    // initialize food in a random position
    let mut rng = rand::thread_rng();
    let mut food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));

    // variable to store score
    let mut score = 0;

    // variable to store the initial direction of the snake
    let mut direction = KeyCode::Right;

    // variable to store the last time the snake moved
    let mut last_update_item = Instant::now();

    // variable to store the interval between each snake movement
    let update_interval = Duration::from_millis(100);

    loop {
        // check for user input
        if event::poll(Duration::from_millis(0)).unwrap() {
            // change direction based on user input
            if let event::Event::Key(key_event) = event::read().unwrap() {
                direction = match key_event.code {
                    KeyCode::Up if direction != KeyCode::Down => KeyCode::Up,
                    KeyCode::Down if direction != KeyCode::Up => KeyCode::Down,
                    KeyCode::Left if direction != KeyCode::Right => KeyCode::Left,
                    KeyCode::Right if direction != KeyCode::Left => KeyCode::Right,
                    KeyCode::Esc => break, // exit the game
                    _ => direction, // default behaviour
                };
            }
        }

        // update game state at regular intervals
        if last_update_item.elapsed() >= update_interval {
            let head = snake[0];

            // determine new head position based on direction
            let new_head = match direction {
                KeyCode::Up => (head.0, head.1 - 1),
                KeyCode::Down => (head.0, head.1 + 1),
                KeyCode::Left => (head.0 - 1, head.1),
                KeyCode::Right => (head.0 + 1, head.1),
                _ => head,
            };

            // add new head
            snake.insert(0, new_head);

            // check if the snake has collided with the wall
            if new_head.0 == 0
                || new_head.0 == WIDTH - 1
                || new_head.1 == 0
                || new_head.1 == HEIGHT - 1
                || snake[1..].contains(&new_head)
            {
                break; // exit the game
            }

            // check if the snake has eaten the food
            if new_head == food {
                score += 1;

                // generate new food in a random position
                food = loop {
                    let new_food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
                    if !snake.contains(&new_food) {
                        break new_food;
                    }
                };
            } else {
                snake.pop(); // remove tail
            }

            last_update_item = Instant::now(); // update last update time
        }

        // clear the screen
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        // draw game state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                    print!("#"); // draw wall
                } else if snake.contains(&(x, y)) {
                    print!("O"); // draw snake
                } else if (x, y) == food {
                    print!("F"); // draw food
                } else {
                    print!(" "); // draw empty space
                }
            }
            println!(); // println!() moves to the next line
        }

        println!("Score: {}", score);

        thread::sleep(Duration::from_millis(100));
    }

    // cleanup terminal
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();
}
