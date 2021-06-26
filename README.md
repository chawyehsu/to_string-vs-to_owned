`to_string` or `to_owned`?
---

**Q**: In Rust, both `to_string()` and `to_owned()` can be used to convert a
[`&str`][1] to a [`String`][2]. So what is the idiomatic way to do this
conversion? Do they have any performance differences?

**A**: The answer for the question of performance differences is **NO**. These
two methods do not have any performance difference. Let's take a look at the
source code of these two methods.

**to_string**:
```rust
// alloc::string
#[stable(feature = "str_to_string_specialization", since = "1.9.0")]
impl ToString for str {
    #[inline]
    fn to_string(&self) -> String {
        String::from(self)
    }
}
```

**to_owned**:
```rust
// alloc::str
#[stable(feature = "rust1", since = "1.0.0")]
impl ToOwned for str {
    type Owned = String;
    #[inline]
    fn to_owned(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }

    fn clone_into(&self, target: &mut String) { ... }
}
```

As we can see, `to_string()` just calls `String::from`. Let's dive into it.

```rust
// alloc::string
#[stable(feature = "rust1", since = "1.0.0")]
impl From<&str> for String {
    #[inline]
    fn from(s: &str) -> String {
        s.to_owned()
    }
}
```

So actually it just calls `to_owned()`. Since they're all inlined, thus there’s
no extra performance cost. Therefore you can use whichever you feel like, there's
no *idiomatic way*.

#### bonus

In fact, `into()` also can be used to convert a `&str` to a `String`, but this
way requires a type annotation for the variable.

```rust
let s: String = "hello, rust!".into();
```

And a `String` annotated call to `into()` just calls `String::from`, which is
the same function `to_string()` calls.

```rust
// core::convert

// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

Although there's no extra performance cost to any of them, they're different
semantically.

 - `to_owned()`: I have a borrowed object and I want an owned version
 - `to_string()`: I want the textual representation of something
 - `into()`: I want a generic type conversion

Therefore, it's better to choose the proper method according to the semantic
context of the code logic you're writing. For converting `&str` to `String`,
I prefer `to_owned()` over the others as per semantics. The difference between
`String` ans `&str` is the ownership, one is owned and the other is not owned.

#### Why you wrote this?

Just like any google oriented programmer, there are often questions that come
from my mind when I'm learning a new thing. I want to know the difference
between `to_owned()` and `to_string()`, then I googled `to_string to_owned` and
the first result it gives me was [Converting &str: to_string vs to_owned (with two benchmarks)][3]. I read the post and coundn't believe the performance difference
they said. After more researches I realized that the post is wrong (or maybe is
outdated becuase it was posted in 2016), and I worried about there might be other
new Rustaceans read that post reached from google search and get the outdated
conclusion. Therefore I wrote this with the name `to_string or to_owned?` for
easy searching purpose, and this is the reason.

### References

- [`to_string()` vs `to_owned()` for string literals][4]
- [What is the idiomatic way to convert &str to String?][5]

[1]: https://doc.rust-lang.org/std/primitive.str.html
[2]: https://doc.rust-lang.org/std/string/struct.String.html
[3]: https://medium.com/@ericdreichert/converting-str-to-string-vs-to-owned-with-two-benchmarks-a66fd5a081ce
[4]: https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441
[5]: https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160
