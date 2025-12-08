# scadman

`scadman` is a Rust library for generating OpenSCAD code programmatically. It provides a
type-safe and structured way to define 2D and 3D geometric objects, apply transformations
and operations, and output valid OpenSCAD code.

## What is this library?

Instead of writing OpenSCAD code directly in `.scad` files, `scadman` allows you to define
your parametric models using Rust code. This approach leverages Rust's strong typing,
module system, testing capabilities, and build tools to manage your designs.

The library represents OpenSCAD primitives, modifiers, and blocks as distinct Rust types,
managed by a powerful generic type system centered around `ScadObjectGeneric<D>`. This system
provides enhanced type safety by tracking object dimensions (2D, 3D, Mixed) at compile time
where possible, while allowing for flexible runtime handling when necessary. It provides
traits and helper functions to build, combine, and transform these objects, handling the
complexities of OpenSCAD syntax generation, including parameter formatting, object nesting,
and indentation.

## Features

*   **Type-Safe Object Model**: Define 2D, 3D, and mixed-dimension objects using a clear
    type hierarchy based on `ScadObjectGeneric<D>` and public-facing wrappers like
    `ScadObject2D`, `ScadObject3D`, and `ScadObjectMixed`.
*   **Comprehensive Primitive Support**: Create basic shapes like `square`, `circle`,
    `sphere`, `cube`, `cylinder`, `polygon`, `polyhedron`, `text`, and `import`.
*   **Extensive Modifier Support**: Apply transformations and operations such as
    `translate`, `rotate`, `scale`, `resize`, `mirror`, `multmatrix`, `offset`,
    `projection`, `color`, `hull`, `render`, `children`, `linear_extrude`,
    `rotate_extrude`, etc.
*   **Boolean and Block Operations**: Combine objects using `union`, `difference`, and
    `intersection`. Blocks (`{ ... }`) are also explicitly supported.
*   **Operator Overloading**: Use standard Rust operators (`+`, `-`, `*`) for `union`,
    `difference`, and `intersection` operations. These operators work on the
    `ScadObject` (untyped alias) and perform a runtime check, requiring both operands
    to have the same internal dimension (`Object2D`, `Object3D`, or `ObjectMixed`).
    Operations between objects of different dimensions will result in a panic.
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
scadman
```

## Working with Typed Dimensions: Primitives, Modifiers, and Conversions

`scadman` uses a robust generic type system to ensure dimensional correctness (2D, 3D, Mixed) at compile time wherever possible, while providing flexibility through conversions.

### Creating Typed Objects: `primitive_*` Functions

To create an object with a specific dimension, use the corresponding factory function:

*   **`primitive_2d(body)`**: Creates a `ScadObject2D` (which wraps `ScadObjectGeneric<D2>`). For example, `primitive_2d(Square::new())`.
*   **`primitive_3d(body)`**: Creates a `ScadObject3D` (which wraps `ScadObjectGeneric<D3>`). For example, `primitive_3d(Cube::new())`.

These functions typically take a primitive "body" (e.g., `Square`, `Cube`) that is often constructed using its `build_with` builder pattern.

### Applying Typed Modifiers: `modifier_*` Functions

Similarly, to apply a modifier to a typed object, use the matching factory function:

*   **`modifier_2d(modifier_body, child_object_2d)`**: Applies a 2D modifier (e.g., `Translate2D`) to a `ScadObject2D`, returning a new `ScadObject2D`. This ensures that you're not trying to apply a 2D translation to a 3D object at compile time.
*   **`modifier_3d(modifier_body, child_object_3d)`**: Applies a 3D modifier (e.g., `Rotate3D`) to a `ScadObject3D`, returning a new `ScadObject3D`.

Some modifiers, like `Color`, can apply to any dimension. For these, use:

*   **`modifier_mixed(modifier_body, child_object_any_dimension)`**: Applies a mixed-dimension modifier to an object that can be `ScadObject2D`, `ScadObject3D`, or `ScadObjectMixed`. This function will implicitly convert the `child_object` into its untyped `ScadObject` (i.e., `ScadObjectGeneric<DMixed>`) representation before applying the modifier. The result will be a `ScadObjectMixed`.

### Understanding Conversions (`.into()`)

`scadman` provides automatic conversions to make composition flexible:

*   Any `ScadObject2D` or `ScadObject3D` can be converted into `ScadObject` (which is an alias for `ScadObjectGeneric<DMixed>`) using the `.into()` method. This is useful when you need to treat a dimensionally specific object in a more generic (untyped) context, such as passing it to `block_mixed` or `modifier_mixed`.

    ```rust
    use scadman::prelude::*;

    let square_2d = primitive_2d(Square::new()); // ScadObject2D
    let cube_3d = primitive_3d(Cube::new());     // ScadObject3D

    // Convert to untyped ScadObject for a mixed context
    let generic_square: ScadObject = square_2d.into();
    let generic_cube: ScadObject = cube_3d.into();

    // Now they can be used together in a mixed block
    let mixed_objects = block_mixed(&[generic_square, generic_cube]);
    println!("{}", mixed_objects.to_code());
    /* Output:
    union() { // block_mixed defaults to union if not specified
      square();
      cube();
    }
    */
    ```

### Typed Blocks: `block_*` Functions

Similar to primitives and modifiers, blocks also come in typed variants:

*   **`block_2d(objects)`**: Creates a `ScadObject2D` containing other `ScadObject2D` instances.
*   **`block_3d(objects)`**: Creates a `ScadObject3D` containing other `ScadObject3D` instances.
*   **`block_mixed(objects)`**: Creates a `ScadObjectMixed` containing any combination of `ScadObject`s (i.e., `ScadObjectGeneric<DMixed>`). When passing `ScadObject2D` or `ScadObject3D` to `block_mixed`, they will be implicitly converted to `ScadObject`.

This type system allows you to catch dimensional mismatches at compile time for common operations, while still providing the flexibility to work with mixed-dimension constructs when OpenSCAD's nature requires it.

Import the prelude to get access to common types and functions:

```rust
use scadman::prelude::*;
```

### Creating Primitives

Use primitives with the `build_with` factory functions with direct value:

```rust
// Create a square with size 10
let square = Square::build_with(|sb| {
    let _ = sb.size(10.0);
});
println!("{}", square.to_code());
// Output: square(size = 10);

// Create a sphere with radius 5 using the builder
let sphere = Sphere::build_with(|cb| {
    let _ = cb.r(5.0);
});
println!("{}", sphere.to_code());
// Output: sphere(r = 5);
to
// Create a cylinder with height 10 and radius 3
let cylinder = Cylinder::build_with(|cb| {
    let _ = cb.h(10.0).r(3.0);
});
println!("{}", cylinder.to_code());
// Output: cylinder(h = 10, r = 3);
```

### Applying Modifiers

Use the `apply_to` factory functions.
Use `apply_to_2d`, `apply_to_3d` for universal modifiers without parameters like `Union`.
Modifiers with parameters like Translate is splitted in like `Translate2D`, `Translate3D`,
due to parameters' dimension.

```rust
// Translate the square by (5, 5)
let translated_square = Translate2D::build_with(|tb| {
    let _ = tb.v([5.0, 5.0]);
}).apply_to(square);
println!("{}", translated_square.to_code());
/* Output:
translate([5, 5])
  square(size = 10);
*/

// Rotate the sphere by 90 degrees around the Y axis
let rotated_sphere = Rotate3D::build_with(|rb| {
    let _ = rb.deg([0.0, 90.0, 0.0]);
}).apply_to(sphere);
println!("{}", rotated_sphere.to_code());
/* Output:
rotate(a = [0, 90, 0])
  sphere(r = 5);
*/

// Apply a color modifier
let colored_cylinder = Color::build_with(|cb| {
    let _ = cb.c(RGB::new(1.0, 0.0, 0.0));
}).apply_to_3d(cylinder);
println!("{}", colored_cylinder.to_code());
/* Output:
color(c = [1, 0, 0])
  cylinder(h = 10, r = 3);
*/
```

### Using Blocks and Boolean Operations

`Vector` or arrays of `ScadObject2D` or `ScadObject3D` can be converted as block object.
You can make this with `ScadObject2D::from`, `ScadObject3D::from`, or
automatically converted in `apply_to` function.
Leverage operators overload for boolean operations
(`+` for `union`, `-` for `difference`, `*` for `intersection`).

```rust
let sphere = Sphere::build_with(|cb| {
    let _ = cb.r(10.0);
});
let cube = Cube::build_with(|cb| {
    let _ = cb.size(15.0).center(true);
});

// Subtract the cube from the sphere using the difference modifier
let result_modifier = Difference::new().apply_to_3d([sphere.clone(), cube.clone()]);
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

Use the `.commented()` method:

```rust
let commented_cube = Cube::build_with(|cb| {
    let _ = cb.size(5.0);
}).commented("This is a simple cube");
println!("{}", commented_cube.to_code());
/* Output:
/* This is a simple cube */
cube(size = 5);
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
    which are then combined using `modifier_3d`, and operator overloading (`+`,
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
    These are stored in `ScadObjectGeneric`.
*   **Value Types**: Custom types in `value_type.rs` (like `Angle`, `RGBA`, `RoundSize`,
    etc.) and standard types (`f64` for `Unit`, `bool`, `String`, vectors from `nalgebra`)
    implement `ScadDisplay` to ensure correct formatting in the generated SCAD code.

