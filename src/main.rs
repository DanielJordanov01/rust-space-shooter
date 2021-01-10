use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult, timer};
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard;
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
use ggez::input::mouse::button_pressed;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const PLAYER_VELOCITY: f32 = 5.0;

const ASTEROID_RADIUS: f32 = 20.;
const ASTEROID_VELOCITY: f32 = 2.0;

const PROJECTILE_WIDTH: f32 = 5.0;
const PROJECTILE_HEIGTH: f32 = 15.0;
const PROJECTILE_VELOCITY: f32 = -5.0;

type Vector = ggez::mint::Vector2<f32>;
#[derive(Debug)]
struct Asteroid {
    rect: Rect,
    vel: Vector,
    has_collided: bool
}

impl Asteroid {
    fn new() -> Self {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut x_pos = rng.gen_range(0.0, SCREEN_HEIGHT);

        Asteroid {
            rect: Rect::new(
            x_pos,
            SCREEN_HEIGHT * 0.01,
            ASTEROID_RADIUS,
            ASTEROID_RADIUS
            ),
            vel: Vector { x: 0.0, y: ASTEROID_VELOCITY},
            has_collided: false
        }
    }
}
#[derive(Debug)]
struct Projectile {
    rect: Rect,
    vel: Vector,
    has_collided: bool
}

impl Projectile {
    fn new(player_pos: Rect) -> Self {
        Projectile {
            rect: Rect::new(
                player_pos.x,
                player_pos.y,
                PROJECTILE_WIDTH,
                PROJECTILE_HEIGTH
            ),
            vel: Vector { x: 0.0, y: PROJECTILE_VELOCITY },
            has_collided: false
        }
    }
}

struct MainState {
    player: Rect,
    asteroids: Vec<Asteroid>,
    projectiles: Vec<Projectile>,
    frames: u32
}

impl MainState {
    fn remove_asteroid(&mut self) {
        self.asteroids.remove(0);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Player movement with collisions on the edges of the window
        if is_key_pressed(ctx, keyboard::KeyCode::W) {
            if !(self.player.y < 90.0 && self.player.top() < 90.0) {
                self.player.y -= PLAYER_VELOCITY;
            }
        }
        if is_key_pressed(ctx, keyboard::KeyCode::S) {
            if !(self.player.y > 0.0 && self.player.bottom() > SCREEN_HEIGHT) {
                self.player.y += PLAYER_VELOCITY;
            }
        }
        if is_key_pressed(ctx, keyboard::KeyCode::A) {
            if !(self.player.x < 0.0 && self.player.left() < 0.0) {
                self.player.x -= PLAYER_VELOCITY;
            }
        }
        if is_key_pressed(ctx, keyboard::KeyCode::D) {
            if !(self.player.x > 0.0 && self.player.right() > SCREEN_WIDTH) {
                self.player.x += PLAYER_VELOCITY;
            }
        }

        // move asteroids down
        for aster in self.asteroids.iter_mut() {
            aster.rect.translate(aster.vel);
        }

        // spawn new asteroids
        if self.frames == 20 || self.frames == 50 {
            self.asteroids.push(Asteroid::new());
        }

        // delete asteroids when they go off the screen
        self.asteroids.retain(|aster| aster.rect.y > 0.0 && aster.rect.y < SCREEN_HEIGHT);

        // move projectiles up
        for proj in self.projectiles.iter_mut() {
            proj.rect.translate(proj.vel);
        }

        // delete projectiles when they go off the screen
        self.projectiles.retain(|proj| proj.rect.y > 0.0 && proj.rect.y < SCREEN_HEIGHT);

        // collisions
        for aster in self.asteroids.iter_mut() {
            if aster.rect.overlaps(&self.player) {
                println!("Over");
            }
            // if a projectile collides with a rock set has_collided to true
            // proj and aster will be removed
            for proj in self.projectiles.iter_mut() {
                if proj.rect.overlaps(&aster.rect) {
                    proj.has_collided = true;
                    aster.has_collided = true;
                }
            }
        }

        // remove projectiles and asteroids with prop has_collided = true
        self.projectiles.retain(|proj| proj.has_collided == false);
        self.asteroids.retain(|aster| aster.has_collided == false);

        // used to time the game
        if self.frames < 60 {
            self.frames += 1;
        } else {
            self.frames = 0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::Color;
        use ggez::graphics;

        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0)); // black background

        // Create meshes
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
      graphics::DrawMode::fill(),
            self.player,
            Color::new(1.0, 1.0, 1.0, 1.0) // gray
        ).expect("error creating a player mesh");


        // create and draw asteroids
        for ast in self.asteroids.iter() {
            let asteroid_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                ast.rect,
                Color::new(1.0, 1.0, 1.0, 1.0) // gray
            ).expect("error creating an asteroid mesh");

            graphics::draw(ctx, &asteroid_mesh, graphics::DrawParam::default())
                .expect("error drawing an asteroid");
        }


        // create and draw projectiles
        for proj in self.projectiles.iter() {
            let projectile_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                proj.rect,
                Color::new(1.0, 0.0, 0.0, 1.0)
            ).expect("error creating a projectile mesh");

            graphics::draw(ctx, &projectile_mesh, graphics::DrawParam::default())
                .expect("error drawing a projectile");
        }


        // Draw player
        graphics::draw(ctx, &player_mesh, graphics::DrawParam::default())
            .expect("error drawing a player");


        graphics::present(ctx).expect("error presenting");

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        self.projectiles.push(Projectile::new(self.player));
    }
}

fn main() {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("rocket", "Daniel Yordanov")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let main_state = &mut MainState {
        player: Rect::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT * 0.9, 10.0, 20.0),
        asteroids: vec![Asteroid::new(), Asteroid::new()],
        projectiles: Vec::new(),
        frames: 0
    };

    ggez::event::run(ctx, event_loop, main_state);
}
