// use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MatchedArticles {
    pub slug: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

// impl ArticleResult {
//     pub async fn search_articles(query: String, page: i64, amount: i64) {
//         let sql_query = format!(
//             "SELECT distinct
//             a.slug as slug,
//             snippet(articles_fts,1, '***','***','>>>',2) as tiitle,
//             snippet(articles_fts,2, '***','***','>>>',2) as description,
//             snippet(articles_fts,3, '***','***','>>>',2) as body
//             FROM Articles_fts AS AFTS
//             JOIN  Articles AS A  ON A.oid = AFTS.rowid
//             WHERE Articles_fts MATCH $1
//             order by rank
//             LIMIT $2 OFFSET $3;
//             "
//         );

//         let results = sqlx::query::<'_, ArticleResult, _>(&sql_query)
//             .bind(query)
//             .bind(amount)
//             .bind(page * amount)
//             .fetch_all(crate::database::get_db())
//             .await?;
//     }
// }

// impl ArticleResult {
//     #[tracing::instrument]
//     #[cfg(feature = "ssr")]
//     pub async fn for_articles(
//         query: String,
//         page: i64,
//         amount: i64,
//     ) -> Result<Vec<Self>, sqlx::Error> {
//         sqlx::query!(
//             "
//             SELECT distinct
//             a.slug,
//             snippet(articles_fts,1, '***','***','>>>',2) as "matched_title: ?",
//             snippet(articles_fts,2, '***','***','>>>',2) as "matched_description: ?",
//             snippet(articles_fts,3, '***','***','>>>',2) as "matched_body: ?"
//             FROM Articles_fts AS AFTS
//             JOIN  Articles AS A  ON A.oid = AFTS.rowid
//             WHERE Articles_fts MATCH 'ethical'
//             order by rank
//             LIMIT $1 OFFSET $2",
//             amount,
//             page * amount,
//         )
//         .map(|x| Self {
//             slug: x.slug,
//             title: if x.matched_title.is_empty {
//                 None
//             } else {
//                 Some(x.matched_title)
//             },
//             description: if x.matched_description.is_empty {
//                 None
//             } else {
//                 Some(x.matched_description)
//             },
//             body: if x.matched_body.is_empty {
//                 None
//             } else {
//                 Some(x.matched_body)
//             },
//         })
//         .fetch_all(crate::database::get_db())
//         .await
//     }
// }

impl MatchedArticles {
    #[tracing::instrument]
    #[cfg(feature = "ssr")]
    pub async fn search_articles(
        query: String,
        page: i64,
        amount: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let offset = page * amount;
        sqlx::query!(
            // MatchedArticles,
            r#"
SELECT distinct
a.slug as slug,
snippet(articles_fts,1, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',10) as "title: String",
snippet(articles_fts,2, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',20) as "description: String",
snippet(articles_fts,3, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',20) as "body: String"
FROM Articles_fts AS AFTS
JOIN  Articles AS A  ON A.oid = AFTS.rowid
WHERE Articles_fts MATCH $3
order by rank
LIMIT $1 OFFSET $2"#,
            amount,
            offset,
            query,
        )
        .map(|x| Self {
            slug: x.slug,
            title: x.title,
            description: x.description,
            body: x.body,
        })
        .fetch_all(crate::database::get_db())
        .await
    }
}
