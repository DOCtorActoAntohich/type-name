/// Allows programmatically extracting the `struct` or `enum` name as String.
/// It does not capture namespaces and only uses the type name as it is in code.
///
/// Manual implementation is possible, but may be prone to errors such as
/// "Crap, I renamed the enum, but forgor to change the trait implementation"
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
