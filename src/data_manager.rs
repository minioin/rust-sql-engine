use std::collections::HashMap;

pub struct DataManager {
    data: HashMap<String, Vec<Vec<String>>>
}

impl Default for DataManager {
    fn default() -> DataManager {
        DataManager {
            data: HashMap::default()
        }
    }
}

impl DataManager {
    pub fn save_to<I, D>(&mut self, table_name: I, data: D) -> Result<(), ()>
        where I: Into<String>,
              D: IntoIterator<Item = I> {
        self.data.entry(table_name.into())
            .or_insert_with(Vec::default)
            .push(
                data.into_iter().map(Into::into).collect::<Vec<String>>()
            );
        Ok(())
    }

    pub fn get_row_from(&self, table_name: &str, row_id: usize) -> Option<Vec<&str>> {
        match self.data.get(table_name) {
            None => None,
            Some(table_data) => {
                match table_data.into_iter().nth(row_id) {
                    None => None,
                    Some(vec) => Some(vec.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
                }
            },
        }
    }

    pub fn get_range(&self, table_name: &str, start_from: usize, number_of_rows: usize) -> Option<Vec<Vec<&str>>> {
        match self.data.get(table_name) {
            None => None,
            Some(table_data) =>
                Some(table_data.into_iter()
                        .skip(start_from)
                        .take(number_of_rows)
                        .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>()
                ),
        }
    }
}