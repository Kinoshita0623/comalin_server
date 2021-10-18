pub trait Selectable {
    type Columns;
    fn columns() -> Self::Columns;
}