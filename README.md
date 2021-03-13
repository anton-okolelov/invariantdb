# InvariantDB
*Warning* - это недочерновик, не смотрите дальше и не запускайте)


Это СУБД, расчитанный на более высокоуровневую работу с данными, чем это позволяют классические реляционные SQL-базы.
## Для чего? 


### Простой SQL (postgres)
```sql
CREATE TABLE authors (
    id bigserial primary key,
    name text
);

CREATE TABLE books (
    id bigserial primary key,
    title text
);

-- вспомогательная таблица
CREATE TABLE authors_books (
    book_id bigint REFERENCES books(id),
    author_id bigint REFERENCES authors(id),
);

INSERT INTO authors (name) VALUES ('Lev Tolstoy');

INSERT INTO books (title) VALUES ('War and Peace');

INSERT INTO authors_books (book_id, author_id) VALUES (1, 1);

SELECT b.id, b.title, a.name
FROM books
    JOIN authors_books ab 
        ON books.id = ab.book_id
    JOIN authors a 
        ON ab.author_id = a.id
WHERE a.id = 1;


DELETE FROM authors_books WHERE author_id = 1 AND book_id = 1;
DELETE FROM authors WHERE id = 1;
DELETE FROM books WHERE id = 1;
```

### Тоже самое на invariant db:

```
DEFINE ENTITY Author  (
    id bigint default auto_increment
    name string
) IDENTIFIED BY id;

DEFINE ENTITY Book (
   id bigint
   title string
   authors []Author
) IDENTIFIED BY id;

CREATE Book {
   "title": "War and Peace",
   "authors": [{
        "name": "Lev Tolstoy"   // автоматом создастся автор 
   }]
}

SELECT 
    id, title, authors{id, name} 
FROM 
    Book 
WHERE 
    authors CONTAINS Author{"id": 1};

[
    {"id": 1, "title": "War and Peace", authors: [{"id": 1, "name": "Lev Tolstoy"}]}
]


DELETE Book WHERE Book.authors CONTAIN id = 1;
DELETE Author WHERE id = 1 ;


```
