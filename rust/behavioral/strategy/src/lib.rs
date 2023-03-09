use std::collections::HashMap;

/// Group a list of movies with different grouping strategies
pub trait GroupByStrategy {
    fn group(&self, items: &Vec<Movie>) -> Vec<Vec<Movie>>;
}

pub struct GroupByDirectorStrategy;

impl GroupByStrategy for GroupByDirectorStrategy {
    fn group(&self, items: &Vec<Movie>) -> Vec<Vec<Movie>> {
        let mut map_by_director: HashMap<String, Vec<Movie>> = HashMap::new();
        for item in items.iter() {
            if map_by_director.contains_key(&item.director) {
                map_by_director
                    .get_mut(&item.director)
                    .unwrap()
                    .push(item.clone());
            } else {
                map_by_director.insert(item.director.to_string(), vec![item.clone()]);
            }
        }
        map_by_director.values().cloned().collect()
    }
}

pub struct GroupByYearStrategy;

impl GroupByStrategy for GroupByYearStrategy {
    fn group(&self, items: &Vec<Movie>) -> Vec<Vec<Movie>> {
        let mut map_by_director: HashMap<u32, Vec<Movie>> = HashMap::new();
        for item in items.iter() {
            if map_by_director.contains_key(&item.year) {
                map_by_director
                    .get_mut(&item.year)
                    .unwrap()
                    .push(item.clone());
            } else {
                map_by_director.insert(item.year, vec![item.clone()]);
            }
        }
        map_by_director.values().cloned().collect()
    }
}

#[derive(Clone, Debug)]
pub struct Movie {
    pub id: String,
    pub director: String,
    pub title: String,
    pub year: u32,
    // And probably other fields
}

impl Movie {
    pub fn new(id: &str, director: &str, title: &str, year: u32) -> Self {
        Self {
            id: id.to_string(),
            director: director.to_string(),
            title: title.to_string(),
            year,
        }
    }
}

pub struct Movies<T: GroupByStrategy> {
    group_by_strategy: T,
    items: Vec<Movie>,
}

impl<T: GroupByStrategy> Movies<T> {
    pub fn new(group_by_strategy: T, items: Vec<Movie>) -> Self {
        Self {
            group_by_strategy,
            items,
        }
    }

    pub fn group(&self) -> Vec<Vec<Movie>> {
        self.group_by_strategy.group(&self.items)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_movies() -> Vec<Movie> {
        vec![
            Movie::new(
                "1",
                "Peter Jackson",
                "The Hobbit: The Battle of the Five Armies",
                2014,
            ),
            Movie::new("3", "Christopher Nolan", "Tenet", 2020),
            Movie::new("2", "Wes Anderson", "The Grand Budapest Hotel", 2014),
            Movie::new("4", "Christopher Nolan", "Interstellar", 2014),
            Movie::new("5", "Wes Anderson", "Fantastic Mr. Fox", 2009),
            Movie::new("6", "Denis Villeneuve", "Blade Runner 2049", 2017),
            Movie::new("8", "Denis Villeneuve", "Polytechnique", 2009),
            Movie::new("9", "Ridley Scott", "The Martian", 2015),
            Movie::new("10", "Peter Jackson", "District 9", 2009),
            Movie::new("11", "Quentin Tarantino", "Inglourious Basterds", 2009),
        ]
    }

    #[test]
    fn group_by_director() {
        let movies: Vec<Movie> = get_movies();
        let movies_by_director = Movies::new(GroupByDirectorStrategy, movies);
        let expected = vec![
            vec![
                Movie::new(
                    "1",
                    "Peter Jackson",
                    "The Hobbit: The Battle of the Five Armies",
                    2014,
                ),
                Movie::new("10", "Peter Jackson", "District 9", 2009),
            ],
            vec![
                Movie::new("3", "Christopher Nolan", "Tenet", 2020),
                Movie::new("4", "Christopher Nolan", "Interstellar", 2014),
            ],
            vec![
                Movie::new("2", "Wes Anderson", "The Grand Budapest Hotel", 2014),
                Movie::new("5", "Wes Anderson", "Fantastic Mr. Fox", 2009),
            ],
            vec![
                Movie::new("6", "Denis Villeneuve", "Blade Runner 2049", 2017),
                Movie::new("8", "Denis Villeneuve", "Polytechnique", 2009),
            ],
            vec![
                Movie::new("9", "Ridley Scott", "The Martian", 2015),
            ],
            vec![
                Movie::new("11", "Quentin Tarantino", "Inglourious Basterds", 2009),
            ],
        ];

        assert_eq!(movies_by_director.group().len(), expected.len());
    }

    #[test]
    fn group_by_year() {
        let movies: Vec<Movie> = get_movies();
        let movies_by_year = Movies::new(GroupByYearStrategy, movies);
        let expected = vec![
            vec![
                Movie::new(
                    "1",
                    "Peter Jackson",
                    "The Hobbit: The Battle of the Five Armies",
                    2014,
                ),
                Movie::new("4", "Christopher Nolan", "Interstellar", 2014),
                Movie::new("2", "Wes Anderson", "The Grand Budapest Hotel", 2014),
            ],
            vec![
                Movie::new("3", "Christopher Nolan", "Tenet", 2020),
            ],
            vec![
                Movie::new("6", "Denis Villeneuve", "Blade Runner 2049", 2017),
            ],
            vec![
                Movie::new("9", "Ridley Scott", "The Martian", 2015),
            ],
            vec![
                Movie::new("5", "Wes Anderson", "Fantastic Mr. Fox", 2009),
                Movie::new("10", "Peter Jackson", "District 9", 2009),
                Movie::new("8", "Denis Villeneuve", "Polytechnique", 2009),
                Movie::new("11", "Quentin Tarantino", "Inglourious Basterds", 2009),
            ],
        ];

        assert_eq!(movies_by_year.group().len(), expected.len());
    }
}
