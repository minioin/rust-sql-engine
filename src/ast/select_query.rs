use super::{Condition, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct SelectQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub condition: Option<Condition>
}

impl SelectQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<I>, condition: Option<Condition>) -> SelectQuery {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns.into_iter().map(|c| c.into()).collect::<Vec<String>>(),
            condition: condition
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypedSelectQuery {
    table_name: String,
    columns: Vec<(String, Type)>,
    condition: Option<Condition>
}

impl TypedSelectQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<(I, Type)>, condition: Option<Condition>) -> TypedSelectQuery {
        TypedSelectQuery::new_with_strings(table_name, columns.into_iter().map(|(n, t)| (n.into(), t)).collect::<Vec<(String, Type)>>(), condition)
    }

    pub fn new_with_strings<I: Into<String>>(table_name: I, columns: Vec<(String, Type)>, condition: Option<Condition>) -> TypedSelectQuery {
        TypedSelectQuery {
            table_name: table_name.into(),
            columns: columns,
            condition: condition
        }
    }
}
