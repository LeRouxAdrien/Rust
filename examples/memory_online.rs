use glam::*;

use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::timer;
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::string::*;

use mpsc::TryRecvError;
use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
    time::Duration,
};

use std::sync::mpsc::Sender;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 255;

#[derive(Clone)]
struct ImageMemory {
    pos_x: f32,
    pos_y: f32,
    pos_x1: f32,
    pos_y1: f32,
    id: i32,
    id_card: u8,
    image: graphics::Image,
    image_hidden: graphics::Image,
}

struct Text {
    text: String,
    position: Vec2,
}

trait Score {
    fn two_items_recognized (&mut self, player: Player) -> Player;
    fn giveid_cards_returned (&mut self, index_image: usize) -> u8;
    fn get_returned_card_number (&mut self) -> u32;
    fn use_card (&mut self, clic_index: usize, send_message: bool);
}

#[derive(Clone)]
struct Player {
    name: String,
    score: u32,
}
//#[derive(Copy, Clone)]
#[derive(Clone)]
struct MainState {
    pos_x: f32,
    pos_y: f32,
    previous_card_returned: usize,
    mouse_down: bool,
    images: Vec<ImageMemory>,
    returned_card: u32,
    index_card_to_change: usize,
    players: Vec<Player>,
    print_score: bool,
    allowed : bool,
    my_number : u8,
    tx: Sender<[u8; MSG_SIZE]>,
}

impl MainState {
    /// Load images and create meshes.

    fn new(ctx: &mut Context, _tx:Sender<[u8; MSG_SIZE]>) -> GameResult<MainState> {
        let pos_x = 100.0;
        let pos_y = 100.0;
        let mouse_down = false;
        let allowed = true;
        let my_number = 1;
        let print_score = true;
        let returned_card = 0;
        let index_card_to_change = 0;
        let mut images = Vec::new();
        let mut players = Vec::new();
        let previous_card_returned = 13;
        let player1 = Player {name: "Jaja".to_string(), score: 0,};
        let player2 = Player {name: String::from("Alfred"), score: 0,};
        let tx = _tx;
       
        players.push(player1);
        players.push(player2);
        let image = ImageMemory {
            pos_x: 20.0, pos_y: 20.0, pos_x1: 145.0, pos_y1: 145.0, id: 2, id_card: 1, image: graphics::Image::new(ctx, "/thimothee.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image1 = ImageMemory {
            pos_x: 150.0, pos_y: 20.0, pos_x1: 275.0, pos_y1: 145.0, id: 1, id_card: 2, image: graphics::Image::new(ctx, "/frederic-jouault.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image2 = ImageMemory {
            pos_x: 280.0, pos_y: 20.0, pos_x1: 405.0, pos_y1: 145.0, id: 3, id_card: 3, image: graphics::Image::new(ctx, "/manon.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image3 = ImageMemory {
            pos_x: 410.0, pos_y: 20.0, pos_x1: 535.0, pos_y1: 145.0, id: 2, id_card: 4, image: graphics::Image::new(ctx, "/thimothee.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image4 = ImageMemory {
            pos_x: 20.0, pos_y: 150.0, pos_x1: 145.0, pos_y1: 275.0, id: 3, id_card: 5, image: graphics::Image::new(ctx, "/manon.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image5 = ImageMemory {
            pos_x: 150.0, pos_y: 150.0, pos_x1: 275.0, pos_y1: 275.0, id: 1, id_card: 6, image: graphics::Image::new(ctx, "/frederic-jouault.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image6 = ImageMemory {
            pos_x: 280.0, pos_y: 150.0, pos_x1: 405.0, pos_y1: 275.0, id: 4, id_card: 7, image: graphics::Image::new(ctx, "/manon_petite.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image7 = ImageMemory {
            pos_x: 410.0, pos_y: 150.0, pos_x1: 535.0, pos_y1: 275.0, id: 4, id_card: 8, image: graphics::Image::new(ctx, "/manon_petite.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image8 = ImageMemory {
            pos_x: 20.0, pos_y: 280.0, pos_x1: 145.0, pos_y1: 405.0, id: 6, id_card: 9, image: graphics::Image::new(ctx, "/adrien.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image9 = ImageMemory {
            pos_x: 150.0, pos_y: 280.0, pos_x1: 275.0, pos_y1: 405.0, id: 1, id_card: 10, image: graphics::Image::new(ctx, "/valentin.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image10 = ImageMemory {
            pos_x: 280.0, pos_y: 280.0, pos_x1: 405.0, pos_y1: 405.0, id: 6, id_card: 11, image: graphics::Image::new(ctx, "/valentin.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image11 = ImageMemory {
            pos_x: 410.0, pos_y: 280.0, pos_x1: 535.0, pos_y1: 405.0, id: 5, id_card: 12, image: graphics::Image::new(ctx, "/adrien.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        images.push(image);
        images.push(image1);
        images.push(image2);
        images.push(image3);
        images.push(image4);
        images.push(image5);
        images.push(image6);
        images.push(image7);
        images.push(image8);
        images.push(image9);
        images.push(image10);
        images.push(image11);

        let s = MainState {
            pos_x,
            pos_y,
            mouse_down,
            images,
            returned_card,
            previous_card_returned,
            index_card_to_change,
            players,
            print_score,
            allowed,
            my_number,
            tx
        };

        Ok(s)
    }
}

impl Score for MainState {
    fn two_items_recognized (&mut self, player: Player) -> Player{
        let mut player = player;
        if self.images[self.index_card_to_change].id == self.images[self.previous_card_returned].id {
            player.score = player.score + 10;
        }
        else {
        }
        let new_player = player.clone();
        return new_player;
    }

    fn giveid_cards_returned (&mut self, index_image: usize) -> u8 {
        let id_im = self.images[index_image].id_card;
        return id_im;
    }

    fn get_returned_card_number (&mut self) -> u32 {
        return self.returned_card;
    }

    fn use_card(&mut self, clic_index: usize, send_message: bool) {
        let image_mem = &self.images[clic_index];
   
        if send_message == true {
            let mut msg: [u8; MSG_SIZE] = [0; MSG_SIZE];
            msg[0] = 2;
            msg[1] = self.my_number;
            msg[2] = image_mem.id_card;
            if self.tx.send(msg).is_err() {
                println!(" ERREUR !!!!!!!!!!!!!!!!!!!!!!")
            }
        }
   
        match self.returned_card {
            1 => {
                let mut sel1 = self.clone();
                let index = self.index_card_to_change.clone();
                println!("card returned {}", MainState::giveid_cards_returned(&mut sel1, index) );
                println!("prev : {} ", self.previous_card_returned);
                self.previous_card_returned = self.index_card_to_change;
                self.index_card_to_change = clic_index;
                self.returned_card = 2;
                let mut player1 = self.players[0].clone();
                let player2 = self.players[1].clone();
                self.print_score = false;
               
                println!("Nom du player : {}", self.players[0].name);
                let mut sel = self.clone();
                player1 = MainState::two_items_recognized(&mut sel, player1);
                if self.players[0].score !=  player1.score {
                    println!("Tu as bon ! ");
                    self.players.pop();
                    self.players.pop();
                    self.players.push(player1);
                    self.players.push(player2);

                    let mut msg: [u8; MSG_SIZE] = [0; MSG_SIZE];

                    msg[0] = 3;
                    msg[1] = self.my_number;
                    if self.tx.send(msg).is_err() {
                        println!(" Erreur d'envoi de message OK")
                    }
                }
                else {
                    let mut msg: [u8; MSG_SIZE] = [0; MSG_SIZE];

                    msg[0] = 4;
                    msg[1] = self.my_number;
                    if self.tx.send(msg).is_err() {
                        println!(" Erreur d'envoi de message KO")
                    }
                }
                self.print_score = true;
            },
            0 => {
                println!("prev : {} ", self.previous_card_returned);
                self.returned_card = 1;
                self.index_card_to_change = clic_index;
            },
            _ => { },
        }
    }

}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
                self.mouse_down = true;
                self.pos_x = x;
                self.pos_y = y;
                println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
            }
       
            fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
                if self.allowed == true {
                       
                    self.mouse_down = false;
                    println!("card returned {}", self.get_returned_card_number());

                    let _ret_card = false;
                    for (i, _j) in (self.clone().images).iter().enumerate() {
                        let image_mem = &self.images[i];

                        if (self.pos_x >= image_mem.pos_x && self.pos_x <= image_mem.pos_x1) && (self.pos_y >= image_mem.pos_y && self.pos_y <= image_mem.pos_y1) {
                            MainState::use_card(self, i, true);
                        }                
                    }
                }
            }
         
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let scale = glam::Vec2::new(0.5, 0.5);
        match self.returned_card {
            1 => {          
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            for (i, _j) in (&self.images).iter().enumerate() {

                if i == self.index_card_to_change {
                    graphics::draw(
                        ctx,
                &self.images[self.index_card_to_change].image,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(self.images[self.index_card_to_change].pos_x, self.images[self.index_card_to_change].pos_y))
                    .scale(scale),
            )?;
                }
                else {
                    graphics::draw(
                        ctx,
                &self.images[i].image_hidden,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(self.images[i].pos_x, self.images[i].pos_y))
                    .scale(scale),
            )?;
                }
            }},
            2 => {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

            for (i, _j) in (&self.images).iter().enumerate() {
                if i == self.index_card_to_change || i == self.previous_card_returned {
                    graphics::draw(
                        ctx,
                &self.images[i].image,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(self.images[i].pos_x, self.images[i].pos_y))
                    .scale(scale),
            )?;
                }
                else {
                    graphics::draw(
                        ctx,
                &self.images[i].image_hidden,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(self.images[i].pos_x, self.images[i].pos_y))
                    .scale(scale),
            )?;
                }
            }
            println!(" returned_card {}", self.returned_card);
            self.returned_card = 0;
            let duration1 = timer::f64_to_duration(0.5);
            std::thread::sleep(duration1);
        },
            _ => {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            // Draw an image.
            // Draw an image with some options, and different filter modes.
            for value in &self.images {
                graphics::draw(
                    ctx,
                    &value.image_hidden,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(value.pos_x, value.pos_y))
                        .scale(scale),
                )?;
            }},
        }

        // Create and draw a stroked rectangle mesh.
        let rect = graphics::Rect::new(580.0, 100.0, 200.0, 150.0);
        let r2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            rect,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        graphics::draw(ctx, &r2, DrawParam::default())?;
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        if self.print_score == true{
            let duration = timer::f64_to_duration(0.1);
            std::thread::sleep(duration);
            let text_pla1 = Text {text: String::from("Player 1 : ") + &String::from(&self.players[0].name), position: Vec2::new(595.0, 130.0),};
            let text_score_pla1 = Text {text: String::from("Score : ") + &String::from(&self.players[0].score.to_string()), position: Vec2::new(595.0, 160.0),};
            let text_pla2 = Text {text: String::from("Player 2 : ") + &String::from(&self.players[1].name), position: Vec2::new(595.0, 190.0),};
            let text_score_pla2 = Text {text: String::from("Score : ") + &String::from(&self.players[1].score.to_string()), position: Vec2::new(595.0, 220.0),};
            let mut vec_text = Vec::new();
            vec_text.push(text_pla1);
            vec_text.push(text_score_pla1);
            vec_text.push(text_pla2);
            vec_text.push(text_score_pla2);
            for value in vec_text {
                let score = graphics::Text::new((value.text, font, 15.0));
                graphics::draw(
                    ctx,
                    &score,
                    graphics::DrawParam::new()
                        .dest(value.position)
                        .color(Color::from((192, 128, 64, 255))),
                )?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("memory", "ggez").add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let (tx, rx) = mpsc::channel::<[u8; MSG_SIZE]>();

    // Client connection to the server
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let mut state = MainState::new(&mut ctx, tx).unwrap();
    let state2 = MainState::clone(&state);

    // Client's player number
    let mut local_player_number = 0;
    thread::spawn(move || loop {
        // Buffer construction with the message to send depending on the tag message received by the server
        // Constructed as the messages in the server part
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let tag: u8 = buff[0];

                match tag {
                    // Ask for the player number
                    1 => {
                        if local_player_number == 0 {
                            println!("Give me my number : {}", buff[1]);
                            state.my_number = buff[1];
                            local_player_number = buff[1];
                            println!("I am : {}", local_player_number);
                        }
                    }
                    // Sending the card to return position
                    2 => {
                        println!("I am : {}", local_player_number);
                        let sender = buff[1];
                        let card_position = buff[2];
                        println!("Who has played : {}", sender);
                        println!("Card position : {}", card_position);
                        if sender != local_player_number {
                            let index_card: usize = (card_position-1).into();
                            println!("Call use_card with : {}", index_card);
                            state.use_card(index_card, false);
                        }
                    }
                    // Declare victory : it's still his turn and pair found
                    3 => {
                        println!("Victory : pair found");
                    }
                    // Declare failure : change of player, not his turn anymore
                    4 => {
                        println!("Failure : Change of player");
                        let allowed_player: u8 = buff[1]; // tell if the player is allowed to play
                        if allowed_player == local_player_number {
                            println!("My turn");
                            state.allowed = true;
                        }
                        else {
                            println!("Other player's turn");
                            state.allowed = false;
                        }
                        println!("Alowed NOW {}", state.allowed);
                       
                    }
                    // End of the game                    
                    10 => {
                        println!("End of the game");
                        state.allowed = false;
                    }
                    _ =>  println!("Not recognized"), // the command is not recognized
                }                
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server down");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                client.write_all(&msg).expect("Writing on the socket failed");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("{}", graphics::renderer_info(&ctx)?);
    event::run(ctx, events_loop, state2)
}