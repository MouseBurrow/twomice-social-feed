use easy_errors::define_errors;

define_errors!(
    SocialFeedError {
        UniqueViolation => {
            code: "23505",
            status: CONFLICT,
            message: "Already following this board"
        },
        BoardNotFound => {
            code: "SF001",
            status: NOT_FOUND,
            message: "Board not found"
        },
        Unauthorized => {
            code: "SF002",
            status: UNAUTHORIZED,
            message: "Authentication required"
        },
        UpstreamError => {
            code: "SF003",
            status: BAD_GATEWAY,
            message: "Failed to reach upstream service"
        }
    }
);
