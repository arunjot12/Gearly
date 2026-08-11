use crate::auth::claims::Claims;

pub async fn protected_dashboard(_claims: Claims) -> &'static str {
    "You reached the protected endpoint"
}