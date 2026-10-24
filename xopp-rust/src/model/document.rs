use super::page::{Page, PageBackgroundType};

// Notebook document holding one or more pages
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    // Document title
    pub title: String,
    // Pages ordered sequentially
    pub pages: Vec<Page>,
    // Current active page index
    pub current_page_index: usize,
    // Whether document contains unsaved changes
    pub is_modified: bool,
}

impl Default for Document {
    // Create a new document with an initial ruled A4 page
    fn default() -> Self {
        Self {
            title: String::from("Untitled"),
            pages: vec![Page::new_a4(PageBackgroundType::Ruled)],
            current_page_index: 0,
            is_modified: false,
        }
    }
}

impl Document {
    // Add a new page matching the background format of the current page
    pub fn add_page(&mut self) -> usize {
        let bg = self.current_page().map(|p| p.background.bg_type).unwrap_or(PageBackgroundType::Ruled);
        self.pages.push(Page::new_a4(bg));
        self.current_page_index = self.pages.len() - 1;
        self.is_modified = true;
        self.current_page_index
    }

    // Delete the page at the given index
    pub fn remove_page(&mut self, index: usize) -> bool {
        if self.pages.len() <= 1 || index >= self.pages.len() {
            return false;
        }
        self.pages.remove(index);
        if self.current_page_index >= self.pages.len() {
            self.current_page_index = self.pages.len() - 1;
        }
        self.is_modified = true;
        true
    }

    // Reference to the active page
    pub fn current_page(&self) -> Option<&Page> {
        self.pages.get(self.current_page_index)
    }

    // Mutable reference to the active page
    pub fn current_page_mut(&mut self) -> Option<&mut Page> {
        self.pages.get_mut(self.current_page_index)
    }
}
