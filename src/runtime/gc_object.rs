
struct GcObject<T: Sized> {
  marked: bool,
  value: T
}

struct BaseObject {
  vtable: usize
}
