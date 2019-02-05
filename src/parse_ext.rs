use syn::parse::{Parse, ParseStream};
use syn::Result;

pub trait ParseStreamExt {
    fn try_parse<T: syn::parse::Parse>(&self) -> Result<T>;

    fn try_parse_as<T, R, F>(&self, f: F) -> Result<R>
    where
        T: Parse,
        F: FnOnce(T) -> R,
    {
        self.try_parse::<T>().map(f)
    }

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
