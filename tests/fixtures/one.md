Lorem ipsum dolor sit amet.

```lua
-- Example 1: Missing 'do' in a for loop
for i = 1, 10
    print(i)
end
```

Lorem ipsum dolor sit amet.

```lua
-- Example 15: Nested loop with missing 'end' and incorrect variable scope (syntax & logical)
-- This combines a missing 'end' with a potential confusion about variable scope if it were
-- a valid loop, leading to multiple issues.
for i = 1, 5 do
    for j = 1, 3 do
        print(i * j)
    -- Missing 'end' for the inner loop here
end -- Missing 'end' for the outer loop as well
```
