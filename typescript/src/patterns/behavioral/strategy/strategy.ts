export enum SearchBy {
  Director = "director",
  Title = "title",
}

export interface ISearchStrategy {
    search(items: IMovie[], by: SearchBy, term: string): IMovie[]
}

export class BinarySearchStrategy implements ISearchStrategy {
    search(items: IMovie[], by: SearchBy, term: string): IMovie[] {
        const result: IMovie[] = [];

        let start = 0;
        let end = items.length - 1;
        let middle = Math.floor(end / 2);

        while (start < end) {
            if (items[middle][by] === term) {
                result.push(items[middle]);
                break;
            }

            if (items[middle][by] > term) {
                end = middle;
                middle = Math.floor(end / 2);
            } else {
                start = middle;
                middle = start + Math.floor((end - middle) / 2);
            }
        }

        return result;
    }
}

export class LinearSearchStrategy implements ISearchStrategy {
    search(items: IMovie[], by: SearchBy, term: string): IMovie[] {
        const result: IMovie[] = [];

        for (let i = 0; i < items.length; i++) {
            const value = items[i][by];

            if (value.includes(term)) {
                result.push(items[i]);
            }
        }

        return result;
    }
}

export interface IMovie {
    id: string;
    title: string;
    director: string;
    year: number;
}

export class Movie implements IMovie {
    id: string;
    title: string;
    director: string;
    year: number;

    constructor({ id, title, director, year }: IMovie) {
        this.id = id;
        this.title = title;
        this.director = director;
        this.year = year;
    }
}

export class MoviesCatalog {
    searchStrategy: ISearchStrategy;
    items: IMovie[];

    constructor(searchStrat: ISearchStrategy, items: IMovie[]) {
        this.searchStrategy = searchStrat;
        this.items = items;
    }

    search(by: SearchBy, term: string): IMovie[] {
        return this.searchStrategy.search(this.items, by, term);
    }
}

