//! The change log

/// Release 0.1.3 (2021-04-10)
///
/// Add pub use statements for DuskError and InterplugRequest enums
/// for easier accessibility. Also change name of requests a little
///
/// Swithch all strs for name and string representations for Strings
/// as they proved to be much more convenient to work with
///
/// Make function names of functions provided by autoimplemented
/// get_function_list (and names of types, provided by 
/// get_type_list) resemble actual identificators by which these
/// functions would be called, with complete path to the function
/// included in the name
///
pub mod r0_1_3 {}

/// Release 0.1.2 (2021-04-09)
///
/// Add Debug trait implementations for every single defined structure 
/// and add a rule to throw error on missing_debug_implementation
/// lint so that all the structures created in the future will always
/// implement Debug trait
///
/// Implement all needed ordering functions for the Version structure
/// so that the plugin versions were easily comparable
///
pub mod r0_1_2 {}

/// Release 0.1.1 (2021-04-09)
///
/// Add a changelog.
///
/// Update all documentation and add some more stylistic changes.
///
/// Make more lints give errors on compile and make all warnings lead
/// to compile error, as it is unacceptable for a library to have
/// any warnings.
///
/// Get rid of all unwraps, switching to error handling via match
/// statements.
///
pub mod r0_1_1 {}
