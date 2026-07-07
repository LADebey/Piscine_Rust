pub struct Student(pub u32, pub String, pub String)


pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    student.1.as_str()
}

pub fn last_name(student: &Student) -> &str {
    student.2.as_str()
}

// .as_str() permet de "créer une petite structure de deux mots en mémoire (une adresse de début et une longueur) qui ici pointe 
// vers le texte déjà existant dans mon String. C'est beaucoup plus light et rapide. "