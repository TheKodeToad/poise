//! Small hacky macro to convert any value into a Stream, where the value can be an IntoIterator
//! or a Stream. Used for the return value of autocomplete callbacks

#[doc(hidden)]
pub struct IntoStreamWrap<'a, T>(pub &'a T);

#[doc(hidden)]
pub trait IntoStream<T> {
    type Output;
    fn into_stream(self, x: T) -> Self::Output;
}

impl<T: IntoIterator> IntoStream<T> for &IntoStreamWrap<'_, T> {
    type Output = futures::stream::Iter<T::IntoIter>;
    fn into_stream(self, iter: T) -> Self::Output {
        futures::stream::iter(iter)
    }
}

impl<T: futures::Stream> IntoStream<T> for &&IntoStreamWrap<'_, T> {
    type Output = T;
    fn into_stream(self, stream: T) -> Self::Output {
        stream
    }
}

// Takes an expression that is either an IntoIterator or a Stream, and converts it to a Stream
#[doc(hidden)]
#[macro_export]
macro_rules! into_stream {
    ($e:expr) => {
        match $e {
            value => {
                use $crate::IntoStream as _;
                (&&$crate::IntoStreamWrap(&value)).into_stream(value)
            }
        }
    };
}