//! A collection of semi-random shape and image drawing examples.

use glam::*;

use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::timer;
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::string::*;
//use rand::distributions::{Distribution, Uniform};

#[derive(Clone)]
struct ImageMemory {
    pos_x: f32, // variable which contains the x top left side of the card position
    pos_y: f32, // variable which contains the y top left side of the card position
    pos_x1: f32, // variable which contains the x bootom right side of the card position
    pos_y1: f32, // variable which contains the y bootom right side of the card position
    id: i32, // Id of similary images
    id_card: i32, // Own id of the card
    image: graphics::Image, // Top of the card
    image_hidden: graphics::Image, // Back of the card
}

struct Text {
    text: String,
    position: Vec2,
}
trait Score {
    fn two_item_recognized (&mut self, player: Player) -> Player;
    fn give_id_cards_returned (&mut self, index_image: usize) -> i32; 
    fn get_returned_card_number (&mut self) -> u32; // function used for the server to transfer card(s) to return to clients
}

#[derive(Clone)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Clone)]
struct MainState {
    pos_x: f32, // variable of the x click position 
    pos_y: f32, // variable of the y click position 
    previous_card_returned: usize, // variable which contains the index of the previous card returned 
    mouse_down: bool, // low or high state of the mouse
    images: Vec<ImageMemory>, // vector which contains all images of the game
    return_card: u32, // variable which contains the number of card to return
    index_card_to_change: usize, // variable which contains the card index to return 
    players: Vec<Player>, // vector which contains players
    print_score: bool, // variable which manage the display of the score
    user_turn: u8, // variable which contains the player turn
}

impl MainState {
    
     // Load images with their different positions, the name of users and initialize variables used for the game. */ 
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos_x = 0.0;
        let pos_y = 0.0;
        let mouse_down = false;
        let print_score = true;
        let return_card = 0;
        let user_turn = 1;
        let index_card_to_change = 0;
        let mut images = Vec::new(); 
        let mut players = Vec::new();
        let previous_card_returned = 13;
        let player1 = Player {name: "Jaja".to_string(), score: 0,};
        let player2 = Player {name: String::from("Alfred"), score: 0,};

        
         // Random function allows cards to be randomed parsed in the game - incrément 2 
         



/*         let step = Uniform::new(0, 11);
        let mut numberList = Vec::new();
        let mut rng = rand::thread_rng();
        let mut rd = rng.gen_range(0..11);
        let mut compteur = 0;
        numberList.push(1);
        // let choices: Vec<_> = step.sample_iter(&mut rng).take(150).collect();
        while( numberList.len() < 12) {
            match numberList[compteur] {
                1 => {
                    numberList.push(numberList[compteur] + 4);
} ,
                0 =>{numberList.push(numberList[compteur] + 2); 
                    }
                ,
                11 => {numberList.push(0);},
                    
                _ => {
                    numberList.push(numberList[compteur] + 1);
                },
            }
            compteur = compteur + 1;
        }

        println!("{:?}", choices);

*/


        players.push(player1); 
        players.push(player2); 

        
         //We initialize images and put on a vector
         
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
            pos_x: 150.0, pos_y: 280.0, pos_x1: 275.0, pos_y1: 405.0, id: 5, id_card: 10, image: graphics::Image::new(ctx, "/valentin.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image10 = ImageMemory {
            pos_x: 280.0, pos_y: 280.0, pos_x1: 405.0, pos_y1: 405.0, id: 5, id_card: 11, image: graphics::Image::new(ctx, "/valentin.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
        };
        let image11 = ImageMemory {
            pos_x: 410.0, pos_y: 280.0, pos_x1: 535.0, pos_y1: 405.0, id: 6, id_card: 12, image: graphics::Image::new(ctx, "/adrien.png")?, image_hidden: graphics::Image::new(ctx, "/doscarte.png")?,
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
            return_card,
            previous_card_returned,
            index_card_to_change,
            players,
            print_score,
            user_turn,
        };

        Ok(s)
    }
}

impl Score for MainState {
    
     //Function which compare if both cards are similary or not. Rise the score of the current player and return the player with his updated score
     
    fn two_item_recognized (&mut self, player: Player) -> Player{
        let mut player = player;
        if self.images[self.index_card_to_change].id == self.images[self.previous_card_returned].id {
            player.score = player.score + 10;
        }
        else {
        }
        let new_player = player.clone();
        return new_player;

    }

    
    // Return the card id
     
    fn give_id_cards_returned (&mut self, index_image: usize) -> i32 {
        let id_im = self.images[index_image].id_card;
        return id_im;
    }

    
     // return the number of card to display on the screen
     
    fn get_returned_card_number (&mut self) -> u32 {
        return self.return_card;
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    
    // Function which update information such variable during the game
     
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    
     // Memorize the click position when player press the mouse button
     
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
                self.mouse_down = true;
                self.pos_x = x;
                self.pos_y = y;
                println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
            }
        
        // When the player release the mouse button, for all pictures on the game, we check if 2 images are similar.  
         
         fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
                self.mouse_down = false;
                println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
                println!("Mouse button released: {:?}, x: {}, y: {}", button, self.pos_x, self.pos_y);
                println!("card returned {}", MainState::get_returned_card_number(self) ); 
                let mut _ret_card = false;
                for (i, _j) in (&self.images).iter().enumerate() {
                    let image_mem = &self.images[i];
                    println!(" up Event i : {}", i);

                        if image_mem.pos_x1 as i32 - self.pos_x as i32 >= 0 && image_mem.pos_y1 as i32 - self.pos_y as i32 >= 0 && image_mem.pos_x1 - image_mem.pos_x  >= image_mem.pos_x1 - self.pos_x && image_mem.pos_y1 - image_mem.pos_y >= image_mem.pos_y1 - self.pos_y{
                        println!("index : {} prev : {} return_card : {}", self.index_card_to_change, self.previous_card_returned, self.return_card);
                        match self.return_card {
                            // if a card was returned, 
                            1 => {
                                let mut sel1 = self.clone();
                                let index = self.index_card_to_change.clone();
                                println!("card returned {}", MainState::give_id_cards_returned(&mut sel1, index) );
                                self.previous_card_returned = self.index_card_to_change;
                                self.index_card_to_change = i;
                                self.return_card = 2;
                                let mut player1 = self.players[0].clone();
                                let mut player2 = self.players[1].clone();
                                self.print_score = false;
                                
                                let mut sel = self.clone();
                                
                                 //if it's the turn of the user 1, if both images have same id, we pop previous player inside the vector and replace them by players with their updated score
                                 
                                if self.user_turn == 1 {
                                    player1 = MainState::two_item_recognized(&mut sel, player1);
                                    if self.players[0].score !=  player1.score {
                                        println!("Tu as bon ! ");
                                        self.players.pop();
                                        self.players.pop();
                                        self.players.push(player1);
                                        self.players.push(player2);
                                    }

                                    else {
                                        self.user_turn = 2;
                                    }
                                }     
                                else {
                                    
                                    // if it's the turn of the user 1, if both images have same id, we pop previous player inside the vector 
                                    // and replace them by players with their updated score
                                      
                                    player2 = MainState::two_item_recognized(&mut sel, player2);
                                    if self.players[1].score !=  player2.score { 
                                        println!("Tu as bon ! ");
                                        self.players.pop();
                                        self.players.pop();
                                        self.players.push(player1);
                                        self.players.push(player2);
                                    }
                                    else {
                                        self.user_turn = 1;
                                    }
                                }

                                self.print_score = true;
                            },
                            // if 0 card was returned
                            0 => {
                                println!("prev : {} ", self.previous_card_returned);
                                self.return_card = 1;
                            },
                            _ => { },
                    }
                    self.index_card_to_change = i;
                }
                
                }
            }
    
    // Display the frontend part of the app
     
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let scale = glam::Vec2::new(0.5, 0.5);
        match self.return_card {
            
            // display all the card with the bottom side except the clicked card. The clicked card will be display on the top.
             
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
            
             // display all the card on the bottom side except the clicked card and the previous returned card. The clicked card and the previous returned card
             // will be display on the top.
             
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
            self.return_card = 0;      
            let duration1 = timer::f64_to_duration(0.5);
            std::thread::sleep(duration1);
        },
        
         // Otherwise, display all the card on the bottom side
         
            _ => {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
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
        
         // Create and draw a stroked rectangle mesh to surround players score and name 
         
        let rect = graphics::Rect::new(580.0, 100.0, 200.0, 150.0);
        let r2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            rect,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        
        // Display the name and the score of players
         
        graphics::draw(ctx, &r2, DrawParam::default())?;
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        if self.print_score == true {
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

    println!("{}", graphics::renderer_info(&ctx)?);
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
