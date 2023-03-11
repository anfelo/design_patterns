export enum SearchBy {
    Director = "director",
    Title = "title"
}

export interface ISearchStrategy {
    search(items: IMovie[], by: SearchBy, term: string): IMovie[];
}

export class BinarySearchStrategy implements ISearchStrategy {
    search(items: IMovie[], by: SearchBy, term: string): IMovie[] {
        const result: IMovie[] = [];

        let low = 0;
        let high = items.length - 1;
        let middle;

        while (low < high) {
            middle = low + Math.floor((high - low) / 2);

            if (items[middle][by] === term) {
                result.push(items[middle]);
                break;
            }

            if (items[middle][by] > term) {
                high = middle - 1;
            } else {
                low = middle + 1;
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
