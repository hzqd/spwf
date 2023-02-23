use std::marker::PhantomData;
use aoko::{no_std::functions::ext::AnyExt, standard::functions::ext::StdAnyExt};

struct Id<'a>(PhantomData<&'a ()>);

fn main() {
    Id.type_size().echo();
}