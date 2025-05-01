# scadman

`scadman` is a Rust library for generating OpenSCAD code programmatically. It provides a
type-safe and structured way to define 2D and 3D geometric objects, apply transformations
and operations, and output valid OpenSCAD code.

## What is this library?

Instead of writing OpenSCAD code directly in `.scad` files, `scadman` allows you to define
your parametric models using Rust code. This approach leverages Rust's strong typing,
module system, testing capabilities, and build tools to manage your designs.

The library represents OpenSCAD primitives, modifiers, and blocks as distinct Rust types,
wrapped within a unified `ScadObject` structure. It provides traits and helper functions
to build, combine, and transform these objects, handling the complexities of OpenSCAD
syntax generation, including parameter formatting, object nesting, and indentation.

## Features

*   **Type-Safe Object Model**: Define 2D, 3D, and mixed-dimension objects using a clear
    type hierarchy (`ScadObject2D`, `ScadObject3D`, `ScadObjectMixed`) wrapped in
    `ScadObject`.
*   **Comprehensive Primitive Support**: Create basic shapes like `square`, `circle`,
    `sphere`, `cube`, `cylinder`, `polygon`, `polyhedron`, `text`, and `import`.
*   **Extensive Modifier Support**: Apply transformations and operations such as
    `translate`, `rotate`, `scale`, `resize`, `mirror`, `multmatrix`, `offset`,
    `projection`, `color`, `hull`, `render`, `children`, `linear_extrude`,
    `rotate_extrude`, etc.
*   **Boolean and Block Operations**: Combine objects using `union`, `difference`, and
    `intersection`. Blocks (`{ ... }`) are also explicitly supported.
*   **Operator Overloading**: Use standard Rust operators (`+`, `-`, `*`) for `union`,
    `difference`, and `intersection` operations on `ScadObject`s of the same dimension.
*   **Comment Support**: Easily add comments to individual objects or blocks using the
    `.commented()` method or dedicated factory functions.
*   **Builder Pattern**: Many complex primitives and modifiers provide a type-safe builder
    pattern (`...Builder`) for configuring optional parameters.
*   **Value Handling**: Type-safe representation and formatting for various OpenSCAD value
    types (numbers, vectors, strings, booleans, angles, colors, matrices) via the
    `ScadDisplay` trait.
*   **Code Generation**: Generate clean, correctly indented OpenSCAD code strings from your
    Rust object structures using the `to_code()` method.
*   **Prelude**: A convenient `prelude` module to easily import commonly used items and
    factory functions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
scadman = { tag = "v0.3.0", git = "https://github.com/lum1narie/scadman.git" }
```

## Basic Usage

Import the prelude to get access to common types and functions:

```rust
use scadman::prelude::*;
```

### Creating Primitives

Use the `primitive_2d` or `primitive_3d` factory functions with the corresponding builder or
direct value:

```rust
// Create a square with size 10
let square = primitive_2d(Square::build_with(|sb| {
    let _ = sb.size(10.0);
}));
println!("{}", square.to_code());
// Output: square(size = 10);

// Create a sphere with radius 5 using the builder
let sphere = primitive_3d(Sphere::build_with(|cb| {
    let _ = cb.r(5.0);
}));
println!("{}", sphere.to_code());
// Output: sphere(r = 5);

// Create a cylinder with height 10 and radius 3
let cylinder = primitive_3d(Cylinder::build_with(|cb| {
    let _ = cb.h(10.0).r(3.0);
}));
println!("{}", cylinder.to_code());
// Output: cylinder(h = 10, r = 3);
```

### Applying Modifiers

Use the `modifier_2d`, `modifier_3d`, or `modifier_mixed` factory functions. Note that
modifiers check the dimension compatibility of their child object (except for mixed
modifiers like `color`).

```rust
// Translate the square by (5, 5)
let translated_square = modifier_2d(Translate2D::build_with(|tb| {
    let _ = tb.v([5.0, 5.0]);
}), square);
println!("{}", translated_square.to_code());
/* Output:
translate([5, 5])
  square(size = 10);
*/

// Rotate the sphere by 90 degrees around the Y axis
let rotated_sphere = modifier_3d(Rotate3D::build_with(|rb| {
    let _ = rb.deg([0.0, 90.0, 0.0]);
}), sphere);
println!("{}", rotated_sphere.to_code());
/* Output:
rotate(a = [0, 90, 0])
  sphere(r = 5);
*/

// Apply a color modifier (mixed dimension)
let colored_cylinder = modifier_mixed(Color::build_with(|cb| {
    let _ = cb.c(RGB::new(1.0, 0.0, 0.0));
}), cylinder);
println!("{}", colored_cylinder.to_code());
/* Output:
color(c = [1, 0, 0])
  cylinder(h = 10, r = 3);
*/
```

### Using Blocks and Boolean Operations

Use the `block_2d`, `block_3d`, or `block_mixed` factory functions, or leverage operator
overloading for boolean operations (`+` for `union`, `-` for `difference`, `*` for
`intersection`).

```rust
let sphere = primitive_3d(Sphere::build_with(|cb| {
    let _ = cb.r(10.0);
}));
let cube = primitive_3d(Cube::build_with(|cb| {
    let _ = cb.size(15.0).center(true);
}));

// Subtract the cube from the sphere using the difference modifier
let result_modifier = modifier_3d(Difference::new(), block_3d(&[sphere.clone(), cube.clone()]));
println!("{}", result_modifier.to_code());
/* Output:
difference() {
  sphere(r = 10);
  cube(size = 15, center = true);
}
*/

// Achieve the same result using operator overloading
let result_operator = sphere - cube;
println!("{}", result_operator.to_code());
/* Output:
difference() {
  sphere(r = 10);
  cube(size = 15, center = true);
}
*/
```

### Adding Comments

Use the `.commented()` method or the `_commented` variants of the factory functions:

```rust
let commented_cube = primitive_3d(Cube::build_with(|cb| {
    let _ = cb.size(5.0);
})).commented("This is a simple cube");
println!("{}", commented_cube.to_code());
/* Output:
/* This is a simple cube */
cube(size = 5);
*/

let commented_translated_square = modifier_2d_commented(
    Translate2D::build_with(|tb| {
        let _ = tb.v([5.0, 5.0]);
    }),
    primitive_2d(Square::build_with(|sb| {
        let _ = sb.size(10.0);
    })), // Convert primitive sentence to ScadObject
    "Translated square"
);
println!("{}", commented_translated_square.to_code());
/* Output:
/* Translated square */
translate([5, 5])
  square(size = 10);
*/
```

## Example: Building Complex Models (like `tests/desk_clamp.rs`)

The `tests/desk_clamp.rs` file serves as a practical example of building a more complex model.
It demonstrates several key techniques facilitated by `scadman`:

1.  **Parametric Design with Constants**: Dimensions and other parameters are defined as
    Rust constants (`CLAMP_Z_SIZE`, `HOOK_OUTER_R`, etc.). This makes the design easily
    adjustable and readable.
2.  **Modular Design with Helper Functions**: The complex clamp shape is broken down into
    smaller, manageable parts (`generate_lattice_r_void`, `generate_clamp`, `generate_body`).
    Each function constructs and returns a `ScadObject` representing a component of the
    final assembly.
3.  **Composition via Modifiers and Blocks**: The helper functions return `ScadObject`s,
    which are then combined using `modifier_3d`, `block_3d`, and operator overloading (`+`,
    `-`) to build the final structure. This mirrors how objects are combined in OpenSCAD
    itself.
4.  **Leveraging Builders for Clarity**: Builders (`Translate2D::build_with`,
    `Polygon::build_with`, `Cylinder::build_with`, etc.) are used extensively to set
    parameters for primitives and modifiers, improving code readability compared to
    positional arguments.
5.  **Adding Comments for Readability**: Comments are added to significant parts of the
    model (`.commented()`) to explain the purpose of different objects or sections of the
    code, which translates directly to comments in the generated OpenSCAD file.

This example showcases how `scadman` enables a structured, modular, and maintainable
approach to creating complex parametric designs in OpenSCAD using the power of Rust.

## Key Concepts

*   **`ScadObject`**: The main container struct. It wraps the actual object body
    (`ScadObjectBody`) and holds an optional comment. All functions that build or
    manipulate SCAD geometry ultimately work with `ScadObject`.
*   **`ScadObjectBody`**: An enum (`Object2D`, `Object3D`, `ObjectMixed`) that holds the
    specific type of SCAD object (Primitive, Modifier, or Block) for a given dimension.
*   **`ScadObjectTrait`**: A trait implemented by `ScadObject` and its internal body types,
    providing core functionality like `to_code()` (generating the SCAD string) and
    `get_type()` (determining the object's dimension).
*   **`ScadDisplay`**: A fundamental trait implemented by any type that can be represented
    as a string in OpenSCAD code (numbers, vectors, strings, booleans, and the specific
    primitive/modifier/block body types). The `repr_scad()` method generates the SCAD
    string for that specific value or object part.
*   **`ScadCommentDisplay`**: A trait (delegated from `ScadObjectTrait`) that adds the
    ability to generate SCAD code with a comment (`repr_scad_with_comment`).
*   **`ScadBuilder` / `ScadBuildable`**: Traits supporting the builder pattern for
    configuring complex SCAD sentences with optional parameters. `ScadBuildable::build_with`
    is the primary entry point for using builders.
*   **Primitives, Modifiers, and Blocks**: These correspond directly to OpenSCAD's
    structural elements. The library provides specific types (`ScadPrimitive2D`,
    `ScadModifier3D`, `ScadBlockMixed`, etc.) and enums (`ScadPrimitiveBody2D`,
    `ScadModifierBody3D`, `ScadModifierBodyMixed`) to represent them.
*   **Factory Functions (`primitive_`, `modifier_`, `block_`)**: Functions like
    `primitive_2d`, `modifier_3d`, `block_mixed`, etc., provided in the `lib.rs` root,
    are the primary way to construct `ScadObject` instances from the specific
    primitive/modifier/block types defined in the `scad_2d`, `scad_3d`, and `scad_mixed`
    modules. `try_` variants are provided for operations that might fail due to dimension
    mismatches.
*   **Value Types**: Custom types in `value_type.rs` (like `Angle`, `RGBA`, `RoundSize`,
    etc.) and standard types (`f64` for `Unit`, `bool`, `String`, vectors from `nalgebra`)
    implement `ScadDisplay` to ensure correct formatting in the generated SCAD code.

