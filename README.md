# Data parser, a JSON-superset and data format

## Data specification

JSON Compatible

```
    {
        "format": "data",
        "name": "data parser"
    }
```

Super capabilities

```
    {
        'format': 'data',
        'name': '${format} parser',
    }
```

- Supports cross-data referencing (higher data integrity)
- Supports mutiple quotation formats: double, single, backticks
- Supports dangling commas (easier to create datafiles)
