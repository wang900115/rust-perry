#![allow(unused)]


trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

fn main() {

}