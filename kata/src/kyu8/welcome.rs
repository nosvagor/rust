// Your start-up's BA has told marketing that your website has a large audience
// in Scandinavia and surrounding countries. Marketing thinks it would be great
// to welcome visitors to the site in their own language. Luckily you already
// use an API that detects the user's location, so this is an easy win.

// The Task
//
//     Think of a way to store the languages as a database (eg an object). The
//     languages are listed below so you can copy and paste! Write a 'welcome'
//     function that takes a parameter 'language' (always a string), and returns
//     a greeting - if you have it in your database. It should default to
//     English if the language is not in the database, or in the event of an
//     invalid input.
//
// The Database
//
// ("english", "Welcome"),
// ("czech", "Vitejte"),
// ("danish", "Velkomst"),
// ("dutch", "Welkom"),
// ("estonian", "Tere tulemast"),
// ("finnish", "Tervetuloa"),
// ("flemish", "Welgekomen"),
// ("french", "Bienvenue"),
// ("german", "Willkommen"),
// ("irish", "Failte"),
// ("italian", "Benvenuto"),
// ("latvian", "Gaidits"),
// ("lithuanian", "Laukiamas"),
// ("polish", "Witamy"),
// ("spanish", "Bienvenido"),
// ("swedish", "Valkommen"),
// ("welsh", "Croeso")
//
// Possible invalid inputs include:
//
// IP_ADDRESS_INVALID - not a valid ipv4 or ipv6 ip address
// IP_ADDRESS_NOT_FOUND - ip address not in the database
// IP_ADDRESS_REQUIRED - no ip address was supplied

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref LANGUAGES: HashMap<&'static str, &'static str> = {
        let mut langs = HashMap::new();
        langs.insert("english", "Welcome");
        langs.insert("czech", "Vitejte");
        langs.insert("danish", "Velkomst");
        langs.insert("dutch", "Welkom");
        langs.insert("estonian", "Tere tulemast");
        langs.insert("finnish", "Tervetuloa");
        langs.insert("flemish", "Welgekomen");
        langs.insert("french", "Bienvenue");
        langs.insert("german", "Willkommen");
        langs.insert("irish", "Failte");
        langs.insert("italian", "Benvenuto");
        langs.insert("latvian", "Gaidits");
        langs.insert("lithuanian", "Laukiamas");
        langs.insert("polish", "Witamy");
        langs.insert("spanish", "Bienvenido");
        langs.insert("swedish", "Valkommen");
        langs.insert("welsh", "Croeso");
        langs
    };
}

pub fn greet(language: &str) -> &str {
    LANGUAGES.get(language).unwrap_or(&"Welcome")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }
}
