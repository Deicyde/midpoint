
///Gives the type the ability to call the midpoint function
pub trait Midpoint: Copy {
    ///Returns half the sum of self and other, rounded towards self.
    ///
    ///# Guarentees
    ///
    ///midpoint makes the following guarentees:
    /// - will not panic or cause undefined behavior
    /// - will not overflow or underflow
    /// - will use at most 1 inexact operation (for floating point types)
    /// - will return NaN if either unput is NaN and INF if either input is INF, preferring NaN (for floating point types)
    fn midpoint(self, other: Self) -> Self;
}

