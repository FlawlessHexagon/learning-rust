# Rust Learning Program

## Progress notation

Each lesson has a stable identifier:

- `M2` means Milestone 2.
- `M2.3` means Milestone 2, Lesson 3.
- “Completed through M2.3” means M2.4 is next.

A lesson is complete when you can explain its central idea and complete its exercise without copying a finished solution.

---

## M0 — Orientation and Tooling

**Objective:** Understand what Rust is and prepare a working development environment.

### M0.1 — Compiled and interpreted languages

Learn:

- Source code, machine code, and executable files
- The practical difference between Rust and Python execution
- Compilation versus interpretation
- Why compiled programs target a particular operating system and processor

### M0.2 — The Rust toolchain

Learn:

- `rustup`: installs and manages Rust toolchains
- `rustc`: compiles Rust source code
- Cargo: creates, builds, tests, and manages projects
- Standard library versus the compiler
- Stable, beta, and nightly toolchains at a high level

### M0.3 — Verify the environment

Practice:

- Check installed tool versions
- Compile and run a minimal program
- Locate generated files
- Read compiler errors instead of immediately trying random fixes

**Milestone project:** Compile a “Hello, world!” program directly with `rustc`.

**Completion check:** Explain the separate jobs of `rustup`, `rustc`, and Cargo.

---

## M1 — From Source Code to Running Process

**Objective:** Build a correct mental model of how Rust code becomes a running native program.

### M1.1 — Rust source files

Learn:

- The `.rs` file extension
- The `main` function as an executable’s entry point
- Statements, expressions, blocks, and semicolons
- Source code as human-readable input to the compiler

### M1.2 — The compilation pipeline

Learn at a conceptual level:

1. Parsing source code
2. Name and type checking
3. Ownership and lifetime checking
4. Generating lower-level code
5. Producing machine code
6. Linking the final program

Do not study compiler algorithms yet.

### M1.3 — Linking

Learn:

- Why compiled pieces must be combined
- Static and dynamic linking at a high level
- External system libraries
- Why a program can compile but fail during linking
- Why binaries differ between operating systems and processor architectures

### M1.4 — Executables and processes

Learn:

- An executable is a file containing machine code and supporting data
- A process is a running instance of an executable
- The operating system loads the executable into memory
- Program arguments, environment variables, standard input, and standard output
- Exit codes

### M1.5 — What an executable contains

Learn conceptually:

- Machine instructions
- Constants and static data
- Metadata needed by the operating system
- References to dynamically linked libraries, when applicable
- Debugging information, when enabled

Also learn what it normally does **not** contain:

- Rust source code in its original form
- Cargo
- The Rust compiler
- A Rust interpreter

### M1.6 — Debug and release builds

Learn:

- Fast compilation versus runtime optimization
- Debug information
- Runtime overflow checks and assertions
- Cargo’s `target/debug` and `target/release` directories
- Why “release” does not automatically mean “production-ready”

**Milestone project:** Build the same program directly with `rustc`, with Cargo in debug mode, and with Cargo in release mode. Compare the files and behavior.

**Completion check:** Describe the complete journey from saving `main.rs` to the operating system running the resulting process.

---

## M2 — Syntax, Values, and Types

**Objective:** Read and write small Rust programs without yet tackling the full ownership model.

### M2.1 — Variables and mutability

Learn:

- `let`
- Immutable-by-default variables
- `mut`
- Constants
- Shadowing
- Scope

### M2.2 — Scalar types

Learn:

- Signed and unsigned integers
- Floating-point numbers
- Booleans
- Characters
- Numeric type annotations
- Basic conversions with `as`, including its limitations

### M2.3 — Compound types and strings

Learn:

- Tuples
- Arrays
- Array indexing
- String slices (`&str`)
- Owned strings (`String`) at an introductory level
- Why Rust strings are not simple character arrays

### M2.4 — Functions

Learn:

- Parameters and return types
- Function signatures
- Explicit returns
- Returning a final expression
- The unit type `()`

### M2.5 — Expressions, statements, and blocks

Learn:

- Why `5` and `5;` behave differently
- Blocks as expressions
- Assigning a block’s result
- Why this design matters throughout Rust

### M2.6 — Basic input and output

Learn:

- `println!`
- Formatting placeholders
- Reading command-line arguments
- Basic standard input
- Parsing strings into numbers

**Milestone project:** Build a temperature or unit converter.

**Completion check:** Write functions with typed inputs and outputs and explain where semicolons belong.

---

## M3 — Control Flow and Basic Problem Solving

**Objective:** Express decisions, repetition, and simple algorithms idiomatically.

### M3.1 — Conditions

Learn:

- `if`, `else if`, and `else`
- Conditions must be Boolean
- `if` as an expression
- Branches must produce compatible types

### M3.2 — Loops

Learn:

- `loop`
- `while`
- `for`
- Ranges
- `break` and `continue`
- Returning a value from `loop`

### M3.3 — Basic pattern matching

Learn:

- `match`
- Exhaustiveness
- Catch-all patterns
- `if let`
- `while let`

Pattern matching will be revisited after enums.

### M3.4 — Small algorithmic exercises

Practice:

- FizzBuzz
- Number guessing
- Running totals
- Finding minimum and maximum values
- Counting occurrences

The objective is fluency with syntax, not sophisticated algorithms.

**Milestone project:** Build an interactive number-guessing game.

**Completion check:** Choose appropriately among `if`, `match`, `for`, `while`, and `loop`.

---

## M4 — Memory, Ownership, and Borrowing

**Objective:** Understand the central mechanism that distinguishes Rust from Python and garbage-collected languages.

### M4.1 — Stack and heap

Learn:

- Stack frames and local variables
- Dynamically sized heap allocations
- Why `String` and `Vec<T>` use heap storage
- Scope and destruction
- This model is useful but simplified; compiler optimizations may change physical placement

### M4.2 — Ownership

Learn:

- Every value has an owner
- Only one owner at a time in the ordinary case
- Values are dropped when their owner leaves scope
- Resource management through deterministic destruction

### M4.3 — Moving values

Learn:

- Assignment can move ownership
- Passing a value can move it
- Returning a value transfers ownership
- Why Rust prevents use after move

### M4.4 — Borrowing and references

Learn:

- Shared references: `&T`
- Mutable references: `&mut T`
- Dereferencing
- Borrowing instead of transferring ownership
- The rule preventing simultaneous mutation and unsafe access

### M4.5 — Copying and cloning

Learn:

- `Copy` for inexpensive implicit duplication
- `Clone` for explicit duplication
- Why heap-owning values such as `String` are not normally `Copy`
- Why cloning everything avoids learning the real ownership model

### M4.6 — Slices

Learn:

- String slices
- Array and vector slices
- A slice as a borrowed view into existing data
- Validity of a slice depends on its source

### M4.7 — Lifetimes

Learn:

- Lifetimes describe relationships between references
- Most lifetimes are inferred
- Lifetime annotations do not keep data alive
- Why functions returning references sometimes need annotations
- Avoid advanced lifetime puzzles at this stage

### M4.8 — Destruction and RAII

Learn:

- The `Drop` trait conceptually
- Files, locks, and memory as owned resources
- Cleanup when values leave scope
- Why Rust does not require a tracing garbage collector for ordinary memory management

**Milestone project:** Build text-analysis functions that borrow input rather than consuming or unnecessarily cloning it.

**Completion check:** Predict whether a value is moved, copied, mutably borrowed, or immutably borrowed.

---

## M5 — Modeling Data with Structs, Enums, and Patterns

**Objective:** Represent real concepts using Rust’s type system.

### M5.1 — Structs

Learn:

- Named-field structs
- Tuple structs
- Unit structs
- Construction and field access
- Struct update syntax

### M5.2 — Methods and associated functions

Learn:

- `impl` blocks
- `self`, `&self`, and `&mut self`
- Associated functions such as constructors
- Multiple `impl` blocks

### M5.3 — Enums

Learn:

- Variants
- Variants containing data
- How Rust enums differ from simple numeric enums
- Modeling a fixed set of valid states

### M5.4 — `Option<T>`

Learn:

- Representing the presence or absence of a value
- `Some` and `None`
- Why Rust does not use ordinary null references
- Matching, `if let`, and common helper methods

### M5.5 — Advanced pattern matching

Learn:

- Destructuring structs and enums
- Nested patterns
- Match guards
- Binding values with patterns
- Ignoring parts of a value

**Milestone project:** Model a small task tracker using structs and enums.

**Completion check:** Use types to make invalid states difficult or impossible to represent.

---

## M6 — Collections, Iteration, Closures, and Errors

**Objective:** Process groups of values and handle expected failure safely.

### M6.1 — Vectors

Learn:

- Creating and updating `Vec<T>`
- Indexing versus `.get()`
- Iteration
- Borrowing elements
- How vector reallocation affects references

### M6.2 — Strings

Learn:

- UTF-8 representation
- Appending and formatting
- Why direct numeric indexing is unavailable
- Bytes versus Unicode scalar values versus user-perceived characters

### M6.3 — Hash maps

Learn:

- Keys and values
- Insertion and lookup
- Entry-based updates
- Ownership of inserted values

### M6.4 — Iterators

Learn:

- `iter`, `iter_mut`, and `into_iter`
- Iterator adaptors such as `map` and `filter`
- Consumers such as `collect` and `sum`
- Lazy evaluation
- Ownership differences among iterator forms

### M6.5 — Closures

Learn:

- Closure syntax
- Capturing surrounding values
- Capturing by shared borrow, mutable borrow, or ownership
- Using closures with iterators

### M6.6 — Recoverable errors

Learn:

- `Result<T, E>`
- `Ok` and `Err`
- Pattern matching on results
- The `?` operator
- Propagating errors
- Adding useful context

### M6.7 — Panics

Learn:

- Panic as an unrecoverable failure mechanism
- When panic is and is not appropriate
- `unwrap` and `expect`
- Why library code should generally avoid unnecessary panics

**Milestone project:** Read a text file and report word frequencies with useful error messages.

**Completion check:** Distinguish absence, recoverable failure, and unrecoverable programmer errors.

---

## M7 — Traits, Generics, and Reusable Abstractions

**Objective:** Understand Rust’s primary tools for reusable, type-safe code.

### M7.1 — Generics

Learn:

- Generic functions
- Generic structs and enums
- Type parameters
- Monomorphization at a high level

### M7.2 — Traits

Learn:

- Defining shared behavior
- Implementing traits
- Default methods
- Trait bounds
- Traits compared with Python protocols or interfaces

### M7.3 — Common standard traits

Learn:

- `Debug`
- `Display`
- `Clone`
- `Copy`
- `PartialEq` and `Eq`
- `Default`
- `From` and `Into`

### M7.4 — Derived implementations

Learn:

- The `derive` attribute
- Generated trait implementations
- When deriving is appropriate

### M7.5 — Trait objects

Learn at an introductory level:

- Static dispatch with generics
- Dynamic dispatch with `dyn Trait`
- Basic object-safety constraints
- When each approach is useful

**Milestone project:** Define multiple data types sharing behavior through a trait.

**Completion check:** Explain the difference between a generic trait bound and a trait object.

---

## M8 — Organizing Real Rust Projects

**Objective:** Move from isolated examples to maintainable applications and libraries.

### M8.1 — Cargo packages and crates

Learn:

- A package as a Cargo-managed project
- Binary crates
- Library crates
- Crate roots
- One package containing multiple binaries

### M8.2 — Modules and visibility

Learn:

- `mod`
- `use`
- Public and private items
- Module trees
- Separating modules into files
- Designing a small public interface

### M8.3 — `Cargo.toml`

Learn:

- Package metadata
- Dependencies
- Semantic version requirements
- Features at an introductory level
- Build profiles

### M8.4 — External dependencies

Learn:

- Finding crates
- Reading crate documentation
- Evaluating maintenance and suitability
- `Cargo.lock`
- Reproducible builds
- Supply-chain awareness

### M8.5 — Libraries and executables

Learn:

- The purpose of `src/lib.rs`
- The purpose of `src/main.rs`
- Keeping reusable logic in a library
- Keeping command-line coordination in a binary

### M8.6 — Workspaces

Learn at an introductory level:

- Multiple related packages
- Shared dependency resolution
- When a workspace is useful
- Why small projects do not need one immediately

**Milestone project:** Refactor an earlier program into a library crate plus a small binary crate.

**Completion check:** Explain package, crate, module, library, and binary without treating them as synonyms.

---

## M9 — Testing, Documentation, and Development Quality

**Objective:** Learn the standard workflow used in real Rust projects.

### M9.1 — Unit tests

Learn:

- `#[test]`
- Assertions
- Testing private implementation details when appropriate
- Expected panics

### M9.2 — Integration tests

Learn:

- The `tests` directory
- Testing through a library’s public interface
- Unit versus integration tests

### M9.3 — Documentation

Learn:

- Documentation comments
- Markdown in documentation
- Documentation tests
- Generating and reading local API documentation

### M9.4 — Formatting and linting

Learn:

- Automatic formatting
- Compiler warnings
- Clippy
- Why lints are guidance rather than unquestionable laws

### M9.5 — Debugging

Learn:

- Reading compiler diagnostics systematically
- Printing temporary state
- Backtraces
- Using a debugger at an introductory level
- Reducing a failure to a minimal example

### M9.6 — Build and dependency checks

Learn:

- Checking without producing a final executable
- Building all targets
- Running the full test suite
- Auditing dependency choices conceptually

**Milestone project:** Add tests, documentation, and clean linting to the M8 project.

**Completion check:** Produce a formatted, documented project whose tests pass without ignored compiler warnings.

---

## M10 — Concurrency, Async, and Unsafe Boundaries

**Objective:** Survey Rust’s more advanced systems capabilities without prematurely specializing.

### M10.1 — Threads

Learn:

- Creating and joining threads
- Moving values into threads
- Why ownership helps prevent data races

### M10.2 — Shared state and message passing

Learn:

- Channels
- Mutexes
- Atomic reference counting with `Arc`
- Lock poisoning at a high level
- Deadlocks are still possible

### M10.3 — `Send` and `Sync`

Learn conceptually:

- Types safe to transfer between threads
- Types safe to share between threads
- How the compiler enforces thread-safety constraints

### M10.4 — Async Rust

Learn at a survey level:

- Futures
- `.async` and `.await`
- Executors and runtimes
- Why async is useful for large amounts of waiting work
- Why async does not automatically make computation faster

### M10.5 — Unsafe Rust

Learn:

- Safe Rust is built on carefully checked unsafe foundations
- What `unsafe` permits
- `unsafe` disables only certain compiler checks
- The programmer must uphold explicit safety invariants
- Avoid writing substantial unsafe code at this stage

### M10.6 — Foreign-function interfaces

Learn conceptually:

- Calling native libraries
- C-compatible interfaces
- Memory ownership across language boundaries
- Why FFI usually introduces an unsafe boundary

**Milestone project:** Build a small threaded program that sends messages back to the main thread.

**Completion check:** Explain what Rust prevents in concurrent code and what problems remain possible.

---

## M11 — Deployment and the Rust Ecosystem

**Objective:** Understand how Rust programs leave the development machine and where they fit.

### M11.1 — Platform targets

Learn:

- Operating-system and processor targets
- Native compilation
- Cross-compilation
- Why one executable does not necessarily run everywhere

### M11.2 — Distributing applications

Learn:

- Shipping executables
- Runtime library dependencies
- Configuration and data files
- Exit codes and logs
- Release artifacts
- Basic size and performance considerations

### M11.3 — Command-line applications

Learn:

- Arguments and flags
- Standard streams
- Pipelines
- Meaningful error output
- Exit status conventions

### M11.4 — Backend and networking software

Survey:

- HTTP services
- Network protocols
- Synchronous versus asynchronous I/O
- Reliability and resource usage

### M11.5 — Systems and embedded software

Survey:

- Operating-system components
- Device drivers
- Firmware
- Environments without the full standard library
- Why control over memory and runtime behavior matters

### M11.6 — WebAssembly

Survey:

- Compiling Rust to WebAssembly
- Browser and non-browser runtimes
- Interaction with JavaScript
- Limitations of the execution environment

### M11.7 — Python and ML integration

Survey:

- Calling Rust from Python through extension modules
- Passing arrays and buffers across the boundary
- Using Rust for CPU-intensive preprocessing or inference support
- Measuring before replacing Python code
- Rust does not replace established GPU-oriented ML frameworks by default

**Milestone project:** Package and run a release build outside its source directory, documenting everything it needs at runtime.

**Completion check:** Explain where Rust is a strong choice and where another language may be more practical.

---

## M12 — Final Capstone

**Objective:** Combine the language, tooling, memory model, testing, and deployment knowledge in one complete project.

Choose one:

- A command-line data-processing application
- A multithreaded file indexer
- A small HTTP service
- A reusable Rust library
- A Rust-backed Python extension for a measured CPU bottleneck
- A small WebAssembly module

The project must include:

- A library and executable where appropriate
- Clear module boundaries
- At least one external dependency chosen deliberately
- `Result`-based error handling
- Tests
- Documentation
- Clean formatting and linting
- A release build
- A written explanation of ownership decisions
- Instructions for building and running the result

**Program completion check:** Explain how the source becomes an executable, how Rust manages resources safely, how the project is organized, how correctness is checked, and how the final artifact is distributed.

---

# Recommended progression rules

1. Complete milestones in order through M9.
2. Do not wait for complete theoretical mastery before writing programs.
3. Do not solve ownership problems by adding `clone()` everywhere.
4. Treat compiler messages as part of the learning material.
5. Revisit M1 after M4 and M8; compilation becomes clearer once types, ownership, crates, and dependencies are familiar.
6. Begin specialization only after M9. M10 and M11 provide the vocabulary needed to choose a direction.
7. Keep each milestone project in its own directory so earlier work remains available for comparison.
