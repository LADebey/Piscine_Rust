pub fn first_subword(mut s: String) -> String {


    // déclare la variabe qui va nous servir d'index pour split ( par défaut tout)
    let mut split_at = s.len();


        // boucle jusqu'à trouver une majuscule ou un underscore ( en vérifiant que l'index est différent de la première lette pour pas que ça bloque avec les Pascal Case)
    for(i, c) in s.char_indices() {
        if i > 0 && (c.is_uppercase () || c =='_' ) {
            split_at = i;
            break; 
        }
    }
        // split au bon endroit la string 
    s.truncate(split_at);


    // retourne le premier mot 
    s
}