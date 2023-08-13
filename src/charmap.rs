
pub struct CharMap<T> {
    bucket_array: [u32; 4_294_967_296],
    size: u32,
}