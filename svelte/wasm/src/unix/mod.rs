pub mod cat;
pub mod cd;
pub mod cp;
pub mod ln;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;
pub mod touch;
pub mod uname;
pub mod echo;

// Model for arg parsing:
// &bool Option<&str> Option<&str>
// &bool points to the bool in the structure
// first option is longname
// second option is shortname
// None if one is not present
// May need to set up as a macro
