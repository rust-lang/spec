# Layout [§](r[dynamic.layout])

## Layout Overview [§](r[dynamic.layout.overview])

[!NOTE]: This section is not normative

[!Note]: All sizes and alignments in this section are in bytes.

| Type | Size | Alignment |  Valid           |
|------|------|-----------|------------------|
| `u8`/`i8` | `1`  | `1`       | When initialized |
| `u16`/`i16`| `2`  | `<=2`[^1]    | When initialized |
| `u32`/`i32`| `4`  | `<=4`[^1]    | When initialized |
| `u64`/`i64`| `8`  | `<=8`[^1]    | When initialized |
|`u128`/`i128`| `16` | `<=16`[^1]   | When initialized |
|`char`| `4`  | `<=4`[^1]    | `..0xD800, 0xE000..0x110000`[^4]|
|`bool`| `1`  | `1`       | `..2`[^4] |
|`usize`[^2]| any[^1] | `<=size`[^1] | When initialized |
|`*mut impl Thin`[^2]| any[^1] | `<=size`[^1] | When initialized |
|`*const impl Thin`[^2]|any[^1] | `<=size`[^1] | When initialized |
|`&T`|Same as `*const T`| Same as `*const T`| Non-null, aligned to `T`[^4][^5]|
|`&mut T`|Same as `*mut T`| Same as `*mut T`| Non-null, aligned to `T`[^4][^5]|
|`Option<&T>`|Same as `*const T` | Same as `*const T`| aligned to `T`[^4][^5]|
|`Option<&mut T>`|Same as `*mut T` | Same as `*mut T`| aligned to `T`[^4][^5]|
|`fn()`[^3]| any[^1] | `<=ssize`[^1] | Non-null[^4] |
| `Option<fn()>`[^3]| any[^1] | `<=size`[^1] | When initialized |
| `[T;N]` | `size_of::<T>()*N` | Same as `T` | Each element valid |
| `(T0,T1,..Tn)`[^6] | At least total of `T0`, `T1`, .. `Tn` + padding | At least maximum of `T0`, `T1`, ..`Tn` | Each element valid |
| `()` | `0` | `1`  | Always |
| `struct Foo{f0: T0, f1: T1, ..fN: Tn}`[^6][^7] | At least total of `T0`, `T1`, .. `Tn` + padding | At least maximum of `T0`, `T1`, ..`Tn` | Each field valid |
| `struct Foo{}`[^7][^8] | `0` | `1` | Always |
| `union Foo{f0: T0, f1: T1, ..fN; Tn}`[^6] | At least maximum of `T0`, `T1`, ..`Tn` | At least maximum of `T0`, `T1`, ..`Tn` |  |

[^1]: Target defined

[^2]: `usize` and pointers to `Thin` types have the same size and alignment requirement. Neither type has any padding bytes.

[^3]: `fn()` refers to any fn-pointer type, regardless of return type, arguments, or ABI Tag

[^4]: Range of valid values when computing as a suitably sized integer type (or pointer). Implicitly imposes a constraint of "When initialized"

[^5]: References, and `Option`s of references (when `Some`) have additional aliasing constraints, see r#[dynamic.alias].

[^6]: `repr(Rust)` types have an unspecified and unstable layout

[^7]: Same as equivalent tuple struct. Struct size and alignment requirement may be modified by the `#[repr(align)]`

[^8]: Same as equivalent unit struct

## Layout Properties [§](r[dynamic.layout.properties])

[§](r[dynamic.layout.properties.size]) Each `Sized` type has a property called it's size. The size is a non-negative integer.

[!NOTE]: For example, the size of `u8` is `1`, and the size of `u32` is `4`.

[§](r[dynamic.layout.properties.align]) Each `Sized` type and the slice type has a property call its alignment requirement.  The alignment requirement is a positive integer power of 2.

[!NOTE]: The alignment requirement may simply be called the alignment or align of the type.

[§](r[dynamic.layout.properties.size-align-value]) The size and alignment of any type is representible as a value of type `usize`. The alignment of a type divides its size.

[!NOTE]: The minimum size of a type is `0`, and the minimum alignment requirement is `1`. A type that has size `0` and alignment requirement `1` is called a 1-ZST.

[§](r[dynamic.layout.properties.storage]) Storage for a value is suitable for storing a value of a `Sized` type if it is at least the size of that type and the address of the start of the storage satisfies the alignment requirement.

[§](r[dynamic.layout.properties.representation]) Each `Sized` type has a representation which determines how particular values of that type are layed out when read from or written to suitable storage. The representation of a type is exactly its size.

[§](r[dynamic.layout.properties.padding]) The representation of a type may include padding bytes that do not participate in the computation of the value. All other bytes of the representation are called value bytes.

[!NOTE]: Scalar types and pointer types have no padding bytes.

[§](r[dynamic.layout.properties.validity]) Each `Sized` type has a validity invariant, which constrains the values that can be read from storage. Only value bytes are taken into account in determining the validity of a value.

[§](r[dynamic.layout.properties.underlying]) Each `Sized` type may have an underlying type, which is also `Sized`. Such a type has the same size, alignmentment, representation, and pointer metadata type as the underlying type. Such a type may have an additional validity invariant, and also has the validity invariant of the underlying type.

[!NOTE]: A repr(transparent) type has its transparent field as an underlying type

[§](r[dynamic.layout.properties.ptr-metadata]) Each type has a property called the pointer metadata type. The pointer metadata type is used in determining the layout of pointers to that type.

[§](r[dynamic.layout.properties.ptr-metadata-thin]) The pointer metadata type of any `Thin` type is `()`.

## Scalar Layout [§](r[dynamic.layout.scalar])

[§](r[dynamic.layout.scalar.int-size]) Each integer type of width `N` has a size exactly equal to `N/8`. 

[1NOTE]: The width of the integer type `uN` or `iN` is `N`. 

[§](r[dynamic.layout.scalar.int-align]) Each integer type has a target-dependendant alignment requirement which is at most its size. 

[§](r[dynamic.layout.scalar.unsigned-repr]) Each unsigned integer type is represented as a sequence of bytes, in a target-dependenant order of increasing signifiance. The value, decomposed as groups of order 2^8, is layed out in the target-dependenant signifiance order.

[!NOTE]: The order of the bytes of an integer larger than 1 byte is known as the endianness. All bytes representing an integer type are value bytes.

[§](r[dynamic.layout.scalar.signed-repr]) Each signed integer type is represented the same as the equivalent value of the corresponding unsigned integer type, taken modulo `2^N`.

[§](r[dynamic.layout.scalar.int-validity]) A given value of an integer type is valid if it was computed with only initialized bytes.

[§](r[dynamic.layout.scalar.int-ptr-bytes]) Any pointer portion of a byte used in computing a value of an integer type is discarded.

[§](r[dynamic.layout.scalar.int-size]) The special types `usize` and `isize` are integer types with the same target-dependent width.

[!NOTE]: The width of `usize` and `isize` do not need to correspond to the width of any other integer type. If it does, then they have the same representation as that integer type.

[§](r[dynamic.layout.scalar.char]) The type `char` has an underlying type of `u32`. 

[§](r[dynamic.layout.scalar.char-validity]) A given value of type `char` is valid if the corresponding value of type `u32` is valid, and does not lie in the range `0xD800..0xE000` or the range `0x110000..`. 

[§](r[dynamic.layout.scalar.bool]) The type `bool` has an underlying type of `u8`.

[§](r[dynamic.layout.scalar.bool-repr]) The value `false` is represented the same as the value `0_u8`, and the value `true` is represented the same as the value `1_u8`

[§](r[dynamic.layout.scalar.bool-validity]) A given value of type `bool` is valid if the corresponding value of type `u8` is valid, and the value lies in the range `0..2`.

[§](r[dynamic.layout.scalar.float]) A floating point type with width `N` has an underlying type of `uN`.

[§](r[dynamic.layout.scalar.float-repr]) A value of a floating-point has the representation corresponding to the value of the unsigned integer type computed from the value given by the appropriate interchange format from [IEEE 754](https://ieeexplore.ieee.org/document/5976968).

[!NOTE]: The corresponding value may be called the bit representation of the floating-point value. 

## Pointer Layout [§](r[dynamic.layout.pointer])

[§](r[dynamic.layout.pointer.thin]) A pointer to a `Thin` type has a data portion only, which represents an address and a pointer tag. 

[!NOTE]: Such pointers are called thin pointers. All `Sized` types are also `Thin`. More types may be considered `Thin` in future versions of the spec. The layout of `*const T` and `*mut T` are the same.

[§](r[dynamic.layout.pointer.size-align]) The size and alignment of a thin pointer is the same as the size and alignment of `usize`. 

[!NOTE] All thin pointers have the same

[§](r[dynamic.layout.pointer.repr]) The address of a pointer value is the same as the value of type `usize` computed from the same bytes. The pointer tag of a pointer value is each pointer portion of each byte of the representation. 

[!NOTE]: If the bytes do not have a pointer portion, then the resulting pointer is an address-only pointer. 

[§](r[dynamic.layout.pointer.validity]) A given value of a thin pointer type is valid if it was computed from only initialized bytes.

[§](r[dynamic.layout.pointer.wide]) A pointer to a `!Thin` type has both a data portion and a metadata portion. 

[§](r[dynamic.layout.pointer.wide-underlying]) The wide pointer type `*mut T` has an underlying type of the given *exposition only* definition, where `M` is the pointer metadata type of `T`. The wide pointer type `*const T` has the same underlying type
```rust
struct WidePtr{
    data: *mut (),
    metadata: M
}
```

[§](r[dynamic.layout.pointer.wide-validity]) A given value of a pointer type is valid if the data portion is valid and the metadata portion is valid.

[§](r[dynamic.layout.pointer.ref]) The reference type `&T` has an underlying type of `*const T`. The reference type `&mut T` has an underlying type of `*mut T`.

[§](r[dynamic.layout.pointer.ref-validity]) A value of a reference type is valid if the pointer value has an address which is not the value `0` and is a multiple of the dynamic alignment requirement of the value.

[!NOTE]: The dynamic alignment requirement is the alignment requirement of the type if the type has one, or otherwise if the pointer metadata type is a trait object vtable pointer, the alignment requirement accessible from the pointer metadata. A pointer with the address `0` is known as a null pointer.

[§](r[dynamic.layout.pointer.fn-size-align]) A function pointer type has an target dependant size and alignment requirement. The size and alignment does not depend on the parameters, return type, or ABI tag of the function pointer type.

[!NOTE]: This is typically, but not always, the same as the size and alignment requirement of `*const ()`.

[§](r[dynamic.layout.pointer.fn-repr]) The address of a function pointer value is the same as the value of type `usize` computed from the same bytes. The pointer tag of a pointer value is each pointer portion of each byte of the representation. 

[§](r[dynamic.layout.pointer.fn-validity]) A value of an function pointer type is valid if the address of the pointer is not `0`.

## Aggregate Layout [§](r[dynamic.layout.aggregate])

[§](r[dynamic.layout.aggregate.fields]) Each aggregate type has a list of fields, that each have a type and offset. 

[!NOTE]: This includes built-in tuple types and arrays. This does not include `enum` definitions.

[§](r[dynamic.layout.aggregate.struct-base] The fields of a `struct` definition are each layed at offsets that ensure that each field occupies nonoverlapping storage, which, unless modified by the `repr(packed)` attribute (r#[dynamic.layout.aggregate.packed])) is suitably aligned. The size of a `struct` definition is at least sufficient to store each field, and the alignment requirement of a `struct` definition, unless modified by the `repr(packed)` attribute, is suitable to align each field.

[!NOTE]: The offsets of the fields, sizes, and alignment requirements of two different `struct` types, even with the same field types in the same order, may be different.

[§](r[dynamic.layout.aggregate.union-base]) The fields of a `union` definition are each layed out at offsets that ensure they occupy storage, which, unless modified by the `repr(packed)` attribute is suitably aligned. The size of a `union` definition is at least sufficient to store its largest field, and the alignment requirement of a `union` definition, unless modified by the `repr(packed)` attribute, is suitable to align the field with the strictest alignment requirement.

[!NOTE]: The fields of a union may overlap, and may have the same offset.

[!NOTE]: The offsets of the fields, sizes, and alignment requirements of two different `union` types, even with the same field types in the same order, may be different.

[§](r[dynamic.layout.aggregate.zero-sized]) A `struct` definition with no fields has size `0` and, unless modified by the `repr(align)` attribute, alignment requirement `1`.

[!NOTE]: This constraint applies regardless of any `repr` attributes present, other than `repr(align)`.

[§](r[dynamic.layout.aggregate.repr]) A `#[repr]` attribute may be applied to a `struct` or `union` definition. The attribute takes a list of repr attributes, all of which are apply a constraint to the layout of the type. The `#[repr]` attribute, if present on a `struct` or `union` definition, may contain any of the following repr attributes:
* `Rust`
* `C`
* `transparent`
* `align(N)`
* `packed(N)`

[§](r[dynamic.layout.aggregate.repr-rust]) The field offsets, the size, and the alignment requirement of a `struct` or `union` definition with the `repr(Rust)` repr-attribute may be any valid value. `repr(Rust)` may not be combined with `repr(C)`

[!NOTE]: This is the default layout behaviour of a `struct` or `union` definition.

[!WARN]: The default layout for a `union` does not guarantee that each field starts at offset `0`.

[§](r[dynamic.layout.aggregate.repr-c]) The field offsets of a `struct` or `union` definition with the `repr(C)` repr-attribute are the minimum valid offset for the field such that the field offsets in declaration order are increasing. The size and alignment requirement of such a `struct` or `union` definition are the minimum possible valid size with the given field offsets.

[!NOTE]: For `union`s, the offset of each field is `0` and the size is the size of the largest field. For `struct`s, the offset of the first field is `0`, and each subsequent field's offset is the offset of the previous field, plus it's size, plus the minimum number such that the offset is aligned to the field's alignment requirement.

[§](r[dynamic.layout.aggregate.transparent]) The `repr(transparent)` repr-attribute may be applied to a `struct` definition with no other repr-attribute and with at most 1 field that has a size greater than `0` or an alignment requirement greater than `1`. If such a field exists, the `struct` definition has an underlying type of the type of that field and that field has offset `0`. Otherwise, the `struct` definition has an underlying type of `()`. `repr(transparent)` may not be applied to a `union` definition.

[!NOTE]: The offsets of 1-ZST fields of such a `struct` are not specified, but do not exceed the size of the underlying type.

[§](r[dynamic.layout.aggregate.repr-align]) A `struct` or `union` definition with a `repr(align(N))` repr-attribute, where `N` is an integer power of 2 up to a target-defined value, has an alignment requirement of at least `N`. The `repr(align(N))` repr-attribute may not be combined with any `repr(packed(N))` repr-attributes.

[§](r[dynamic.layout.aggregate.repr-packed]) A `struct` or `union` definition with a `repr(packed(N))` repr-attribute, where `N` is an integer power of 2 up to a target-defined value, sets the alignment requirement of each field to the smallest of the alignment requirement of the type and `N`. The `repr(packed(N))` repr-attribute may not be combined with any `repr(align(N))` repr-attributes.

[!NOTE]: The repr-attribute `repr(packed(1))` may be written as `repr(packed)`

[§](r[dynamic.layout.aggregate.array]) The array type `[T;N]` has `N` consecutive fields of type `T`. The offsets of each field are the minimum offset in order such that each field is nonoverlapping.

[!NOTE]: The "fields" of an array type are known as its elements.

[§](r[dynamic.layout.aggregate.array-size-align]) The size of an array type is the size of `T` times `N`. The alignment requirement of an array type is the alignment requirement of `T`.

[§](r[dynamic.layout.aggregate.tuple]) The built in tuple type with `n` elements `(T0, T1, ..Tn)` has an underlying type of the *exposition-only definition `struct Tuple(T0, T1, ..Tn)`.

[!NOTE]: This means that the unit type `()` has size `1` and alignment requirement `0`. Built-in tuples have no requirements for field offsets other than those of r#[dynamic.layout.aggregate.struct-base].

[§](r[dynamic.layout.aggregate.repr]) The representation of an aggregate type is the representation of each of its fields placed at the offset of the field. A byte of the representation is a padding byte if it is not a value byte in the representation of any field that overlaps that byte of the representation.

[!NOTE]: Portions of union fields that overlap are represented only once from all fields.

[§](r[dynamic.layout.aggregate.validity]) An aggregate type, other than a union, is valid if each field is valid.

[§](r[dynamic.layout.aggregate.slice-align]) The alignment requirement of a slice type `[T]` is the alignment requirement of  `T`.

[§](r[dynamic.layout.aggregate.slice-metadata]) The pointer metadata type of a slice type is `usize`. 

[§](r[dynamic.layout.aggregate.struct-metadata]) The pointer metadata type of a `struct` type is the pointer metadata type of the last field of the struct in declaration order.

[!NOTE]: If the struct has no fields, then it is `Sized` and thus has a pointer metadata type of `()`.

[§](r[dynamic.layout.aggregate.str]) The `str` type has an underlying type of `[u8]`.

## Trait Object Type [§](r[dynamic.layout.dyn])

[§](r[dynamic.layout.dyn.metadata]) A trait object type `dyn Trait+Markers` has an unspecified pointer metadata type that has an underlying type of `*const VTable<Trait>` where the type `VTable<Trait>` is an unspecified `Thin` type. 

[!NOTE]: The pointer metadata type is called the vptr type.

[§](r[dynamic.layout.dyn.metadata-repr]) The representation of the vptr type is an unspecified address and unspecified pointer tag.

[§](r[dynamic.layout.dyn.metadata-validity]) The validity invariant of the vptr type is unspecified, such that the result of unsizing a type as the trait object `dyn Trait+Markers` is valid.

[§](r[dynamic.layout.dyn.metadata-size-align]) The size and alignment requirement acessible from a value of the vptr type obtained as the result of unsizing a type as the trait object `dyn Trait+Markers` is valid are the size and alignment of that type.

## Enum definitions [§](r[dynamic.layout.enum])


[§](r[dynamic.layout.enum.variants]) Each enum type has a list of variants, each of which has a list of fields.

[§](r[dynamic.layout.enum.field-offset]) Each field of a variant has an offset, such that, given suitable storage for the enum, each field of each variant has suitable storage starting at the offset of that field, and the storage for each field does not overlap with the storage for any other field of the same variant.

[§](r[dynamic.layout.enum.repr]) The representation of an enum is the representation of the corresponding variant. The representation of an enum variant is such that, if all fields of the variant are valid, each field of the variant is represented at the offset of that field, and the representation is distinguishable from the representation of any other variant with all fields of that variant valid.

[!NOTE]: A variant is distinguishable from another variant if, given all of its fields are valid, the representation of that variant also does not represent the other variant, such that all of its fields are valid.

[§](r[dynamic.layout.enum.value]) A byte of the representation of an enum is a value byte if it is used as a value by in the representation of a field of any variant. An unspecified set of all other bytes of the representation are padding bytes.

[§](r[dynamic.layout.enum.validity]) A value of an enum type is valid if it is one of the variants of the enum, and each of the fields of that variant are valid. 

[§](r[dynamic.layout.enum.repr-attr]) A `#[repr]` attribute may be applied to an `enum` definition. The attribute takes a list of repr attributes, all of which are apply a constraint to the layout of the type. The `#[repr]` attribute, if present on a `enum` definition, may contain at most one of the following repr attributes, and may optionally have the repr attribute `align(N)`:
* `Rust`
* `C`
* `C,Int` where `Int` is the name of an integer type
* `Int` where `Int` is the name of an integer type.
 
[§](r[dynamic.layout.enum.repr-int]) An `enum` definition with the `Int` repr attribute, where `Int` is the name of an integer type, and with `n` variants `V0`, `V1`, `..Vn`, where variant `i<n` has `m` fields with types `ViF0`, `ViF1`, `..ViFm`, has an underlying type of the following *exposition only* `union` declaration, which is modified by the `align(N)` repr attribute if the `enum` definition is
```rust
#[repr(C)]
pub union Repr{
    pub V0: V0Repr,
    pub V1: V1Repr,
    ..pub Vn: VnRepr,
}

#[repr(C)]
pub struct V0Repr(Int, V0F0, V0F1, ..V0Fm);
#[repr(C)]
pub struct V1Repr(Int, V1F0, V1F1, ..V1Fm);
..#[repr(C)]
pub struct VnRepr(Int, VnF0, VnF1, ..VnFm);
```


[§](r[dynamic.layout.enum.repr-c-int]) An `enum` definition with the `C,Int` repr-attribute, where `Int` is the name of an integer type, and with `n` variants `V0`, `V1`, `..Vn`, where variant `i<n` has `m` fields with types `ViF0`, `ViF1`, `..ViFm`, has an underlying type of the following *exposition only* `struct` declaration, which is modified by the `align(N)` repr attribute if the `enum` definition is
```rust
#[repr(C)]
pub struct Repr(Int, VariantsRepr);
#[repr(C)]
pub union VariantsRepr{
    pub V0: V0Repr,
    pub V1: V1Repr,
    ..pub Vn: VnRepr,
}

#[repr(C)]
pub struct V0Repr(V0F0, V0F1, ..V0Fm);
#[repr(C)]
pub struct V1Repr(V1F0, V1F1, ..V1Fm);
..#[repr(C)]
pub struct VnRepr(VnF0, VnF1, ..VnFm);
```

[§](r[dynamic.layout.enum.repr-c]) An `enum` definition with the `C` repr-attribute is the same as the same definition with the `C,Int` repr-attribute, where `Int` is a *target dependenant* signed integer type.

[§](r[dynamic.layout.enum.discrim]) For an enum declaration with the `C,Int` repr-attribute, or the `C` repr attribute, the first field of the exposition only `Repr` struct is initialized to the discriminant of the variant which is being represented, and the fields of the corresponding variant repr type are initialized to the fields of that variant. For an enum declaration with the `Int` repr-attribute, the first field of the variant repr type is the discriminant of the variant, and the remaining fields are initialized to the fields of that variant.

[§](r[dynamic.layout.enum.size-align]) The size of an `enum` definition is at least sufficient to store its representation. The alignment of an `enum` definition is at least sufficient to satisfy the alignment requirement of all fields of all variants in its representation.

[!NOTE]: In the case of an `enum` with a repr-attribute other than `repr(Rust)` or `repr(align(N))`, the size and alignment are given by the appropriate *exposition only* `Repr` type.

[§](r[dynamic.layout.enum.repr-align]) If the `align(N)` repr-attribute is present on an `enum` definition, then the alignment requirement of the enum is at least `N`.

[§](r[dynamic.layout.enum.option]) The special type `Option<T>` is an enum type, such that if `T` is one of the following types or a type with an underlying type that is one of the following (recursively), it has the corresponding underlying type.
* `&T`: `*const T`
* `&mut T`: `*mut T`
* fn-ptr type: An unspecified, exposition-only, type with the same size and alignment as `fn()` that is valid for any initialized value
* `core::num::NonZeroUN`: `uN`
* `core::num::NonZeroIN`: `iN`
* `core::ptr::NonNull<T>`: `*mut T`
* `alloc::boxed::Box<T>`: `*mut T`

[§](r[dynamic.layout.enum.option-repr]) The representation of the value `None` of type `Option<T>` where `T` is any type refered to in r#[dynamic.layout.enum.option] is the initialized value with each value byte set to `0` with no pointer part, except that the metadata field of `Option<&T>`, `Option<&mut T>`, `Option<NonNull<T>>`, or `Option<Box<T>>` (where `T` is `!Thin`) is unspecified. The representation of the value `Some(x)` of type `Option<T>` where `T` is any type refered to in r#[dynamic.layout.enum.option] is the representation of `x`.

[§](r[dynamic.layout.enum.option-ref-validity]) A value of type `Option<&T>` or `Option<&mut T>` is valid if it corresponds to a pointer with address `0`, or a pointer with an address that satisfies the dynamic alignment requirement of `T`.

[§](r[dynamic.layout.enum.result]) The special type `Result<T,E>` where `E` has size 0 and alignment 1, and `T` is a type mentioned in r#[dynamic.layout.enum.option], then `Result<T,E>` has an underlying type of `Option<T>`. The special type `Result<T,E>` where `T` has size 0 and alignment 1, and `E` is a type mentioned in r#[dynamic.layout.enum.option], then `Result<T,E>` has an underlying type of `Option<T>`

[§](r[dynamic.layout.enum.result-repr]) Where `Result<T,E>` is a type referred to by r#[dynamic.layout.enum.result] The representation of the value `Err(e)` of type `Result<T,E>` is the same as the representation for `Some(e)` of type `Option<E>` if `T` has size 0 and alignment 1, and `None` of type `Option<T>` otherwise, and the representation of `Ok(t)` of type `Result<T,E>` is the same as the representation for `Some(t)` of type `Option<T>` if `E` has size 0 and alignment 1, and `None` of the type `Option<E>` otherwise.

