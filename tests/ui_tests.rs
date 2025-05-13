use std::time::{Duration, Instant};

// This test mocks the notification system from the UI
struct NotificationSystem {
    notification: Option<(String, Instant)>, // (message, time shown)
}

impl NotificationSystem {
    fn new() -> Self {
        Self {
            notification: None,
        }
    }
    
    fn show_notification(&mut self, message: &str) {
        self.notification = Some((message.to_string(), Instant::now()));
    }
    
    fn has_active_notification(&self) -> bool {
        if let Some((_, time)) = &self.notification {
            time.elapsed() < Duration::from_secs(3)
        } else {
            false
        }
    }
    
    fn get_notification_text(&self) -> Option<String> {
        if let Some((text, _)) = &self.notification {
            Some(text.clone())
        } else {
            None
        }
    }
    
    // Mock the update logic to clear expired notifications
    fn update(&mut self) {
        if let Some((_, time)) = &self.notification {
            if time.elapsed() >= Duration::from_secs(3) {
                self.notification = None;
            }
        }
    }
}

#[test]
fn test_notification_system() {
    let mut notification_system = NotificationSystem::new();
    
    // Initially no notification
    assert!(!notification_system.has_active_notification());
    assert_eq!(notification_system.get_notification_text(), None);
    
    // Show notification
    notification_system.show_notification("Test notification");
    
    // Notification should be active
    assert!(notification_system.has_active_notification());
    assert_eq!(notification_system.get_notification_text(), Some("Test notification".to_string()));
    
    // Simulate update shortly after - notification should still be active
    notification_system.update();
    assert!(notification_system.has_active_notification());
    
    // Override with new notification
    notification_system.show_notification("New notification");
    assert!(notification_system.has_active_notification());
    assert_eq!(notification_system.get_notification_text(), Some("New notification".to_string()));
}

#[test]
fn test_notification_expiration() {
    let mut notification_system = NotificationSystem::new();
    
    // Show notification with a mocked old timestamp
    let three_seconds_ago = Instant::now() - Duration::from_secs(3);
    notification_system.notification = Some(("Expired notification".to_string(), three_seconds_ago));
    
    // Before updating, notification data exists but is expired
    assert!(!notification_system.has_active_notification());
    assert_eq!(notification_system.get_notification_text(), Some("Expired notification".to_string()));
    
    // After update, notification should be cleared
    notification_system.update();
    assert!(!notification_system.has_active_notification());
    assert_eq!(notification_system.get_notification_text(), None);
} 