struct Object {
    name: String
}

struct RefObj<'a> {
    r#ref: &'a Object 
}

impl RefObj<'_> {
    fn name(&self) -> &str {
        &*(self.r#ref).name.as_str()
    }
}

fn main() {
    let obj = Object { name: String::from("This is a object") };
    let obj_ref = RefObj { r#ref: &obj };

    println!("Hello, name is: {}",obj_ref.name());
}
