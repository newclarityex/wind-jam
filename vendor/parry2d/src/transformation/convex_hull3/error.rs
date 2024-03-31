#[derive(Debug, PartialEq)]
/// Errors generated by the convex-hull calculation.
pub enum ConvexHullError {
    /// Reached an impossible configuration in the convex-hull calculation,
    /// likely because of a bug.
    InternalError(&'static str),
    /// The convex hull calculation was unable to find a support point.
    /// This generally happens if the input point set contains invalid points (with NaN coordinates)
    /// or if they are almost coplanar.
    MissingSupportPoint,
    /// Reached a piece of code we shouldn’t (internal error).
    Unreachable,
}

impl std::fmt::Display for ConvexHullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvexHullError::InternalError(reason) => write!(f, "InternalError({})", reason),
            ConvexHullError::MissingSupportPoint => write!(f, "MissingSupportPoint"),
            ConvexHullError::Unreachable => write!(f, "Unreachable"),
        }
    }
}

impl std::error::Error for ConvexHullError {}
