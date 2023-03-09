use strategy::{Movie, Movies, GroupByDirectorStrategy, GroupByYearStrategy};

fn main() {
    let movies: Vec<Movie> = vec![
        Movie::new("1", "Peter Jackson", "The Hobbit: The Battle of the Five Armies", 2014),
        Movie::new("3", "Christopher Nolan", "Tenet", 2020),
        Movie::new("2", "Wes Anderson", "The Grand Budapest Hotel", 2014),
        Movie::new("4", "Christopher Nolan", "Interstellar", 2014),
        Movie::new("5", "Wes Anderson", "Fantastic Mr. Fox", 2009),
        Movie::new("6", "Denis Villeneuve", "Blade Runner 2049", 2017),
        Movie::new("7", "Peter Jackson", "King Kong", 2005),
        Movie::new("8", "Denis Villeneuve", "Polytechnique", 2009),
        Movie::new("9", "Ridley Scott", "The Martian", 2015),
        Movie::new("10", "Peter Jackson", "District 9", 2009),
        Movie::new("11", "Quentin Tarantino", "Inglourious Basterds", 2009),
    ];

    let movies_by_director = Movies::new(GroupByDirectorStrategy, movies.clone());
    let movies_by_year = Movies::new(GroupByYearStrategy, movies.clone());

    println!("{:?}", movies_by_director.group());
    println!("{:?}", movies_by_year.group());
}
