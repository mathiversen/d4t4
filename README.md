# Data parser

**WIP - work in progress, use at your own risk**

A modern, JSON compatible, data parser

## Features

- JSON compatible
- Trailing commas after objects and items in arrays
- Comments
- References (single & multiline)
- Single and double quotes

Parse normal JSON-files:

```
    {
        "hex": "\u1234",
        "bool": true,
        "objects": {},
        "arrays": [],
        "numbers": 1,
        "quote": "\"",
        "backslash": "\\",
        "controls": "\b\f\n\r\t",
        "strings": "check the tests directory for more test-cases"
    }
```

Parse data-files with super-json capabilities:

```
    {
        /*
            My favorite colors!
        */
        'color': {
            'r': '10',
            'g': '10',
            'b': '100',
        },
        'color': 'rgba(${color.r} ${color.g} ${color.b} 0.1)',
    }
```
