### Summary of this chapter

**Crate** : A tree of module that produces a library or executable.

**Module** : Lets you organize code within a crate (like folders)

**Path** : A way of naming an item (eg crate::std::io)

**mod x** : Tells rust: "Load code for module x from a file named `x.rs` or `x/mod.rs`."

**use** : Brings a path into scope so you don't have to write full path everytime (helps cleaner code).

**pub** : By default everything is private, this keyword makes it accessible tot he parent module.