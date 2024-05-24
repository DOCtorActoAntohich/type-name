/// Allows programmatically extracting the `struct` or enum name as String.
/// It does not capture namespaces, but simply writes a
///
/// - When derived automagically, it will simply take
///   the struct name as it's written in code.
/// - Manual implementation is more prone to errors
///   (such as renaming and then forgetting to change the string in `impl`),
///   but allows for any custom short (or even LOOOOONG) name.
///
/// Good part is: no `Debug` required.
///
/// ## Examples
///
/// ### Struct
///
/// ```rust
/// use type_name::ShortTypeName;
///
/// #[derive(ShortTypeName)]
/// struct MyEpicStruct {
///     _unused: i32,
///     _actually_used: Box<MyEpicStruct>,
/// }
///
/// assert_eq!("MyEpicStruct", MyEpicStruct::as_type_name());
/// ```
///
/// ### Enum
///
/// ```rust
/// use type_name::ShortTypeName;
///
/// #[derive(ShortTypeName)]
/// enum NonSusEnum {
///     Red,
///     Yellow,
///     Blue
/// }
///
/// assert_eq!("NonSusEnum", NonSusEnum::as_type_name());
/// ```
pub trait ShortTypeName {
    fn as_type_name() -> &'static str;
}

pub use type_name_derive::ShortTypeName;
