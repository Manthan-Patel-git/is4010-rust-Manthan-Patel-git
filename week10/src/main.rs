// Week 10: Mastering ownership and borrowing
//
// This lab has two parts:
//
// PART 1 — Borrow checker puzzles (7 problems)
//   Each problem is a function that has a compile error related to ownership or
//   borrowing. Read the comment above each one, fix the bug, then uncomment the
//   call in main() to verify it runs.
//
// PART 2 — Implementation exercises (5 functions)
//   Write functions that demonstrate correct ownership/borrowing patterns.
//   The test suite at the bottom verifies your implementations.
//
// Run: cargo test

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // Uncomment problems one at a time after fixing them:
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
// Error: This function tries to use a value after ownership has moved.
// Fix: Change calculate_length to borrow instead of taking ownership.
//
// Learning goal: Understand move semantics and when to use references.
// ============================================================================

fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");

    // We borrow s1 by adding '&'. s1 stays alive!
    let len = calculate_length(&s1);

    // Now we can use s1 directly here because we still own it
    println!("  The length of '{}' is {}.", s1, len);
}

// We change the parameter to &str (Clippy's favorite)
fn calculate_length(s: &str) -> usize {
    s.len()
    // No need to return 's' back anymore!
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
// Error: Tries to create a mutable borrow while an immutable borrow exists.
// Fix: Ensure immutable borrows are no longer used before creating a mutable borrow.
//
// Learning goal: Understand the "one mutable OR many immutable" rule.
// ============================================================================

fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");

    let r1 = &s;
    println!("  Initial value: {}", r1); // Use r1 here!

    // Now that r1 is finished being used, we can create a mutable borrow
    let r2 = &mut s;
    r2.push_str(", world");

    println!("  Updated value: {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through an immutable reference
// ============================================================================
// Error: Tries to mutate a value through an immutable reference.
// Fix: Change both the variable declaration and function signature to use &mut.
//
// Learning goal: Know when to use &T vs &mut T.
// ============================================================================

fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    // 1. Make the variable itself mutable
    let mut s = String::from("hello");

    // 2. Pass a mutable reference using &mut
    add_to_string(&mut s);

    println!("  Result: {}", s);
}

// 3. Update signature to accept a mutable reference (&mut String)
fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
// Error: Creates two mutable references to the same data simultaneously.
// Fix: Use scopes to limit the lifetime of the first mutable borrow.
//
// Learning goal: Control borrow lifetimes with scopes.
// ============================================================================

fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("  First borrow (r1): {}", r1);
    } // r1 "dies" here, so the lock on 's' is released!

    let r2 = &mut s; // Now this is allowed!
    r2.push('!');

    println!("  Second borrow (r2): {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
// Error: Returns a reference to data that will be dropped at end of scope.
// Fix: Return the owned String instead of a reference.
//
// Learning goal: Prevent use-after-free bugs.
// ============================================================================

fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string(); // r now OWNS the string
    println!("  Got: {}", r);
}

// 1. Change return type from &String to String
fn create_string() -> String {
    // Return the expression directly
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
// Error: Tries to move a value multiple times in a loop.
// Fix: Pass a reference instead of transferring ownership.
//
// Learning goal: Understand ownership with iteration.
// ============================================================================

fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        // 1. Pass a reference instead of the whole String
        print_with_number(&data, i);
    }
}

// 2. Update the function signature to accept &String
fn print_with_number(s: &String, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime — reference doesn't live long enough
// ============================================================================
// Error: The reference outlives the data it points to.
// Fix: Move the String declaration outside the inner scope.
//
// Learning goal: Understand scope and lifetime relationships.
// ============================================================================

fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let result;
    {
        let s = String::from("inner scope");
        // FIX: Transfer ownership by moving 's' into 'result'
        // Remove the '&' ampersand
        result = s;
    }
    // Now result owns the string, so it's safe to print!
    println!("  Result: {}", result);
}

// ============================================================================
// PART 2 — Implementation exercises
//
// Write these functions from scratch using correct ownership/borrowing.
// The tests below verify your implementations.
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
///
/// Demonstrates: move in, transform, move out ("consume and return" pattern).
pub fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a String immutably and returns its length.
///
/// Demonstrates: read-only borrowing.
#[allow(clippy::ptr_arg)]
pub fn string_length(s: &String) -> usize {
    s.len()
}

/// Borrows a String mutably and appends `suffix` to it in place.
///
/// Demonstrates: in-place mutation through a mutable borrow.
pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new owned String by concatenating two borrowed string slices.
///
/// Demonstrates: producing owned data from borrowed inputs.
pub fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        // Original string must still be usable after the borrow.
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
