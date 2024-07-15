// tests/task_management.rs

use todo::{task::Task, tasks::Tasks};

#[test]
fn test_add_task() {
    let mut tasks = Tasks::new();
    tasks.add(Task::new("Test".to_string(), "Testing".to_string()));
    assert_eq!(tasks.len(), 1);
}

#[test]
fn test_remove_task() {
    let mut tasks = Tasks::new();
    tasks.add(Task::new("Test".to_string(), "Testing".to_string()));
    assert_eq!(tasks.remove(1), Ok(()));
    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_remove_invalid_index() {
    let mut tasks = Tasks::new();
    tasks.add(Task::new("Test".to_string(), "Testing".to_string()));
    assert!(tasks.remove(2).is_err());
}

#[test]
fn test_done_task() {
    let mut tasks = Tasks::new();
    tasks.add(Task::new("Test".to_string(), "Testing".to_string()));
    tasks.done(1).unwrap();
    // Aquí deberías verificar que la tarea está marcada como completada,
    // esto depende de cómo implementes la funcionalidad de completar tareas.
}

#[test]
fn test_done_invalid_index() {
    let mut tasks = Tasks::new();
    tasks.add(Task::new("Test".to_string(), "Testing".to_string()));
    assert!(tasks.done(2).is_err());
}