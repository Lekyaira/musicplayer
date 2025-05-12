use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_mutex_thread_safety() -> Result<()> {
    // Create a shared state
    let counter = Arc::new(Mutex::new(0));
    let finished = Arc::new(Mutex::new(false));
    
    // Clone for thread
    let counter_clone = Arc::clone(&counter);
    let finished_clone = Arc::clone(&finished);
    
    // Spawn a thread that increments the counter
    let handle = thread::spawn(move || -> Result<()> {
        if let Ok(mut count) = counter_clone.lock() {
            *count += 5;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to lock counter in thread"))
        }
    });
    
    // Wait for thread to complete
    handle.join().unwrap()?;
    
    // Check the counter value
    let value = if let Ok(count) = counter.lock() {
        *count
    } else {
        return Err(anyhow::anyhow!("Failed to lock counter in main thread"));
    };
    
    assert_eq!(value, 5, "Counter should be incremented by 5");
    
    // Set finished flag
    if let Ok(mut flag) = finished.lock() {
        *flag = true;
    } else {
        return Err(anyhow::anyhow!("Failed to lock finished flag"));
    }
    
    // Check finished flag
    let is_finished = if let Ok(flag) = finished.lock() {
        *flag
    } else {
        return Err(anyhow::anyhow!("Failed to lock finished flag for checking"));
    };
    
    assert!(is_finished, "Finished flag should be true");
    
    Ok(())
} 