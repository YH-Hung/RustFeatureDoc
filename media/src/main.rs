use content::catalog::Catalog;
use content::media::Media;

mod content;

fn main() {
    let audiobook = Media::AudioBook { title: String::from("The Hobbit") };
    let book = Media::Book { title: String::from("The wan's life"), author: String::from("Wan") };
    let movie = Media::Movie { title: String::from("The wan's body"), director: String::from("WanWan") };
    let podcast = Media::Podcast(1);

    println!("{}", audiobook.description());
    println!("{}", book.description());
    println!("{}", movie.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);

    let item = catalog.get_by_index(0);
    println!("{:#?}", item);
}
