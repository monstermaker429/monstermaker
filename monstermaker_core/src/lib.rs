#![deny(missing_docs)]

//! Core traits for the Monster Maker ecosystem.
//!
//! This crate provides shared traits for types in the Monster Maker 
//! ecosystem. Composition of these traits allows for custom types that 
//! can be used across other crates in the Monster Maker ecosystem 
//! (both official and third-party).

/// A named type.
///
/// A type can implement this trait to provide a name, possibly defined 
/// internally in the type. For example:
///
/// ```
/// use monstermaker_core::Name;
///
/// struct Foo {
///     name: &'static str,
/// }
///
/// impl Name for Foo {
///     fn name(&self) -> &str {
///         self.name
///     }
/// }
/// ```
pub trait Name {
    /// Returns the object's name.
    fn name(&self) -> &str;
}

/// An identified type.
///
/// A type can implement this trait to provide an id, possibly defined 
/// interally in the type. For example:
///
/// ```
/// use monstermaker_core::Id;
///
/// struct Foo {
///     id: u32,
/// }
///
/// impl Id for Foo {
///     fn id(&self) -> u32 {
///         self.id
///     }
/// }
/// ```
pub trait Id {
    /// Returns the object's id.
    fn id(&self) -> u32;
}
