use todo::{task::Task, tasks::Tasks};

#[test]
fn test_add_task() {
    let mut tasks = Tasks::new();
    let task = Task::new("Test".to_string(), "Testing".to_string()).expect("Failed to create task");
    tasks.add(task);
    assert_eq!(tasks.len(), 1);
}

#[test]
fn test_remove_task() {
    let mut tasks = Tasks::new();
    let task = Task::new("Test".to_string(), "Testing".to_string()).expect("Failed to create task");
    tasks.add(task);
    assert_eq!(tasks.remove(1), Ok(()));
    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_remove_invalid_index() {
    let mut tasks = Tasks::new();
    let task = Task::new("Test".to_string(), "Testing".to_string()).expect("Failed to create task");
    tasks.add(task);
    assert!(tasks.remove(2).is_err());
}

#[test]
fn test_done_task() {
    let mut tasks = Tasks::new();
    let task = Task::new("Test".to_string(), "Testing".to_string()).expect("Failed to create task");
    tasks.add(task);
    tasks.done(1).expect("Failed to mark task as done");
    // Aquí deberías verificar que la tarea está marcada como completada,
    // esto depende de cómo implementes la funcionalidad de completar tareas.
}

#[test]
fn test_done_invalid_index() {
    let mut tasks = Tasks::new();
    let task = Task::new("Test".to_string(), "Testing".to_string()).expect("Failed to create task");
    tasks.add(task);
    assert!(tasks.done(2).is_err());
}
