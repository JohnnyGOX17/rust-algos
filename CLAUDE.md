# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

- **Run all tests**: `cargo test`
- **Run a specific test**: `cargo test test_name`
- **Run tests for a specific module**: `cargo test module_name`
- **Build project**: `cargo build`
- **Generate and view documentation**: `cargo doc --open`
- **Check code without building**: `cargo check`

## Architecture Overview

This is a Rust algorithms and data structures learning repository organized into five main modules:

### Module Structure

1. **`leetcode/`** - LeetCode problem solutions, each in its own file named with pattern `problem_name_number.rs` (e.g., `two_sum_1.rs`, `valid_parentheses_20.rs`)

2. **`sorting/`** - Sorting algorithm implementations using a **trait-based architecture**:
   - `sorter.rs` defines the `Sorter<T: Ord + Copy>` trait with `name()` and `sort()` methods
   - Individual algorithms (e.g., `insertion_sort.rs`, `selection_sort.rs`) implement this trait
   - Integration tests in `tests/sorting_tests.rs` use the trait to test all sorters uniformly
   - **Pattern**: When adding a new sorting algorithm:
     1. Create a new file implementing the `Sorter` trait
     2. Add the module to `sorting/mod.rs`
     3. Add the sorter to `get_all_sorters()` in `tests/sorting_tests.rs`

3. **`data_structures/`** - Data structure implementations (hash tables, heaps)

4. **`search/`** - Search algorithm implementations (binary search, etc.)

5. **`interview_questions/`** - Practice interview problems with embedded/low-level focus, including concurrency challenges

### Documentation Standards

- All public modules and implementations use Rust doc comments (`//!` for module-level, `///` for item-level)
- Mathematical notation in docs is supported via KaTeX headers configured in `.cargo/config.toml` and `katex_header.html`
- Use LaTeX notation in doc comments for complexity analysis (e.g., `$O(n^{2})$`)

### Testing Pattern

- Integration tests live in `tests/` directory (e.g., `sorting_tests.rs`)
- Tests use trait-based polymorphism to verify all implementations against the same test cases
- The pattern: clone input, sort with standard library for expected result, then test each algorithm implementation

### Adding New Algorithms

1. Create implementation file in appropriate module directory
2. Implement relevant trait if one exists (e.g., `Sorter` for sorting algorithms)
3. Add module declaration to parent `mod.rs`
4. Add to integration test suite if trait-based testing exists
5. Include doc comments with complexity analysis using LaTeX notation
