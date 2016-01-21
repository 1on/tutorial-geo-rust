pub struct Response<T> {
    success: bool,
    data: Option<T>,
    pagination: Option<Pagination>,
    error: Option<ResponseError>
}

pub struct ResponseError {
    code: String,
    message: String,
}

pub struct Pagination {
    current_page: i32,
    limit: i32,
    total_count: i32,
    total_page_count: i32,
}

