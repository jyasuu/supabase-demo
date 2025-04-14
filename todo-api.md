# todo api

## Select id
```sh
curl 'http://localhost:8000/rest/v1/todos?select=id' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```

## Select user_id
```sh
curl 'http://localhost:8000/rest/v1/todos?select=user_id' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Select task
```sh
curl 'http://localhost:8000/rest/v1/todos?select=task' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Select is_complete
```sh
curl 'http://localhost:8000/rest/v1/todos?select=is_complete' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Select inserted_at
```sh
curl 'http://localhost:8000/rest/v1/todos?select=inserted_at' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Read all rows
```sh
curl 'http://localhost:8000/rest/v1/todos?select=*' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Read specific columns
```sh
curl 'http://localhost:8000/rest/v1/todos?select=some_column,other_column' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## Read referenced tables
```sh
curl 'http://localhost:8000/rest/v1/todos?select=some_column,other_table(foreign_key)' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```
## With pagination
```sh
curl 'http://localhost:8000/rest/v1/todos?select=*' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Range: 0-9"
```
## With filtering
```sh
curl --get 'http://localhost:8000/rest/v1/todos' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Range: 0-9" \
-d "select=*" \
\
`# Filters` \
-d "column=eq.Equal+to" \
-d "column=gt.Greater+than" \
-d "column=lt.Less+than" \
-d "column=gte.Greater+than+or+equal+to" \
-d "column=lte.Less+than+or+equal+to" \
-d "column=like.*CaseSensitive*" \
-d "column=ilike.*CaseInsensitive*" \
-d "column=is.null" \
-d "column=in.(Array,Values)" \
-d "column=neq.Not+equal+to" \
\
`# Arrays` \
-d "array_column=cs.{array,contains}" \
-d "array_column=cd.{contained,by}" \
\
`# Logical operators` \
-d "column=not.like.Negate+filter" \
-d "or=(some_column.eq.Some+value,other_column.eq.Other+value)"
```

## Insert a row
```sh
curl -X POST 'http://localhost:8000/rest/v1/todos' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Content-Type: application/json" \
-H "Prefer: return=minimal" \
-d '{ "some_column": "someValue", "other_column": "otherValue" }'
```
## Insert many rows
```sh
curl -X POST 'http://localhost:8000/rest/v1/todos' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Content-Type: application/json" \
-d '[{ "some_column": "someValue" }, { "other_column": "otherValue" }]'
```
## Upsert matching rows
```sh
curl -X POST 'http://localhost:8000/rest/v1/todos' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Content-Type: application/json" \
-H "Prefer: resolution=merge-duplicates" \
-d '{ "some_column": "someValue", "other_column": "otherValue" }'
```

## Update matching rows
```sh
curl -X PATCH 'http://localhost:8000/rest/v1/todos?some_column=eq.someValue' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY" \
-H "Content-Type: application/json" \
-H "Prefer: return=minimal" \
-d '{ "other_column": "otherValue" }'
```

## Delete matching rows
```sh
curl -X DELETE 'http://localhost:8000/rest/v1/todos?some_column=eq.someValue' \
-H "apikey: SUPABASE_KEY" \
-H "Authorization: Bearer SUPABASE_KEY"
```