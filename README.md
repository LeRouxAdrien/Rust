Jeu du Memory

Pour le jeu du « Memory » offline :

## Déroulement du jeu :
Le jeu est un peu plus élaboré que le mode online. 
	-	2 joueurs appelé « Jaja » et « Alfred » dit « player1 » et « player2 » sont affiliés au jeu dès le début avec un score de 0. 
	-	Sur cette plateforme, un seul PC sera utilisé et seul le pavé numérique ou la souris pourront servir d’interface user-machine. Par conséquent, le jeu n’est pas à sa finalité car il n’y pas de fin de partie et que les cartes ne s’enlèvent pas du jeu après avoir été trouvé. 
	-	Le score du joueur augmente de 10 lorsqu’il clique sur 2 cartes identiques. 
	-	Le joueur ne perd pas de point lorsque les cartes ne sont pas identiques. 
	-	Le joueur 1 commence la partie en premier.

## Ce qu’il reste à améliorer : 
Le temps nous a manqué durant ce projet. Par conséquent, nous avons commencé et prévu de :
	-	Réalisé un algorithme de nombres aléatoires qui a été mis en place mais n’a pas été intégré au projet (l.80 à l.106)
	-	De gérer le double clique sur une image
	-	D'implémenter la fin de la partie


## Pour lancer le projet il suffit de :
	-	 Aller dans le dossier ggez 
	-	 Lancer ```cargo run –example memory_offline``` 

Pour le jeu du « Memory » online :

## Lancer le jeu online

Ouvrir un terminal pour le serveur<br>
Taper ```cd server && cargo run```

Ouvrir un terminal pour chaque joueur (client)<br>

Taper ```sudo cargo run --example memory_online```

## Le déroulement de la partie

Une fenêtre s'ouvre avec le jeu pour chaque joueur.

Le premier joueur clique sur une carte, elle se retourne face visible. Il choisit une seconde carte qui se retourne aussi face visible.

Si les deux cartes sont identiques alors :
	- Le score de ce joueur augmente de 10 points
	- C'est toujours à son tour de jouer
	
Si les deux cartes ne sont pas identiques :
	- Les scores ne changent pas
	- C'est au tour de l'autre joueur
	
## Ce qu’il reste à améliorer : 
Le gestion du jeu en multijoueur n'est pas bien terminée ; les joueurs ne sont pas correctement bloqués lorsque ce n'est pas leur tour et si on troisième joueur tente de se connecter il ne sera pas refusé.
Lorsqu'une paire est trouvée, les cartes ne restent pas retournées face visible.

