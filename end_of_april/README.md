# Program Overview: Todo List Manager

A Rust program that demonstrates core Rust concepts through a task/todo management system.

## Concepts Covered

| Concept | File(s) | Description |
|---------|---------|-------------|
| **Structs & Enums** | `types.rs`, `task.rs` | Data types for tasks, priorities, categories |
| **Traits** | `traits.rs` | Shared behavior definitions |
| **Lifetimes** | `task.rs` | References with lifetimes |
| **Borrowing** | `todo.rs`, `main.rs` | Immutable/mutable borrows |
| **Modules** | All files | Import/export between files |
| **Generic Types** | `linked_list.rs` | Generic linked list implementation |

## File Structure

```
src/
├── main.rs        # Entry point - orchestrates everything
├── types.rs       # Enums: Priority, Status, Category
├── traits.rs     # Shared traits: Display, Sortable
├── task.rs       # Task struct with trait implementations
├── todo.rs       # TodoList struct - task management
└── linked_list.rs # Generic linked list for storage
```

## Implementation Order

1. **types.rs** - Define core enums (Priority, Status, Category)
2. **traits.rs** - Define shared traits (Display, Sortable)
3. **linked_list.rs** - Generic singly-linked list `<T>`
4. **task.rs** - Task struct implementing traits
5. **todo.rs** - TodoList using linked list, manages tasks
6. **main.rs** - Demo all functionality

## Key Features

### types.rs
- `Priority` enum: Low, Medium, High
- `Status` enum: Pending, InProgress, Completed
- `Category` enum: Work, Personal, Health, Learning, Other(String)

### traits.rs
- `Display` trait: Formatted output for tasks
- `Sortable` trait: Sorting logic by different criteria

### linked_list.rs
- Generic `Node<T>` and `LinkedList<T>`
- Basic operations: push, get, length, print

### task.rs
- `Task` struct with title, description, priority, status, category
- Lifetimes for description reference
- Implement Display and Sortable traits

### todo.rs
- `TodoList` using `LinkedList<Task>`
- Methods: add, remove, update, list, sort, filter

### main.rs
- Create tasks
- Add to todo list
- Display, sort, filter operations
- Demonstrate borrowing

## Running the Program

```bash
cd end_of_april
cargo run
```