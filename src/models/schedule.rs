use super::anime::Anime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Pagination { 
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub items: PaginationItems,
}

#[derive(Debug, Deserialize)]
pub struct PaginationItems { 
    pub count: u32,
    pub total: u32,
    pub per_page: u32,
}

