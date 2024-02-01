# truth-table-generator

Generates fancy Markdown truth tables from the given boolean expression.

## Example

```md
> a && b || c
|-------+-------+-------+-------|
| a     | b     | c     | F     |
|-------+-------+-------+-------|
| false | false | false | false |
| false | false | true  | true  |
| false | true  | false | false |
| false | true  | true  | true  |
| true  | false | false | false |
| true  | false | true  | true  |
| true  | true  | false | true  |
| true  | true  | true  | true  |
|-------+-------+-------+-------|

> (x || z) && (not x || y) && (z || y)
|-------+-------+-------+-------|
| x     | y     | z     | F     |
|-------+-------+-------+-------|
| false | false | false | false |
| false | false | true  | true  |
| false | true  | false | false |
| false | true  | true  | true  |
| true  | false | false | false |
| true  | false | true  | false |
| true  | true  | false | true  |
| true  | true  | true  | true  |
|-------+-------+-------+-------|
```
