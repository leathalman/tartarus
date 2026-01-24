use axum::extract::{Query, State};
use axum::Json;

use crate::app::AppState;
use crate::error::AppError;
use crate::models::prowlarr::{SearchParams, SearchResult};

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<SearchResult>>, AppError> {
    let results = state
        .prowlarr
        .search(&params.q, params.categories.as_deref())
        .await?;

    Ok(Json(results))
}
