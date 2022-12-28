use std::env;
use std::process;
use minigrep_rust_learning::Config;

fn main() {
    
    //? auslesen der Kommandozeile:
    // let args: Vec<String> = env::args().collect(); // collect() erzeugt eine Kollektion mit allen Elementen des Iterators env::args()

    //? parsen der config-Argumente
    let config = Config::build(env::args()).unwrap_or_else(|err|{ // Wenn Result Ok ist, gibt unwrap_or_else den Wert zurück, ansonsten gibt es den Err() Wert an den Rumpf der anonymen Funktion in der nächsten Zeile weiter
        //? AUßERGEWÖHNLICHE Funktionssignatur:  Config::build(&args).unwrap_or_else( |err| {...anonyme Funktion...} );
        // Dies ist ein closure das unwrap_or_else() bei einem Fehler ausführt. Hier ist es auch möglich eine Fehlermeldung ohne panic! auszugeben.
        eprintln!("Fehler beim parsen der Argumente: {err}"); // Hier wird die Fehlermeldung Err() aus der Config::build() auf der Standardfehlerausgabe stderr ausgegeben
        process::exit(1); // Hier wird das Pogramm beendet
    });

    println!("Suche nach {} in Datei {}:", config.query, config.file_path);

    if let Err(e) = minigrep_rust_learning::run(config) { // Weil run() keinen Wert (nur  Einheitswert ()) zurückgibt wenn Result OK, benutzen wir if let statt unwrap_or_else, um zu prüfen, ob run einen Err-Wert zurückgibt und rufen process::exit(1) auf, wenn dies der Fall is
        // Wird nur ausgeführt wenn run() => Result Err zurückgibt
        eprintln!("Anwendungsfehler: {e}");
        process::exit(1);
    }

}

