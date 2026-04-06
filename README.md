# Reproducing

1. Run

    ```sh
    cargo r
    ```

1. open http://localhost:3000/graphiql
1. query `{hello(unknown: true)}`
1. observe the absence of validation error
