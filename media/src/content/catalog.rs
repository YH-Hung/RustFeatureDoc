use super::media::Media;    // super means parent

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> &Media {
        &self.items[index]
    }
}