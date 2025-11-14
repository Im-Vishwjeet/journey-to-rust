# Variable Bindings

This chapter covers the fundamental concepts of variable bindings in Rust, including mutability, scoping, shadowing, declare-first patterns, and freezing.

> **Note:** The code in this project is organized using Rust modules for better structure. Modules will be covered in detail in a further chapter. For now, you can consider each module file (e.g., `3.1_mutability.rs`, `3.2_shadowing.rs`, etc.) as if it were a separate main file demonstrating a specific concept.

## Topics Covered

### 1. Mutability (`3.1_mutability.rs`)

**Mutability** is the ability to change the value of a variable after it's been declared.

- By default, all variables in Rust are **immutable** - once a value is bound to a name, it cannot be changed
- To make a variable mutable, use the `mut` keyword
- Immutability helps prevent bugs and makes code safer by default

```rust
let immutable_binding = 1;  // Cannot be changed
let mut mutable_binding = 1; // Can be changed
mutable_binding += 1; // ✓ Allowed
immutable_binding += 1; // ✗ Error!
```

### 2. Scoping and Shadowing (`3.2_shadowing.rs`)

**Scoping** refers to where a variable is valid and accessible. In Rust, variables are scoped to the block (code within curly braces `{}`) where they are declared.

**Shadowing** is the ability to declare a new variable with the same name as a previous variable, effectively "shadowing" the previous one. This is different from mutability:
- Shadowing creates a completely new variable (can even change the type)
- The shadowed variable is only valid within its scope
- Once you leave the scope, the original variable becomes visible again

```rust
let shadowed_binding = 1;
{
    let shadowed_binding = "abc"; // Shadows the outer binding
    println!("{}", shadowed_binding); // Prints "abc"
} // Inner binding goes out of scope
println!("{}", shadowed_binding); // Prints 1 (original binding)
```

### 3. Declare First (`3.3_declare_first.rs`)

**Declare first** is a pattern where you declare a variable without initializing it, and then initialize it later. This is useful when:
- The initialization depends on conditions
- You want to initialize a variable in a different scope
- You need to ensure a variable is initialized before use

Important rules:
- You must initialize a variable before using it
- The compiler enforces this at compile time

```rust
let a_binding; // Declaration
{
    let x = 2;
    a_binding = x * x; // Initialization
} // x goes out of scope, but a_binding remains valid
println!("{}", a_binding); // ✓ Allowed
```

### 4. Freezing (`3.4_freezing.rs`)

**Freezing** occurs when a mutable variable is shadowed by an immutable binding. When this happens:
- The original mutable variable becomes "frozen" (cannot be mutated) while the shadowing binding is in scope
- Once the shadowing binding goes out of scope, the original mutable variable becomes mutable again

This is a safety feature that prevents accidental mutations when you intentionally create an immutable shadow.

```rust
let mut mutable_integer = 7;
{
    let mutable_integer = 4; // Shadow with immutable binding
    mutable_integer = 50; // ✗ Error! Frozen in this scope
} // Shadow goes out of scope
mutable_integer = 3; // ✓ Allowed! Original is mutable again
```

## Running the Code

To run all examples:

```bash
cargo run
```

To run a specific example, you can uncomment the relevant function call in `src/main.rs`.

## File Structure

```
src/
├── main.rs                 # Entry point that calls all examples
└── modules/
    ├── mod.rs             # Module declarations
    ├── 3.1_mutability.rs  # Mutability examples
    ├── 3.2_shadowing.rs   # Scoping and shadowing examples
    ├── 3.3_declare_first.rs # Declare-first pattern examples
    └── 3.4_freezing.rs    # Freezing examples
```

## Key Takeaways

1. **Immutability by default** makes Rust code safer and prevents many bugs
2. **Shadowing** allows you to reuse variable names and even change types
3. **Scope** determines where a variable is accessible
4. **Declare-first** pattern provides flexibility in variable initialization
5. **Freezing** protects mutable variables when they're shadowed by immutable bindings

