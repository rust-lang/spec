## Array Construction [exprs.array]

Syntax:
```abnf 
array-expr := "[" [<expr> [*("," <expr>)] [","] ] "]"

simple-expr /= <array-expr>
```

1. Constraints: Each `expr` must, after implicit coercions, have the same type `T` such that `T: core::marker::Sized`.

2. The array-expr `[E0, E1, .., En]` constructs the `n` element array of type `T`, such that the `i`th element of the resulting array is equivalent to `Ei` for `0<=i<n`. 

3. Given the array-expr `[E0, E1, .., En]`, each `Ei` for `0<=i<n` is a value expression, and the array-expr is a value expression.

4. An array-expr is ill-formed if `n` is not representible as a value of type `usize`, or if the size of the resulting array type is greater than `isize::MAX as usize`.

[Note 1: The latter constraint means that the effectively largest `n` is `isize::MAX`, unless `T` is a zero-sized type. Types larger than 1 byte further constraint `n`]

5. If all of `E0, E1, .., En` are *constant expressions*, then `[E0, E1, .., En]` is a *constant expression*.


Syntax:
```abnf
repeat-expr := "[" <expr> ";" <expr> "]"

simple-expr /= <repeat-expr>
```

1. Constraints: Given the repeat-expr `[val ; N]`,  `N` shall be a *potentially-dependant* constant expression of `usize`, and `val` shall be of a type `T`, such that `T: core::marker::Sized`. If `N` is a *dependent constant expression* or `N > 2`, then either `T: core::marker::Copy` or `val` shall be a (potentially parenthesized) path-expr that denotes a const item. 

2. The repeat-expr `[val;N]` constructs the `N` element array of type `T`, such that the `i`th element of the resulting array is equivalent to `val`, for `0<=i<N`. 

3. If `N` is equal to zero, then `val` is dropped. If `N` is equal to 1, then `val` is evaluated exactly once as the sole element of the resulting array. Otherwise, if `val` is a (potentially parenthesized) path-expr that denotes a const item, then it is evaluated `N` times to initialize each corresponding element of the array. Otherwise, `val` is evaluated exactly once, and copied to each element of the array. 

4. Given the repeat-expr `[val;N]`, `val` is a value expression, and the resulting repeat-expr is a value expression.

5. A repeat-expr is ill-formed if the size of the resulting array type is greater than `isize::MAX as usize`.

[Note 1: The latter constraint means that the effectively largest `N` is `isize::MAX`, unless `T` is a zero-sized type. Types larger than 1 byte further constraint `n`]

6. If `val` is a *constant expression*, then `[val; N]` is a *constant expression*.

