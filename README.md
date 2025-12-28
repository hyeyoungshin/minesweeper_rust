# Minesweeper in Rust

A functional minesweeper implementation written in Rust, designed with a focus on clean architecture, idiomatic Rust patterns, and software design best practices.

## Overview

This project demonstrates how to build a real game in Rust while maintaining code quality and clarity. It serves as a learning resource for understanding Rust idioms, proper error handling, and separation of concerns in a practical application.

## Features

- Classic minesweeper gameplay with reveal, flag, and unflag actions
- Robust input validation and parsing
- Clean separation between I/O, parsing, and business logic
- Idiomatic Rust error handling using `Result` types
- Interactive command-line interface

## Building & Running

### Prerequisites
- Rust 1.70+ installed
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

## How to Play

1. Start the game and enter board dimensions
2. For each turn, provide:
   - **Coordinates**: `x,y` (comma-separated)
   - **Action**: `Reveal`, `Flag`, or `Unflag`
3. Win by revealing all non-mine tiles
4. Lose by revealing a mine

Example input:
```
Enter coordinates: 3,5
Enter action: Reveal
```

## Design Principles

### Separation of Concerns
- **I/O Layer**: Handles user input in dedicated functions (`get_coordinate`, `get_action`)
- **Parsing Layer**: Converts strings to typed values (`parse_coordinate`, `parse_action`)
- **Validation Layer**: Enforces business rules (`validate_coordinate`, `validate_action`)
- **Game Logic**: Core minesweeper mechanics in the `Game` struct

### Idiomatic Rust Patterns

- **`Result` types** for error handling instead of panics
- **Match expressions** for pattern matching and control flow
- **Iterator methods** (`map`, `collect`, `filter`) for functional programming
- **Enum pattern matching** with exhaustiveness checking
- **Trait implementations** for custom behavior
- **Ownership and borrowing** to prevent memory errors at compile time

### Error Handling

The project uses Rust's `Result` type extensively:
- I/O errors are propagated with the `?` operator
- Parse errors return meaningful error messages
- Validation errors are handled gracefully with retry loops

### Code Organization

```
src/
├── main.rs           # Game loop and user interaction
├── game              
├    └── board.rs     # Board representation
├── game.rs           # Core game logic
└── parse.rs          # Parsing for text-ui based game 
```

## Key Learning Points

This project showcases:

1. **Interactive loops with error recovery** — Prompting users to re-enter data on invalid input
2. **Fisher-Yates shuffling** — For randomly placing mines with guaranteed uniqueness
3. **Type safety** — Leveraging Rust's type system to prevent bugs
4. **Immutable data by default** — Functional approach to state management
5. **Pattern matching** — More expressive than traditional if/else chains

## Dependencies

- `rand` — Random number generation for mine placement

## Future Enhancements

- Difficulty levels (easy, medium, hard)
- Save/load game state
- Configurable board size
- Win/loss statistics
- Graphical interface using a UI framework

## Contributing to Your Learning

This project is designed to evolve as your Rust skills improve. Consider:

- Refactoring with new patterns you learn
- Adding unit tests for game logic
- Implementing the enhancements above
- Comparing your design choices with other Rust game projects

## License

This project is a learning exercise and is free to use and modify.