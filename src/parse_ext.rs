use syn::parse::{Parse, ParseStream};
use syn::Result;

/// An extension trait for syn's `ParseStream`
pub trait ParseStreamExt {
    /// Try parsing a value, but do not consume the stream if it failed
    ///
    /// This is a potentially costly method, better used when one knows the parsing will fail early.
    fn try_parse<T: syn::parse::Parse>(&self) -> Result<T>;

    /// Try parsing a value and apply a function, but do not consume the stream if it failed
    ///
    /// This method is useful when working with enums. See `parse_as` for an example.
    ///
    /// This is a potentially costly method, better used when one knows the parsing will fail early.
    fn try_parse_as<T, R, F>(&self, f: F) -> Result<R>
    where
        T: Parse,
        F: FnOnce(T) -> R,
    {
        self.try_parse::<T>().map(f)
    }

    /// Parse a value and apply a function.
    ///    
    /// This method is useful when working with enums.
    ///
    /// ```
    /// use syn::Result;
    /// use syn::parse::{Parse, ParseStream};
    /// use synattra::types::KVOption;
    /// use synattra::ParseStreamExt;
    ///
    /// mod kw {
    ///     syn::custom_keyword!(registry);
    ///     syn::custom_keyword!(registry_expr);
    /// }
    ///
    /// pub type MeteredRegistryOption = KVOption<kw::registry, syn::Ident>;
    ///
    /// pub type MeteredRegistryExprOption = KVOption<kw::registry_expr, syn::Expr>;
    ///
    /// pub enum MeteredOption {
    ///     Registry(MeteredRegistryOption),
    ///     RegistryExpr(MeteredRegistryExprOption),
    /// }
    ///
    /// impl Parse for MeteredOption {
    ///     fn parse(input: ParseStream) -> Result<Self> {
    ///         if MeteredRegistryOption::peek(input) {
    ///             Ok(input.parse_as(MeteredOption::Registry)?)
    ///         } else if MeteredRegistryExprOption::peek(input) {
    ///             Ok(input.parse_as(MeteredOption::RegistryExpr)?)
    ///         } else {
    ///             let err = format!("invalid metered option: {}", input);
    ///             Err(input.error(err))
    ///         }
    ///     }
    /// }
    /// ````
    fn parse_as<T, R, F>(&self, f: F) -> Result<R>
    where
        T: Parse,
        F: FnOnce(T) -> R;
}

impl<'a> ParseStreamExt for ParseStream<'a> {
    // Advance the stream only if parsing was successful
    fn try_parse<T: syn::parse::Parse>(&self) -> Result<T> {
        let fork = self.fork();
        fork.parse::<T>()?;
        self.parse::<T>()
    }

    fn parse_as<T, R, F>(&self, f: F) -> Result<R>
    where
        T: Parse,
        F: FnOnce(T) -> R,
    {
        self.parse::<T>().map(f)
    }
}
