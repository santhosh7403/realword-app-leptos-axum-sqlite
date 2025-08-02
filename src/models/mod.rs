mod user;
pub use user::{User, UserPreview};
mod pagination;
pub use pagination::Pagination;
mod article;
pub use article::Article;
mod comment;
pub use comment::Comment;
// mod search;
// pub use search::MatchedArticles;

#[cfg(feature = "ssr")]
const DATE_FORMAT: &str = "%d/%m/%Y %H:%M";
