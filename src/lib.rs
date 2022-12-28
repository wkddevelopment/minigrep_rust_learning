use std::fs;
use std::error::Error;


#[cfg(test)]
mod test {
    use super::*;

    // schreiben eines Tests
    #[test]
    fn one_result(){
        let query = "dukt";
        let contents = "\
Rust: 
sicher, schnell, produktiv.
Nimm drei.";

        assert_eq!(vec!["sicher, schnell, produktiv."], search(query, contents)); // prüft ob search aufgrund der des querys "dukt" die zweite Zeile in contents findet und zurückgibt
    }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    /*
    let mut results = Vec::new(); // Erstellen eines veränderlichen Vektors zum halten der Resultate
    for line in contents.lines(){ // Mit lines kann man Zeile für Zeile über Zeichenketten iterieren- Sie gibt einen Iterator zurück
        if line.contains(query){ // contains() prüft ob das Argument vorkommt...
            results.push(line); // ...wenn es vorkommt, dann push() die ganze line in den result Vector
        }
    }

    results
    */

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>, // args kann jeder Typ sein der Iterator implementiert und String zurück gibt (Merkmalsabgrenzung)
    ) -> Result<Config, &'static str>{ // Implementieren einer Build/Konstruktor Funktion für Config
        /* alter code...
        // Fehlerprüfung auf Eingabe von 2 Umgebungsvariablen:
        if args.len() < 3 {
            return Err("Nicht genügend Argumente eingegeben (2 Argumente benötigt): Das erste Argument ist der Suchbegriff, das zweite der Dateipfad!");
        }
        // sideinfo: let name_dieser_binaerdatei = &args[0] ---> target/debug/minigrep_rust_learning; Die Umgebungsvariablen sind: &args[0] == Name Binärdatei, args[1] == Suchbegriff & args[2] == Dateipfad
        let query = args[1].clone(); // Suchbegriff... clone() damit Eigentümerschaft des Wertes auf Variable query übergeht -> etwas zient
        let file_path = args[2].clone(); // Dateipfad zur Datei in der gesucht werden soll
        Ok(Config { query, file_path }) // return des Result Wertes Ok mit der Struktur Config mit den geparsten Argumenten aus der Kommandozeile als &str (Zeichenkettenanteilstyp)
        */

        args.next(); // Der erste Rückgabewert von args ist der Name des Programms, den wir nicht brauchen, deswegen gleich next()
        
        let query = match args.next() { // Mit arg.next() rufen wir den nächsten Wert, ist er Some speichern wir ihn in der Variable, ist er None ist die Argumentsliste zu kurz
            Some(arg) => arg,
            None => return Err("Keine Abfragezeichenkette erhalten"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Keinen Dateinamen erhalten"),
        };

        //let ignore_case = env::var("IGNORE_CASE").is_err();

        Ok(Config {
            query,
            file_path,
        })



    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ // Box<dyn Error> bedeutet, dass die Funktion einen Typ zurückgibt, der das Merkmal Error implementiert, aber wir müssen nicht angeben, welcher bestimmte Typ der Rückgabewert sein wird. 
    
    //? Datei einlesen
    let contents = fs::read_to_string(config.file_path)?; // Abfangen eines Fehlers beim Lesen der Datei, ? gibt den Fehlerwert aus der aktuellen Funktion zurück

    for line in search(&config.query, &contents){
        println!("{line}");
    }

    Ok(()) // Wir haben den Erfolgstyp der Funktion run mit () in der Signatur deklariert, was bedeutet, dass wir den Wert des Einheitstyps in den Wert Ok einpacken müssen. Diese Syntax Ok((()) mag zunächst etwas merkwürdig aussehen, aber wenn wir () so verwenden, ist das der idiomatische Weg, um anzuzeigen, dass wir run nur wegen seiner Nebenwirkungen aufrufen; es gibt keinen Wert zurück, den wir brauchen

}
