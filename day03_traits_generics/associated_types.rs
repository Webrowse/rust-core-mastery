// Associated Types are for placing a placeholder type inside a trait definition.
// They allow implementers of the trait to specify concrete types for those placeholders.
// Generics, on the other hand, are used to define functions, structs, or enums
// that can operate on different types without being tied to a specific one.

// Associated types and generics are two ways to achieve similar goals in Rust,
// but they have different use cases and trade-offs.

// Traits with Associated Types
trait GraphNode {
    type IdType;

    fn id(&self) -> Self::IdType;
}

struct User {
    user_id: u32,
    name: String,
}

impl GraphNode for User {
    type IdType = u32;

    fn id(&self) -> Self::IdType {
        self.user_id
    }
}

struct Admin {
    user_id: String,
    level: u8,
}
impl GraphNode for Admin {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.user_id.clone()
    }
}

fn main() {
    let user = User {
        user_id: 2304,
        name: String::from("Romy"),
    };
    let admin = Admin {
        user_id: String::from("Role_X"),
        level: 5,
    };

    println!("User ID: {}, {}", user.id(), user.name);
    println!("Admin ID: {}, level: {}", admin.id(), admin.level);
    
}