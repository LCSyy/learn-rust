/*
GraphicsItem
Renderer
Abstract Graphics Layer
*/
/*
workspace
|- package1 
|  |- Cargo.toml
|  |- src/
|  |  |- main.rs -------------- binary crate root
|  |  |- lib.rs  -------------- lib crate root
|  |  |- bin/
|  |  |  |- <other_binary_crate>.rs
|  |  |  |- ...
|- package2
|- Cargo.toml

scope
path - refer to  an item in the module tree.
pub use - re-exporting

module
|- mod <module_name>
|- <module_name>.rs
|- <module_name>/
|  |- [mod.rs]
|  |- <child_module_name>.rs
*/

// if statement
// let if
// loop statement
// let loop -> break statement;
// while
// for, need a range for

use renderer::math::{ self, Vector2 };

fn main() {
    let angle = 35.0;
    println!("angle {} to radius {}",angle, math::a2r(angle));

    let v1 = Vector2::new(1.0,1.0);
    let v2 = Vector2::new(1.0,2.0);

    println!("{:?}",v1.dot(v2));
}
