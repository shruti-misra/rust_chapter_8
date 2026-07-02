# Rust Chapter 8 Exercises

This repository contains solutions to the three practice exercises from Chapter 8 of *The Rust Programming Language* book, focusing on common collections (`Vec<T>`, `String`, and `HashMap<K, V>`).

## Exercises Included

### 1. Vector Statistics (`ex1.rs`)
*   **Description:** Given a list of integers, uses a vector to return the mean (average), median (the middle value when sorted), and mode (the value that occurs most often) of the list.
*   **Key Concepts:** Vector manipulation, sorting, integer division vs. floats, and using a `HashMap` to count element frequencies.

### 2. Pig Latin Converter (`ex2.rs`)
*   **Description:** Converts strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added (e.g., “first” becomes “irst-fay”). Words that start with a vowel have “hay” added to the end instead (e.g., “apple” becomes “apple-hay”).
*   **Key Concepts:** String slicing, `char` iteration, handling UTF-8 boundaries safely, and string building.

### 3. Company Directory CLI (`ex3.rs`)
*   **Description:** A text-based command-line interface (CLI) allowing users to add employees to company departments and retrieve sorted employee lists.
*   **Supported Commands:**
    *   `Add <Name> to <Department>` — Adds a user to a specific department (case-insensitive for departments, retains name casing).
    *   `List <Department>` — Displays an alphabetically sorted list of all employees in that department.
    *   `Get all people` — Displays all departments and their respective employees, sorted alphabetically.
*   **Key Concepts:** `HashMap` entries (`.entry().or_default()`), mutable references inside map vectors, matching user inputs, and input bounds checking.

---

## Upgrades & Fixes

When returning to optimize this codebase, implement the following improvements:

### 1. Command Routing Fix (Exercise 3)
*   **Issue:** Currently, typing `Get Engineering` falls into the `"get"` match block but defaults to asking the user if they meant `Get all people`.
*   **Fix:** Enhance the `"get"` match arm branch logic:
    ```rust
    if words.len() >= 2 && words[1].to_lowercase() == "all" {
        get_all_people(&mut directory);
    } else {
        // Intercept and redirect "Get <department>" directly to list_department
        list_department(&words, &mut directory);
    }
    ```
