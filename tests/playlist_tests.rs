use std::path::PathBuf;

// Mock implementation to test playlist management without GUI dependencies
struct PlaylistManager {
    playlist: Vec<PathBuf>,
    current_index: Option<usize>,
}

impl PlaylistManager {
    fn new() -> Self {
        Self {
            playlist: Vec::new(),
            current_index: None,
        }
    }
    
    fn add_item(&mut self, path: PathBuf) {
        self.playlist.push(path);
        // If this is the first item, set it as current
        if self.playlist.len() == 1 {
            self.current_index = Some(0);
        }
    }
    
    fn remove_item(&mut self, index: usize) -> bool {
        if index >= self.playlist.len() {
            return false;
        }
        
        // Update current index if needed
        if let Some(current) = self.current_index {
            if index == current {
                // If removing current item, move to next or previous
                if self.playlist.len() > 1 {
                    if index == self.playlist.len() - 1 {
                        self.current_index = Some(index - 1);
                    }
                    // Otherwise current index stays the same but will point to the next item
                } else {
                    // If removing the only item
                    self.current_index = None;
                }
            } else if index < current {
                // If removing an item before current, decrement current index
                self.current_index = Some(current - 1);
            }
        }
        
        self.playlist.remove(index);
        true
    }
    
    fn move_up(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.playlist.len() {
            return false;
        }
        
        // Swap with previous item
        self.playlist.swap(index, index - 1);
        
        // Update current index if affected
        if let Some(current) = self.current_index {
            if current == index {
                self.current_index = Some(current - 1);
            } else if current == index - 1 {
                self.current_index = Some(current + 1);
            }
        }
        
        true
    }
    
    fn move_down(&mut self, index: usize) -> bool {
        if index >= self.playlist.len() - 1 {
            return false;
        }
        
        // Swap with next item
        self.playlist.swap(index, index + 1);
        
        // Update current index if affected
        if let Some(current) = self.current_index {
            if current == index {
                self.current_index = Some(current + 1);
            } else if current == index + 1 {
                self.current_index = Some(current - 1);
            }
        }
        
        true
    }
    
    fn next_item(&mut self) -> Option<&PathBuf> {
        if self.playlist.is_empty() {
            return None;
        }
        
        let next_index = if let Some(current) = self.current_index {
            if current + 1 < self.playlist.len() {
                Some(current + 1)
            } else {
                None // End of playlist
            }
        } else if !self.playlist.is_empty() {
            Some(0) // Start of playlist
        } else {
            None // Empty playlist
        };
        
        self.current_index = next_index;
        
        if let Some(index) = self.current_index {
            Some(&self.playlist[index])
        } else {
            None
        }
    }
    
    fn current_item(&self) -> Option<&PathBuf> {
        if let Some(index) = self.current_index {
            self.playlist.get(index)
        } else {
            None
        }
    }
}

#[test]
fn test_add_items() {
    let mut manager = PlaylistManager::new();
    assert_eq!(manager.playlist.len(), 0);
    
    manager.add_item(PathBuf::from("file1.mp3"));
    assert_eq!(manager.playlist.len(), 1);
    assert_eq!(manager.current_index, Some(0));
    
    manager.add_item(PathBuf::from("file2.mp3"));
    assert_eq!(manager.playlist.len(), 2);
    assert_eq!(manager.current_index, Some(0)); // Current index should stay on first item
}

#[test]
fn test_remove_items() {
    let mut manager = PlaylistManager::new();
    
    // Add three items
    manager.add_item(PathBuf::from("file1.mp3"));
    manager.add_item(PathBuf::from("file2.mp3"));
    manager.add_item(PathBuf::from("file3.mp3"));
    
    // Remove middle item
    assert!(manager.remove_item(1));
    assert_eq!(manager.playlist.len(), 2);
    assert_eq!(manager.current_index, Some(0)); // Current index unchanged
    assert_eq!(manager.current_item().unwrap(), &PathBuf::from("file1.mp3"));
    
    // Remove first item (current)
    assert!(manager.remove_item(0));
    assert_eq!(manager.playlist.len(), 1);
    assert_eq!(manager.current_index, Some(0)); // Current index now points to the next item
    assert_eq!(manager.current_item().unwrap(), &PathBuf::from("file3.mp3"));
    
    // Remove last item
    assert!(manager.remove_item(0));
    assert_eq!(manager.playlist.len(), 0);
    assert_eq!(manager.current_index, None); // No current index with empty playlist
    assert_eq!(manager.current_item(), None);
    
    // Try to remove from empty playlist
    assert!(!manager.remove_item(0));
}

#[test]
fn test_move_items() {
    let mut manager = PlaylistManager::new();
    
    manager.add_item(PathBuf::from("file1.mp3"));
    manager.add_item(PathBuf::from("file2.mp3"));
    manager.add_item(PathBuf::from("file3.mp3"));
    
    // Can't move first item up
    assert!(!manager.move_up(0));
    
    // Can't move last item down
    assert!(!manager.move_down(2));
    
    // Move middle item up
    assert!(manager.move_up(1));
    assert_eq!(manager.playlist[0], PathBuf::from("file2.mp3"));
    assert_eq!(manager.playlist[1], PathBuf::from("file1.mp3"));
    
    // Move last item up
    assert!(manager.move_up(2));
    assert_eq!(manager.playlist[1], PathBuf::from("file3.mp3"));
    assert_eq!(manager.playlist[2], PathBuf::from("file1.mp3"));
    
    // Move first item down
    assert!(manager.move_down(0));
    assert_eq!(manager.playlist[0], PathBuf::from("file3.mp3"));
    assert_eq!(manager.playlist[1], PathBuf::from("file2.mp3"));
}

#[test]
fn test_navigation() {
    let mut manager = PlaylistManager::new();
    
    // Test with empty playlist
    assert_eq!(manager.next_item(), None);
    
    // Add some items
    manager.add_item(PathBuf::from("file1.mp3"));
    manager.add_item(PathBuf::from("file2.mp3"));
    manager.add_item(PathBuf::from("file3.mp3"));
    
    // Test current item
    assert_eq!(manager.current_item().unwrap(), &PathBuf::from("file1.mp3"));
    
    // Test next item
    assert_eq!(manager.next_item().unwrap(), &PathBuf::from("file2.mp3"));
    assert_eq!(manager.current_index, Some(1));
    
    // Test next item again
    assert_eq!(manager.next_item().unwrap(), &PathBuf::from("file3.mp3"));
    assert_eq!(manager.current_index, Some(2));
    
    // Test end of playlist
    assert_eq!(manager.next_item(), None);
    assert_eq!(manager.current_index, None);
} 