# 🔢 Integers in Rust

In Rust, an **integer** is a whole number (no decimal part) that can be positive, negative, or zero. Integers are very common in programming for counting, indexing, and managing data that doesn’t need a fractional part (like 10, -3, 42, etc.).

## 🌟 Types of Integers in Rust

Rust has two main types of integers:

1. **Signed Integers** (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`)
2. **Unsigned Integers** (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`)

### 🧾 What Do "Signed" and "Unsigned" Mean?

- **Signed Integers** can be positive or negative, like `-10`, `0`, or `42`.
- **Unsigned Integers** can only be positive or zero, like `0`, `10`, or `255`.

Think of them like bank balances:

- A **signed integer** is like a bank account that can go negative if you spend more than you have.
- An **unsigned integer** is like cash in your wallet – it can’t be negative; you either have money or none.

## 📏 Integer Sizes

The number after `i` or `u` (like `8`, `16`, `32`, etc.) represents the number of **bits** used to store the integer. More bits mean a larger range of values can be stored.

| Type   | Bits | Signed Range         | Unsigned Range      |
|--------|------|-----------------------|----------------------|
| `i8`   | 8    | -128 to 127          | `u8`: 0 to 255      |
| `i16`  | 16   | -32,768 to 32,767    | `u16`: 0 to 65,535  |
| `i32`  | 32   | -2,147,483,648 to 2,147,483,647 | `u32`: 0 to 4,294,967,295 |
| `i64`  | 64   | Big range! 😅         | `u64`: Even bigger! 😅 |

### Real-World Example

Imagine you’re tracking items in two scenarios:

1. **Signed Integer Example:** Tracking temperature, which can go below zero or above zero.
    - You would use `i8`, `i16`, etc., depending on the range you expect (like `i8` for `-128` to `127`).

    ```rust
    let temperature: i8 = -10; // Temperature can be negative
    println!("Current temperature is {}°C", temperature);
    ```

2. **Unsigned Integer Example:** Tracking the number of items in stock. Since you can’t have a negative count, an unsigned integer makes sense.
    - You would use `u8`, `u16`, etc., depending on the largest count you might need (like `u8` for `0` to `255`).

    ```rust
    let stock: u8 = 200; // Stock count can’t be negative
    println!("Current stock is {}", stock);
    ```

In this way, choosing between signed and unsigned helps make sure we’re using the right type based on whether negative values make sense for our data.

## 🚀 Quick Summary

- **Signed Integers (i)**: Can be positive or negative (e.g., `-5`, `0`, `7`).
- **Unsigned Integers (u)**: Only positive (e.g., `0`, `100`).
- **Size Matters**: Choose the size based on the largest number you need to store (`8`, `16`, `32`, etc.).