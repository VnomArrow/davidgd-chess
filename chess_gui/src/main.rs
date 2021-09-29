use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
use chess_logic;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx).unwrap();

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    black_rook: graphics::Image,
    white_rook: graphics::Image,
    black_queen: graphics::Image,
    white_queen: graphics::Image,
    black_pawn: graphics::Image,
    white_pawn: graphics::Image,
    black_bishop: graphics::Image,
    white_bishop: graphics::Image,
    black_knight: graphics::Image,
    white_knight: graphics::Image,
    black_king: graphics::Image,
    white_king: graphics::Image,
    black_square: graphics::Image,
    white_square: graphics::Image,
    mouse_button_press_down: Option<ggez::mint::Point2<f32>>,
    game: chess_logic::GAME
}

pub fn get_square_from_mouse_pos(pos: ggez::mint::Point2<f32>) -> ggez::mint::Point2<u8> {
    ggez::mint::Point2{
        x: ((pos.x*8.0) / SCREEN_WIDTH) as u8,
        y: ((pos.y*8.0) / SCREEN_HEIGHT) as u8,
    }
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let black_rook = graphics::Image::new(ctx, "/Chess_rdt60.png")?;
        let white_rook = graphics::Image::new(ctx, "/Chess_rlt60.png")?;
        let black_queen = graphics::Image::new(ctx, "/Chess_qdt60.png")?;
        let white_queen = graphics::Image::new(ctx, "/Chess_qlt60.png")?;
        let black_pawn = graphics::Image::new(ctx, "/Chess_pdt60.png")?;
        let white_pawn = graphics::Image::new(ctx, "/Chess_plt60.png")?;
        let black_bishop = graphics::Image::new(ctx, "/Chess_bdt60.png")?;
        let white_bishop = graphics::Image::new(ctx, "/Chess_blt60.png")?;
        let black_knight = graphics::Image::new(ctx, "/Chess_ndt60.png")?;
        let white_knight = graphics::Image::new(ctx, "/Chess_nlt60.png")?;
        let black_king = graphics::Image::new(ctx, "/Chess_kdt60.png")?;
        let white_king = graphics::Image::new(ctx, "/Chess_klt60.png")?;
        let black_square = graphics::Image::new(ctx, "/black_square.png")?;
        let white_square = graphics::Image::new(ctx, "/white_square.png")?;

        let game = chess_logic::init_game();

        let s = MyGame {
            black_rook,
            white_rook,
            black_queen,
            white_queen,
            black_pawn,
            white_pawn,
            black_bishop,
            white_bishop,
            black_knight,
            white_knight,
            black_king,
            white_king,
            black_square,
            white_square,
            mouse_button_press_down: None,
            game,
        };

        Ok(s)
    }

    pub fn draw_chess_board(&mut self, board: [u8; 64], ctx: &mut Context) -> GameResult<()> {
        use chess_logic::*;
        let mut rank  = 0;
        for piece in board.iter() {
            let piece = *piece;
            let x = rank % 8;
            let y = rank / 8;

            let image = &self.black_square;
            let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
            let scale = glam::Vec2::new(scale_factor, scale_factor);
            let size = image.dimensions().h * scale.x;
            let dst = glam::Vec2::new(size*x as f32, size*y as f32);

            if (x + y) % 2 == 0 {
                graphics::draw(ctx, &self.black_square, graphics::DrawParam::new()
                .dest(dst)
                .scale(scale),)?;
            }
            else {
                graphics::draw(ctx, &self.white_square, graphics::DrawParam::new()
                .dest(dst)
                .scale(scale),)?;
            }

            let mut image: Option<&graphics::Image> = None;
            if piece == 0 {
                image = None;
            } else if is_black_king(piece) {
                image = Some(&self.black_king);
            } else if is_black_queen(piece) {
                image = Some(&self.black_queen);
            } else if is_black_rook(piece) {
                image = Some(&self.black_rook);
            } else if is_black_bishop(piece) {
                image = Some(&self.black_bishop);
            } else if is_black_knight(piece) {
                image = Some(&self.black_knight);
            } else if is_black_pawn(piece) {
                image = Some(&self.black_pawn);
            } else if is_white_king(piece) {
                image = Some(&self.white_king);
            } else if is_white_queen(piece) {
                image = Some(&self.white_queen);
            } else if is_white_rook(piece) {
                image = Some(&self.white_rook);
            } else if is_white_bishop(piece) {
                image = Some(&self.white_bishop);
            } else if is_white_knight(piece) {
                image = Some(&self.white_knight);
            } else if is_white_pawn(piece) {
                image = Some(&self.white_pawn);
            }
             
            if image.is_some() {
                let image = image.unwrap();
                let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
                let scale = glam::Vec2::new(scale_factor, scale_factor);
                let size = image.dimensions().h * scale.x;
                let dst = glam::Vec2::new(size*x as f32, size*y as f32);
                graphics::draw(ctx, image, graphics::DrawParam::new()
                    .dest(dst)
                    .scale(scale),)?;
            }

            
            rank += 1;
        } 
        return Ok(());
    }
}

pub fn get_mouse_position(ctx: &mut Context) -> ggez::mint::Point2<f32> {
    ggez::input::mouse::position(ctx)
}


pub fn draw_rectangle(ctx: &mut Context, rect: Rect, color: Color) -> GameResult<()> {
    // First we set the color to draw with, in this case all food will be
    // colored blue.
    //let color = [0.0, 0.0, 1.0, 1.0].into();
    // Then we draw a rectangle with the Fill draw mode, and we convert the
    // Food's position into a `ggez::Rect` using `.into()` which we can do
    // since we implemented `From<GridPosition>` for `Rect` earlier.
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        // Store where the mouse was pressed down
        self.mouse_button_press_down = Some(ggez::mint::Point2{x, y});
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        let mouse_up = ggez::mint::Point2{x, y};

        let mouse_up_board_pos = get_square_from_mouse_pos(mouse_up);
        let mouse_down_board_pos = get_square_from_mouse_pos(self.mouse_button_press_down.unwrap());
        let pos_array = ['a', 'b','c','d','e','f','g','h',];
        let move_notation_mouse_up = pos_array[(mouse_up_board_pos.x) as usize].to_string() + (8-mouse_up_board_pos.y).to_string().as_str();
        let move_notation_mouse_down = pos_array[(mouse_down_board_pos.x) as usize].to_string() + (8-mouse_down_board_pos.y).to_string().as_str();

        let is_valid = chess_logic::move_piece_from_to(move_notation_mouse_down.as_str(), move_notation_mouse_up.as_str(), &mut self.game);
        println!("{}, {}", move_notation_mouse_down, move_notation_mouse_up);
        if !is_valid {
            println!("{}", "not a valid move");
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...

        let board = self.game.get_board();
        self.draw_chess_board(board, ctx);

        graphics::present(ctx)
    }
}