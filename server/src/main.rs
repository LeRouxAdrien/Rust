use std::{
    io::{ErrorKind, Read},
    net::TcpListener,
    sync::mpsc,
    thread,
    };
    
    const LOCAL: &str = "127.0.0.1:6000";
    const MSG_SIZE: usize = 255;
    
    fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
    }
    
    fn main() {

    // Connexion clients/serveur
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind"); 
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");
    
    let mut clients = vec![]; // On crée un vecteur pour stocker tous les joueurs (correspond aux clients)
    let (tx, _rx) = mpsc::channel::<[u8; MSG_SIZE]>();

    let mut player_number: u8 = 0; // Numéro de joueur
    let mut players_score: [u8; 2] = [0, 0]; // Initialise les scores des 2 joueurs à 0
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();

            // On initialise un buffer avec des 0, de la taille des messages qui vont être échangés entre les clients et le serveur
            // Le premier octet correspond à l'identifiant de la commande (le tag) et les suivant serviront pour le message à transmettre
            let mut returned_buffer: [u8; MSG_SIZE] = [0; MSG_SIZE]; 
            player_number += 1; // On incrémente le nombre de joueurs
            returned_buffer[0] = 1; // A la connexion on considère qu'il s'agit du tag 1 : une demande de numéro de joueur pour que le client sache qui il est
            returned_buffer[1] = player_number;
            println!("JOUEUR  {} ", player_number);
            tx.send(returned_buffer).expect("failed to send message to rx"); // On envoie le le numéro du joueur aux clients

            clients.push(socket.try_clone().expect("failed to clone client"));

            // Thread pour les échanges 
            thread::spawn(move || loop {
                let mut buff: [u8; MSG_SIZE] = [0; MSG_SIZE];

                // On va lire les données qu'on reçoit d'un client
                match socket.read_exact(&mut buff) { 
                    Ok(_) => {
                        println!("tag message recv {}", buff[0]);
                        let tag = buff[0];                

                        // Chaque client a un booleen lui indiquant s'il peut jouer - par défaut c'est non
                        // en dur dans le code des clients : quand le client reçoit son numéro de joueur, s'il reçoit 2 il a le droit de jouer
                        let mut returned_buffer: [u8; MSG_SIZE] = [0; MSG_SIZE];

                        // On construit le message à envoyer en fonction de la commande que l'on reçoit
                        match tag {

                            // Demande de numéro de joueur
                            1 => {
                                player_number += 1;
                                returned_buffer[0] = 1;
                                returned_buffer[1] = player_number;
                                println!("Donne moi mon numéro : {}", player_number);
                            }
                            
                            // Envoi de l'id de la carte à retourner
                            2 => {
                                returned_buffer[0] = 2;
                                returned_buffer[1] = buff[1];
                                returned_buffer[2] = buff[2];
                                println!("Id de la carte : {} ", buff[1]);
                            }
                            
                            // Déclare victoire : toujours à lui, paire trouvée
                            3 => {
                                let player_number: usize = buff[1].into();
                                players_score[player_number - 1] += 1;
                                returned_buffer[0] = 3;
                                println!("Réussite : Paire trouvée");
                            }
                            
                            // Déclare échec : à l'autre joueur
                            4 => {
                                let player_number: u8 = buff[1];
                                // Retourne numéro joueur autorisé
                                returned_buffer[0] = 4;
                                if player_number == 1 {
                                    returned_buffer[1] = 2;
                                }
                                else {
                                    returned_buffer[1] = 1;
                                }
                                println!("Echec : Change de joueur : {}", returned_buffer[1]);
                            }
                            
                            // Fin de la partie                           
                            10 => {
                                
                                // Retourne numéro joueur autorisé
                                returned_buffer[0] = 10;
                                if players_score[0] > players_score[1] {
                                    // Victoire du joueur 1
                                    returned_buffer[1] = 1;
                                }
                                else if players_score[0] < players_score[1] {
                                    // Victoire du joueur 2
                                    returned_buffer[1] = 2;
                                }
                                else {
                                    // Match nul
                                    returned_buffer[1] = 0;
                                }
                                println!("Partie finie");
                            }
                            _ =>  println!("Non reconnu"), // Erreur, la commande reçue n'est pas reconnue
                        }
                        tx.send(returned_buffer).expect("failed to send message to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),

                    // Perte de connexion ; on enlève un joueur/client
                    Err(_) => {
                        // player_number -= 1; //but not used here
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });

        }
    }
}
    