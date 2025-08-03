#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
        // if let Media::Book { title, author } = self
        match self {
            Media::Book { title, author } => format!("{} by {}", title, author),
            Media::Movie { title, director } => format!("{} directed by {}", title, director),
            Media::AudioBook { title } => format!("{} audiobook", title),
            Media::Podcast(episode_num) =>  format!("Podcast episode {}", episode_num),
            Media::Placeholder => String::from("Placeholder")
        }
    }
}