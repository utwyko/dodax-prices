use clap::{App, Arg};
use scraper::{Html, Selector};

fn main() {
    let matches = App::new("dodax")
        .version("0.0.1")
        .author("Wyko Rijnsburger <utwyko@gmail.com")
        .arg(Arg::with_name("Query")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Query to search"))
        .get_matches();

    let query = matches.value_of("Query").unwrap();

    println!("{}", query);


    let shops = [
        "https://dodax.nl/nl-nl",
        "https://dodax.de/de-de",
        "https://dodax.es/es-es",
        "https://dodax.it/it-it",
        "https://dodax.pl/pl-pl",
        "https://dodax.fr/fr-fr",
        "https://dodax.co.uk/en-gb/"
    ];

    for shop in shops.iter() {
        let url = format!("{}{}{}", shop, "/search/?s=", query);

        let html = reqwest::blocking::get(&url)
            .unwrap()
            .text()
            .unwrap();

        let document = Html::parse_document(&html);

        let buy_buttons = Selector::parse(".buy-button").unwrap();

        let first_buy_button = document.select(&buy_buttons).next().unwrap();
        let txt = first_buy_button.text().collect::<Vec<_>>()[4];
        println!("Shop: {} Price: {:?}", shop, txt);
    }
}
