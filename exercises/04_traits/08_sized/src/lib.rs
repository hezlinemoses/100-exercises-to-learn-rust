pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.

    // why is str's size not known in compile time tho?
    // we know that str is a fat pointer, so it have 
    // a pointer and the length, so its size is 16bytes..
    
    std::mem::size_of::<&str>();
}
