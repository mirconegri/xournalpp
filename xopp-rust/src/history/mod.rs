use crate::model::Stroke;

// Undoable action applied to the document
#[derive(Debug, Clone)]
pub enum Action {
    // A new stroke was drawn onto a page layer
    AddStroke { page_idx: usize, layer_idx: usize, stroke: Stroke },
    // A stroke was removed from a page layer
    RemoveStroke { page_idx: usize, layer_idx: usize, stroke: Stroke, stroke_idx: usize },
}

// Stack managing undo and redo history
#[derive(Debug, Default)]
pub struct HistoryManager {
    // Undo stack
    undo_stack: Vec<Action>,
    // Redo stack
    redo_stack: Vec<Action>,
}

impl HistoryManager {
    // Record a new action and clear the redo stack
    pub fn push(&mut self, action: Action) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }

    // Pop the most recent action for undoing
    pub fn pop_undo(&mut self) -> Option<Action> {
        let action = self.undo_stack.pop();
        if let Some(ref a) = action {
            self.redo_stack.push(a.clone());
        }
        action
    }

    // Pop the most recent undone action for redoing
    pub fn pop_redo(&mut self) -> Option<Action> {
        let action = self.redo_stack.pop();
        if let Some(ref a) = action {
            self.undo_stack.push(a.clone());
        }
        action
    }

    // Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    // Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
