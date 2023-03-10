import { BinarySearchStrategy, IMovie, LinearSearchStrategy, Movie, MoviesCatalog, SearchBy } from "./strategy";

function getMovies(): IMovie[] {
    return [
        new Movie({
            id: "1",
            director: "Peter Jackson",
            title: "The Hobbit: The Battle of the Five Armies",
            year: 2014,
        }),
        new Movie({ id: "3", director: "Christopher Nolan", title: "Tenet", year: 2020 }),
        new Movie({ id: "2", director: "Wes Anderson", title: "The Grand Budapest Hotel", year: 2014 }),
        new Movie({ id: "4", director: "Christopher Nolan", title: "Interstellar", year: 2014 }),
        new Movie({ id: "5", director: "Wes Anderson", title: "Fantastic Mr. Fox", year: 2009 }),
        new Movie({ id: "6", director: "Denis Villeneuve", title: "Blade Runner 2049", year: 2017 }),
        new Movie({ id: "8", director: "Denis Villeneuve", title: "Polytechnique", year: 2009 }),
        new Movie({ id: "9", director: "Ridley Scott", title: "The Martian", year: 2015 }),
        new Movie({ id: "10", director: "Peter Jackson", title: "District 9", year: 2009 }),
        new Movie({ id: "11", director: "Quentin Tarantino", title: "Inglourious Basterds", year: 2009 }),
    ];
}

describe("SearchStrategy", () => {
    test("LinearSearchStrategy", () => {
        const linearSearchStrat = new LinearSearchStrategy();
        const movies = getMovies();
        const moviesCatalog = new MoviesCatalog(linearSearchStrat, movies);

        expect(moviesCatalog.search(SearchBy.Title, "The").length).toEqual(3);
    });

    test("BinarySearchStrategy", () => {
        const binarySearchStrat = new BinarySearchStrategy();
        const movies = getMovies();

        // Must be sorted on order to use the binary sort
        movies.sort((a, b) => {
            const directorA = a.director.toUpperCase();
            const directorB = b.director.toUpperCase();

            if (directorA < directorB) {
                return -1;
            }
            if (directorA > directorB) {
                return 1;
            }

            return 0;
        });
        const moviesCatalog = new MoviesCatalog(binarySearchStrat, movies);

        expect(moviesCatalog.search(SearchBy.Director, "Ridley Scott").length).toEqual(1);
        expect(moviesCatalog.search(SearchBy.Director, "Ridley Scott")[0].director).toEqual("Ridley Scott");
    });
});

