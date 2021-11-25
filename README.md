# Sqlite database file structure visualization

Basically, it dissects sqlite database btree file, and visulize the btree node(page) relationship and node(page) content.

## Run the application

- http_main --file test-data/Chinbook.db.4.analyze

## Visualize

- Open http://127.0.0.1:8080/index.html
- Move mouse around btree node(page), it show parent node in green, its children nodes in red.
- Mouse left click to show node(page) content in json format.
- Mouse right click to  