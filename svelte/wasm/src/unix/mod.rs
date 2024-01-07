pub mod cat;
pub mod ls;
pub mod uname;

// Model for arg parsing:
// &bool Option<&str> Option<&str>
// &bool points to the bool in the structure
// first option is longname
// second option is shortname
// None if one is not present
// May need to set up as a macro
