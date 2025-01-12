use std::{collections::VecDeque, ops::Not};
use c3::todo_app::{App, Restriction, TodoList};

#[derive(Clone, Default)]
struct SearchPosition {
    tree_path: Vec<usize>,
    matching_indices: Vec<usize>,
}

#[derive(Default)]
pub struct TreeSearch {
    positions: Vec<SearchPosition>,
    list_index: usize,
    pos_index: usize,
}

impl TreeSearch {
    #[inline]
    fn current_tree_position(&self) -> Option<(usize,&Vec<usize>)> {
        self.positions.is_empty().not().then(|| {
            let item = &self.positions[self.list_index];
            let index = item.matching_indices[self.pos_index];
            (index, &item.tree_path)
        })
    }

    #[inline]
    pub fn search(&mut self, query: String, todo_list: &TodoList, restriction: Restriction) {
        self.positions = vec![];
        self.pos_index = 0;
        self.list_index = 0;
        if query.is_empty() {
            return;
        }
        let mut lists: VecDeque<(Vec<usize>, &TodoList)> =
            VecDeque::from([(vec![], todo_list)]);
        while let Some((indices, current_list)) = lists.pop_back() {
            let mut matching_indices: Vec<usize> = vec![];
            for (i, todo) in current_list.filter(&restriction).enumerate() {
                let mut todo_indices = indices.clone();
                todo_indices.push(i);
                if todo.matches(&query) {
                    matching_indices.push(i)
                }
                if let Some(list) = todo.dependency.as_ref().and_then(|dep| dep.todo_list()) {
                    lists.push_back((todo_indices, list))
                }
            }
            if !matching_indices.is_empty() {
                self.positions.push(SearchPosition {
                    tree_path: indices.to_vec(),
                    matching_indices,
                })
            }
        }
    }

    #[inline]
    pub fn next(&mut self) {
        if !self.positions.is_empty() {
            let list_size = self.positions.len();
            let pos_size = self.positions[self.list_index]
                .matching_indices
                .len();
            if self.list_index + 1 < list_size {
                self.list_index += 1
            } else if self.pos_index + 1 < pos_size {
                self.pos_index += 1
            } else {
                self.pos_index = 0;
                self.list_index = 0;
            }
        }
    }

    #[inline]
    pub fn set_to_app(&self, todo_app:&mut App) {
        if let Some((index, path)) =  self.current_tree_position() {
            todo_app.index = index;
            todo_app.tree_path.clone_from(path);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs::remove_dir_all, rc::Rc};
    use c3::todo_app::test_helpers::*;
    use std::io;
    use super::*;

    #[test]
    fn test_tree_search() -> io::Result<()> {
        let dir = dir("test-tree-search")?;
        let app = write_test_todos(&dir)?;
        remove_dir_all(dir)?;
        let query = String::from("nod");
        let mut tree_search = TreeSearch::default();
        tree_search.search(query, app.current_list(), Rc::clone(app.get_restriction()));
        let position = &tree_search.positions[0];
        assert_eq!(position.tree_path, vec![2, 0]);
        assert_eq!(position.matching_indices, vec![0]);
        Ok(())
    }
}
