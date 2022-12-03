fn main() {
    //Determine which operating system is being used
    if cfg!(windows)
    {
        println!("Windows");
    }
    else if cfg!(unix)
    {
        if cfg!(target_os = "macos") 
        {
            println!("Mac");
        }
        else 
        {
            println!("Linux");
        }
    }
}
